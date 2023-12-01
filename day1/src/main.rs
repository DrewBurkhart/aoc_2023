use regex::Regex;
use std::fs;

fn main() {
    let mut sum = 0;

    let test_input: String = fs::read_to_string("input.txt")
        .expect("Should have been able to read")
        .parse()
        .expect("Should have been able to parse");
    let lines = test_input.lines();
    for line in lines {
        let (int1, int2) = extract_integers(line);
        println!("{}", ((int1 * 10) + int2));
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
