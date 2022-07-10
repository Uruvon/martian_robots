use std::process::Command;

#[test]
pub fn test_input_right() {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "cargo run ./resources/basic_input_right.txt"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("cargo run ./resources/basic_input_right.txt")
            .output()
            .expect("failed to execute process")
    };

    assert_eq!(String::from_utf8(output.stdout).unwrap(), "1 1 E \n")
}

#[test]
pub fn test_input_left() {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "cargo run ./resources/basic_input_left.txt"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("cargo run ./resources/basic_input_left.txt")
            .output()
            .expect("failed to execute process")
    };

    assert_eq!(String::from_utf8(output.stdout).unwrap(), "1 1 E \n")
}

#[test]
pub fn test_input_lost() {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "cargo run ./resources/lost_movement.txt"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("cargo run ./resources/lost_movement.txt")
            .output()
            .expect("failed to execute process")
    };

    assert_eq!(String::from_utf8(output.stdout).unwrap(), "3 3 N LOST \n")
}

#[test]
pub fn test_input_multi_robot() {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "cargo run ./resources/multi_robot.txt"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("cargo run ./resources/multi_robot.txt")
            .output()
            .expect("failed to execute process")
    };

    assert_eq!(String::from_utf8(output.stdout).unwrap(), "1 1 E \n3 3 N LOST \n2 3 S \n")
}