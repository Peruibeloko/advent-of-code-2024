use std::io::{self, Write};
use utils::{read_puzzle_input, InputType};

mod utils;

mod day1;
// mod day2;
// mod day3;
// mod day4;
// mod day5;
// mod day6;
// mod day7;
// mod day8;
// mod day9;
// mod day10;
// mod day11;
// mod day12;
// mod day13;
// mod day14;
// mod day15;
// mod day16;
// mod day17;
// mod day18;
// mod day19;
// mod day20;
// mod day21;
// mod day22;
// mod day23;
// mod day24;
// mod day25;

fn main() {
    let mut day = String::new();
    let mut part = String::new();
    let mut input_type = String::new();

    print!("Qual dia? ");
    let _ = io::stdout().flush();

    io::stdin()
        .read_line(&mut day)
        .expect("Failed to read line");
    
    print!("Parte 1 ou 2? ");
    let _ = io::stdout().flush();

    io::stdin()
        .read_line(&mut part)
        .expect("Failed to read line");

    print!("[1] Input\n[2] Teste\n> ");
    let _ = io::stdout().flush();

    io::stdin()
        .read_line(&mut input_type)
        .expect("Failed to read line");

    let day = day
        .trim()
        .parse::<u8>()
        .expect("Dia informado não é um número");
    
    let part = part
        .trim()
        .parse::<u8>()
        .expect("Parte informada não é um número");
    
    let input_type = match input_type
        .trim()
        .parse::<u8>()
        .expect("Tipo informado não é um número") {
        1 => InputType::Input,
        2 => InputType::Test,
        _ => InputType::Input,
    };

    let input = read_puzzle_input(day, input_type);

    let result = match (day, part) {
        (1, 1) => day1::part1::solution(&input),
        _ => panic!("AAAAAAAAAAAA")
    };
     
    print!("{result}")
}
