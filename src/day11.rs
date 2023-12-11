use std::fs;

#[derive(Debug)]
struct Galaxy {
    x: i64,
    y: i64,
}

pub(crate) fn problem1() {
    let input = fs::read_to_string("inputs/input11.txt").expect("should've been able to read");
    let (mut galaxies, (width, height)) = parse(&input);
    galaxies = expand_galaxies(galaxies, width, height, None);

    println!("{}", distances(&galaxies));
}

fn expand_galaxies(
    mut galaxies: Vec<Galaxy>,
    width: i64,
    height: i64,
    scale: Option<i64>,
) -> Vec<Galaxy> {
    let x_expansions = (0..width)
        .rev()
        .filter(|n| !galaxies.iter().any(|galaxy| galaxy.x == *n))
        .collect::<Vec<_>>();
    let y_expansions = (0..height)
        .rev()
        .filter(|n| !galaxies.iter().any(|galaxy| galaxy.y == *n))
        .collect::<Vec<_>>();

    for x in x_expansions {
        for galaxy in &mut galaxies {
            if galaxy.x > x {
                galaxy.x += if let Some(scale) = scale {
                    scale - 1
                } else {
                    1
                };
            }
        }
    }
    for y in y_expansions {
        for galaxy in &mut galaxies {
            if galaxy.y > y {
                galaxy.y += if let Some(scale) = scale {
                    scale - 1
                } else {
                    1
                };
            }
        }
    }
    galaxies
}

fn parse(s: &str) -> (Vec<Galaxy>, (i64, i64)) {
    let mut galaxies = Vec::new();
    let mut width = 0;
    let mut height = 0;
    for (y, line) in s.lines().enumerate() {
        height = y + 1;
        for (x, c) in line.chars().enumerate() {
            width = x + 1;
            if c == '#' {
                galaxies.push(Galaxy {
                    x: x as i64,
                    y: y as i64,
                });
            }
        }
    }

    (galaxies, (width as i64, height as i64))
}

fn find_taxi_distance(galaxy_1: &Galaxy, galaxy_2: &Galaxy) -> i64 {
    (galaxy_1.x - galaxy_2.x).abs() + (galaxy_1.y - galaxy_2.y).abs()
}

fn distances(galaxies: &[Galaxy]) -> i64 {
    (0..galaxies.len())
        .map(|i| {
            (i + 1..galaxies.len())
                .map(|j| find_taxi_distance(&galaxies[i], &galaxies[j]))
                .sum::<i64>()
        })
        .sum()
}

pub(crate) fn problem2() {
    let input = fs::read_to_string("inputs/input11.txt").expect("should've been able to read");
    let (mut galaxies, (width, height)) = parse(&input);
    galaxies = expand_galaxies(galaxies, width, height, Some(1_000_000));

    println!("{}", distances(&galaxies));
}
