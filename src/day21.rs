use itertools::Itertools;
use std::{collections::HashSet, fs};

fn breadth_first_search(grid: &[&[u8]], start: (isize, isize), steps: usize) -> usize {
    let mut pos = HashSet::from_iter([start]);
    let mut new_pos = HashSet::new();
    for _ in 0..steps {
        new_pos.clear();
        for &(row, col) in &pos {
            for (d_row, d_col) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let (new_row, new_col) = (row + d_row, col + d_col);
                let tile = grid[new_row as usize % grid.len()][new_col as usize % grid.len()];
                if tile != b'#' {
                    new_pos.insert((new_row, new_col));
                }
            }
        }
        (pos, new_pos) = (new_pos, pos);
    }
    pos.len()
}

fn find_polynomial_length(grid: &[&[u8]], start: (isize, isize), target_step: usize) -> usize {
    let n1 = breadth_first_search(grid, start, target_step % grid.len() + grid.len() * 0);
    let n2 = breadth_first_search(grid, start, target_step % grid.len() + grid.len() * 1);
    let n3 = breadth_first_search(grid, start, target_step % grid.len() + grid.len() * 2);
    let target_step = target_step / grid.len();
    let [a, b, c] = [n1, n2 - n1, n3 - n2];
    return a + b * target_step + (target_step * (target_step - 1) / 2) * (c - b);
}

fn get_grid_and_start(input: &str) -> (Vec<&[u8]>, (isize, isize)) {
    let grid = input
        .trim()
        .split('\n')
        .map(str::as_bytes)
        .collect::<Vec<_>>();
    let start = (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .find(|&(r, c)| grid[r][c] == b'S')
        .map(|(r, c)| (r as isize, c as isize))
        .unwrap();
    (grid, start)
}

pub(crate) fn problem1() {
    let input = fs::read_to_string("inputs/input21.txt").expect("should've been able to read");
    let (grid, start) = get_grid_and_start(&input);
    println!("{}", breadth_first_search(&grid, start, 64));
}

pub(crate) fn problem2() {
    let input = fs::read_to_string("inputs/input21.txt").expect("should've been able to read");
    let (grid, start) = get_grid_and_start(&input);
    println!("{}", find_polynomial_length(&grid, start, 26501365));
}
