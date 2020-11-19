use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use anyhow::Result;

pub fn load_inputs_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<i64>> {
    let file = File::open(path)?;
    let lines = BufReader::new(file).lines();
    Ok(lines
        .into_iter()
        // TODO: avoid unwrap
        .map(|i| i.unwrap().parse())
        .filter_map(Result::ok)
        .collect())
}
