use std::fs;

pub(crate) fn problem1() {
    let input = fs::read_to_string("inputs/input18.txt").expect("should've been able to read");
    let mut points = parse_input(&input);

    points.insert(0, (0, 0));

    println!(
        "{}",
        calculate_polygon_area(&points) + count_ext_points(&points)
    );
}

fn calculate_polygon_area(points: &[(i64, i64)]) -> i64 {
    calculate_area(points) - count_ext_points(points) / 2 + 1
}

fn calculate_area(points: &[(i64, i64)]) -> i64 {
    ((1..points.len() - 1)
        .map(|i| points[i].0 * (points[i + 1].1 - points[i - 1].1))
        .sum::<i64>()
        / 2)
    .abs()
}

fn count_ext_points(points: &[(i64, i64)]) -> i64 {
    let mut point_count = 0;
    for i in 0..points.len() - 1 {
        point_count +=
            (points[i].0 - points[i + 1].0).abs() + (points[i].1 - points[i + 1].1).abs();
    }

    point_count
}

fn parse_input(s: &str) -> Vec<(i64, i64)> {
    let mut curr_pos = (0, 0);

    s.lines()
        .map(|line| line.split(' '))
        .map(|mut vals| {
            (
                vals.next().unwrap(),
                vals.next().unwrap().parse::<i64>().unwrap(),
            )
        })
        .map(|(dir, dist)| {
            curr_pos = match dir {
                "D" => (curr_pos.0, curr_pos.1 + dist),
                "U" => (curr_pos.0, curr_pos.1 - dist),
                "L" => (curr_pos.0 - dist, curr_pos.1),
                "R" => (curr_pos.0 + dist, curr_pos.1),
                _ => unreachable!(),
            };
            curr_pos
        })
        .collect::<Vec<_>>()
}

pub(crate) fn problem2() {
    println!("not implemented");
}
