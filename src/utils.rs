use std::{fs, path::Path};
pub enum InputType {
    Input,
    Test,
}

pub fn read_puzzle_input(day: u8, input_type: InputType) -> String {
    let day_folder = format!("day{day}");

    let path = match input_type {
        InputType::Input => Path::new(".").join("src").join(&day_folder).join("input.txt"),
        InputType::Test => Path::new(".").join("src").join(&day_folder).join("test.txt"),
    };

    fs::read_to_string(path).expect("Puzzle input not present")
}
