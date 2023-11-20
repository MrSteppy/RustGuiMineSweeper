#[derive(Debug, Clone)]
pub enum Action {
  CloseRequested,
  ButtonPress {
    button_id: String,
  },
  TileClick {
    x: usize,
    y: usize,
    ///alternate click is usually right click, but may also refer to other types of clicks
    alternate_click: bool,
  },
}
