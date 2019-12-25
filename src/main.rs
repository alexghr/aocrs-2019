extern crate aocrs;

use aocrs::day1;
use aocrs::day2;
use aocrs::day3;
use aocrs::day4;
use aocrs::day5;

use std::env;
use std::fs;

fn main() {
    let config = parse_args();

    match config.day {
        1 => run_day_1(&config),
        2 => run_day_2(&config),
        3 => run_day_3(&config),
        4 => run_day_4(&config),
        5 => run_day_5(&config),
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

fn read_file(filename: &str) -> Vec<String> {
    fs::read_to_string(&filename)
        .unwrap()
        .split('\n')
        .map(|val| String::from(val))
        .collect()
}

fn read_intcode_src(filename: &str) -> Vec<isize> {
    fs::read_to_string(filename)
        .unwrap()
        // ignore whitespace
        .split_whitespace()
        .flat_map(|line| {
            line.split(',').flat_map(|val| {
                let word = val.parse::<isize>();
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
    let intcode_src = read_intcode_src(config.input_filename.as_ref().unwrap());
    day2::run(&intcode_src);
}

fn run_day_1(config: &Config) {
    let input_data = read_input_data(config.input_filename.as_ref().unwrap());

    let total_fuel = day1::calculate_total_fuel(&input_data);
    println!("total fuel needed: {}", total_fuel);
}

fn run_day_3(config: &Config) {
    let input = read_file(config.input_filename.as_ref().unwrap());
    let wire_a = &input[0];
    let wire_b = &input[1];

    let distance = day3::run(&wire_a, &wire_b);
    match distance {
        Some(d) => println!("intersection point distance: {}", d),
        None => println!("Wires do not intersect"),
    }
}

fn run_day_4(_: &Config) {
    println!("matched: {}", day4::brute_force(231832, 767346));
}

fn run_day_5(_: &Config) {
    day5::part1();
}

fn parse_args() -> Config {
    let options: Vec<String> = env::args().collect();
    Config {
        day: options[1].parse().unwrap(),
        input_filename: if options.len() >= 3 {
            Some(String::clone(&options[2]))
        } else {
            None
        },
    }
}

struct Config {
    input_filename: Option<String>,
    day: u32,
}
