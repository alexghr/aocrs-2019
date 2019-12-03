extern crate aocrs;

use aocrs::day1;
use aocrs::day2;

use std::env;
use std::fs;

fn main() {
    let config = parse_args();

    match config.day {
        1 => run_day_1(&config),
        2 => run_day_2(&config),
        _ => panic!("Unrecognized day"),
    }
}

fn read_input_data(filename: &str) -> Vec<u32> {
    fs::read_to_string(filename)
        .unwrap()
        .split_whitespace()
        .map(|val| val.parse().unwrap())
        .collect()
}

fn read_intcode_src(filename: &str) -> Vec<i32> {
    fs::read_to_string(filename)
        .unwrap()
        // ignore whitespace
        .split_whitespace()
        .flat_map(|line| {
            line.split(',').flat_map(|val| {
                let word = val.parse::<i32>();
                match word {
                    Ok(val) => vec![val],
                    Err(_) => {
                        println!("[{}] did not product a good value", val);
                        vec![]
                    }
                }
            })
        })
        .collect()
}

fn run_day_2(config: &Config) {
    let mut intcode_src = read_intcode_src(&config.input_filename);

    // the challenge says we have to do this
    intcode_src[1] = 12;
    intcode_src[2] = 2;

    let mut program = day2::IntcodeInterpreter::new(intcode_src);
    program.execute();

    println!("value at position 0: {}", program.get(0));
}

fn run_day_1(config: &Config) {
    let input_data = read_input_data(&config.input_filename);

    let total_fuel = day1::calculate_total_fuel(&input_data);
    println!("total fuel needed: {}", total_fuel);
}

fn parse_args() -> Config {
    let options: Vec<String> = env::args().collect();
    Config {
        day: options[1].parse().unwrap(),
        input_filename: String::clone(&options[2]),
    }
}

struct Config {
    input_filename: String,
    day: u32,
}
