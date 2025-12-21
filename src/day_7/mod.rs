use std::{collections::HashSet, fs, iter::repeat};

#[allow(dead_code)]
fn read_input() -> Vec<Vec<char>> {
    fs::read_to_string("src/day_7/input.txt")
        .unwrap()
        .lines()
        .map(|x| x.chars().collect())
        .collect()
}

#[allow(dead_code)]
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
                if j > 0 {
                    beam_indices.insert(j - 1);
                }
                if j < diagram[0].len() {
                    beam_indices.insert(j + 1);
                }
            }
        }
    }

    result
}

#[allow(dead_code)]
pub fn solve_2() -> i64 {
    let diagram = read_input();

    let mut tree: Vec<i64> = Vec::from_iter(repeat(0).take(diagram[0].len()));
    let start_index = diagram[0].iter().position(|&x| x == 'S').unwrap();
    tree[start_index] = 1;

    for i in 1..diagram.len() {
        for j in 0..diagram[0].len() {
            if diagram[i][j] == '^' {
                if j > 0 {
                    tree[j - 1] += tree[j];
                }
                if j < diagram[0].len() {
                    tree[j + 1] += tree[j];
                }
                tree[j] = 0;
            }
        }
    }


    tree.iter().sum()
}