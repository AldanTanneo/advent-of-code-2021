//! # Day 2: Dive!
//!
//! ## Part One
//!
//! Now, you need to figure out how to pilot this thing.
//!
//! It seems like the submarine can take a series of commands like `forward 1`,
//! `down 2`, or `up 3`:
//!
//! - `forward X` increases the horizontal position by `X` units.
//! - `down X` **increases** the depth by `X` units.
//! - `up X` **decreases** the depth by `X` units.
//!
//! Note that since you're on a submarine, down and up affect your **depth**,
//! and so they have the opposite result of what you might expect.
//!
//! The submarine seems to already have a planned course (your puzzle input).
//! You should probably figure out where it's going. For example:
//!
//! ```txt
//! forward 5
//! down 5
//! forward 8
//! up 3
//! down 8
//! forward 2
//! ```
//!
//! Your horizontal position and depth both start at `0`. The steps above would
//! then modify them as follows:
//!
//! - `forward 5` adds `5` to your horizontal position, a total of `5`.
//! - `down 5` adds `5` to your depth, resulting in a value of `5`.
//! - `forward 8` adds `8` to your horizontal position, a total of `13`.
//! - `up 3` decreases your depth by `3`, resulting in a value of `2`.
//! - `down 8` adds `8` to your depth, resulting in a value of `10`.
//! - `forward 2` adds `2` to your horizontal position, a total of `15`.
//!
//! After following these instructions, you would have a horizontal position of
//! `15` and a depth of `10`. (Multiplying these together produces **`150`**.)
//!
//! Calculate the horizontal position and depth you would have after following
//! the planned course. **What do you get if you multiply your final horizontal
//! position by your final depth?**
//!
//! ## Part Two
//!
//! Based on your calculations, the planned course doesn't seem to make any
//! sense. You find the submarine manual and discover that the process is
//! actually slightly more complicated.
//!
//! In addition to horizontal position and depth, you'll also need to track a
//! third value, **aim**, which also starts at `0`. The commands also mean
//! something entirely different than you first thought:
//!
//! - `down X` **increases** your aim by `X` units.
//! - `up X` **decreases** your aim by `X` units.
//! - `forward X` does two things:
//!   - It increases your horizontal position by `X` units.
//!   - It increases your depth by your aim **multiplied by** `X`.
//!
//! Again note that since you're on a submarine, down and up do the opposite of
//! what you might expect: "down" means aiming in the positive direction.
//!
//! Now, the above example does something different:
//!
//! - `forward 5` adds `5` to your horizontal position, a total of `5`. Because
//!   your aim is `0`, your depth does not change.
//! - `down 5` adds `5` to your aim, resulting in a value of `5`.
//! - `forward 8` adds `8` to your horizontal position, a total of `13`.
//!   Because your aim is `5`, your depth increases by `8*5=40`.
//! - `up 3` decreases your aim by `3`, resulting in a value of `2`.
//! - `down 8` adds `8` to your aim, resulting in a value of `10`.
//! - `forward 2` adds `2` to your horizontal position, a total of `15`.
//!   Because your aim is `10`, your depth increases by `2*10=20` to a total of
//!   `60`.
//!
//! After following these new instructions, you would have a horizontal
//! position of `15` and a depth of `60`. (Multiplying these produces `900`.)
//!
//! Using this new interpretation of the commands, calculate the horizontal
//! position and depth you would have after following the planned course.
//! **What do you get if you multiply your final horizontal position by your
//! final depth?**
use std::error::Error;
use std::str::FromStr;

use crate::utils;

/// Name of the challenge
const NAME: &str = "Dive!";

/// Input file name
const INPUT: &str = "input/2.txt";

/// Enum for recording commands
enum Command<T: FromStr>
where
    <T as FromStr>::Err: Error,
{
    Forward(T),
    Down(T),
    Up(T),
}

/// Command parsing error type
#[derive(Debug)]
enum ParseMoveError<T>
where
    T: FromStr,
    <T as FromStr>::Err: Error,
{
    DirectionError,
    TypeError(<T as FromStr>::Err),
}

impl<T> std::fmt::Display for ParseMoveError<T>
where
    T: FromStr,
    <T as FromStr>::Err: Error,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseMoveError::DirectionError => write!(f, "Could not parse direction"),
            ParseMoveError::TypeError(err) => err.fmt(f),
        }
    }
}

impl<T> Error for ParseMoveError<T>
where
    T: FromStr + std::fmt::Debug,
    <T as FromStr>::Err: Error,
{
}

impl<T> FromStr for Command<T>
where
    T: FromStr,
    <T as FromStr>::Err: Error,
{
    type Err = ParseMoveError<T>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let direction = split.next().unwrap_or_default();
        let num = split.next().unwrap_or_default();

        let n = num.parse().map_err(Self::Err::TypeError)?;

        match direction {
            "forward" => Ok(Self::Forward(n)),
            "down" => Ok(Self::Down(n)),
            "up" => Ok(Self::Up(n)),
            _ => Err(Self::Err::DirectionError),
        }
    }
}

/// Solve part one
fn part_1() -> i64 {
    let data = utils::load_data::<Command<i64>>(INPUT);
    let (mut depth, mut horizontal_position) = (0, 0);

    for c in data {
        match c {
            Command::Forward(n) => {
                horizontal_position += n;
            }
            Command::Down(n) => {
                depth += n;
            }
            Command::Up(n) => {
                depth -= n;
            }
        }
    }

    depth * horizontal_position
}

/// Solve part two
fn part_2() -> i64 {
    let data = utils::load_data::<Command<i64>>(INPUT);
    let (mut depth, mut horizontal_position, mut aim) = (0, 0, 0);

    for c in data {
        match c {
            Command::Forward(n) => {
                horizontal_position += n;
                depth += n * aim;
            }
            Command::Down(n) => {
                aim += n as i64;
            }
            Command::Up(n) => {
                aim -= n as i64;
            }
        }
    }

    depth * horizontal_position
}

super::solve!();
