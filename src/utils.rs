use std::fs;
use chrono::Utc;

use tracing::{instrument, error};

use crate::{mars::Mars, VALID_DIRECTIONS, robot::Robot, VALID_COMMANDS};

/**
Create a Martian Grid; with a maximum side length of 50.

Default Character is 'O', and empty,&& unexplored space
*/
#[instrument]
pub fn make_grid(y_size: usize, x_size: usize) -> Vec<Vec<char>> {
	if x_size == 0 {
		panic!("Cannot make a grid with no X size!")
	}

	if y_size == 0 {
		panic!("Cannot make a grid with no Y size!")
	}

    let y = if y_size > 50 { 50 } else { y_size };
    let x = if x_size > 50 { 50 } else { x_size };

    return vec![vec!['O'; x]; y];
}

#[instrument]
pub fn load_simulation_file(filename: &'static str) -> Mars {
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
				error!("{} | malformed input, world configuration has more than two input groups", Utc::now());
				panic!();
			}

			let x_size = usize::from_str_radix(world_configuration[0], 10).unwrap_or_else(|e| {
				error!("{} | malformed input, world configuration x not a valid integer | {:?}", Utc::now(), e);
				panic!();
			});
			let y_size = usize::from_str_radix(world_configuration[1], 10).unwrap_or_else(|e| {
				error!("{} | malformed input, world configuration y not a valid integer | {:?}", Utc::now(), e);
				panic!();
			});
			
			let mut mars = Mars::new(y_size, x_size);

			for idx in (1..lines.len()).step_by(2) {
				let robot_seed = lines[idx];
				let robot_commands = lines[idx + 1];

				// TODO: Regex would be much more robust here; but given the requirements explicitly state
				// Whitespace input separation, i'll use this for now.
				let robot_seed_split: Vec<&str> = robot_seed.split(" ").collect();

				let x_coord = usize::from_str_radix(robot_seed_split[0], 10).unwrap_or_else(|e| {
					error!("{} | malformed input, robot x coord not a valid integer | {:?}", Utc::now(), e);
					panic!();
				});

				let y_coord = usize::from_str_radix(robot_seed_split[1], 10).unwrap_or_else(|e| {
					error!("{} | malformed input, robot y coord  not a valid integer | {:?}", Utc::now(), e);
					panic!();
				});

				let direction_str = robot_seed_split[2];
				if direction_str.len() > 1 || direction_str.len() == 0 {
					error!("{} | malformed input, robot direction not a valid direction | {:?}", Utc::now(), direction_str);
					panic!();
				}

				let direction: char = direction_str.chars().next().unwrap_or_else(|| {
					error!("{} | malformed input, robot direction  not a valid direction | {:?}", Utc::now(), direction_str);
					panic!();
				});

				if !VALID_DIRECTIONS.contains(&direction) {
					error!("{} | malformed input, robot direction not a valid direction | {:?}", Utc::now(), direction);
					panic!();
				}

				let invalid_command_chars: Vec<char> = robot_commands.chars().filter(|c| { return !VALID_COMMANDS.contains(c); }).collect();
				if !invalid_command_chars.len() == 0 {
					error!("{} | malformed input, robot commands contains invalid command characters | {:?}", Utc::now(), invalid_command_chars);
					panic!();
				}

				let command_queue: Vec<char> = robot_commands.chars().collect();
				if command_queue.len() > 100 || command_queue.len() == 0 {
					error!("{} | malformed input, robot command queue is an invalid length| {:?}", Utc::now(), command_queue.len());
					panic!();
				}
				let robot = Robot::new(
					[x_coord, y_coord],
					command_queue,
					direction
				);

				mars.robots.push(robot);
			}

			return mars
		},
		Err(e) => { 
			error!("{} | could not load file contents | {:?} | {:?}", Utc::now(), filename, e);
			panic!();
		}
	}
}