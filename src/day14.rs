use std::{collections::HashMap, fs};

pub(crate) fn problem1() {
    let input = fs::read_to_string("inputs/input14.txt").expect("Failed to read input file");
    let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
    let mut platform: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    tilt_platform_north(&mut platform);
    println!("{}", calculate_load(&platform));
}

fn tilt_platform_north(platform: &mut Vec<Vec<char>>) {
    for col in 0..platform[0].len() {
        let mut stop_row = 0;

        for row in 0..platform.len() {
            if platform[row][col] == '#' {
                stop_row = row + 1;
            } else if platform[row][col] == 'O' {
                if row <= platform.len() {
                    platform[stop_row][col] = 'O';
                    stop_row += 1;
                }

                if stop_row - 1 != row {
                    platform[row][col] = '.';
                }
            }
        }
    }
}

fn calculate_load(platform: &Vec<Vec<char>>) -> usize {
    (0..platform.len())
        .map(|row| {
            (0..platform[0].len())
                .filter(|&col| platform[row][col] == 'O')
                .map(|_| platform.len() - row)
                .sum::<usize>()
        })
        .sum()
}

fn rotate_platform(platform: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut rotated_platform = vec![vec!['.'; platform.len()]; platform[0].len()];
    for row in 0..platform.len() {
        for col in 0..platform[0].len() {
            rotated_platform[col][platform.len() - 1 - row] = platform[row][col];
        }
    }
    rotated_platform
}

pub(crate) fn problem2() {
    let input = fs::read_to_string("inputs/input14.txt").expect("Failed to read input file");
    let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
    let mut platform: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    let mut memory = HashMap::new();
    for i in 1..1000000000 {
        for _ in 0..4 {
            tilt_platform_north(&mut platform);
            platform = rotate_platform(&platform);
        }
        // If we've seen this iteration of the platform before, check if it is
        // evenly divisible by the number of checks remaining. If it is, we can
        // stop iterating.
        if let Some(seen_at) = memory.insert(platform.clone(), i) {
            if (1000000000 - i) % (i - seen_at) == 0 {
                break;
            }
        }
    }

    println!("{}", calculate_load(&platform));
}
