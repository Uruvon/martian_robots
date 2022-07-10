use chrono::Utc;
use tracing::{error, instrument, warn};

use crate::{VALID_COMMANDS, VALID_DIRECTIONS};

/// A Struct representing a Martian exploratory robot.
#[derive(Debug, Clone)]
pub struct Robot {
    pub command_queue: Vec<char>,

    /// Position of this robot on a martian planetary grid.
    /// Of the form [x, y],
    /// where X & Y are both positive [i8]
    pub coordinate: [i8; 2],

    /// Direction of this robot.
    /// North is defined as Positive Y facing.
    /// Possible directions are:
    /// 'N', 'S', 'E', 'W'
    pub direction: char,

    /// If this robot should be simulated.
    ///
    /// Set to 'false' in the event that a robot falls off the planetary grid
    pub is_simulating: bool,
}

impl Robot {
    #[instrument]
    pub fn new_from_lines(robot_seed: &str, robot_commands: &str) -> Self {
        // TODO: Regex would be much more robust here; but given the requirements explicitly state
        // Whitespace input separation, i'll use this for now.
        let robot_seed_split: Vec<&str> = robot_seed.split(" ").collect();

        let x_coord = i8::from_str_radix(robot_seed_split[0], 10).unwrap_or_else(|e| {
            error!(
                "{} | malformed input, robot x coord not a valid integer | {:?}",
                Utc::now(),
                e
            );
            panic!();
        });

        let y_coord = i8::from_str_radix(robot_seed_split[1], 10).unwrap_or_else(|e| {
            error!(
                "{} | malformed input, robot y coord  not a valid integer | {:?}",
                Utc::now(),
                e
            );
            panic!();
        });

        let direction_str = robot_seed_split[2];
        if direction_str.len() > 1 || direction_str.len() == 0 {
            error!(
                "{} | malformed input, robot direction not a valid direction | {:?}",
                Utc::now(),
                direction_str
            );
            panic!();
        }

        let direction: char = direction_str.chars().next().unwrap_or_else(|| {
            error!(
                "{} | malformed input, robot direction  not a valid direction | {:?}",
                Utc::now(),
                direction_str
            );
            panic!();
        });

        if !VALID_DIRECTIONS.contains(&direction) {
            error!(
                "{} | malformed input, robot direction not a valid direction | {:?}",
                Utc::now(),
                direction
            );
            panic!();
        }

        let invalid_command_chars: Vec<char> = robot_commands
            .chars()
            .filter(|c| {
                return !VALID_COMMANDS.contains(c);
            })
            .collect();
        if !invalid_command_chars.len() == 0 {
            error!(
                "{} | malformed input, robot commands contains invalid command characters | {:?}",
                Utc::now(),
                invalid_command_chars
            );
            panic!();
        }

        let command_queue: Vec<char> = robot_commands.chars().collect();
        if command_queue.len() > 100 || command_queue.len() == 0 {
            error!(
                "{} | malformed input, robot command queue is an invalid length| {:?}",
                Utc::now(),
                command_queue.len()
            );
            panic!();
        }
        return Robot {
            coordinate: [x_coord, y_coord],
            command_queue,
            direction,
            is_simulating: true,
        };
    }

    #[instrument]
    pub fn rotate(input: &char, current: char) -> char {
        let cur_rot_idx = VALID_DIRECTIONS.iter().position(|&r| r == current).unwrap() as i8;

        match input {
            'L' => {
                let val = cur_rot_idx as i8 - 1_i8;

                if val < 0 {
                    return VALID_DIRECTIONS[VALID_DIRECTIONS.len() - 1];
                }

                let new_idx = val % VALID_DIRECTIONS.len() as i8;
                return VALID_DIRECTIONS[new_idx as usize];
            }
            'R' => {
                let new_idx = (cur_rot_idx + 1_i8) % VALID_DIRECTIONS.len() as i8;
                return VALID_DIRECTIONS[new_idx as usize];
            }
            _ => {
                warn!(
                    "{} | error processing robot, invalid rotation | {:?}",
                    Utc::now(),
                    input
                );
                return current;
            }
        }
    }

    #[instrument]
    pub fn translate(direction: &char, current: [i8; 2]) -> [i8; 2] {
        match direction {
            'N' => return [current[0], current[1] + 1_i8],
            'E' => return [current[0] + 1_i8, current[1]],
            'S' => return [current[0], current[1] - 1_i8],
            'W' => return [current[0] - 1_i8, current[1]],
            _ => {
                warn!(
                    "{} | error processing robot, invalid translation direction | {:?}",
                    Utc::now(),
                    direction
                );
                return current;
            }
        }
    }
}
