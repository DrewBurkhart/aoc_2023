use std::{collections::HashMap, fs};

pub(crate) fn problem1() {
    let input = fs::read_to_string("inputs/input8.txt").expect("should've been able to read");

    let directions = get_directions(input.lines().next().unwrap())
        .into_iter()
        .cycle();

    let mut paths: HashMap<String, (String, String)> = HashMap::new();
    for line in input.lines().skip(1).filter(|l| !l.is_empty()) {
        let parts = line
            .chars()
            .filter(|c| c.is_ascii_alphabetic() || c.is_whitespace())
            .collect::<String>();
        let mut parts = parts.split_ascii_whitespace();

        let (key, left, right) = (
            parts.next().unwrap().to_string(),
            parts.next().unwrap().to_string(),
            parts.next().unwrap().to_string(),
        );
        paths.insert(key, (left, right));
    }

    let mut current_location = "AAA";
    let mut count = 0;

    for direction in directions {
        count += 1;
        current_location = match direction {
            Direction::Left => &paths[current_location].0,
            Direction::Right => &paths[current_location].1,
        };

        if current_location == "ZZZ" {
            println!("{}", count);
            return;
        }
    }
    println!("0");
}

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
}

fn get_directions(input: &str) -> Vec<Direction> {
    input
        .chars()
        .filter_map(|char| match char {
            'L' => Some(Direction::Left),
            'R' => Some(Direction::Right),
            _ => None,
        })
        .collect()
}

pub(crate) fn problem2() {
    println!("not implemented");
}
