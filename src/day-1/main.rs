use advent_2019_common::load_inputs_from_file;

fn compute_fuel_requirements(mass: i64) -> i64 {
    (mass / 3) - 2
}
fn main() {
    println!("Advent 2019 - Day 1");
    let inputs = load_inputs_from_file("./src/day-1/input.txt").expect("Input error.");
    let total_fuel_required = inputs
        .iter()
        .fold(0, |sum, mass| sum + compute_fuel_requirements(*mass));
    println!("Answer: {}", total_fuel_required);
}

#[cfg(test)]
mod tests {
    use crate::compute_fuel_requirements;

    #[test]
    fn test_compute_fuel_requirements() {
        assert_eq!(compute_fuel_requirements(12), 2);
        assert_eq!(compute_fuel_requirements(14), 2);
        assert_eq!(compute_fuel_requirements(1969), 654);
        assert_eq!(compute_fuel_requirements(100756), 33583);
    }
}
