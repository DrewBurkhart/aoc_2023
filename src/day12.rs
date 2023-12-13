use std::fs;

use itertools::Itertools;

fn calculate_possibilities<'a>(
    s: &'a [u8],
    within: Option<usize>,
    remaining: &'a [usize],
) -> usize {
    if s.is_empty() {
        if within.is_none() && remaining.is_empty() {
            return 1;
        }
        if remaining.len() == 1 && within == Some(remaining[0]) {
            return 1;
        }
        return 0;
    }
    let possible = s.iter().filter(|&&c| c == b'#' || c == b'?').count();
    let remaining_sum = remaining.iter().sum();
    if possible + within.unwrap_or(0) < remaining_sum {
        return 0;
    }
    if within.is_some() && remaining.is_empty() {
        return 0;
    }
    if s[0] == b'.' && within.is_some_and(|x| x != remaining[0]) {
        return 0;
    }
    let mut possible = 0;
    if matches!((s[0], within), (b'.', Some(_))) {
        possible += calculate_possibilities(&s[1..], None, &remaining[1..]);
    }
    if matches!((s[0], within), (b'?', Some(x)) if x == remaining[0]) {
        possible += calculate_possibilities(&s[1..], None, &remaining[1..]);
    }
    if matches!((s[0], within), (b'#' | b'?', Some(_))) {
        possible += calculate_possibilities(&s[1..], within.map(|x| x + 1), remaining);
    }
    if matches!((s[0], within), (b'#' | b'?', None)) {
        possible += calculate_possibilities(&s[1..], Some(1), remaining);
    }
    if matches!((s[0], within), (b'.' | b'?', None)) {
        possible += calculate_possibilities(&s[1..], None, remaining);
    }
    possible
}

pub(crate) fn problem1() {
    let input = fs::read_to_string("inputs/input12.txt").expect("should've been able to read");
    let p1 = input
        .trim()
        .split('\n')
        .map(|l| {
            let (vents, rest) = l.split_once(' ').unwrap();
            let nums = rest
                .split(',')
                .map(|w| w.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let p1 = calculate_possibilities(vents.as_bytes(), None, &nums);
            p1
        })
        .sum::<usize>();
    println!("{}", p1);
}

pub(crate) fn problem2() {
    let input = fs::read_to_string("inputs/input12.txt").expect("should've been able to read");
    let p2 = input
        .trim()
        .split('\n')
        .map(|l| {
            let (vents, rest) = l.split_once(' ').unwrap();
            let nums = rest
                .split(',')
                .map(|w| w.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let expanded_vents = (0..5).map(|_| vents).join("?");
            let expanded_nums = (0..5).flat_map(|_| &nums).copied().collect::<Vec<_>>();
            let p2 = calculate_possibilities(expanded_vents.as_bytes(), None, &expanded_nums);
            p2
        })
        .sum::<usize>();
    println!("{}", p2);
}
