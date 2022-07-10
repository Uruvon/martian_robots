use crate::{robot::Robot, utils::make_grid};

pub struct Mars {
    pub planetary_grid: Vec<Vec<char>>,
    pub robots: Vec<Robot>,
}

impl Mars {
    pub fn new(y_size: usize, x_size: usize) -> Self {
        return Mars {
            planetary_grid: make_grid(y_size, x_size),
            robots: Vec::new(),
        };
    }
}
