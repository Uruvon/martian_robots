/**
Create a Martian Grid; with a maximu mside length of 50.

Default Character is 'O', and empty, unexplored space
*/
pub fn make_grid(side_length: usize) -> Vec<Vec<char>> {
  if side_length > 50 {
      return vec![vec!['O'; 50]; 50]
  }

  return vec![vec!['O'; side_length]; side_length]
}