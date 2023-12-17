use std::fs;

pub(crate) fn problem1() {
    let input = fs::read_to_string("inputs/input15.txt").expect("should've been able to read");

    let sum: u64 = input
        .split(',')
        .map(|string| {
            let cleaned = string
                .chars()
                .filter(|&char| char != ' ' && char != '\n')
                .collect::<String>();
            cleaned
                .chars()
                .fold(0, |acc, char| ((acc + char as u64) * 17) % 256)
        })
        .sum();

    println!("{}", sum);
}

pub(crate) fn problem2() {
    let input = fs::read_to_string("inputs/input15.txt").expect("should've been able to read");
    let mut boxes = vec![vec![]; 256];

    for instruction in input.trim().split(',') {
        if instruction.ends_with('-') {
            let key = instruction[0..instruction.len() - 1].to_string();
            let h = hash(&key);
            if let Some(position) = boxes[h].iter().position(|v: &Lens| v.key == key) {
                boxes[h].remove(position);
            }
        } else {
            let (key, value) = instruction.split_once('=').unwrap();
            let value = value.parse().unwrap();
            let key = key.to_string();
            let h = hash(&key);
            if let Some(position) = boxes[h].iter().position(|v| v.key == key) {
                boxes[h][position].value = value;
            } else {
                boxes[h].push(Lens {
                    key: key.to_string(),
                    value,
                });
            }
        }
    }

    println!(
        "{}",
        boxes
            .into_iter()
            .enumerate()
            .map(|(i, b)| {
                (i + 1)
                    * b.into_iter()
                        .enumerate()
                        .map(|(k, lens)| ((k + 1) * lens.value))
                        .sum::<usize>()
            })
            .sum::<usize>()
    );
}

fn hash(s: &str) -> usize {
    s.chars().fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
}

#[derive(Clone, Debug)]
struct Lens {
    key: String,
    value: usize,
}
