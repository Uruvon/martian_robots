use tracing::instrument;

use crate::{utils::load_simulation_file};

#[cfg(test)]
mod test;

mod mars;
mod robot;
mod utils;

const VALID_COMMANDS: [char; 3] = ['L', 'R', 'F'];
const VALID_DIRECTIONS: [char; 4] = ['N', 'E', 'S', 'W'];
const FILENAME: &'static str = "resources/basic_input_right.txt";

#[instrument]
fn main() {
    tracing_subscriber::fmt::init();

    let mut mars = load_simulation_file(FILENAME);
    mars.simulate();
}
