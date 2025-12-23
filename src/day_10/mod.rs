use std::{collections::VecDeque, fs, iter};

fn read_input() -> Vec<(Vec<u8>, Vec<Vec<u8>>, Vec<i32>)> {
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

            let mut buttons: Vec<Vec<u8>> = vec![];

            for i in 1..split.len() - 1 {
                let mut button: Vec<u8> = Vec::from_iter(iter::repeat(0).take(light_diagram.len()));

                for index in split[i].split(',').map(|x| x.replace("(", "").replace(")", "").parse::<usize>().unwrap()) {
                    button[index] = 1;
                }

                buttons.push(button);
            }

            let jotages: Vec<i32> = split[split.len() - 1]
                .split(',')
                .map(|x| x.replace("{", "").replace("}", "").parse().unwrap())
                .collect();

            (light_diagram, buttons, jotages)
        })
        .collect()
}

pub fn solve_1() -> i32 {
    let manual = read_input();

    let mut result = 0;

    for (light_diagram, buttons, _) in manual {
        let mut bfs: VecDeque<Vec<Vec<u8>>> = VecDeque::from_iter(
            buttons.iter().map(|x| vec![x.clone()])
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

            for button in buttons.iter() {
                if current_buttons.contains(button) { continue; }

                let mut new_buttons = current_buttons.clone();
                new_buttons.push(button.clone());
                bfs.push_back(new_buttons);
            }
        }
    }

    result
}