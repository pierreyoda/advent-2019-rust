use anyhow::Result;

use advent_2019_common::run_with_scaffolding;

fn compute_fuel_requirements(mass: i64) -> i64 {
    (mass / 3) - 2
}
fn main() -> Result<()> {
    run_with_scaffolding("day-1", |inputs| {
        inputs
            .iter()
            .fold(0, |sum, mass| sum + compute_fuel_requirements(*mass))
    })?;

    Ok(())
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
