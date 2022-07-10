use chrono::Utc;
use tracing::{warn, instrument, info};

use crate::{robot::Robot, utils::make_grid};

#[derive(Debug, Clone)]
pub struct Mars {
    pub planetary_grid: Vec<Vec<char>>,
    pub robots: Vec<Robot>,
}

#[instrument]
pub fn execute_robot(robot: &mut Robot, planetary_grid: &Vec<Vec<char>>) -> Option<([usize; 2], char)> {
    for command in robot.command_queue.iter() {
        match command {
            'R' | 'L' => {
                robot.direction = Robot::rotate(command, robot.direction);
                info!("{} | new direction: {:?}", Utc::now(), robot.direction);
            },
            'F' => {

            }
            _ => {
                warn!("{} | error processing robot, invalid command | {:?}", Utc::now(), command);
            }
        }
    }

    return None
}

impl Mars {
    #[instrument]
    pub fn new(y_size: usize, x_size: usize) -> Self {
        return Mars {
            planetary_grid: make_grid(y_size, x_size),
            robots: Vec::new(),
        };
    }

    /// Simulate all robots in a given martian simulation;
    /// updating the planetary grid as robots become lost
    #[instrument]
    pub fn simulate (&mut self) {
        let range = self.robots.len();

        for i in 0..range {
            let robot = &mut self.robots[i];

            let operation = execute_robot(robot, &self.planetary_grid);

            // Update the planetary grid, if a robot becomes lost
            if operation.is_some() {
                let (coords, mutation) = operation.unwrap();

                self.planetary_grid[coords[0]][coords[1]] = mutation;
            }
        }
    }
}
