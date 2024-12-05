use std::{
    env,
    io::{self, Write},
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

fn parse_int(input: String, err_msg: &str) -> usize {
    input.trim().parse::<usize>().expect(err_msg)
}

fn parse_input_type(in_type: usize) -> InputType {
    match in_type {
        1 => InputType::Input,
        2 => InputType::Test,
        _ => InputType::Input,
    }
}

fn prompt(prompt: &str, err_msg: &str) -> usize {
    print(prompt);
    parse_int(read_line(), err_msg)
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

    let (day, part, input_type) = match args.as_slice() {
        [_, d, p, i] => (
            parse_int(d.to_string(), ""),
            parse_int(p.to_string(), ""),
            parse_input_type(parse_int(i.to_string(), "")),
        ),
        _ => cli(),
    };

    let input = read_puzzle_input(day, input_type);

    let result = match (day, part) {
        (1, 1) => day1::part1::solution(&input),
        _ => panic!("AAAAAAAAAAAA"),
    };

    print!("{result}")
}
