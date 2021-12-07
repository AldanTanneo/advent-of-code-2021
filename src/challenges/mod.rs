//! # Challenges
//!
//! Module containing the solutions as submodules

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;

/// Display the challenge title
macro_rules! display_title {
    () => {{
        println!(
            "\x1B[1mDay {} - {}\x1B[0m",
            module_path!()
                .trim_start_matches("advent_of_code_2021::challenges::day")
                .trim_start_matches('0'),
            self::NAME
        );
    }};
}

/// Display the solution, with an optional title
macro_rules! display_solution {
    ($sol:expr) => {{
        println!("\x1B[3m  Solution: \x1B[23;32m{}\x1B[0m", $sol);
    }};
    ($sol:expr, $title:expr) => {{
        println!("> {}", $title);
        crate::challenges::display_solution!($sol);
    }};
}

/// Declare a solve function that shows results
macro_rules! solve {
    () => {
        crate::challenges::solve!(part_1, part_2);
    };
    ($part_1:ident) => {
        /// Show results of part one
        pub(crate) fn solve() {
            crate::challenges::display_title!();
            let part_one = $part_1();
            super::display_solution!(part_one, "Part One");
        }
    };
    ($part_1:ident, $part_2:ident) => {
        /// Show results of part one and two
        pub(crate) fn solve() {
            crate::challenges::display_title!();
            let part_one = part_1();
            crate::challenges::display_solution!(part_one, "Part One");
            let part_two = part_2();
            crate::challenges::display_solution!(part_two, "Part Two");
        }
    };
}

pub(self) use display_solution;
pub(self) use display_title;
pub(self) use solve;
