use std::fs;

fn step(r: usize, c: usize, d: usize) -> (usize, usize, usize) {
    let (dr, dc) = [(-1, 0), (0, 1), (1, 0), (0, -1)][d];
    ((r as isize + dr) as _, (c as isize + dc) as _, d)
}

fn energized_tiles(grid: &[&[u8]], start: (usize, usize, usize)) -> usize {
    let mut memory = vec![vec![[false; 4]; grid[0].len()]; grid.len()];
    let mut beams = vec![start];
    while !beams.is_empty() {
        let mut new_beams = Vec::with_capacity(beams.capacity());
        for (row, col, direction) in beams {
            if row >= grid.len() || col >= grid[0].len() {
                continue;
            }
            if memory[row][col][direction] {
                continue;
            }
            memory[row][col][direction] = true;
            match (grid[row][col], direction) {
                (b'.', _) => new_beams.push(step(row, col, direction)),
                (b'/', _) => new_beams.push(step(row, col, [1, 0, 3, 2][direction])),
                (b'\\', _) => new_beams.push(step(row, col, [3, 2, 1, 0][direction])),
                (b'|', 0 | 2) => new_beams.push(step(row, col, direction)),
                (b'-', 1 | 3) => new_beams.push(step(row, col, direction)),
                (b'|', _) => new_beams.extend([step(row, col, 0), step(row, col, 2)]),
                (b'-', _) => new_beams.extend([step(row, col, 1), step(row, col, 3)]),
                _ => unreachable!(),
            }
        }
        beams = new_beams;
    }
    memory
        .iter()
        .flat_map(|row| row)
        .filter(|x| x.iter().any(|&b| b))
        .count()
}

pub(crate) fn problem1() {
    let input = fs::read_to_string("inputs/input16.txt").expect("should've been able to read");
    let grid = input
        .trim()
        .split('\n')
        .map(str::as_bytes)
        .collect::<Vec<_>>();
    println!("{}", energized_tiles(&grid, (0, 0, 1)))
}

pub(crate) fn problem2() {
    let input = fs::read_to_string("inputs/input16.txt").expect("should've been able to read");
    let grid = input
        .trim()
        .split('\n')
        .map(str::as_bytes)
        .collect::<Vec<_>>();
    let p2 = (0..grid.len())
        .flat_map(|r| [(r, 0, 1), (r, grid[0].len() - 1, 3)])
        .chain((0..grid[0].len()).flat_map(|c| [(0, c, 2), (grid.len() - 1, c, 0)]))
        .map(|start| energized_tiles(&grid, start))
        .max()
        .unwrap();
    println!("{}", p2)
}
