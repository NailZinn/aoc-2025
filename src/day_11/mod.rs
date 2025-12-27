use std::{collections::{HashMap, HashSet}, fs};

fn read_input() -> HashMap<String, Vec<String>> {
    HashMap::from_iter(
        fs::read_to_string("src/day_11/input.txt")
            .unwrap()
            .lines()
            .map(|x| x.split(": ").collect())
            .map(|x: Vec<&str>| (String::from(x[0]), x[1].split(' ').map(String::from).collect()))
    )
}

pub fn solve_1() -> i32 {
    let adjacency_list = read_input();

    let mut result = 0;

    let mut dfs: Vec<(&str, HashSet<&str>)> = vec![];
    dfs.push(("you", HashSet::from(["you"])));

    while dfs.len() > 0 {
        let (device, mut path) = dfs.pop().unwrap();

        if device == "out" {
            result += 1;
            continue;
        }

        if let Some(adjacent_devices) = adjacency_list.get(device) {
            for adjacent_device in adjacent_devices {
                if path.insert(adjacent_device) {
                    dfs.push((adjacent_device, path.clone()));
                }
            }
        }
    }

    result
}