use crate::{robot::Robot, utils::make_grid};

pub struct Mars {
  pub planetary_grid: Vec<Vec<char>>,
  pub robots: Vec<Robot>,
}

impl Mars {
  pub fn new (side_length: usize) -> Self {
    return Mars {
      planetary_grid: make_grid(side_length),
      robots: Vec::new(),
    }
  }
}