use std::collections::{Bound, HashMap};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::{Range, RangeBounds};

use crate::mine_sweeper_gui::gui_config::symbol::Symbol;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Grid {
  size: (usize, usize),
  symbols: HashMap<(usize, usize), Symbol>,
}

impl Grid {
  pub fn new(width: usize, height: usize) -> Result<Self, InvalidDimensionError> {
    if width == 0 {
      Err(InvalidDimensionError::new("width"))?
    }
    if height == 0 {
      Err(InvalidDimensionError::new("height"))?
    }

    Ok(Self {
      size: (width, height),
      symbols: Default::default(),
    })
  }

  pub fn set(&mut self, x: usize, y: usize, symbol: Symbol) -> Result<(), InvalidCoordinatesError> {
    let coordinates = (x, y);
    if x >= self.size.0 {
      Err(InvalidCoordinatesError {
        grid_size: self.size,
        coordinates,
      })?
    }
    if y >= self.size.1 {
      Err(InvalidCoordinatesError {
        grid_size: self.size,
        coordinates,
      })?
    }

    self.symbols.insert(coordinates, symbol);
    Ok(())
  }

  pub fn copy_of_range<W, H>(&self, columns: W, rows: H) -> Result<Self, InvalidDimensionError>
  where
    W: RangeBounds<usize>,
    H: RangeBounds<usize>,
  {
    let width_bound = Self::bound_to_range(columns, self.size.0);
    let height_bound = Self::bound_to_range(rows, self.size.1);

    let width_offset = width_bound.start;
    let height_offset = height_bound.start;

    let mut copy = Self::new(width_bound.len(), height_bound.len())?;

    copy.symbols = self
      .symbols
      .iter()
      .filter(|((x, y), _)| width_bound.contains(x) && height_bound.contains(y))
      .map(|((x, y), symbol)| ((x - width_offset, y - height_offset), *symbol))
      .collect();

    Ok(copy)
  }

  fn bound_to_range<B: RangeBounds<usize>>(bound: B, upper_unbound: usize) -> Range<usize> {
    let start = match bound.start_bound() {
      Bound::Included(bound) => *bound,
      Bound::Excluded(bound) => bound + 1,
      Bound::Unbounded => 0,
    };
    let end = match bound.end_bound() {
      Bound::Included(bound) => bound + 1,
      Bound::Excluded(bound) => *bound,
      Bound::Unbounded => upper_unbound,
    };
    start..end
  }
}

#[derive(Debug)]
pub struct InvalidDimensionError {
  dimension: String,
}

impl InvalidDimensionError {
  fn new<S: ToString>(dimension: S) -> Self {
    Self {
      dimension: dimension.to_string(),
    }
  }
}

impl Display for InvalidDimensionError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Invalid dimension: {} can't be 0", self.dimension)
  }
}

impl Error for InvalidDimensionError {}

#[derive(Debug)]
pub struct InvalidCoordinatesError {
  grid_size: (usize, usize),
  coordinates: (usize, usize),
}

impl Display for InvalidCoordinatesError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "Invalid coordinates: ({}, {}) not in grid ({} x {})",
      self.coordinates.0, self.coordinates.1, self.grid_size.0, self.grid_size.1
    )
  }
}

impl Error for InvalidCoordinatesError {}
