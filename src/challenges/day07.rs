/// Name of the challenge
const NAME: &str = "The Treachery of Whales";

/// Input file name
const INPUT: &str = "input/7.txt";

/// Sum the distances between the crabs and `pos`
fn sum_distances(data: &[u64], pos: u64) -> u64 {
    data.iter()
        .map(|x| if x > &pos { x - pos } else { pos - x })
        .sum()
}

/// Solve part one
fn part_1() -> u64 {
    let mut data = std::fs::read_to_string(INPUT)
        .unwrap_or_else(|_| panic!("Failed to open file: {}", INPUT))
        .trim()
        .split(',')
        .filter_map(|s| s.parse::<u64>().ok())
        .collect::<Vec<_>>();
    let mid = data.len() / 2;
    data.select_nth_unstable(mid);
    sum_distances(&data, data[mid])
}

/// Sum the fuel consumed to get to `pos`, calculated according to part 2
fn sum_squared_distances(data: &[u64], pos: u64) -> u64 {
    data.iter()
        .map(|x| {
            let val = if x > &pos { x - pos } else { pos - x };
            val * (val + 1) / 2
        })
        .sum()
}

/// Solve part two
fn part_2() -> u64 {
    let data = std::fs::read_to_string(INPUT)
        .unwrap_or_else(|_| panic!("Failed to open file: {}", INPUT))
        .trim()
        .split(',')
        .filter_map(|s| s.parse::<u64>().ok())
        .collect::<Vec<_>>();
    let length = data.len() as u64;
    let mean = data.iter().sum::<u64>() / length;
    sum_squared_distances(&data, mean)
}

super::solve!();
