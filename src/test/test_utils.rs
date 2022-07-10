use crate::utils::make_grid;

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