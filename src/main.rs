use tracing::instrument;

use crate::utils::load_simulation_file;

#[cfg(test)]
mod test;

mod mars;
mod robot;
mod utils;

const VALID_COMMANDS: [char; 3] = ['L', 'R', 'F'];
const VALID_DIRECTIONS: [char; 4] = ['N', 'S', 'E', 'W'];
const FILENAME: &'static str = "resources/basic_input.txt";

#[instrument]
fn main() {
   let mars = load_simulation_file(FILENAME);
}
