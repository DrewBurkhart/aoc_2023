struct Game<'a> {
    player_numbers: Vec<&'a str>,
    winning_numbers: Vec<&'a str>,
}

pub(crate) fn problem1() {
    let input = std::fs::read_to_string("inputs/input4.txt").expect("should've been able to read");
    let lines: Vec<&str> = input.lines().collect();
    let sum: usize = lines
        .iter()
        .map(|&l| {
            let game = extract_game(l);
            let win_count = score_game(&game);
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
    let (player_numbers, winning_numbers) = nums
        .split_once('|')
        .expect("should've had player and winning numbers");
    Game {
        player_numbers: player_numbers.trim().split_whitespace().collect(),
        winning_numbers: winning_numbers.trim().split_whitespace().collect(),
    }
}

fn score_game(game: &Game) -> usize {
    game.player_numbers
        .iter()
        .filter(|&&item| game.winning_numbers.contains(&item))
        .count()
}

pub(crate) fn problem2() {
    let input = std::fs::read_to_string("inputs/input4.txt").expect("should've been able to read");
    let lines: Vec<&str> = input.lines().collect();
    let mut matrix = vec![1; lines.len()];

    for (i, &l) in lines.iter().enumerate() {
        let game = extract_game(l);
        for x in 1..=score_game(&game) {
            matrix[i + x] += matrix[i];
        }
    }
    println!("{}", matrix.iter().sum::<i32>());
}
