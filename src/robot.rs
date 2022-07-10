use chrono::Utc;
use tracing::{error, instrument, warn, trace, info};

use crate::{VALID_DIRECTIONS, VALID_COMMANDS};

/// A Struct representing a Martian exploratory robot.
#[derive(Debug, Clone)]
pub struct Robot {
    pub command_queue: Vec<char>,

    /// Position of this robot on a martian planetary grid.
    /// Of the form [x, y],
    /// where X & Y are both positive [usize]
    pub coordinate: [usize; 2],

    /// Direction of this robot.
    /// North is defined as Positive Y facing.
    /// Possible directions are:
    /// 'N', 'S', 'E', 'W'
    pub direction: char,
}

impl Robot {
    #[instrument]
    pub fn new_from_lines(robot_seed: &str, robot_commands: &str) -> Self {
        // TODO: Regex would be much more robust here; but given the requirements explicitly state
        // Whitespace input separation, i'll use this for now.
        let robot_seed_split: Vec<&str> = robot_seed.split(" ").collect();

        let x_coord = usize::from_str_radix(robot_seed_split[0], 10).unwrap_or_else(|e| {
            error!(
                "{} | malformed input, robot x coord not a valid integer | {:?}",
                Utc::now(),
                e
            );
            panic!();
        });

        let y_coord = usize::from_str_radix(robot_seed_split[1], 10).unwrap_or_else(|e| {
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
        };
    }

    #[instrument]
    pub fn rotate <'a> (input: &char, current: char) -> char {
        let cur_rot_idx = VALID_DIRECTIONS.iter().position(|&r| r == current).unwrap();

        match input {
            'L' => {
                let val = cur_rot_idx as i64 - 1_i64;

                if val < 0 {
                    return VALID_DIRECTIONS[VALID_DIRECTIONS.len() - 1]
                }

                let new_idx = val as usize % VALID_DIRECTIONS.len();
                return VALID_DIRECTIONS[new_idx as usize];
            },
            'R' => {
                let new_idx = (cur_rot_idx + 1_usize) % VALID_DIRECTIONS.len();
                return VALID_DIRECTIONS[new_idx];
            },
            _ => {
                warn!("{} | error processing robot, invalid rotation | {:?}", Utc::now(), input);
                return current
            }
        }
    }
}
