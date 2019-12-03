pub fn calculate_total_fuel(masses: &Vec<u32>) -> u32 {
  masses.iter().fold(0, |acc, &x| acc + calculate_fuel(x))
}

// https://adventofcode.com/2019/day/1#part2
fn calculate_fuel(mass: u32) -> u32 {
  let fuel_required = naive_calculate_fuel(mass);
  if (fuel_required > 0) {
    fuel_required + calculate_fuel(fuel_required)
  } else {
    fuel_required
  }
}

// https://adventofcode.com/2019/day/1#part1
fn naive_calculate_fuel(mass: u32) -> u32 {
  let fuel: f64 = (mass as f64) / 3.0 - 2.0;
  if fuel > 0.0 {
    fuel as u32
  } else {
    0
  }
}

#[cfg(test)]
mod tests {
  use super::{*};

  #[test]
  fn test_naive_calculate_fuel() {
    assert_eq!(naive_calculate_fuel(12), 2);
    assert_eq!(naive_calculate_fuel(14), 2);
    assert_eq!(naive_calculate_fuel(1969), 654);
    assert_eq!(naive_calculate_fuel(100756), 33583);
  }

  #[test]
  fn test_negative_fuel() {
    assert_eq!(naive_calculate_fuel(0), 0);
    assert_eq!(naive_calculate_fuel(1), 0);
    assert_eq!(naive_calculate_fuel(2), 0);
    assert_eq!(naive_calculate_fuel(3), 0);
  }

  #[test]
  fn test_calculate_fuel() {
    assert_eq!(calculate_fuel(14), 2);
    assert_eq!(calculate_fuel(1969), 966);
    assert_eq!(calculate_fuel(100756), 50346);
  }
}