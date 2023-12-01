use rayon::prelude::*;
use regex::Regex;
use std::borrow::Cow;
use std::fs;

fn main() {
    let mut sum = 0;

    let test_input: String = fs::read_to_string("input.txt")
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
            let (int1, int2) = extract_integers(line, &re);
            (int1 * 10) + int2
        })
        .sum();

    println!("{}", sum);
}

fn extract_integers(input: &str, re: &Regex) -> (i32, i32) {
    let mod_string = parse_to_mod_string(input, re);
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
        return (*integers.first().unwrap(), *integers.last().unwrap());
    }
}

fn parse_to_mod_string(input: &str, re: &Regex) -> String {
    // Initialize an empty result string
    let mut replaced_input;

    let mut input = input;
    loop {
        // Replace the first occurrence of the pattern in the input
        let new_input = re.replace(input, |caps: &regex::Captures| {
            match &caps[0] {
                "one" => Cow::Borrowed("o1e"),
                "two" => Cow::Borrowed("t2o"),
                "three" => Cow::Borrowed("t3e"),
                "four" => Cow::Borrowed("f4r"),
                "five" => Cow::Borrowed("f5e"),
                "six" => Cow::Borrowed("s6x"),
                "seven" => Cow::Borrowed("s7n"),
                "eight" => Cow::Borrowed("e8t"),
                "nine" => Cow::Borrowed("n9e"),
                _ => Cow::Owned(caps[0].to_string()), // If not recognized, create an owned string
            }
        });

        replaced_input = new_input.to_string();

        // Check if there are more occurrences of the pattern in the new input
        if input == new_input {
            break; // No more replacements needed
        } else {
            input = &new_input; // Continue with the new input
        }
    }
    replaced_input
}

