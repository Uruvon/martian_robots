/**
Create a Martian Grid; with a maximum side length of 50.

Default Character is 'O', and empty, unexplored space
*/
pub fn make_grid(y_size: usize, x_size: usize) -> Vec<Vec<char>> {
    let y = if y_size > 50 { 50 } else { y_size };
    let x = if x_size > 50 { 50 } else { x_size };

    return vec![vec!['O'; x]; y];
}
