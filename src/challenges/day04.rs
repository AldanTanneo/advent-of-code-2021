//! # Day 4: Giant Squid
//!
//! ## Part One
//!
//! You're already almost 1.5km (almost a mile) below the surface of the ocean,
//! already so deep that you can't see any sunlight. What you **can** see,
//! however, is a giant squid that has attached itself to the outside of your
//! submarine.
//!
//! Maybe it wants to play
//! [bingo](https://en.wikipedia.org/wiki/Bingo_(American_version))?
//!
//! Bingo is played on a set of boards each consisting of a 5x5 grid of
//! numbers. Numbers are chosen at random, and the chosen number is **marked**
//! on all boards on which it appears. (Numbers may not appear on all boards.)
//! If all numbers in any row or any column of a board are marked, that board
//! **wins**. (Diagonals don't count.)
//!
//! The submarine has a bingo subsystem to help passengers (currently, you and
//! the giant squid) pass the time. It automatically generates a random order
//! in which to draw numbers and a random set of boards (your puzzle input).
//! For example:
//!
//! ```txt
//! 7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1
//!
//! 22 13 17 11  0
//!  8  2 23  4 24
//! 21  9 14 16  7
//!  6 10  3 18  5
//!  1 12 20 15 19
//!
//!  3 15  0  2 22
//!  9 18 13 17  5
//! 19  8  7 25 23
//! 20 11 10 24  4
//! 14 21 16 12  6
//!
//! 14 21 17 24  4
//! 10 16 15  9 19
//! 18  8 23 26 20
//! 22 11 13  6  5
//!  2  0 12  3  7
//! ```
//!
//! After the first five numbers are drawn (7, 4, 9, 5, and 11), there are no
//! winners, but the boards are marked as follows (shown here adjacent to each
//! other to save space):
//!
//! ```txt
//! 22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
//!  8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
//! 21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
//!  6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
//!  1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
//! ```
//!
//! After the next six numbers are drawn (17, 23, 2, 0, 14, and 21), there are
//! still no winners:
//!
//! ```txt
//! 22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
//!  8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
//! 21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
//!  6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
//!  1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
//! ```
//!
//! Finally, 24 is drawn:
//!
//! ```txt
//! 22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
//!  8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
//! 21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
//!  6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
//!  1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
//! ```
//!
//! At this point, the third board wins because it has at least one complete
//! row or column of marked numbers (in this case, the entire top row is
//! marked: 14 21 17 24 4).
//!
//! The score of the winning board can now be calculated. Start by finding the
//! sum of all unmarked numbers on that board; in this case, the sum is 188.
//! Then, multiply that sum by the number that was just called when the board
//! won, 24, to get the final score, 188 * 24 = 4512.
//!
//! To guarantee victory against the giant squid, figure out which board will
//! win first. What will your final score be if you choose that board?
//!
//! ## Part Two
//!
//! On the other hand, it might be wise to try a different strategy: let the giant squid win.
//!
//! You aren't sure how many bingo boards a giant squid could play at once, so rather than waste time counting its arms, the safe thing to do is to figure out which board will win last and choose that one. That way, no matter which boards it picks, it will win for sure.
//!
//! In the above example, the second board is the last to win, which happens after 13 is eventually called and its middle column is completely marked. If you were to keep playing until this point, the second board would have a sum of unmarked numbers equal to 148 for a final score of 148 * 13 = 1924.
//!
//! Figure out which board will win last. Once it wins, what would its final score be?
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Name of the challenge
const NAME: &str = "Giant Squid";

/// Input file name
const INPUT: &str = "input/4.txt";

/// Stores a bingo number and wether or not it is marked
#[derive(Debug, Clone, Copy)]
struct BingoNumber {
    pub marked: bool,
    pub value: u32,
}

impl BingoNumber {
    /// Creates a new Bingo number
    fn new(value: u32) -> Self {
        Self {
            marked: false,
            value,
        }
    }
}

/// Load grids and numbers from a file
fn load_grids(filename: &str) -> (Vec<u32>, Vec<[[BingoNumber; 5]; 5]>) {
    let file = File::open(filename).unwrap_or_else(|_| panic!("Failed to open file: {}", filename));
    let mut reader = BufReader::new(file);

    let mut buf = String::new();
    if let Err(e) = reader.read_line(&mut buf) {
        println!("Error reading line in `{}`: {}", filename, e);
    }
    let numbers = buf
        .split(',')
        .filter_map(|s| s.parse::<u32>().ok())
        .collect::<Vec<_>>();

    let mut grids = Vec::new();
    let mut current_index = 0;
    let mut current_grid = [[BingoNumber::new(0); 5]; 5];
    for line in reader
        .lines()
        .filter_map(Result::ok)
        .filter(|s| !s.is_empty())
    {
        let line_iter = line
            .split(' ')
            .filter(|s| !s.is_empty())
            .filter_map(|s| s.parse::<u32>().ok())
            .take(5);
        for (i, v) in line_iter.enumerate() {
            current_grid[current_index][i].value = v;
        }
        current_index += 1;
        if current_index == 5 {
            grids.push(current_grid);
            current_index = 0;
        }
    }

    (numbers, grids)
}

/// Check if a grid has won
fn is_winner(grid: &[[BingoNumber; 5]; 5]) -> bool {
    for line in grid {
        let mut line_win = true;
        for number in line {
            if !number.marked {
                line_win = false;
                break;
            }
        }
        if line_win {
            return true;
        }
    }
    for column in grid {
        let mut column_win = true;
        for number in column {
            if !number.marked {
                column_win = false;
                break;
            }
        }
        if column_win {
            return true;
        }
    }

    false
}

/// Computes a winning grid's score
fn score(grid: &[[BingoNumber; 5]; 5], n: u32) -> u64 {
    let mut sum = 0;
    for line in grid {
        for number in line {
            if !number.marked {
                sum += number.value as u64;
            }
        }
    }
    sum * n as u64
}

/// Solve part one
fn part_1() -> u64 {
    let (numbers, mut grids) = load_grids(INPUT);
    for n in numbers {
        for g in &mut grids {
            for i in 0..5 {
                for j in 0..5 {
                    if g[i][j].value == n {
                        g[i][j].marked = true;
                        if is_winner(g) {
                            return score(g, n);
                        }
                    }
                }
            }
        }
    }
    0
}

/// Solve part two
fn part_2() -> u64 {
    let (numbers, mut grids) = load_grids(INPUT);
    let mut winners = vec![false; grids.len()];
    let mut losers = winners.len();
    for n in numbers {
        for (index, g) in grids.iter_mut().enumerate() {
            for i in 0..5 {
                for j in 0..5 {
                    if g[i][j].value == n {
                        g[i][j].marked = true;
                        if is_winner(g) && !winners[index] {
                            winners[index] = true;
                            losers -= 1;
                            if losers == 0 {
                                return score(g, n);
                            }
                        }
                    }
                }
            }
        }
    }
    0
}

super::solve!();
