use std::fs;

fn read_file() -> Vec<String> {
    fs::read_to_string("src/day_3/input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect::<Vec<String>>()
}

pub fn solve_1() -> i32 {
    let banks = read_file();

    let mut result = 0;

    for bank in banks {
        let batteries = bank
            .chars()
            .map(|x| { x.to_digit(10).unwrap() as i32 })
            .collect::<Vec<i32>>();

        let battery_enumeration = batteries
            .iter()
            .enumerate()
            .map(|(index, &value)| (index, value));

        let (mut max_index, mut max) = (0, 0);

        for (index, battery) in battery_enumeration {
            if battery > max {
                (max_index, max) = (index, battery);
            }
        }

        if max_index == bank.len() - 1 {
            result += max + 10 * batteries
                .iter()
                .take(max_index)
                .max()
                .unwrap();
        } else {
            result += 10 * max + batteries
                .iter()
                .skip(max_index + 1)
                .max()
                .unwrap();
        }
    }

    result
}