use std::fs;

fn main() {
    let test_input: String = fs::read_to_string("input.txt")
        .expect("Should have been able to read")
        .parse()
        .expect("Should have been able to parse");
    println!("{}", test_input);
}
