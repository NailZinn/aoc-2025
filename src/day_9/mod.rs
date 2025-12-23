use std::{collections::HashSet, fs};

#[allow(dead_code)]
fn read_input() -> Vec<(i64, i64)> {
    fs::read_to_string("src/day_9/input.txt")
        .unwrap()
        .lines()
        .map(|x| x.split(',').collect())
        .map(|x: Vec<&str>| (x[0].parse().unwrap(), x[1].parse().unwrap()))
        .collect()
}

#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn solve_2() -> i64 {
    let coordinates = read_input();

    let mut figure_outline: HashSet<(i64, i64)> = HashSet::new();

    let (mut x1, mut y1) = coordinates[0];
    let mut index = 1;

    loop {
        if coordinates[index].0 != x1 {
            if coordinates[index].0 < x1 {
                for x in (coordinates[index].0 + 1..x1 + 1).rev() { figure_outline.insert((x, y1)); }
            } else {
                for x in x1..coordinates[index].0 { figure_outline.insert((x, y1)); }
            }
        } else if coordinates[index].1 != y1 {
            if coordinates[index].1 < y1 {
                for y in (coordinates[index].1 + 1..y1 + 1).rev() { figure_outline.insert((x1, y)); }
            } else {
                for y in y1..coordinates[index].1 { figure_outline.insert((x1, y)); }
            }
        }

        if index == 0 { break; }

        (x1, y1) = coordinates[index];
        index = (index + 1) % coordinates.len();
    }

    let mut rectangles: Vec<((i64, i64), (i64, i64), i64)> = vec![];

    for i in 0..coordinates.len() {
        for j in i + 1..coordinates.len() {
            rectangles.push((coordinates[i], coordinates[j], ((coordinates[i].0 - coordinates[j].0).abs() + 1) * ((coordinates[i].1 - coordinates[j].1).abs() + 1)));
        }
    }

    rectangles.sort_by(|x, y| y.2.cmp(&x.2));

    'outer:
    for rectangle in rectangles {
        for point in figure_outline.iter() {
            if rectangle.0.0.min(rectangle.1.0) < point.0 && point.0 < rectangle.0.0.max(rectangle.1.0) &&
               rectangle.0.1.min(rectangle.1.1) < point.1 && point.1 < rectangle.0.1.max(rectangle.1.1) {
                continue 'outer;
            }
        }

        return rectangle.2;
    }

    unreachable!();
}