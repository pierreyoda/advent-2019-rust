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

pub fn load_inputs_from_file<N, P>(path: P) -> Result<Vec<N>>
where
    N: Copy + Num + Ord + FromStr,
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    let lines = BufReader::new(file).lines();
    Ok(lines
        .into_iter()
        // TODO: avoid unwrap
        .map(|i| i.unwrap().parse())
        .filter_map(Result::ok)
        .collect())
}

pub fn run_with_scaffolding<N, F>(label: &'static str, compute: F) -> Result<N>
where
    N: Copy + Num + Ord + FromStr + Display,
    F: Fn(Vec<N>) -> N,
{
    // Read input(s)
    let input_start = Instant::now();
    let input = load_inputs_from_file(format!("./src/{}/input.txt", label))?;
    let input_time = input_start.elapsed();
    println!("Inputs read in {:?}", input_time);

    // Run computing function
    let compute_start = Instant::now();
    let output = compute(input);
    let compute_time = compute_start.elapsed();
    println!("Computing done in {:?}", compute_time);

    // Output
    println!("Result = {}", output);
    Ok(output)
}
