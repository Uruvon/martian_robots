use crate::utils::load_simulation_file;

#[test]
#[should_panic]
pub fn test_bad_robot_x() {
    let mars = load_simulation_file("resources/test/test_input_bad_robot_x.txt");
}

#[test]
#[should_panic]
pub fn test_bad_robot_y() {
    let mars = load_simulation_file("resources/test/test_input_bad_robot_y.txt");
}

#[test]
#[should_panic]
pub fn test_bad_robot_direction() {
    let mars = load_simulation_file("resources/test/test_input_bad_robot_direction.txt");
}

#[test]
#[should_panic]
pub fn test_bad_robot_command() {
    let mars = load_simulation_file("resources/test/test_input_bad_robot_command.txt");
}

#[test]
#[should_panic]
pub fn test_robot_command_too_long() {
    let mars = load_simulation_file("resources/test/test_input_robot_command_too_long.txt");
}
