use std::{collections::{HashMap, HashSet}, fs};

#[allow(dead_code)]
fn read_input() -> HashMap<String, Vec<String>> {
    HashMap::from_iter(
        fs::read_to_string("src/day_11/input.txt")
            .unwrap()
            .lines()
            .map(|x| x.split(": ").collect())
            .map(|x: Vec<&str>| (String::from(x[0]), x[1].split(' ').map(String::from).collect()))
    )
}

#[allow(dead_code)]
pub fn solve_1() -> i32 {
    let adjacency_list = read_input();

    let mut result = 0;

    let mut dfs: Vec<(&str, HashSet<&str>)> = vec![];
    dfs.push(("you", HashSet::from(["you"])));

    while dfs.len() > 0 {
        let (device, path) = dfs.pop().unwrap();

        if device == "out" {
            result += 1;
            continue;
        }

        if let Some(adjacent_devices) = adjacency_list.get(device) {
            for adjacent_device in adjacent_devices {
                if !path.contains(adjacent_device.as_str()) {
                    let mut new_path = path.clone();
                    new_path.insert(adjacent_device);
                    dfs.push((adjacent_device, path.clone()));
                }
            }
        }
    }

    result
}

#[allow(dead_code)]
pub fn solve_2() -> i64 {
    let adjacency_list = read_input();

    let mut memo: HashMap<(&str, &str), i64> = HashMap::new();

    let svr_to_fft = number_of_paths("svr", "fft", Some("dac"), &adjacency_list, &mut memo);
    let svr_to_dac = number_of_paths("svr", "dac", Some("fft"), &adjacency_list, &mut memo);
    let fft_to_dac = number_of_paths("fft", "dac", None, &adjacency_list, &mut memo);
    let dac_to_fft = number_of_paths("dac", "fft", None, &adjacency_list, &mut memo);
    let fft_to_out = number_of_paths("fft", "out", Some("dac"), &adjacency_list, &mut memo);
    let dac_to_out = number_of_paths("dac", "out", Some("fft"), &adjacency_list, &mut memo);

    svr_to_fft * fft_to_dac * dac_to_out +
    svr_to_dac * dac_to_fft * fft_to_out
}

#[allow(dead_code)]
fn number_of_paths<'a>(
    from: &str, to: &'a str, skip: Option<&str>,
    adjacency_list: &'a HashMap<String, Vec<String>>,
    memo: &mut HashMap<(&'a str, &'a str), i64>
) -> i64 {
    if from == to { return 1; }

    match adjacency_list.get(from) {
        Some(devices) => devices
            .iter()
            .fold(0, |acc, cur| {
                if Some(cur.as_str()) == skip {
                    return acc;
                }
                if let Some(number_of_paths) = memo.get(&(cur, to)) {
                    return acc + *number_of_paths;
                }
                let number_of_paths = number_of_paths(cur, to, skip, adjacency_list, memo);
                memo.insert((cur, to), number_of_paths);
                acc + number_of_paths
            }),
        None => 0
    }
}