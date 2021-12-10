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
pub mod utils;

use std::env::args;

/// Number of solved challenges
const N: usize = 10;

/// Array of function pointers to solutions
const SOLUTIONS: [fn(); N] = [
    challenges::day01::solve,
    challenges::day02::solve,
    challenges::day03::solve,
    challenges::day04::solve,
    challenges::day05::solve,
    challenges::day06::solve,
    challenges::day07::solve,
    challenges::day08::solve,
    challenges::day09::solve,
    challenges::day10::solve,
];

/// Show the answer to the given challenge
fn dispatch_challenge(challenge: usize) {
    match challenge {
        1..=N => SOLUTIONS[challenge - 1](),
        _ => println!(
            "\x1B[1mAdvent of Code 2021\x1B[0m

Unknown challenge: {}. Try a number between 1 and {}!",
            challenge, N
        ),
    }
}

/// Main function
fn main() {
    if let Some(challenge) = args().nth(1).map(|c| c.parse::<usize>().ok()).flatten() {
        dispatch_challenge(challenge);
    } else {
        println!("\x1B[1mAdvent of Code 2021\x1B[0m");
        println!("\nUsage: ./advent-of-code-2021 <challenge number>");
    }
}
