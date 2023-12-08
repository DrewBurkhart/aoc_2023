use std::{collections::HashMap, fs};

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
}

pub(crate) fn problem1() {
    let input = fs::read_to_string("inputs/input8.txt").expect("should've been able to read");
    println!("{}", solve(&input, false));
}

pub(crate) fn problem2() {
    let input = fs::read_to_string("inputs/input8.txt").expect("should've been able to read");
    println!("{}", solve(&input, true));
}

fn solve(input: &str, is_ghost: bool) -> i64 {
    let mut lines = input.lines();
    let directions = get_directions(lines.next().unwrap()).into_iter().cycle();

    let mut paths: HashMap<String, (String, String)> = HashMap::new();
    for line in lines.filter(|l| !l.is_empty()) {
        let parts = if !is_ghost {
            line.chars()
                .filter(|c| c.is_ascii_alphabetic() || c.is_whitespace())
                .collect::<String>()
        } else {
            line.chars()
                .filter(|c| c.is_alphabetic() || c.is_whitespace())
                .collect::<String>()
        };
        let mut parts = parts.split_ascii_whitespace();

        let (key, left, right) = (
            parts.next().unwrap().to_string(),
            parts.next().unwrap().to_string(),
            parts.next().unwrap().to_string(),
        );
        paths.insert(key, (left, right));
    }

    if !is_ghost {
        let mut current_location = "AAA";
        let mut count = 0;

        for direction in directions {
            count += 1;
            current_location = match direction {
                Direction::Left => &paths[current_location].0,
                Direction::Right => &paths[current_location].1,
            };

            if current_location == "ZZZ" {
                return count;
            }
        }
        return 0;
    } else {
        let end_with_a_counts = paths
            .keys()
            .filter(|key| key.ends_with('A'))
            .map(|mut curr_location| {
                let mut count = 0;

                for direction in directions.clone() {
                    count += 1;
                    curr_location = match direction {
                        Direction::Left => &paths[curr_location].0,
                        Direction::Right => &paths[curr_location].1,
                    };

                    if curr_location.ends_with('Z') {
                        return count;
                    }
                }

                return 0;
            })
            .collect::<Vec<_>>();

        end_with_a_counts.into_iter().reduce(lcm).unwrap()
    }
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

fn lcm(first: i64, second: i64) -> i64 {
    first * second / gcd(first, second)
}

fn gcd(first: i64, second: i64) -> i64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        std::mem::swap(&mut max, &mut min);
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}
