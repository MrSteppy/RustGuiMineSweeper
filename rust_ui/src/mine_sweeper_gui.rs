#[no_mangle]
pub extern "C" fn mod_hello() {
  println!("Hello from module!")
}

pub enum Tile {
  Zero,
  One,
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Covered,
  Flagged,
}

impl Default for Tile {
  fn default() -> Self {
    Tile::Covered
  }
}
