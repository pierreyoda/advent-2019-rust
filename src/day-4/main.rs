use advent_2019_common::{run_day_puzzle_solver, DayPuzzlePart};
use anyhow::{Context, Error, Result};

type PasswordScalar = u32;

#[derive(Clone, Debug, PartialEq, Eq)]
struct PasswordsRange {
    min: PasswordScalar,
    max: PasswordScalar,
}

impl TryFrom<String> for PasswordsRange {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut parts = value.split('-');
        let (min_string, max_string) = (
            parts
                .next()
                .with_context(|| format!("no password min number for: {}", value))?,
            parts
                .next()
                .with_context(|| format!("no password max number for: {}", value))?,
        );
        Ok(Self {
            min: min_string
                .parse()
                .with_context(|| format!("cannot parse password min number for: {}", min_string))?,
            max: max_string
                .parse()
                .with_context(|| format!("cannot parse password max number for: {}", max_string))?,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PasswordValidationResult {
    Valid,
    IncorrectLength(usize),
    NoTwoAdjacentDigits,
    SuccessiveDigitsDecrease,
}

#[derive(Debug)]
struct Password {
    raw: String,
    parsed: PasswordScalar,
}

impl Password {
    pub fn is_valid(&self) -> bool {
        matches!(self.validate(), PasswordValidationResult::Valid)
    }

    pub fn validate(&self) -> PasswordValidationResult {
        if self.raw.len() != 6 {
            return PasswordValidationResult::IncorrectLength(self.raw.len());
        }

        let mut adjacent_digits_encountered = false;
        let raw_chars: Vec<char> = self.raw.chars().collect();
        for (i, letter) in raw_chars.iter().enumerate() {
            let digit: PasswordScalar = letter.to_digit(10).expect(&format!(
                "password validation: cannot parse digit: {}",
                letter
            ));

            match raw_chars.get(i + 1) {
                Some(next_letter) if next_letter == letter => {
                    adjacent_digits_encountered = true;
                }
                Some(next_letter) => {
                    let next_digit: PasswordScalar = next_letter.to_digit(10).expect(&format!(
                        "password validation: cannot parse digit: {}",
                        letter
                    ));
                    if next_digit < digit {
                        return PasswordValidationResult::SuccessiveDigitsDecrease;
                    }
                }
                None => (),
            }
        }
        if !adjacent_digits_encountered {
            return PasswordValidationResult::NoTwoAdjacentDigits;
        }

        PasswordValidationResult::Valid
    }
}

fn compute_solution_1(passwords_range: PasswordsRange) -> Result<usize> {
    let valid_passwords: Vec<u32> = (passwords_range.min..passwords_range.max)
        .filter(|digits_password| {
            Password {
                raw: digits_password.to_string(),
                parsed: *digits_password,
            }
            .is_valid()
        })
        .collect();
    Ok(valid_passwords.len())
}

fn main() -> Result<()> {
    // Part 1
    run_day_puzzle_solver(
        4,
        DayPuzzlePart::One,
        b'\n',
        |input: Vec<PasswordsRange>| compute_solution_1(input[0].clone()),
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{Password, PasswordValidationResult, PasswordsRange};

    #[test]
    fn test_day_4_passwords_range_parsing() {
        assert!(PasswordsRange::try_from("278384_824795".to_string()).is_err());
        assert!(PasswordsRange::try_from("278384e-824795".to_string()).is_err());
        assert!(PasswordsRange::try_from("278384-824a795".to_string()).is_err());
        assert_eq!(
            PasswordsRange::try_from("278384-824795".to_string()).unwrap(),
            PasswordsRange {
                min: 278384,
                max: 824795
            }
        );
    }

    #[test]
    fn test_day_4_password_validation() {
        let testing_pairs = [
            (111111, PasswordValidationResult::Valid),
            (12345, PasswordValidationResult::IncorrectLength(5)),
            (223450, PasswordValidationResult::SuccessiveDigitsDecrease),
            (123789, PasswordValidationResult::NoTwoAdjacentDigits),
        ];
        for (parsed_password, expected) in testing_pairs {
            let password = Password {
                parsed: parsed_password,
                raw: parsed_password.to_string(),
            };
            assert_eq!(
                password.validate(),
                expected,
                "tried validating: {}",
                parsed_password
            );
        }
    }
}
