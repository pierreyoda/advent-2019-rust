use anyhow::Result;

use advent_2019_common::run_with_scaffolding;

fn compute_fuel_requirements(mass: i32) -> i32 {
    (mass / 3) - 2
}

fn compute_compounded_fuel_requirements(mass: i32) -> i32 {
    // avoid recursivity to prevent stack overflow
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
    run_with_scaffolding("day-1", |inputs| {
        inputs
            .iter()
            .fold(0, |sum, mass| sum + compute_fuel_requirements(*mass))
    })?;

    // Part 2
    run_with_scaffolding("day-1", |inputs| {
        inputs.iter().fold(0, |sum, mass| {
            sum + compute_compounded_fuel_requirements(*mass)
        })
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
