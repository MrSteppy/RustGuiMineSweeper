#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: u8,
}

impl Color {
  pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
    Color { r, g, b, a }
  }

  pub fn rgb(r: u8, g: u8, b: u8) -> Self {
    Self::rgba(r, g, b, 255)
  }
}

impl Default for Color {
  fn default() -> Self {
    Self::rgb(0, 0, 0)
  }
}

impl From<(u8, u8, u8, u8)> for Color {
  fn from(value: (u8, u8, u8, u8)) -> Self {
    Self::rgba(value.0, value.1, value.2, value.3)
  }
}

impl From<(u8, u8, u8)> for Color {
  fn from(value: (u8, u8, u8)) -> Self {
    Self::rgb(value.0, value.1, value.2)
  }
}
