use std::fs;

pub(crate) fn problem1() {
    let input = fs::read_to_string("inputs/input9.txt").expect("shoud've been able to read");
    let lines = input.lines().map(|line| {
        line.split_ascii_whitespace()
            .map(|n| n.parse::<i64>().unwrap())
            .collect::<Vec<_>>()
    });

    println!(
        "{}",
        lines
            .map(|line| predict_next_value(line))
            .sum::<i64>()
    );
}

fn predict_next_value(values: Vec<i64>) -> i64 {
    let mut differences = Vec::new();

    let mut curr_difference = values;

    while !curr_difference.iter().all(|n| *n == 0) {
        differences.push(curr_difference.clone());
        curr_difference = curr_difference[1..]
            .iter()
            .zip(curr_difference.iter())
            .map(|(a, b)| a - b)
            .collect();
    }

        differences.into_iter().map(|v| *v.last().unwrap()).sum()

}

pub(crate) fn problem2() {
    println!("not implemented");
}
