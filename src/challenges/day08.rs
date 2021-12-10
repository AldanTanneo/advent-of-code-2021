/// Name of the challenge
const NAME: &str = "Seven Segment Search";

/// Input file name
const INPUT: &str = "input/8.txt";

/// Solve part one
fn part_1() -> u64 {
    // 1: two segments, 4: four segments, 7: three segments, 8: seven segments
    let data = crate::utils::load_data::<String>(INPUT).collect::<Vec<_>>();
    let mut ret = 0;
    for output in data.iter().filter_map(|s| s.split('|').nth(1)) {
        for val in output.split_whitespace() {
            ret += match val.len() {
                2 | 4 | 3 | 7 => 1,
                _ => 0,
            };
        }
    }
    ret
}

/// Decrypt the randomized connections between digit inputs and the seven segments
fn decrypt(input: &str) -> [&str; 10] {
    let mut ret = [""; 10];
    let strings = input.split_whitespace().collect::<Vec<_>>();

    /* Unique char count */
    ret[1] = strings.iter().find(|&&s| s.len() == 2).unwrap();
    ret[7] = strings.iter().find(|&&s| s.len() == 3).unwrap();
    ret[4] = strings.iter().find(|&&s| s.len() == 4).unwrap();
    ret[8] = strings.iter().find(|&&s| s.len() == 7).unwrap();

    /* 5 characters */
    ret[5] = strings
        .iter()
        .find(|&&s| {
            s.len() == 5
                && ret[4]
                    .chars()
                    .filter(|&c| !ret[1].contains(c))
                    .all(|c| s.contains(c))
        })
        .unwrap();
    ret[3] = strings
        .iter()
        .find(|&&s| s.len() == 5 && ret[1].chars().all(|c| s.contains(c)))
        .unwrap();
    ret[2] = strings
        .iter()
        .find(|&&s| s.len() == 5 && s != ret[3] && s != ret[5])
        .unwrap();

    let middle_segment = ret[4]
        .chars()
        .filter(|&c| !ret[1].contains(c) && ret[2].contains(c))
        .next()
        .unwrap();
    let top_right_segment = ret[1]
        .chars()
        .filter(|&c| ret[2].contains(c))
        .next()
        .unwrap();

    /* Six characters */
    ret[0] = strings
        .iter()
        .find(|&&s| s.len() == 6 && !s.contains(middle_segment))
        .unwrap();
    ret[6] = strings
        .iter()
        .find(|&&s| s.len() == 6 && !s.contains(top_right_segment))
        .unwrap();
    ret[9] = strings
        .iter()
        .find(|&&s| s.len() == 6 && s != ret[6] && s != ret[0])
        .unwrap();

    ret
}

/// Solve part two
fn part_2() -> usize {
    let data = crate::utils::load_data::<String>(INPUT).collect::<Vec<_>>();
    let mut sum = 0;
    for (input, output) in data.iter().filter_map(|s| s.split_once('|')) {
        let decrypt_map = decrypt(input);
        let mut res = 0;
        for o in output.split_whitespace() {
            res = 10 * res
                + decrypt_map
                    .iter()
                    .enumerate()
                    .find(|(_, &s)| {
                        s.chars().all(|c| o.contains(c)) && o.chars().all(|c| s.contains(c))
                    })
                    .unwrap()
                    .0;
        }
        sum += res;
    }
    sum
}

super::solve!();
