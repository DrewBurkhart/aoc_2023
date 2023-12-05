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
    let input = std::fs::read_to_string("inputs/input5.txt").expect("should've been able to read");
    let (seeds, rest) = input.trim_end().split_once("\n\n").unwrap();
    let seeds = parse_seeds(seeds);
    let maps = parse_maps(rest);
    println!("{}", find_lowest_location_from_range(seeds, &maps));
}

fn find_lowest_location_from_range(
    seed_ranges: Vec<usize>,
    maps: &[Vec<(usize, usize, usize)>],
) -> usize {
    let seed_ranges = seed_ranges
        .iter()
        .tuples()
        .map(|(&a, len)| (a, a + len))
        .collect::<Vec<_>>();
    let mapped_seed_ranges = maps.iter().fold(seed_ranges, |seeds, mappings| {
        seeds
            .iter()
            .flat_map(|&(range_start, range_len)| {
                let mut mapped_ranges = Vec::new();
                let mut unmapped_ranges = vec![(range_start, range_len)];
                for &(dest, src, map_len) in mappings {
                    let mut new_unmapped_ranges = Vec::new();
                    for (start, end) in unmapped_ranges {
                        let seg_a = (start, end.min(src));
                        let seg_b = (start.max(src), (src + map_len).min(end));
                        let seg_c = ((src + map_len).max(start), end);
                        if seg_a.1 > seg_a.0 {
                            new_unmapped_ranges.push(seg_a);
                        }
                        if seg_b.1 > seg_b.0 {
                            mapped_ranges.push((seg_b.0 - src + dest, seg_b.1 - src + dest));
                        }
                        if seg_c.1 > seg_c.0 {
                            new_unmapped_ranges.push(seg_c);
                        }
                    }
                    unmapped_ranges = new_unmapped_ranges;
                }
                mapped_ranges.extend(unmapped_ranges);
                mapped_ranges
            })
            .collect::<Vec<_>>()
    });
    mapped_seed_ranges
        .iter()
        .map(|&(start, _)| start)
        .min()
        .unwrap()
}
