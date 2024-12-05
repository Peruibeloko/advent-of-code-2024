use std::{
    env,
    io::{self, Write},
    process::exit,
};
use utils::{read_puzzle_input, InputType};

mod utils;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn read_line() -> String {
    io::read_to_string(io::stdin()).expect("Failed to read line")
}

fn print(string: &str) {
    print!("{string}");
    let _ = io::stdout().flush();
}

fn parse_int(input: String) -> Option<usize> {
    input.trim().parse::<usize>().ok()
}

fn parse_input_type_argument(in_type: String) -> InputType {
    match in_type.as_str() {
        "1" => InputType::Input,
        "2" => InputType::Test,
        _ => {
            println!("Argumento não reconhecido: {in_type}");
            exit(-1);
        }
    }
}

fn parse_int_argument(arg: String) -> usize {
    match arg.parse::<usize>() {
        Ok(val) => val,
        Err(_) => {
            println!("Argumento não reconhecido: {arg}");
            exit(-1);
        }
    }
}

fn prompt(prompt: &str, err_msg: &str) -> usize {
    loop {
        print(prompt);
        match parse_int(read_line()) {
            Some(val) => return val,
            None => println!("{err_msg}"),
        }
    }
}

fn cli() -> (usize, usize, InputType) {
    let day = prompt("Qual dia? ", "Dia informado não é um número");
    let part = prompt("Parte 1 ou 2? ", "Parte informada não é um número");
    let input_type = prompt("[1] Input\n[2] Teste\n> ", "Tipo informado não é um número");

    let input_type = match input_type {
        1 => InputType::Input,
        2 => InputType::Test,
        _ => InputType::Input,
    };

    (day, part, input_type)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let (day, part, input_type): (usize, usize, InputType) = match args.as_slice() {
        [_, d, p, i] => (
            parse_int_argument(d.to_string()),
            parse_int_argument(p.to_string()),
            parse_input_type_argument(i.to_string()),
        ),
        _ => cli(),
    };

    let input = read_puzzle_input(day, input_type);

    let result = match (day, part) {
        (1, 1) => day1::part1::solution(&input),
        (1, 2) => day1::part2::solution(&input),
        (2, 1) => day2::part1::solution(&input),
        (2, 2) => day2::part2::solution(&input),
        (3, 1) => day3::part1::solution(&input),
        (3, 2) => day3::part2::solution(&input),
        (4, 1) => day4::part1::solution(&input),
        (4, 2) => day4::part2::solution(&input),
        (5, 1) => day5::part1::solution(&input),
        (5, 2) => day5::part2::solution(&input),
        (6, 1) => day6::part1::solution(&input),
        (6, 2) => day6::part2::solution(&input),
        (7, 1) => day7::part1::solution(&input),
        (7, 2) => day7::part2::solution(&input),
        (8, 1) => day8::part1::solution(&input),
        (8, 2) => day8::part2::solution(&input),
        (9, 1) => day9::part1::solution(&input),
        (9, 2) => day9::part2::solution(&input),
        (10, 1) => day10::part1::solution(&input),
        (10, 2) => day10::part2::solution(&input),
        (11, 1) => day11::part1::solution(&input),
        (11, 2) => day11::part2::solution(&input),
        (12, 1) => day12::part1::solution(&input),
        (12, 2) => day12::part2::solution(&input),
        (13, 1) => day13::part1::solution(&input),
        (13, 2) => day13::part2::solution(&input),
        (14, 1) => day14::part1::solution(&input),
        (14, 2) => day14::part2::solution(&input),
        (15, 1) => day15::part1::solution(&input),
        (15, 2) => day15::part2::solution(&input),
        (16, 1) => day16::part1::solution(&input),
        (16, 2) => day16::part2::solution(&input),
        (17, 1) => day17::part1::solution(&input),
        (17, 2) => day17::part2::solution(&input),
        (18, 1) => day18::part1::solution(&input),
        (18, 2) => day18::part2::solution(&input),
        (19, 1) => day19::part1::solution(&input),
        (19, 2) => day19::part2::solution(&input),
        (20, 1) => day20::part1::solution(&input),
        (20, 2) => day20::part2::solution(&input),
        (21, 1) => day21::part1::solution(&input),
        (21, 2) => day21::part2::solution(&input),
        (22, 1) => day22::part1::solution(&input),
        (22, 2) => day22::part2::solution(&input),
        (23, 1) => day23::part1::solution(&input),
        (23, 2) => day23::part2::solution(&input),
        (24, 1) => day24::part1::solution(&input),
        (24, 2) => day24::part2::solution(&input),
        (25, 1) => day25::part1::solution(&input),
        (25, 2) => day25::part2::solution(&input),
        _ => {
            println!("Solução para o dia {day} parte {part} não foi encontrada.");
            exit(-1)
        }
    };

    print!("{result}")
}
