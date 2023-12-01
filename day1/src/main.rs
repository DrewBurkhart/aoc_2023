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
        println!("VALUE: {}", line);
        let (int1, int2) = extract_integers(line);
        println!("ANSWER: {}\n", ((int1 * 10) + int2));
        sum += (int1 * 10) + int2;
    }
    println!("{}", sum);
}

fn extract_integers(input: &str) -> (i32, i32) {
    let matcher = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();

    // Find all integer matches in the input string
    let matches: Vec<_> = matcher.find_iter(input).collect();

    if matches.is_empty() {
        println!("No integers found.");
        return (0, 0);
    } else if matches.len() == 1 {
        // Only one integer found
        let matched_text = matches[0].as_str();
        if let Ok(parsed_integer) = matched_text.parse::<i32>() {
            return (parsed_integer, parsed_integer);
        } else {
            let parsed_integer = parse_to_int(matched_text);
            return (parsed_integer, parsed_integer);
        }
    } else {
        // First and last integers found
        let first_matched_text = matches[0].as_str();
        let last_matched_text = matches.last().unwrap().as_str();
        let first_integer = first_matched_text
            .parse::<i32>()
            .unwrap_or_else(|_| parse_to_int(first_matched_text));
        let last_integer = last_matched_text
            .parse::<i32>()
            .unwrap_or_else(|_| parse_to_int(last_matched_text));

        return (first_integer, last_integer);
    }
}

fn parse_to_int(input: &str) -> i32 {
    return match input {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => 0,
    };
}
