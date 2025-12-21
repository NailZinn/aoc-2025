use std::fs;

fn read_input() -> Vec<(i64, i64, i64)> {
    fs::read_to_string("src/day_8/input.txt")
        .unwrap()
        .lines()
        .map(|x| x.split(',').collect())
        .map(|x: Vec<&str>| (x[0].parse().unwrap(), x[1].parse().unwrap(), x[2].parse().unwrap()))
        .collect()
}

pub fn distance(p1: (i64, i64, i64), p2: (i64, i64, i64)) -> f64 {
    f64::sqrt((p1.0 - p2.0).pow(2) as f64 + (p1.1 - p2.1).pow(2) as f64 + (p1.2 - p2.2).pow(2) as f64)
}

pub fn solve_1() -> i64 {
    let coordinates = read_input();
    let mut leaders: Vec<usize> = coordinates.iter().enumerate().map(|(i, _)| i).collect();
    let mut circuits: Vec<Vec<usize>> = leaders.iter().map(|&x| vec![x]).collect();
    let mut distances: Vec<(usize, usize, f64)> = vec![];

    for i in 0..coordinates.len() {
        for j in i + 1..coordinates.len() {
            distances.push((i, j, distance(coordinates[i], coordinates[j])));
        }
    }

    distances.sort_by(|x, y| x.2.total_cmp(&y.2));

    for (i, j, _) in distances.into_iter().take(coordinates.len()) {
        if leaders[i] == leaders[j] { continue; }

        let mut circuit_to_move: Vec<usize> = vec![];
        let moved_leader = leaders[j];

        for &index in circuits[leaders[j]].iter() {
            circuit_to_move.push(index);
            leaders[index] = leaders[i];
        }

        circuits[moved_leader] = vec![];

        for index in circuit_to_move.into_iter() {
            circuits[leaders[i]].push(index);
        }
    }

    circuits.sort_by(|x, y| y.len().cmp(&x.len()));

    circuits
        .iter()
        .take(3)
        .map(|x| x.len() as i64)
        .product()
}