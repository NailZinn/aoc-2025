use std::{collections::VecDeque, fs, iter};

fn read_input() -> Vec<(Vec<u8>, Vec<Vec<usize>>, Vec<i32>)> {
    fs::read_to_string("src/day_10/input.txt")
        .unwrap()
        .lines()
        .map(|x| {
            let split: Vec<&str> = x.split(' ').collect();

            let light_diagram: Vec<u8> = split[0]
                .chars()
                .filter(|&x| x != '[' && x != ']')
                .map(|x| match x {
                    '.' => 0,
                    '#' => 1,
                    _ => unreachable!()
                })
                .collect();

            let mut buttons: Vec<Vec<usize>> = vec![];

            for i in 1..split.len() - 1 {
                let mut button: Vec<usize> = vec![];

                for index in split[i].split(',').map(|x| x.replace("(", "").replace(")", "").parse::<usize>().unwrap()) {
                    button.push(index);
                }

                buttons.push(button);
            }

            let joltages: Vec<i32> = split[split.len() - 1]
                .split(',')
                .map(|x| x.replace("{", "").replace("}", "").parse().unwrap())
                .collect();

            (light_diagram, buttons, joltages)
        })
        .collect()
}

pub fn solve_1() -> i32 {
    let manual = read_input();

    let mut result = 0;

    for (light_diagram, buttons, _) in manual {
        let mut light_buttons: Vec<Vec<u8>> = vec![];

        for button in buttons {
            let mut light_button: Vec<u8> = Vec::from_iter(iter::repeat(0).take(light_diagram.len()));

            for index in button {
                light_button[index] = 1;
            }

            light_buttons.push(light_button);
        }

        let mut bfs: VecDeque<Vec<Vec<u8>>> = VecDeque::from_iter(
            light_buttons.iter().map(|x| vec![x.clone()])
        );

        loop {
            let current_buttons = bfs.pop_front().unwrap();
            if current_buttons.len() % 10 == 0 { println!("{}", current_buttons.len()); }
            let mut current_light_diagram: Vec<u8> = Vec::from_iter(iter::repeat(0).take(light_diagram.len()));

            for current_button in current_buttons.iter() {
                for i in 0..current_button.len() {
                    current_light_diagram[i] ^= current_button[i];
                }
            }

            if current_light_diagram == light_diagram {
                result += current_buttons.len() as i32;
                break;
            }

            for light_button in light_buttons.iter() {
                if current_buttons.contains(light_button) { continue; }

                let mut new_buttons = current_buttons.clone();
                new_buttons.push(light_button.clone());
                bfs.push_back(new_buttons);
            }
        }
    }

    result
}

pub fn solve_2() -> i64 {
    let manual = read_input();

    let mut result = 0;

    for (_, buttons, joltages) in manual {
        let mut press_combinations: Vec<(Vec<i32>, i32)> = vec![];

        for mask in 0..(1 << buttons.len()) {
            let mut presses: Vec<i32> = Vec::from_iter(iter::repeat(0).take(joltages.len()));
            let mut buttons_count: i32 = 0;

            for i in 0..buttons.len() {
                if (1 << i) & mask != 0 {
                    buttons_count += 1;
                    for &index in buttons[i].iter() {
                        presses[index] += 1;
                    }
                }
            }

            press_combinations.push((presses, buttons_count));
        }

        result += traverse_press_combinations(&joltages, &press_combinations);
    }

    result
}

fn traverse_press_combinations(joltages: &Vec<i32>, press_combinations: &Vec<(Vec<i32>, i32)>) -> i64 {
	if joltages.iter().all(|&x| x == 0) { return 0; }

	let mut result = i64::MAX;

	for combination in press_combinations {
        if joltages.iter().enumerate().any(|(i, &x)| combination.0[i] > x || x % 2 != combination.0[i] % 2) {
            continue;
        }      

        let mut current_joltages: Vec<i32> = vec![];

        for (i, joltage) in joltages.iter().enumerate() {
            current_joltages.push((joltage - combination.0[i]) / 2);
        }

        let traverse_result = traverse_press_combinations(&current_joltages, press_combinations);

        if traverse_result > i32::MAX as i64 { continue; }

		let traverse_result = traverse_result * 2 + combination.1 as i64;

        result = result.min(traverse_result);
	}

    result
}