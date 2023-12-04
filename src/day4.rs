struct Game<'a> {
    player_numbers: Vec<&'a str>,
    winning_numbers: Vec<&'a str>,
}

pub(crate) fn problem1() {
    let input = std::fs::read_to_string("inputs/input4.txt").expect("should've been able to read");
    let lines = input.lines();
    let mut sum: usize = 0;
    for l in lines {
        let game = extract_game(l);
        let win_count = game
            .player_numbers
            .iter()
            .filter(|&&item| game.winning_numbers.contains(&item))
            .count();
        let score = if win_count > 1 {
            let mut sum = 1;
            for _ in 1..win_count {
                sum *= 2;
            }
            sum
        } else {
            win_count
        };

        sum += score
    }
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
