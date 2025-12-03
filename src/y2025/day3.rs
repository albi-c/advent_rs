use advent::prelude::*;

parse!(Vec<Vec<usize>>, lines(digit+));

fn find_largest(row: &[usize], n: usize) -> usize {
    let mut result = 0;
    let mut index = 0;
    for i in 0..n {
        let remaining = n - i - 1;
        let (j, m) = row[index..row.len()-remaining].iter().enumerate().max_by_key(|&(j, x)| (*x, -(j as isize))).unwrap();
        result *= 10;
        result += *m;
        index += j + 1;
    }
    result
}

pub fn part1(input: Input) -> impl Display {
    input.into_iter()
        .map(|row| find_largest(&row, 2))
        .sum::<usize>()
}

pub fn part2(input: Input) -> impl Display {
    input.into_iter()
        .map(|row| find_largest(&row, 12))
        .sum::<usize>()
}
