use std::fs;

#[allow(dead_code)]
const DIAL_SIZE: i32 = 100;

#[allow(dead_code)]
pub fn solve_1() -> i32 {
    let rotations = fs::read_to_string("src/day_1/input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let mut dial = 50;
    let mut result = 0;

    for rotation in rotations {
        let (direction, steps_str) = rotation.split_at(1);
        let steps = steps_str.parse::<i32>().unwrap();

        dial = match direction {
            "L" => (dial - steps + DIAL_SIZE) % DIAL_SIZE,
            "R" => (dial + steps) % DIAL_SIZE,
            _ => dial
        };

        if dial == 0 {
            result += 1;
        }
    }

    result
}

#[allow(dead_code)]
pub fn solve_2() -> i32 {
    let rotations = fs::read_to_string("src/day_1/input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let mut dial = 50;
    let mut result = 0;

    for rotation in rotations {
        let (direction, steps_str) = rotation.split_at(1);
        let steps = steps_str.parse::<i32>().unwrap();

        let mut left_to_right_rotation_conversion_offset = 0;

        if dial == 0
        {
            if steps < DIAL_SIZE || steps > DIAL_SIZE && steps % DIAL_SIZE != 0
            {
                left_to_right_rotation_conversion_offset = DIAL_SIZE;
            }
        }
        else
        {
            if (steps - dial) % DIAL_SIZE == 0
            {
                left_to_right_rotation_conversion_offset = DIAL_SIZE;
            }
            else if steps > dial && steps % DIAL_SIZE != 0 && steps % DIAL_SIZE > dial
            {
                left_to_right_rotation_conversion_offset = 2 * DIAL_SIZE;
            }
        }

        let unbounded_dial = match direction {
            "L" => dial + (steps + left_to_right_rotation_conversion_offset) / DIAL_SIZE * DIAL_SIZE - steps % DIAL_SIZE,
            "R" => dial + steps,
            _ => 0
        };

        result += unbounded_dial / DIAL_SIZE;
        dial = unbounded_dial % DIAL_SIZE;
    }

    result
}