use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use winit::event::{ElementState, Event, MouseButton, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

use action::Action;
use render_state::RenderState;

use crate::mine_sweeper_gui::gui_config::GuiConfig;
use crate::mine_sweeper_gui::render_state::ClickableElement;

pub mod action;
pub mod gui_config;
mod render_state;

#[derive(Debug)]
pub struct MineSweeperGui {
  instruction_sender: Arc<Mutex<Option<Instruction>>>,
  expected_update_id: Option<u8>,
  graphic_thread_join_handle: Option<JoinHandle<()>>,
  output_receiver: Receiver<Output>,
}

impl MineSweeperGui {
  pub fn new() -> Self {
    let instruction_sender: Arc<Mutex<Option<Instruction>>> = Default::default();
    let instruction_receiver = instruction_sender.clone();
    let (output_sender, output_receiver) = channel();
    let join_handle = thread::spawn(move || {
      run_event_loop(instruction_receiver, output_sender, 20);
    });

    Self {
      instruction_sender,
      expected_update_id: None,
      graphic_thread_join_handle: Some(join_handle),
      output_receiver,
    }
  }

  fn send_instruction(&self, instruction: Instruction) {
    if let Ok(mut channel) = self.instruction_sender.lock() {
      *channel = Some(instruction);
    } //panics will be caught when fetching actions
  }

  pub fn configure(&mut self, config: GuiConfig) {
    let update_id = self
      .expected_update_id
      .map(|id| id.wrapping_add(1))
      .unwrap_or_default();
    self.expected_update_id = Some(update_id);
    self.send_instruction(Instruction::Configure { config, update_id })
  }

  pub fn close(&mut self) {
    self.send_instruction(Instruction::CloseWindow)
  }

  pub fn shut_down(self) {
    self.send_instruction(Instruction::ShutDown)
  }

  pub fn fetch_next_action(&mut self) -> Result<Option<Action>, GraphicError> {
    if let Some(join_handle) = &self.graphic_thread_join_handle {
      if join_handle.is_finished() {
        if let Err(e) = self.graphic_thread_join_handle.take().unwrap().join() {
          return Err(GraphicError {
            detail_message: format!("graphic thread panicked: {:?}", e),
          });
        }
      }
    }

    loop {
      match self.output_receiver.try_recv() {
        Ok(output) => match output {
          Output::Action {
            action,
            configuration_independent,
          } => {
            if configuration_independent || self.expected_update_id.is_none() {
              return Ok(Some(action));
            }
          }
          Output::Error { detail_message } => return Err(GraphicError { detail_message }),
          Output::UpdateAcknowledgement { update_id } => {
            if let Some(expected_update_id) = &self.expected_update_id {
              if update_id == *expected_update_id {
                self.expected_update_id = None;
              }
            } else {
              eprintln!("received update_id {} but didn't expect any", update_id)
            }
          }
        },
        Err(_) => return Ok(None),
      }
    }
  }
}

impl Drop for MineSweeperGui {
  fn drop(&mut self) {
    self.send_instruction(Instruction::ShutDown)
  }
}

#[derive(Debug)]
enum Instruction {
  Configure { config: GuiConfig, update_id: u8 },
  CloseWindow,
  ShutDown,
}

#[derive(Debug)]
enum Output {
  Action {
    action: Action,
    configuration_independent: bool,
  },
  Error {
    detail_message: String,
  },
  UpdateAcknowledgement {
    update_id: u8,
  },
}

#[derive(Debug)]
pub struct GraphicError {
  pub detail_message: String,
}

impl Display for GraphicError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "GraphicError: {}", self.detail_message)
  }
}

impl Error for GraphicError {}

fn run_event_loop<F: Into<Option<u32>>>(
  instruction_receiver: Arc<Mutex<Option<Instruction>>>,
  output_sender: Sender<Output>,
  default_fps: F,
) {
  let event_loop = EventLoop::new();
  let default_control_flow = {
    let mut flow = ControlFlow::Poll;
    if let Some(fps) = default_fps.into() {
      flow.set_wait_timeout(Duration::from_secs_f64(1.0 / fps as f64))
    }
    flow
  };
  let mut render_state: Option<(RenderState, GuiConfig)> = None;
  let receive_instruction = move || {
    instruction_receiver
      .lock()
      .ok()
      .and_then(|mut channel| channel.take())
  };
  let mut last_cursor_pos: Option<(usize, usize)> = None;
  event_loop.run(move |event, event_loop, control_flow| {
    *control_flow = default_control_flow;

    let mut send_output = |output| {
      if let Err(_) = output_sender.send(output) {
        eprintln!("gui handle has gone out of scope without shutting down graphic thread");
        control_flow.set_exit();
      }
    };

    match event {
      Event::WindowEvent { event, .. } => {
        if let Some((render_state, _)) = &render_state {
          match event {
            WindowEvent::CloseRequested => send_output(Output::Action {
              action: Action::CloseRequested,
              configuration_independent: true,
            }),
            WindowEvent::MouseInput {
              state: ElementState::Pressed,
              button: button @ (MouseButton::Left | MouseButton::Right),
              ..
            } => {
              if let Some((px, py)) = last_cursor_pos {
                if let Some(element) = render_state.element_at(px, py) {
                  send_output(Output::Action {
                    action: match element {
                      ClickableElement::GridTile {
                        coordinates: (x, y),
                      } => Action::TileClick {
                        x,
                        y,
                        alternate_click: button != MouseButton::Left,
                      },
                      ClickableElement::Button { id } => Action::ButtonPress { button_id: id },
                    },
                    configuration_independent: false,
                  })
                }
              }
            }
            WindowEvent::CursorMoved { position, .. } => {
              last_cursor_pos = Some((position.x as usize, position.y as usize));
            }
            WindowEvent::CursorLeft { .. } => {
              last_cursor_pos = None;
            }
            _ => {}
          }
        }
      }
      Event::MainEventsCleared => {
        //check for new instruction
        if let Some(new_instruction) = receive_instruction() {
          match new_instruction {
            Instruction::Configure {
              config: new_config,
              update_id,
            } => {
              if let Some((_, config)) = &mut render_state {
                *config = new_config;
              } else {
                match RenderState::new(&new_config, event_loop) {
                  Ok(state) => render_state = Some((state, new_config)),
                  Err(detail_message) => send_output(Output::Error { detail_message }),
                }
              }

              send_output(Output::UpdateAcknowledgement { update_id });
            }
            Instruction::CloseWindow => render_state = None,
            Instruction::ShutDown => control_flow.set_exit(),
          }
        }
      }
      Event::RedrawRequested(_) => {
        if let Some((render_state, config)) = &mut render_state {
          if let Err(detail_message) = render_state.render(config) {
            send_output(Output::Error {
              detail_message: format!("render failure: {}", detail_message),
            });
          }
        }
      }
      _ => {}
    }
  });
}
