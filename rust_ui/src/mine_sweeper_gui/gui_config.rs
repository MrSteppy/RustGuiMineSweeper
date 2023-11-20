use std::collections::HashMap;

use crate::mine_sweeper_gui::gui_config::button::{Button, ButtonSection};
use crate::mine_sweeper_gui::gui_config::grid::Grid;

pub mod button;
pub mod color;
pub mod grid;
pub mod symbol;

#[derive(Clone, Debug)]
pub struct GuiConfig {
  pub buttons: HashMap<ButtonSection, Vec<Button>>,
  pub grid: Grid,
}
