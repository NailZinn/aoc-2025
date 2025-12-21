use std::fs;

fn read_input() -> Vec<(i64, i64)> {
    fs::read_to_string("src/day_9/input.txt")
        .unwrap()
        .lines()
        .map(|x| x.split(',').collect())
        .map(|x: Vec<&str>| (x[0].parse().unwrap(), x[1].parse().unwrap()))
        .collect()
}

pub fn solve_1() -> i64 {
    let coordinates = read_input();

    let mut result = 0;

    for i in 0..coordinates.len() {
        for j in i + 1..coordinates.len() {
            result = result.max(((coordinates[i].0 - coordinates[j].0).abs() + 1) * ((coordinates[i].1 - coordinates[j].1).abs() + 1));
        }
    }

    result
}