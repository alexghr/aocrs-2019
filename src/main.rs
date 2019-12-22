extern crate aocrs;

use aocrs::day1;
use aocrs::day2;
use aocrs::day3;
use aocrs::day4;

use std::env;
use std::fs;

fn main() {
    let config = parse_args();

    match config.day {
        1 => run_day_1(&config),
        2 => run_day_2(&config),
        3 => run_day_3(&config),
        4 => run_day_4(&config),
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
    let intcode_src = read_intcode_src(&config.input_filename);
    day2::run(&intcode_src);
}

fn run_day_1(config: &Config) {
    let input_data = read_input_data(&config.input_filename);

    let total_fuel = day1::calculate_total_fuel(&input_data);
    println!("total fuel needed: {}", total_fuel);
}

fn run_day_3(config: &Config) {
    let input = read_file(&config.input_filename);
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
