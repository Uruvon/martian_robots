use crate::{mars::Mars, utils::load_simulation_file};

#[test]
pub fn test_new_mars() {
    let mars = Mars::new(28, 28);

    assert!(mars.robots.len() == 0);
    assert!(mars.planetary_grid.len() == 29);
    assert!(mars.planetary_grid[0].len() == 29);
    assert_eq!(mars.planetary_grid[3][15], 'O');
}

#[test]
#[should_panic]
pub fn test_mars_too_short() {
    let _mars = load_simulation_file("resources/test/test_input_short.txt");
}

#[test]
#[should_panic]
pub fn test_bad_mars() {
    let _mars = load_simulation_file("resources/test/test_input_bad_world.txt");
}

#[test]
#[should_panic]
pub fn test_bad_mars_x() {
    let _mars = load_simulation_file("resources/test/test_input_bad_world_x.txt");
}

#[test]
#[should_panic]
pub fn test_bad_mars_y() {
    let _mars = load_simulation_file("resources/test/test_input_bad_world_y.txt");
}
