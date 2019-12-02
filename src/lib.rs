pub fn calculate_total_fuel(masses: &Vec<u32>) -> u32 {
  masses.iter().fold(0, |acc, &x| acc + calculate_fuel(x))
}

// https://adventofcode.com/2019/day/1
fn calculate_fuel(mass: u32) -> u32 {
  ((mass as f64) / 3.0 - 2.0) as u32
}

#[cfg(test)]
mod tests {
  use super::{calculate_fuel};

  #[test]
  fn test_calculate_fuel() {
    assert_eq!(calculate_fuel(12), 2);
    assert_eq!(calculate_fuel(14), 2);
    assert_eq!(calculate_fuel(1969), 654);
    assert_eq!(calculate_fuel(100756), 33583);
  }
}