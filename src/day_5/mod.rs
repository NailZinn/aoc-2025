use std::fs;

fn read_input() -> (Vec<(i64, i64)>, Vec<i64>) {
    let lines: Vec<String> = fs::read_to_string("src/day_5/input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let split_index = lines.iter()
        .position(|line| line.is_empty())
        .unwrap();

    let (ranges, ids) = lines.split_at(split_index);

    let parsed_ranges: Vec<(i64, i64)> = ranges
        .iter()
        .map(|x| x
            .split('-')
            .map(|x| x.parse::<i64>().unwrap())
            .collect()
        )
        .map(|x: Vec<i64>| (x[0], x[1]))
        .collect();

    let parsed_ids: Vec<i64> = ids
        .iter()
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    (parsed_ranges, parsed_ids)
}

pub fn solve_1() -> i32 {
    let (ranges, ids) = read_input();

    let mut result = 0;

    for id in ids {
        if ranges.iter().any(|x| x.0 <= id && id <= x.1) {
            result += 1;
            continue;
        }
    }

    result
}