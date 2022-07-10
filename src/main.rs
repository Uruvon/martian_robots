use tracing::instrument;

use crate::{mars::Mars, utils::load_simulation_file};

#[cfg(test)]
mod test;

mod mars;
mod robot;
mod utils;

const FILENAME: &'static str = "resources/basic_input.txt";

#[instrument]
fn main() {
    load_simulation_file(FILENAME);
}
