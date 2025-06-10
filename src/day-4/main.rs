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
    // Part 1
    IncorrectLength(usize),
    NoTwoAdjacentDigits,
    SuccessiveDigitsDecrease,
    // Part 2
    TooManySuccessiveDigits,
}

const PASSWORD_EXPECTED_LENGTH: usize = 6;

#[derive(Debug)]
struct Password {
    raw: String,
    parsed: PasswordScalar,
}

impl Password {
    pub fn is_valid(&self, part: DayPuzzlePart) -> bool {
        matches!(self.validate(part), PasswordValidationResult::Valid)
    }

    pub fn validate(&self, part: DayPuzzlePart) -> PasswordValidationResult {
        if self.raw.len() != PASSWORD_EXPECTED_LENGTH {
            return PasswordValidationResult::IncorrectLength(self.raw.len());
        }

        fn parse_digit(raw_digit: Option<&char>) -> Option<PasswordScalar> {
            match raw_digit {
                Some(r) => Some(
                    r.to_digit(10)
                        .expect(&format!("password validation: cannot parse digit: {}", r)),
                ),
                None => None,
            }
        }

        // println!("\n\np={}", self.raw);
        let raw_chars: Vec<char> = self.raw.chars().collect();
        let (mut at_least_one_pair_digit, mut more_than_two_adjacent_digits) = (false, false);
        let mut iter = raw_chars.iter().enumerate();
        while let Some((i, c0)) = iter.next() {
            let d0 = parse_digit(Some(c0));
            let d1 = parse_digit(raw_chars.get(i + 1));
            let d2 = parse_digit(raw_chars.get(i + 2));

            // println!("d0={:?}\td1={:?}\td2={:?}", d0, d1, d2);
            if d1.is_some() && d1 < d0 {
                return PasswordValidationResult::SuccessiveDigitsDecrease;
            }

            if d1 != d0 {
                continue;
            }

            if d2 != d0 {
                at_least_one_pair_digit = true;
                continue;
            }

            'inner: while let Some((j, c)) = iter.next() {
                // println!("j={}\t\tloop={:?}", j, parse_digit(Some(c)));
                let d = parse_digit(Some(c));
                if d != d0 || parse_digit(raw_chars.get(j + 1)) != d {
                    break 'inner;
                }
                more_than_two_adjacent_digits = true;
            }
        }

        // println!(
        //     "for={},\t\t\tat_least_one_pair_digit={};more_than_two_adjacent_digits={}",
        //     self.raw, at_least_one_pair_digit, more_than_two_adjacent_digits
        // );

        if part == DayPuzzlePart::Two {
            if !at_least_one_pair_digit {
                if more_than_two_adjacent_digits {
                    return PasswordValidationResult::TooManySuccessiveDigits;
                }
                return PasswordValidationResult::NoTwoAdjacentDigits;
            }
        }

        if !at_least_one_pair_digit && !more_than_two_adjacent_digits {
            return PasswordValidationResult::NoTwoAdjacentDigits;
        }

        PasswordValidationResult::Valid
    }
}

fn compute_solution_for_part(
    passwords_range: PasswordsRange,
    part: DayPuzzlePart,
) -> Result<usize> {
    let valid_passwords: Vec<u32> = (passwords_range.min..passwords_range.max)
        .filter(|digits_password| {
            Password {
                raw: digits_password.to_string(),
                parsed: *digits_password,
            }
            .is_valid(part)
        })
        .collect();
    Ok(valid_passwords.len())
}

fn compute_solution_1(passwords_range: PasswordsRange) -> Result<usize> {
    compute_solution_for_part(passwords_range, DayPuzzlePart::One)
}

fn compute_solution_2(passwords_range: PasswordsRange) -> Result<usize> {
    compute_solution_for_part(passwords_range, DayPuzzlePart::Two)
}

fn main() -> Result<()> {
    // Part 1
    run_day_puzzle_solver(
        4,
        DayPuzzlePart::One,
        b'\n',
        |input: Vec<PasswordsRange>| compute_solution_1(input[0].clone()),
    )?;

    // Part 2
    run_day_puzzle_solver(
        4,
        DayPuzzlePart::Two,
        b'\n',
        |input: Vec<PasswordsRange>| compute_solution_2(input[0].clone()),
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use advent_2019_common::DayPuzzlePart;

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
    fn test_day_4_password_validation_part_one() {
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
                password.validate(DayPuzzlePart::One),
                expected,
                "tried validating: {}",
                parsed_password
            );
        }
    }

    #[test]
    fn test_day_4_password_validation_part_two() {
        let testing_pairs = [
            (112233, PasswordValidationResult::Valid),
            (12345, PasswordValidationResult::IncorrectLength(5)),
            (123444, PasswordValidationResult::TooManySuccessiveDigits),
            (111234, PasswordValidationResult::TooManySuccessiveDigits),
            (111122, PasswordValidationResult::Valid),
            (223450, PasswordValidationResult::SuccessiveDigitsDecrease),
            (123789, PasswordValidationResult::NoTwoAdjacentDigits),
        ];
        for (parsed_password, expected) in testing_pairs {
            let password = Password {
                parsed: parsed_password,
                raw: parsed_password.to_string(),
            };
            assert_eq!(
                password.validate(DayPuzzlePart::Two),
                expected,
                "tried validating: {}",
                parsed_password
            );
        }
    }
}
