use chrono::Utc;
use tracing::{info, instrument, warn};

use crate::{
    robot::Robot,
    utils::{make_grid, valid_bounded_move},
};

#[derive(Debug, Clone)]
pub struct Mars {
    pub planetary_grid: Vec<Vec<char>>,
    pub robots: Vec<Robot>,
}

impl Mars {
    #[instrument]
    pub fn new(y_size: i8, x_size: i8) -> Self {
        return Mars {
            planetary_grid: make_grid(y_size, x_size),
            robots: Vec::new(),
        };
    }

    /// Simulate all robots in a given martian simulation;
    /// updating the planetary grid as robots become lost
    #[instrument]
    pub fn simulate(&mut self) {
        let range = self.robots.len();

        for i in 0..range {
            Mars::execute_robot(&mut self.robots[i], &mut self.planetary_grid);
        }
    }

    #[instrument]
    fn execute_robot(robot: &mut Robot, planetary_grid: &mut Vec<Vec<char>>) {
        let max_x = planetary_grid[0].len() as i8;
        let max_y = planetary_grid.len() as i8;

        for command in robot.command_queue.iter() {
            // Exit the loop if simulation has stopped
            if !robot.is_simulating {
                break;
            }

            match command {
                'R' | 'L' => {
                    robot.direction = Robot::rotate(command, robot.direction);
                }
                'F' => {
                    // TODO: Check that this movement won't result in running out of the world
                    let new_coord = Robot::translate(&robot.direction, robot.coordinate);
                    let valid_move = valid_bounded_move(new_coord, max_x, max_y);

                    let current_cell =
                        planetary_grid[robot.coordinate[1] as usize][robot.coordinate[0] as usize];

                    let has_scent = current_cell == 'X';

                    if !has_scent {
                        if !valid_move {
                            planetary_grid[robot.coordinate[1] as usize]
                                [robot.coordinate[0] as usize] = 'X';
                            robot.is_simulating = false;
                            print!("LOST ");
                        }
                        robot.coordinate = new_coord;
                    }
                }
                _ => {
                    warn!(
                        "{} | error processing robot, invalid command | {:?}",
                        Utc::now(),
                        command
                    );
                }
            }
        }

        // If we exit the command loop and are still simulating, print the output
        if robot.is_simulating {
            let str_to_print = format!(
                "{:?} {:?} {:?} ",
                robot.coordinate[0], robot.coordinate[1], robot.direction
            )
            .replace("'", "");
            print!("{}", str_to_print);
        }
    }
}
