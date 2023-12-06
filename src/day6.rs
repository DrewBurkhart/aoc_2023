pub(crate) fn problem1() {
    let input = std::fs::read_to_string("inputs/input6.txt").expect("should've been able to read");
    let (race_times, record_distances) = parse_lines_multiple(&input);
    println!(
        "{}",
        race_times
            .iter()
            .zip(record_distances)
            .map(|(&time, distance)| find_ways_to_win(time, distance))
            .product::<usize>()
    )
}

fn parse_lines_multiple(input: &str) -> (Vec<usize>, Vec<usize>) {
    let (line_1, line_2) = input.split_once('\n').unwrap();
    let race_times = line_1
        .split_whitespace()
        .skip(1)
        .map(|word| word.parse().unwrap())
        .collect::<Vec<_>>();
    let record_distances = line_2
        .split_whitespace()
        .skip(1)
        .map(|word| word.parse().unwrap())
        .collect::<Vec<_>>();
    (race_times, record_distances)
}

fn find_ways_to_win(time: usize, distance: usize) -> usize {
    let diff = ((time * time - 4 * distance) as f64).sqrt();
    let lower_bound = ((time as f64 - diff) / 2.0) + 1.0;
    let upper_bound = ((time as f64 + diff) / 2.0) - 1.0;
    (upper_bound.ceil() - lower_bound.floor()) as usize + 1
}

pub(crate) fn problem2() {
    let input = std::fs::read_to_string("inputs/input6.txt").expect("should've been able to read");
    let (race_times, record_distances) = parse_lines_single(&input);
    println!("{}", find_ways_to_win(race_times, record_distances));
}

fn parse_lines_single(input: &str) -> (usize, usize) {
    let (line_1, line_2) = input.split_once('\n').unwrap();
    let race_times = line_1
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse()
        .unwrap();
    let record_distances = line_2
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse()
        .unwrap();
    (race_times, record_distances)
}
