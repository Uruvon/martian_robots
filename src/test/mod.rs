use crate::mars::Mars;

pub mod test_utils;

#[test]
pub fn test_new_mars() {
  let mars = Mars::new(28, 28);

  assert!(mars.robots.len() == 0);
  assert!(mars.planetary_grid.len() == 28);
  assert!(mars.planetary_grid[0].len() == 28);
  assert_eq!(mars.planetary_grid[3][15], 'O');
}