use std::{collections::HashSet, fs};

fn read_input() -> Vec<Vec<char>> {
    fs::read_to_string("src/day_7/input.txt")
        .unwrap()
        .lines()
        .map(|x| x.chars().collect())
        .collect()
}

pub fn solve_1() -> i32 {
    let diagram = read_input();

    let mut result = 0;

    let mut beam_indices: HashSet<usize> = HashSet::new();

    'outer:
    for i in 0..diagram.len() {
        for j in 0..diagram[0].len() {
            if diagram[i][j] == 'S' {
                beam_indices.insert(j);
                continue 'outer;
            }
            if diagram[i][j] == '^' && beam_indices.contains(&j) {
                result += 1;
                beam_indices.remove(&j);
                if j != 0 {
                    beam_indices.insert(j - 1);
                }
                if j != diagram[0].len() {
                    beam_indices.insert(j + 1);
                }
            }
        }
    }

    result
}