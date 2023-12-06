pub(crate) fn problem1() {
    let input = std::fs::read_to_string("inputs/input6.txt").expect("should've been able to read");
    let (race_times, record_distances) = parse_lines(input);
    println!(
        "{}",
        race_times
            .iter()
            .zip(record_distances)
            .map(|(&time, distance)| find_ways_to_win(time, distance))
            .product::<usize>()
    )
}

fn parse_lines(input: String) -> (Vec<usize>, Vec<usize>) {
    let (line_1, line_2) = input.split_once('\n').unwrap();
    let race_times = line_1
        .split_whitespace()
        .skip(1)
        .map(|w| w.parse().unwrap())
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
    let lower_bound = (time as f64 - diff) / 2.0;
    let upper_bound = (time as f64 + diff) / 2.0;
    (upper_bound.floor() - lower_bound.ceil()) as usize + 1
}

pub(crate) fn problem2() {
    println!("not implemented");
}
