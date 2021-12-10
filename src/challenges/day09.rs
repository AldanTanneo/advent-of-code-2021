use std::collections::HashSet;

/// Name of the challenge
const NAME: &str = "Smoke Basin";

/// Input file name
const INPUT: &str = "input/9.txt";

/// Converts a character to the corresponding digit. Panics on unexpected characters
fn char_to_digit(c: char) -> u64 {
    match c {
        '0'..='9' => (c as u8 - b'0') as u64,
        _ => panic!("Invalid character!"),
    }
}

/// Calls function `f` on every low point in the given `data` matrix
fn at_low_points(data: &[Vec<u64>], mut f: impl FnMut(usize, usize)) {
    let n = data.len();
    let m = data[0].len();
    /* Corners */
    if data[0][0] < data[1][0] && data[0][0] < data[0][1] {
        f(0, 0);
    }
    if data[0][m - 1] < data[1][m - 1] && data[0][m - 1] < data[0][m - 2] {
        f(0, m - 1);
    }
    if data[n - 1][0] < data[n - 2][0] && data[n - 1][0] < data[n - 1][1] {
        f(n - 1, 0);
    }
    if data[n - 1][m - 1] < data[n - 2][m - 1] && data[n - 1][m - 1] < data[n - 1][m - 2] {
        f(n - 1, m - 1);
    }
    /* Borders */
    for i in 1..n - 1 {
        if data[i][0] < data[i - 1][0] && data[i][0] < data[i][1] && data[i][0] < data[i + 1][0] {
            f(i, 0);
        }
        if data[i][m - 1] < data[i - 1][m - 1]
            && data[i][m - 1] < data[i][m - 2]
            && data[i][m - 1] < data[i + 1][m - 1]
        {
            f(i, m - 1);
        }
    }
    for j in 1..m - 1 {
        if data[0][j] < data[0][j - 1] && data[0][j] < data[1][j] && data[0][j] < data[0][j + 1] {
            f(0, j);
        }
        if data[n - 1][j] < data[n - 1][j - 1]
            && data[n - 1][j] < data[n - 2][j]
            && data[n - 1][j] < data[n - 1][j + 1]
        {
            f(n - 1, j);
        }
    }

    /* Inside */
    for i in 1..n - 1 {
        for j in 1..m - 1 {
            if data[i][j] < data[i - 1][j]
                && data[i][j] < data[i][j - 1]
                && data[i][j] < data[i][j + 1]
                && data[i][j] < data[i + 1][j]
            {
                f(i, j);
            }
        }
    }
}

/// Solve part one
fn part_1() -> u64 {
    let data = crate::utils::load_data::<String>(INPUT)
        .map(|s| s.chars().map(char_to_digit).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut risk = 0;

    at_low_points(&data, |i, j| {
        risk += data[i][j] + 1;
    });

    risk
}

/// Solve part two
fn part_2() -> usize {
    let data = crate::utils::load_data::<String>(INPUT)
        .map(|s| s.chars().map(char_to_digit).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let n = data.len();
    let m = data[0].len();

    #[derive(Debug, Clone)]
    struct Basin {
        points: HashSet<(usize, usize)>,
        low_point: (usize, usize),
    }

    impl Basin {
        pub fn new(i: usize, j: usize) -> Self {
            Self {
                points: HashSet::new(),
                low_point: (i, j),
            }
        }

        pub fn size(&self) -> usize {
            self.points.len()
        }
    }

    let mut basins: Vec<Basin> = Vec::new();

    at_low_points(&data, |i, j| {
        basins.push(Basin::new(i, j));
    });

    let mut stack = Vec::new();
    for (index, Basin { low_point, .. }) in basins.iter().cloned().enumerate() {
        stack.push((low_point, index));
    }

    while let Some(((i, j), basin_index)) = stack.pop() {
        if data[i][j] < 9 {
            basins[basin_index].points.insert((i, j));
            if i > 0 && data[i][j] < data[i - 1][j] {
                stack.push(((i - 1, j), basin_index));
            }
            if j > 0 && data[i][j] < data[i][j - 1] {
                stack.push(((i, j - 1), basin_index));
            }
            if i < n - 1 && data[i][j] < data[i + 1][j] {
                stack.push(((i + 1, j), basin_index));
            }
            if j < m - 1 && data[i][j] < data[i][j + 1] {
                stack.push(((i, j + 1), basin_index));
            }
        }
    }

    basins.sort_unstable_by_key(|b| b.size());

    let b = basins.len();

    basins[b - 1].size() * basins[b - 2].size() * basins[b - 3].size()
}

super::solve!();
