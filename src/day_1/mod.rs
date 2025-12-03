use std::fs;

const DIAL_SIZE: i32 = 100;

pub fn solve_1() -> i32 {
    let rotations = fs::read_to_string("src/day_1/input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let mut dial = 50;
    let mut result = 0;

    for rotation in rotations {
        let (direction, steps_str) = rotation.split_at(1);
        let steps = steps_str.parse::<i32>().unwrap();

        dial = match direction {
            "L" => (dial - steps + DIAL_SIZE) % DIAL_SIZE,
            "R" => (dial + steps) % DIAL_SIZE,
            _ => dial
        };

        if dial == 0 {
            result += 1;
        }
    }

    result
}