extern crate aocrs;

use std::env;
use std::fs;

fn read_input_data(filename: &str) -> Vec<u32> {
    fs::read_to_string(filename)
        .unwrap()
        .split_whitespace()
        .map(|val| val.parse().unwrap())
        .collect()
}

fn main() {
    let config = parse_args();
    let input_data = read_input_data(&config.input_filename);

    let total_fuel = aocrs::calculate_total_fuel(&input_data);
    println!("total fuel needed: {}", total_fuel);
}

fn parse_args() -> Config {
    let options: Vec<String> = env::args().collect();
    Config {
        input_filename: String::clone(&options[1]),
    }
}

struct Config {
    input_filename: String,
}
