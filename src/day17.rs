use std::{
    collections::{BinaryHeap, HashMap},
    fs,
};

fn find_shortest_path(grid: &[&[u8]], min_step: isize, max_step: isize) -> i64 {
    let mut distances = HashMap::new();
    let mut priority_queue = BinaryHeap::from_iter([(0, (0, 0, (0, 0)))]);

    while let Some((cost, (row, col, direction))) = priority_queue.pop() {
        if (row, col) == (grid.len() - 1, grid[0].len() - 1) {
            return -cost;
        }

        if let Some(&current_cost) = distances.get(&(row, col, direction)) {
            if -cost > current_cost {
                continue;
            }
        }

        for (delta_row, delta_col) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if direction == (delta_row, delta_col) || direction == (-delta_row, -delta_col) {
                continue;
            }

            let mut next_cost = -cost;

            for distance in 1..=max_step {
                let next_row = (row as isize + delta_row * distance) as usize;
                let next_col = (col as isize + delta_col * distance) as usize;

                if next_row >= grid.len() || next_col >= grid[0].len() {
                    continue;
                }

                next_cost += (grid[next_row][next_col] - b'0') as i64;

                if distance < min_step {
                    continue;
                }

                let key = (next_row, next_col, (delta_row, delta_col));

                if next_cost < *distances.get(&key).unwrap_or(&i64::MAX) {
                    distances.insert(key, next_cost);
                    priority_queue.push((-next_cost, key));
                }
            }
        }
    }

    unreachable!()
}

pub(crate) fn problem1() {
    let input = fs::read_to_string("inputs/input17.txt").expect("should've been able to read");
    let grid = input
        .trim()
        .split('\n')
        .map(str::as_bytes)
        .collect::<Vec<_>>();
    println!("{}", find_shortest_path(&grid, 1, 3),)
}

pub(crate) fn problem2() {
    let input = fs::read_to_string("inputs/input17.txt").expect("should've been able to read");
    let grid = input
        .trim()
        .split('\n')
        .map(str::as_bytes)
        .collect::<Vec<_>>();
    println!("{}", find_shortest_path(&grid, 4, 10));
}
