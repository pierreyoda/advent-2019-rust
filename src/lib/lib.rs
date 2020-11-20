use std::str::from_utf8;
use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    str::FromStr,
    time::Instant,
};

use anyhow::Result;
use num_traits::Num;

pub fn load_inputs_from_file<N, P>(path: P, separator: u8) -> Result<Vec<N>>
where
    N: Copy + Num + Ord + FromStr,
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    let lines = BufReader::new(file);
    Ok(lines
        .split(separator)
        .into_iter()
        // TODO: avoid unwrap
        .map(|l| {
            let raw = l.unwrap();
            let string = from_utf8(&raw).unwrap();
            string.parse()
        })
        .filter_map(Result::ok)
        .collect())
}

pub fn run_with_scaffolding<N, F>(
    label: &'static str,
    inputs_separator: u8,
    compute: F,
) -> Result<N>
where
    N: Copy + Num + Ord + FromStr + Display,
    F: Fn(Vec<N>) -> Result<N>,
{
    // Read input(s)
    let input_start = Instant::now();
    let input = load_inputs_from_file(format!("./src/{}/input.txt", label), inputs_separator)?;
    let input_time = input_start.elapsed();
    println!("Inputs read in {:?}", input_time);

    // Run computing function
    let compute_start = Instant::now();
    let output = compute(input)?;
    let compute_time = compute_start.elapsed();
    println!("Computing done in {:?}", compute_time);

    // Output
    println!("Result = {}", output);
    Ok(output)
}
