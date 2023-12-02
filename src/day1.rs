use rayon::prelude::*;
use regex::Regex;
use std::fs;

pub(crate) fn problem1() {
    let mut sum = 0;

    let test_input: String = fs::read_to_string("inputs/input1.txt")
        .expect("Should have been able to read")
        .parse()
        .expect("Should have been able to parse");
    let lines = test_input.lines();
    for line in lines {
        let (int1, int2) = extract_integers(line);
        sum += (int1 * 10) + int2;
    }
    println!("{}", sum);
}

fn extract_integers(input: &str) -> (i32, i32) {
    let matcher = Regex::new(r"\d").unwrap();

    // Find all integer matches in the input string
    let matches: Vec<_> = matcher.find_iter(input).collect();

    if matches.is_empty() {
        println!("No integers found.");
        return (0, 0);
    } else if matches.len() == 1 {
        // Only one integer found
        let integer = matches[0].as_str().parse::<i32>().unwrap();
        return (integer, integer);
    } else {
        // First and last integers found
        let first_integer = matches[0].as_str().parse::<i32>().unwrap();
        let last_integer = matches.last().unwrap().as_str().parse::<i32>().unwrap();
        return (first_integer, last_integer);
    }
}

pub(crate) fn problem2() {
    let sum: i32;

    let test_input: String = fs::read_to_string("inputs/input1.txt")
        .expect("Should have been able to read")
        .parse()
        .expect("Should have been able to parse");
    let lines = test_input.lines();

    // Define a regular expression pattern to match spelled-out digits
    let re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)").unwrap();

    // Use Rayon's parallel iterator to process lines in parallel
    sum = lines
        .par_bridge()
        .map(|line| {
            let (int1, int2) = extract_integers_from_string(line, &re);
            (int1 * 10) + int2
        })
        .sum();

    println!("{}", sum);
}

fn extract_integers_from_string(input: &str, re: &Regex) -> (i32, i32) {
    let mod_string = parse_to_mod_string(String::from(input), re);
    let matcher = Regex::new(r"\d").unwrap();
    let integers: Vec<i32> = matcher
        .find_iter(&mod_string)
        .map(|m| m.as_str().parse::<i32>())
        .filter_map(Result::ok)
        .collect();

    if integers.is_empty() {
        return (0, 0);
    } else if integers.len() == 1 {
        return (integers[0], integers[0]);
    } else {
        return (
            integers.first().unwrap().clone(),
            integers.last().unwrap().clone(),
        );
    }
}

fn parse_to_mod_string(input: String, re: &Regex) -> String {
    // Initialize an empty result string
    let mut replaced_input;

    let mut input = input;
    loop {
        // Replace the first occurrence of the pattern in the input
        let new_input = re
            .replace(&input, |caps: &regex::Captures| {
                match &caps[0] {
                    "one" => "o1e".to_string(),
                    "two" => "t2o".to_string(),
                    "three" => "t3e".to_string(),
                    "four" => "f4r".to_string(),
                    "five" => "f5e".to_string(),
                    "six" => "s6x".to_string(),
                    "seven" => "s7n".to_string(),
                    "eight" => "e8t".to_string(),
                    "nine" => "n9e".to_string(),
                    _ => caps[0].to_string(), // If not recognized, keep the original text
                }
            })
            .to_string();

        replaced_input = String::from(new_input.clone());

        // Check if there are more occurrences of the pattern in the new input
        if input == new_input {
            break; // No more replacements needed
        } else {
            input = new_input; // Continue with the new input
        }
    }
    replaced_input
}
