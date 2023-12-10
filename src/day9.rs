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
            .map(|line| predict_next_value(line, false))
            .sum::<i64>()
    );
}

fn predict_next_value(values: Vec<i64>, beginning: bool) -> i64 {
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

    if !beginning {
        differences.into_iter().map(|v| *v.last().unwrap()).sum()
    } else {
        let mut running_difference = 0;
        differences.into_iter().rev().for_each(|v| {
            running_difference = v.first().unwrap() - running_difference;
        });

        running_difference
    }
}

pub(crate) fn problem2() {
    let input = fs::read_to_string("inputs/input9.txt").expect("shoud've been able to read");
    let lines = input.lines().map(|line| {
        line.split_ascii_whitespace()
            .map(|n| n.parse::<i64>().unwrap())
            .collect::<Vec<_>>()
    });

    println!(
        "{}",
        lines
            .map(|line| predict_next_value(line, true))
            .sum::<i64>()
    );
}
