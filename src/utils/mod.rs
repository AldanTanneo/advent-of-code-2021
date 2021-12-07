//! Utilities for loading data
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

/// Returns an iterator over sequential data in a file
pub fn load_data<T: FromStr>(filename: &str) -> impl Iterator<Item = T>
where
    T::Err: std::error::Error,
{
    let file = File::open(filename).unwrap_or_else(|_| panic!("Failed to open file: {}", filename));
    let reader = BufReader::new(file);

    reader
        .lines()
        .inspect(|line| {
            if let Err(e) = line {
                println!("Error reading line: {}", e);
            }
        })
        .filter_map(Result::ok)
        .map(|line| line.trim().parse::<T>())
        .inspect(|value| {
            if let Err(e) = value {
                println!("Error parsing value: {}", e);
            }
        })
        .filter_map(Result::ok)
}
