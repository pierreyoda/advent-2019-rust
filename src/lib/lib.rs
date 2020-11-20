use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    str::FromStr,
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
