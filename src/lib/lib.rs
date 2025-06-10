use std::fmt::Debug;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    time::Instant,
};

use anyhow::{Context, Error, Result};
use colored::*;

fn parse_inputs_from_file<T, P>(path: P, separator: u8) -> Result<Vec<T>>
where
    T: TryFrom<String, Error = Error>,
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    let content = BufReader::new(file);
    let lines = content.split(separator);
    let mut parsed_lines = Vec::with_capacity(lines.size_hint().0);
    for (i, bytes_line_result) in lines.enumerate() {
        let bytes_line = bytes_line_result.with_context(|| {
            format!("parse_inputs_from_file cannot read line with index: {}", i)
        })?;
        let raw_line = String::from_utf8(bytes_line).with_context(|| {
            format!(
                "parse_inputs_from_file cannot stringify line with index: {}",
                i
            )
        })?;
        let trimmed_raw_line = raw_line.trim();
        if trimmed_raw_line.is_empty() {
            continue;
        }
        parsed_lines.push(T::try_from(trimmed_raw_line.to_string())?);
    }
    Ok(parsed_lines)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DayPuzzlePart {
    One,
    Two,
}

impl DayPuzzlePart {
    pub fn as_word(&self) -> &str {
        match self {
            DayPuzzlePart::One => "One",
            DayPuzzlePart::Two => "Two",
        }
    }
}

pub fn run_day_puzzle_solver<T, C, O>(
    day_number: usize,
    part: DayPuzzlePart,
    separator: u8,
    compute: C,
) -> Result<O>
where
    T: TryFrom<String, Error = Error>,
    C: FnOnce(Vec<T>) -> Result<O>,
    O: Debug,
{
    println!(
        "{}",
        format!("=== Day {} - Part {} ===", day_number, part.as_word()).bright_blue()
    );

    // Read input
    let input_start = Instant::now();
    let parsed_input: Vec<T> =
        parse_inputs_from_file(format!("./src/day-{}/input.txt", day_number), separator)?;
    println!(
        "{}",
        format!("=> Input read in {:?}", input_start.elapsed()).cyan()
    );

    // Computing function
    let compute_start = Instant::now();
    let output = compute(parsed_input)?;
    println!(
        "{}",
        format!("=> Computing done in {:?}", compute_start.elapsed()).cyan()
    );

    // Output
    println!("{}\n", format!("=> Result = {:?}", output).green());
    Ok(output)
}
