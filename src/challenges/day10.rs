/// Name of the challenge
const NAME: &str = "Syntax Scoring";

/// Input file name
const INPUT: &str = "input/10.txt";

/// Return the corresponding opening character
fn opening(c: char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => c,
    }
}

/// Return the score corresponding to the character
fn value(c: char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

/// Solve part one
fn part_1() -> u64 {
    let data = crate::utils::load_data::<String>(INPUT);
    let mut score = 0;
    for line in data {
        let mut stack = Vec::new();
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' | ']' | '}' | '>' => {
                    match stack.pop() {
                        None => break, // incomplete line
                        Some(o) if o != opening(c) => {
                            score += value(c);
                            break;
                        }
                        _ => continue,
                    }
                }
                _ => (),
            }
        }
    }
    score
}

/// Solve part two
fn part_2() -> u64 {
    let data = crate::utils::load_data::<String>(INPUT);
    let mut scores = Vec::new();
    for line in data {
        let mut stack = Vec::new();
        let mut valid = true;
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' | ']' | '}' | '>' => {
                    match stack.pop() {
                        None => break, // incomplete line
                        Some(o) if o != opening(c) => {
                            valid = false;
                            break;
                        }
                        _ => continue,
                    }
                }
                _ => (),
            }
        }
        if valid {
            let mut score = 0;
            for c in stack.iter().rev() {
                score = score * 5
                    + match c {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => 0,
                    };
            }
            scores.push(score);
        }
    }
    let mid = scores.len() / 2;
    *scores.select_nth_unstable(mid).1
}

super::solve!();
