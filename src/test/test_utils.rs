use crate::utils::{make_grid, load_simulation_file};

#[test]
pub fn test_make_grid_square() {
  let side_25 = make_grid(25, 25);
  assert_eq!(side_25.len(), 25);
  assert_eq!(side_25[0].len(), 25);
  assert_eq!(side_25[0][5], 'O');
}

#[test]
pub fn test_make_grid_rectangle() {
  let side_25 = make_grid(25, 15);
  assert_eq!(side_25.len(), 25);
  assert_eq!(side_25[0].len(), 15);
  assert_eq!(side_25[0][5], 'O');
}

#[test]
pub fn test_make_grid_oversized_y() {
  let side_25 = make_grid(55, 25);
  assert_eq!(side_25.len(), 50);
  assert_eq!(side_25[0].len(), 25);
  assert_eq!(side_25[0][5], 'O');
}

#[test]
pub fn test_make_grid_oversized_x() {
  let side_25 = make_grid(25, 55);
  assert_eq!(side_25.len(), 25);
  assert_eq!(side_25[0].len(), 50);
  assert_eq!(side_25[0][5], 'O');
}






#[test]
pub fn test_load_simulation_simple() {
  let mars = load_simulation_file("resources/test/test_input_simple.txt");
  assert!(mars.planetary_grid.len() == 3);
  assert!(mars.planetary_grid[0].len() == 5);
  assert!(mars.robots.len() == 1);
}

#[test]
#[should_panic]
pub fn test_load_simulation_too_short() {
  let mars = load_simulation_file("resources/test/test_input_short.txt");
}

#[test]
#[should_panic]
pub fn test_load_simulation_bad_world() {
  let mars = load_simulation_file("resources/test/test_input_bad_world.txt");
}

#[test]
#[should_panic]
pub fn test_load_simulation_bad_world_x() {
  let mars = load_simulation_file("resources/test/test_input_bad_world_x.txt");
}

#[test]
#[should_panic]
pub fn test_load_simulation_bad_world_y() {
  let mars = load_simulation_file("resources/test/test_input_bad_world_y.txt");
}