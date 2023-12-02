use std::cmp;
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

pub fn problem2() {
    let mut sum: i32 = 0;
    let input = fs::read_to_string("inputs/input2.txt").expect("should've been able to read file");
    let lines: Vec<&str> = input.lines().collect();
    for l in lines {
        let (_, rounds) = l.split_once(':').unwrap();
        let [r, g, b] = rounds.split([',', ';']).fold([0, 0, 0], |mut acc, round| {
            let (num, color) = round[1..].split_once(" ").unwrap();
            let color = match color {
                "red" => 0,
                "green" => 1,
                _ => 2,
            };
            acc[color] = cmp::max(num.parse::<i32>().unwrap(), acc[color]);
            acc
        });
        sum += r * g * b;
    }
    println!("{}", sum);
}
