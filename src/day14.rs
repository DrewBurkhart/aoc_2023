use std::fs;

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
pub(crate) fn problem2() {
    println!("not implemented");
}
