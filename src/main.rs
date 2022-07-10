use std::fs;

use chrono::Utc;
use tracing::instrument;

use crate::utils::load_simulation_file;

#[cfg(test)]
mod test;

mod mars;
mod robot;
mod utils;

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

        print!("{} | Simulation {:?} | ", Utc::now(), i);
        let mut mars = load_simulation_file(path_str);
        mars.simulate();

        println!();
    }
}
