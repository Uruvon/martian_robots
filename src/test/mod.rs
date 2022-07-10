use crate::{utils::make_grid, mars::Mars};

#[test]
pub fn test_make_grid() {
  let side_25 = make_grid(25);
  assert_eq!(side_25.len(), 25);
  assert_eq!(side_25[0].len(), 25);
  assert_eq!(side_25[0][5], 'O');

  let side_over = make_grid(55);
  assert_eq!(side_over.len(), 50);
  assert_eq!(side_over[0].len(), 50);
  assert_eq!(side_over[25][5], 'O');
}

#[test]
pub fn test_new_mars() {
  let mars = Mars::new(28);

  assert!(mars.robots.len() == 0);
  assert!(mars.planetary_grid.len() == 28);
  assert!(mars.planetary_grid[0].len() == 28);
  assert_eq!(mars.planetary_grid[3][15], 'O');
}