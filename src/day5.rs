use itertools::Itertools;

pub(crate) fn problem1() {
    let input = std::fs::read_to_string("inputs/input5.txt").expect("should've been able to read");
    let (seeds, rest) = input.trim_end().split_once("\n\n").unwrap();
    let seeds = parse_seeds(seeds);
    let maps = parse_maps(rest);
    println!("{}", find_lowest_location(seeds.clone(), &maps));
}

fn parse_seeds(seeds: &str) -> Vec<usize> {
    seeds
        .split_whitespace()
        .skip(1)
        .map(|seed| seed.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

fn parse_maps(input: &str) -> Vec<Vec<(usize, usize, usize)>> {
    input
        .split("\n\n")
        .map(|map| {
            map.split('\n')
                .skip(1)
                .map(|line| {
                    line.split_whitespace()
                        .map(|val| val.parse::<usize>().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn find_lowest_location(seeds: Vec<usize>, maps: &[Vec<(usize, usize, usize)>]) -> usize {
    let answer = maps.iter().fold(seeds, |seeds, mappings| {
        seeds
            .iter()
            .map(|&seed| {
                mappings
                    .iter()
                    .find(|&&(_, src, range)| (src..src + range).contains(&seed))
                    .map(|(dest, src, _)| dest + seed - src)
                    .unwrap_or(seed)
            })
            .collect::<Vec<_>>()
    });
    *answer.iter().min().unwrap()
}
pub(crate) fn problem2() {
    println!("not implemented");
}
