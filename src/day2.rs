use std::fs;

pub(crate) fn problem1() {
    let input = fs::read_to_string("inputs/input2.txt").expect("should've been able to read file");
    let lines: Vec<&str> = input.lines().collect();
    let mut sum: i32 = 0;
    for l in lines {
        let (game_nb, turns) = l[5..].split_once(":").unwrap();
        if !turns.split([',', ';']).any(is_invalid) {
            sum += game_nb.parse::<i32>().unwrap();
        };
    }
    println!("{}", sum);
}

fn is_invalid(turns: &str) -> bool {
    let (nb, color) = turns[1..].split_once(" ").unwrap();
    let nb = nb.parse::<i32>().unwrap();
    color == "red" && nb > 12 || color == "green" && nb > 13 || color == "blue" && nb > 14
}
