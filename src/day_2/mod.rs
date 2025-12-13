use std::fs;

#[allow(dead_code)]
pub fn solve_1() -> i64 {
    let ranges = fs::read_to_string("src/day_2/input.txt")
        .unwrap()
        .trim()
        .split(',')
        .map(|x| { x.split('-').collect::<Vec<&str>>() })
        .map(|x| { (String::from(x[0]), String::from(x[1])) })
        .collect::<Vec<(String, String)>>();

    let mut result: i64 = 0;

    for range in ranges {
        let (left, right) = (range.0.parse::<i64>().unwrap(), range.1.parse::<i64>().unwrap());
        let mut id = left;

        'outer:
        while id <= right {
            let id_string = id.to_string();
            let id_length = id_string.len();

            if id_length % 2 != 0 {
                id += 1;
                continue;
            }

            for i in 0..id_length / 2 {
                if id_string.as_bytes()[i] != id_string.as_bytes()[i + id_length / 2] {
                    id += 1;
                    continue 'outer;
                }
            }

            result += id;
            id += 1;
        }
    }

    result
}

#[allow(dead_code)]
pub fn solve_2() -> i64 {
    let ranges = fs::read_to_string("src/day_2/input.txt")
        .unwrap()
        .trim()
        .split(',')
        .map(|x| { x.split('-').collect::<Vec<&str>>() })
        .map(|x| { (String::from(x[0]), String::from(x[1])) })
        .collect::<Vec<(String, String)>>();

    let mut result: i64 = 0;

    for range in ranges {
        let (left, right) = (range.0.parse::<i64>().unwrap(), range.1.parse::<i64>().unwrap());
        let mut id = left;

        'ids:
        while id <= right {
            let id_string = id.to_string();
            let id_length = id_string.len();

            'pattern_sizes:
            for pattern_size in 1..id_length / 2 + 1 {

                if id_length % pattern_size != 0 {
                    continue;
                }

                let mut i = pattern_size;

                while i < id_length - pattern_size + 1 {
                    if id_string.as_bytes()[0..pattern_size] != id_string.as_bytes()[i..i + pattern_size] {
                        continue 'pattern_sizes;
                    }
                    i += pattern_size;
                }

                result += id;
                id += 1;

                continue 'ids;
            }

            id += 1;
        }
    }

    result
}