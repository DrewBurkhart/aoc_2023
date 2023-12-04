struct Game<'a> {
    player_numbers: Vec<&'a str>,
    winning_numbers: Vec<&'a str>,
}

pub(crate) fn problem1() {
    let input = std::fs::read_to_string("inputs/input4.txt").expect("should've been able to read");
    let sum: usize = input
        .lines()
        .map(|l| extract_game(l))
        .map(|game| {
            let win_count = game
                .player_numbers
                .iter()
                .filter(|&&item| game.winning_numbers.contains(&item))
                .count();
            if win_count > 1 {
                2usize.pow(win_count as u32 - 1)
            } else {
                win_count
            }
        })
        .sum();

    println!("{}", sum);
}

fn extract_game(l: &str) -> Game {
    let (_id, nums) = l.split_once(":").expect("should've had game id");
    let parts: Vec<&str> = nums.split('|').map(|s| s.trim()).collect();

    let player_numbers: Vec<&str> = parts[0].split_whitespace().collect();
    let winning_numbers: Vec<&str> = parts[1].split_whitespace().collect();
    Game {
        player_numbers,
        winning_numbers,
    }
}

pub(crate) fn problem2() {
    println!("Not implemented");
}
