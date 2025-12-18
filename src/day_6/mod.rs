use std::fs;

fn read_input() -> (Vec<Vec<i64>>, Vec<String>) {
    let lines: Vec<Vec<String>> = fs::read_to_string("src/day_6/input.txt")
        .unwrap()
        .lines()
        .map(|x| x.split(' ').map(String::from).filter(|x| !x.is_empty()).collect())
        .collect();

    println!("{:?}", lines);

    let mut operands: Vec<Vec<i64>> = Vec::new();

    for j in 0..lines[0].len() {
        operands.push(Vec::new());

        for i in 0..lines.len() - 1 {
            operands[j].push(lines[i][j].parse().unwrap());
        }
    }

    let operators: Vec<String> = lines
        .last()
        .unwrap()
        .to_vec();

    (operands, operators)
}

pub fn solve_1() -> i64 {
    let (operands, operators) = read_input();

    operands
        .iter()
        .enumerate()
        .map(|(i, x)| match operators[i].as_str() {
            "*" => x.iter().product::<i64>(),
            "+" => x.iter().sum(),
            _ => panic!("incorrect operator {}", operators[i])
        })
        .sum()
}