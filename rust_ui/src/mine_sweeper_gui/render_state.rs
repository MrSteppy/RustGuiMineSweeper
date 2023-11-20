use wgpu::Surface;
use winit::event_loop::EventLoopWindowTarget;
use winit::window::Window;

use crate::mine_sweeper_gui::gui_config::GuiConfig;

const BUTTON_HEIGHT: u32 = 17;

pub(super) struct RenderState {
  pub button_hit_boxes: Vec<Rect>,
  surface: Surface,
  pub window: Window, //keep window declaration behind surface declaration for drop order -> Safety!
}

impl RenderState {
  pub fn new(config: &GuiConfig, event_loop: &EventLoopWindowTarget<()>) -> Self {
    todo!()
  }
}

pub struct Rect {
  pub x: i32,
  pub y: i32,
  pub width: u32,
  pub height: u32,
}

impl Rect {
  pub fn contains(&self, x: i32, y: i32) -> bool {
    self.x <= x
      && x <= self.x + self.width as i32
      && self.y <= y
      && y <= self.y + self.height as i32
  }
}

#[cfg(test)]
mod test_rect {
  use crate::mine_sweeper_gui::render_state::Rect;

  #[test]
  fn test_contains() {
    let rect = Rect {
      x: 2,
      y: 2,
      width: 3,
      height: 2,
    };
    assert!(rect.contains(2, 2));
    assert!(rect.contains(5, 4));
    assert!(rect.contains(3, 3));
    assert!(!rect.contains(3, 1));
    assert!(!rect.contains(1, 3));
    assert!(!rect.contains(3, 5));
    assert!(!rect.contains(6, 3));
  }
}
