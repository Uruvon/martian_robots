use chrono::Utc;
use std::fs;

use tracing::{error, instrument};

use crate::{mars::Mars, robot::Robot};

/**
Create a Martian Grid; with a maximum side length of 50.

Default Character is 'O', and empty,&& unexplored space
*/
#[instrument]
pub fn make_grid(y_size: i8, x_size: i8) -> Vec<Vec<char>> {
    if x_size == 0 {
        panic!("Cannot make a grid with no X size!")
    }

    if y_size == 0 {
        panic!("Cannot make a grid with no Y size!")
    }

    let y = if y_size >= 50 { 50 } else { y_size + 1 };
    let x = if x_size >= 50 { 50 } else { x_size + 1 };

    return vec![vec!['O'; x as usize]; y as usize];
}

#[instrument]
pub fn load_simulation_file(filename: &str) -> Mars {
    match fs::read_to_string(filename) {
        Ok(contents) => {
            let lines: Vec<&str> = contents.split("\n").collect();

            if lines.len() < 3 {
                error!("{} | malformed input, less than 3 lines - may not have a single robot or world configuration", Utc::now());
                panic!();
            }

            // TODO: Regex would be much more robust here; but given the requirements explicitly state
            // Whitespace input separation, i'll use this for now.
            let world_configuration: Vec<&str> = lines[0].split(" ").collect();
            if world_configuration.len() > 2 {
                error!(
                    "{} | malformed input, world configuration has more than two input groups",
                    Utc::now()
                );
                panic!();
            }

            let x_size = i8::from_str_radix(world_configuration[0], 10).unwrap_or_else(|e| {
                error!(
                    "{} | malformed input, world configuration x not a valid integer | {:?}",
                    Utc::now(),
                    e
                );
                panic!();
            });
            let y_size = i8::from_str_radix(world_configuration[1], 10).unwrap_or_else(|e| {
                error!(
                    "{} | malformed input, world configuration y not a valid integer | {:?}",
                    Utc::now(),
                    e
                );
                panic!();
            });

            let mut mars = Mars::new(y_size, x_size);

            let filtered_lines: Vec<&str> = lines.into_iter().filter(|l| !l.is_empty()).collect();
            for idx in (1..filtered_lines.len()).step_by(2) {
                mars.robots.push(Robot::new_from_lines(
                    filtered_lines[idx],
                    filtered_lines[idx + 1],
                ));
            }

            return mars;
        }
        Err(e) => {
            error!(
                "{} | could not load file contents | {:?} | {:?}",
                Utc::now(),
                filename,
                e
            );
            panic!();
        }
    }
}

#[instrument]
pub fn valid_bounded_move(coord: [i8; 2], max_x: i8, max_y: i8) -> bool {
    if coord[0] > max_x || coord[0] < 0 {
        return false;
    }

    if coord[1] > max_y || coord[1] < 0 {
        return false;
    }

    return true;
}
