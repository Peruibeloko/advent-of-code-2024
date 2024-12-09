use std::{cmp::Eq, collections::HashMap, fs, hash::Hash, path::Path};

pub enum InputType {
    Input,
    Test,
}

pub fn read_puzzle_input(day: usize, input_type: InputType) -> String {
    let day_folder = format!("day{day}");

    let path = match input_type {
        InputType::Input => Path::new(".")
            .join("src")
            .join(&day_folder)
            .join("input.txt"),
        InputType::Test => Path::new(".")
            .join("src")
            .join(&day_folder)
            .join("test.txt"),
    };

    fs::read_to_string(path).expect("Puzzle input not present")
}

pub fn frequencies<T: Eq + Hash>(arr: impl Iterator<Item = T>) -> HashMap<T, usize> {
    let mut freqs = HashMap::<T, usize>::new();

    for el in arr {
        match freqs.get(&el) {
            None => freqs.insert(el, 1),
            Some(val) => freqs.insert(el, val + 1),
        };
    }

    freqs
}

pub fn frequency_of<T: Eq>(arr: impl Iterator<Item = T>, subject: T) -> i32 {
    let mut count = None;

    for el in arr {
        if el == subject {
            count = match count {
                None => Some(1),
                Some(x) => Some(x + 1),
            };
        }
    }

    match count {
        None => 0,
        Some(x) => x,
    }
}