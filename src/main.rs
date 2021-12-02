//! # Advent of Code 2021
//!
//! ## About
//!
//! [**Advent of Code**](https://adventofcode.com/) is an Advent calendar of
//! small programming puzzles for a variety of skill sets and skill levels that
//! can be solved in any programming language you like. People use them as a
//! speed contest, interview prep, company training, university coursework,
//! practice problems, or to challenge each other.
//!
//! You don't need a computer science background to participate - just a little
//! programming knowledge and some problem solving skills will get you pretty
//! far. Nor do you need a fancy computer; every problem has a solution that
//! completes in at most 15 seconds on ten-year-old hardware.
pub mod challenges;
pub(crate) mod utils;

use std::env::args;
use std::num::NonZeroU8;

/// Show the answer to challenge n
fn dispatch_challenge(challenge: NonZeroU8) {
    match challenge.get() {
        1 => challenges::day01::solve(),
        2 => challenges::day02::solve(),
        _ => println!("Unknown challenge!"),
    }
}

/// Main function
fn main() {
    if let Some(challenge) = args().nth(1).map(|c| c.parse::<NonZeroU8>().ok()).flatten() {
        dispatch_challenge(challenge);
    } else {
        println!("\x1B[1mAdvent of Code 2021\x1B[0m");
        println!("\nUsage: ./advent-of-code-2021 <challenge number>");
    }
}
