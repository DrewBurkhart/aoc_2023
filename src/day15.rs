use std::fs;

pub(crate) fn problem1() {
    let input = fs::read_to_string("inputs/input15.txt").expect("should've been able to read");

    let sum: u64 = input
        .split(',')
        .map(|string| {
            let cleaned = string
                .chars()
                .filter(|&char| char != ' ' && char != '\n')
                .collect::<String>();
            cleaned
                .chars()
                .fold(0, |acc, char| ((acc + char as u64) * 17) % 256)
        })
        .sum();

    println!("{}", sum);
}

pub(crate) fn problem2() {
    println!("not implemented");
}
