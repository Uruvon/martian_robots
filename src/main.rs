use std::fs;

use chrono::Utc;
use tracing::instrument;

use crate::utils::load_simulation_file;

#[cfg(test)]
mod test;

mod mars;
mod robot;
mod utils;

// These constants can be extended to allow for additional commands at a later stage

// If i were to have more time, i would load both these constants from configuration files
// Allowing expansions without needed to recompile.
// However, since the logic for handling these operations is currently hard coded, a re-compile would be needed anyway.

// In future, these operations could pre programmed to have an opcode & opeffect, like this:
// { op-code: 'L', op-mutation: '-90', op-type: 'rotation' } for an example rotation,
// { op-code: 'F', op-mutation: '1', op-type: 'translation' } for an example translation.

// These operations could be parsed by a small parser written by the engineer; allowing 
// future expansions of the control system without needing to recompile each time a change
// is made to the control structure of the robot.
const VALID_COMMANDS: [char; 3] = ['L', 'R', 'F'];
const VALID_DIRECTIONS: [char; 4] = ['N', 'E', 'S', 'W'];

#[instrument]
fn main() {
    tracing_subscriber::fmt::init();

    let paths = fs::read_dir("./resources").unwrap();

    for (i, path) in paths
        // Skip all non-files in target directory
        .filter(|p| p.as_ref().unwrap().metadata().unwrap().is_file())
        .enumerate()
    {
        let path = path.unwrap().path();
        let path_str = path.to_str().unwrap();

        println!("{} | Simulation {:?} | ", Utc::now(), i);
        let mut mars = load_simulation_file(path_str);
        mars.simulate();
    }
}
