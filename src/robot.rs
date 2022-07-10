/// A Struct representing a Martian exploratory robot.
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