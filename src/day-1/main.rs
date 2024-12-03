use anyhow::{Context, Error, Result};

use advent_2019_common::{run_day_puzzle_solver, DayPuzzlePart};

pub struct Mass(i32);

impl TryFrom<String> for Mass {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self {
            0: value
                .parse()
                .with_context(|| format!("Mass: cannot parse raw input: {}", value))?,
        })
    }
}

fn compute_fuel_requirements(mass: i32) -> i32 {
    (mass / 3) - 2
}

fn compute_compounded_fuel_requirements(mass: i32) -> i32 {
    // avoid recursion to prevent stack overflow
    let mut final_mass = compute_fuel_requirements(mass);
    let mut fuel_additional_mass = compute_fuel_requirements(final_mass);

    while fuel_additional_mass > 0 {
        final_mass += fuel_additional_mass;
        fuel_additional_mass = compute_fuel_requirements(fuel_additional_mass);
    }

    final_mass
}

fn main() -> Result<()> {
    // Part 1
    run_day_puzzle_solver(1, DayPuzzlePart::One, b'\n', |input: Vec<Mass>| {
        Ok(input
            .iter()
            .fold(0, |sum, mass| sum + compute_fuel_requirements(mass.0)))
    })?;

    // Part 2
    run_day_puzzle_solver(1, DayPuzzlePart::Two, b'\n', |input: Vec<Mass>| {
        Ok(input.iter().fold(0, |sum, mass| {
            sum + compute_compounded_fuel_requirements(mass.0)
        }))
    })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{compute_compounded_fuel_requirements, compute_fuel_requirements};

    #[test]
    fn test_compute_fuel_requirements() {
        assert_eq!(compute_fuel_requirements(12), 2);
        assert_eq!(compute_fuel_requirements(14), 2);
        assert_eq!(compute_fuel_requirements(1969), 654);
        assert_eq!(compute_fuel_requirements(100756), 33583);
    }

    #[test]
    fn test_compute_compounded_fuel_requirements() {
        assert_eq!(compute_compounded_fuel_requirements(14), 2);
        assert_eq!(compute_compounded_fuel_requirements(1969), 966);
        assert_eq!(compute_compounded_fuel_requirements(100756), 50346);
    }
}
