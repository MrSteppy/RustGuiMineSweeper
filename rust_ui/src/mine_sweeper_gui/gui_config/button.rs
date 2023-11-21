use crate::mine_sweeper_gui::gui_config::color::Color;
use crate::mine_sweeper_gui::gui_config::symbol::Symbol;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
pub enum ButtonSection {
  #[default]
  Up,
  Down,
  OverGrid,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Button {
  pub id: String,
  pub color: Option<Color>,
  pub label: Vec<LabelPart>,
  pub click_able: bool,
}

impl Button {
  pub fn new<S: ToString, C: Into<Option<Color>>, L: Into<Vec<P>>, P: Into<LabelPart>>(
    id: S,
    color: C,
    label: L,
    click_able: bool,
  ) -> Self {
    Button {
      id: id.to_string(),
      color: color.into(),
      label: label.into().into_iter().map(|p| p.into()).collect(),
      click_able,
    }
  }

  pub fn add<P: Into<LabelPart>>(mut self, label_part: P) -> Self {
    self.label.push(label_part.into());
    self
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LabelPart {
  Text(String),
  Symbol(Symbol),
}
