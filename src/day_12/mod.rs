use std::fs;

fn read_input() -> (Vec<Vec<Vec<char>>>, Vec<((i32, i32), Vec<i32>)>) {
    let lines: Vec<String> = fs::read_to_string("src/day_12/input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .filter(|x| !x.is_empty())
        .collect();

    let mut presents: Vec<Vec<Vec<char>>> = vec![];
    let mut presents_count = 0;
    let mut regions: Vec<((i32, i32), Vec<i32>)> = vec![];

    let mut insert_presents = true;
    
    for line in lines {
        if line.contains('x') { insert_presents = false; }

        if insert_presents {
            if line.contains(':') {
                presents.push(vec![]);
                presents_count += 1;
                continue;
            }
            presents[presents_count - 1].push(line.chars().collect());
        }
        else {
            let split: Vec<&str> = line.split(": ").collect();
            let size: Vec<i32> = split[0].split('x').map(|x| x.parse().unwrap()).collect();
            let presents_count: Vec<i32> = split[1].split(' ').map(|x| x.parse().unwrap()).collect();

            regions.push(((size[0], size[1]), presents_count));
        }
    }

    (presents, regions)
}

pub fn solve_1() -> i32 {
    let (presents, regions) = read_input();

    let squares: Vec<i32> = presents
        .iter()
        .map(|present| {
            let (mut length, mut width) = (0, 0);

            for y in 0..present.len() {
                let mut width_increased = false;
                for x in 0..present[0].len() {
                    if present[y][x] == '.' { continue; }
                    width_increased = true;
                    length = length.max(x + 1);
                }
                if width_increased { width += 1; }
            }

            (length * width) as i32
        })
        .collect();

    regions
        .iter()
        .filter(|((width, length), presents_count)| width * length >= presents_count.iter().enumerate().map(|(i, x)| squares[i] * x).sum())
        .count() as i32
}