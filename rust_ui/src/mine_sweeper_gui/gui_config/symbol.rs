#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub enum Symbol {
  #[default]
  Cover,
  Flag,
  Number(NumberSymbol),
  Bomb,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default, Ord, PartialOrd)]
pub struct NumberSymbol {
  value: u8,
}

impl TryFrom<u8> for NumberSymbol {
  type Error = ();

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    if value > 8 {
      return Err(());
    }

    Ok(NumberSymbol { value })
  }
}

impl From<NumberSymbol> for u8 {
  fn from(value: NumberSymbol) -> Self {
    return value.value;
  }
}
