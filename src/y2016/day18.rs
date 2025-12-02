use advent::prelude::*;

parse!(Vec<bool>, { '^' => true, '.' => false }+);

fn next_row(row: &[bool]) -> Vec<bool> {
    (0..row.len())
        .map(|i| i.checked_sub(1).map_or(false, |i| row[i]) != *row.get(i + 1).unwrap_or(&false))
        .collect_vec()
}

fn solve(mut row: Vec<bool>, rows: usize) -> usize {
    let mut result = 0;
    for _ in 0..rows {
        result += row.iter().filter(|&&x| !x).count();
        row = next_row(&row);
    }
    result
}

pub fn part1(input: Input) -> impl Display {
    solve(input, 40)
}

pub fn part2(input: Input) -> impl Display {
    solve(input, 400000)
}
