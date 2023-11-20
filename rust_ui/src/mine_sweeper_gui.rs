use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use winit::event_loop::EventLoop;

use action::Action;
use render_state::RenderState;

use crate::mine_sweeper_gui::gui_config::GuiConfig;

pub mod action;
pub mod gui_config;
mod render_state;

#[derive(Debug)]
pub struct MineSweeperGui {
  instruction_sender: Arc<Mutex<Option<Instruction>>>,
  last_update_id: Option<u8>,
  graphic_thread_join_handle: JoinHandle<()>,
  output_receiver: Receiver<Output>,
}

impl MineSweeperGui {
  pub fn new() -> Self {
    let instruction_sender: Arc<Mutex<Option<Instruction>>> = Default::default();
    let instruction_receiver = instruction_sender.clone();
    let (output_sender, output_receiver) = channel();
    let graphic_thread_join_handle = thread::spawn(move || {
      run_event_loop(instruction_receiver, output_sender);
    });

    Self {
      instruction_sender,
      last_update_id: Default::default(),
      graphic_thread_join_handle,
      output_receiver,
    }
  }

  fn send_instruction(&self, instruction: Instruction) {
    if let Ok(channel) = self.instruction_sender.lock() {
      *channel = Some(instruction);
    } //panics will be caught when fetching actions
  }

  pub fn configure(&mut self, config: GuiConfig) {
    let update_id = self
      .last_update_id
      .map(|id| id.wrapping_add(1))
      .unwrap_or_default();
    self.last_update_id = Some(update_id);
    self.send_instruction(Instruction::Configure { config, update_id })
  }

  pub fn close(&mut self) {
    self.send_instruction(Instruction::CloseWindow)
  }

  pub fn shut_down(self) {
    self.send_instruction(Instruction::ShutDown)
  }

  pub fn fetch_next_action(&mut self) -> Result<Option<Action>, GraphicError> {
    if self.graphic_thread_join_handle.is_finished() {
      if let Err(e) = self.graphic_thread_join_handle.join() {
        Err(GraphicError {
          detail_message: format!("graphic thread panicked: {:?}", e),
        })?
      }
    }

    loop {
      match self.output_receiver.try_recv() {
        Ok(output) => match output {
          Output::Action(action) => {
            if self.last_update_id.is_none() {
              return Ok(Some(action));
            }
          }
          Output::Error { detail_message } => return Err(GraphicError { detail_message }),
          Output::UpdateAcknowledgement { update_id } => {
            if let Some(expected_update_id) = &self.last_update_id {
              if update_id == *expected_update_id {
                self.last_update_id = None;
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

#[derive(Debug)]
enum Instruction {
  Configure { config: GuiConfig, update_id: u8 },
  CloseWindow,
  ShutDown,
}

#[derive(Debug)]
enum Output {
  Action(Action),
  Error { detail_message: String },
  UpdateAcknowledgement { update_id: u8 },
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

fn run_event_loop(
  instruction_receiver: Arc<Mutex<Option<Instruction>>>,
  output_sender: Sender<Output>,
) {
  let event_loop = EventLoop::new();
  let mut render_state: Option<RenderState> = None;
  event_loop.run(move |event, event_loop, control_flow| {
    control_flow.set_wait_timeout(Duration::from_millis(50)); //20 fps

    todo!("check for new instruction and process event")
  });
}
