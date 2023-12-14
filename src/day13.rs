use std::fs;

fn find_vertical_line(grid: &[&[u8]], limit: usize) -> Option<usize> {
    (0..grid[0].len() - 1).find(|&col| {
        let incorrect = (0..=col.min(grid[0].len() - col - 2))
            .map(|dc| {
                let left_col = col - dc;
                let right_col = col + 1 + dc;
                (0..grid.len())
                    .filter(|&row| grid[row][left_col] != grid[row][right_col])
                    .count()
            })
            .sum::<usize>();
        incorrect == limit
    })
}

fn find_horizontal_line(grid: &[&[u8]], limit: usize) -> Option<usize> {
    (0..grid.len() - 1).find(|&row| {
        let incorrect = (0..=row.min(grid.len() - row - 2))
            .map(|dr| {
                let top_row = row - dr;
                let bottom_row = row + 1 + dr;
                (0..grid[0].len())
                    .filter(|&col| grid[top_row][col] != grid[bottom_row][col])
                    .count()
            })
            .sum::<usize>();
        incorrect == limit
    })
}

fn solve(grids: &[Vec<&[u8]>], limit: usize) -> usize {
    grids
        .iter()
        .map(|grid| {
            find_horizontal_line(grid, limit)
                .map(|row| (row + 1) * 100)
                .or_else(|| find_vertical_line(grid, limit).map(|col| col + 1))
                .unwrap()
        })
        .sum()
}

pub(crate) fn problem1() {
    let input = fs::read_to_string("inputs/input13.txt").expect("should've been able to read");
    let trimmed_input = input.trim();
    let grids = trimmed_input
        .split("\n\n")
        .map(|s| s.split('\n').map(|l| l.as_bytes()).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let result = solve(&grids, 0);
    println!("{}", result);
}

pub(crate) fn problem2() {
    println!("not implemented");
}
