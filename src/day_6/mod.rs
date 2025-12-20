use std::fs;

fn read_input() -> (Vec<Vec<i64>>, Vec<char>) {
    let lines: Vec<Vec<String>> = fs::read_to_string("src/day_6/input.txt")
        .unwrap()
        .lines()
        .map(|x| x.split(' ').map(String::from).filter(|x| !x.is_empty()).collect())
        .collect();

    let mut operands: Vec<Vec<i64>> = Vec::new();

    for j in 0..lines[0].len() {
        operands.push(Vec::new());

        for i in 0..lines.len() - 1 {
            operands[j].push(lines[i][j].parse().unwrap());
        }
    }

    let operators: Vec<char> = lines[lines.len() - 1]
        .iter()
        .map(|x| x.chars().collect::<Vec<char>>()[0])
        .collect();

    (operands, operators)
}

pub fn solve_1() -> i64 {
    let (operands, operators) = read_input();

    operands
        .iter()
        .enumerate()
        .map(|(i, x)| match operators[i] {
            '*'=> x.iter().product::<i64>(),
            '+' => x.iter().sum(),
            _ => unreachable!()
        })
        .sum()
}

pub fn solve_2() -> i64 {
    let lines: Vec<String> = fs::read_to_string("src/day_6/input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let operators: Vec<char> = lines[lines.len() - 1]
        .chars()
        .filter(|x| !x.is_whitespace())
        .collect();

    let raw_operand_lines: Vec<Vec<char>> = lines
        .iter()
        .take(lines.len() - 1)
        .map(|x| x.chars().collect())
        .collect();

    let mut operands: Vec<Vec<i64>> = vec![vec![]];
    let mut current_operands: Vec<i64> = vec![];
    let mut column = 0;

    for j in 0..raw_operand_lines[0].len() {
        let mut is_empty_column = true;

        for i in 0..raw_operand_lines.len() {
            if !raw_operand_lines[i][j].is_whitespace() {
                is_empty_column = false;
                break;
            }
        }

        if is_empty_column {
            for &operand in current_operands.iter() {
                operands[column].push(operand);
            }
            operands.push(Vec::new());
            column += 1;
            current_operands.clear();
            continue;
        }

        let mut number = 0;

        for i in 0..raw_operand_lines.len() {
            if let Some(ch) = raw_operand_lines[i].get(j) && !ch.is_whitespace() {
                number = number * 10 + ch.to_digit(10).unwrap() as i64;
            }
        }

        current_operands.push(number);
    }

    for &operand in current_operands.iter() {
        operands[column].push(operand);
    }

    operands
        .iter()
        .enumerate()
        .map(|(i, x)| match operators[i] {
            '*' => x.iter().product::<i64>(),
            '+' => x.iter().sum(),
            _ => unreachable!()
        })
        .sum()
}