use advent::prelude::*;

parse!(custom Vec<[usize; 3]>; input {
    input
        .lines()
        .map(|line| line
            .trim()
            .split_ascii_whitespace()
            .map(|s| usize::from_str(s).unwrap())
            .collect_array::<3>()
            .unwrap())
        .collect()
});

fn check_triangle(tri: &[usize; 3]) -> bool {
    let &[a, b, c] = tri;
    a + b > c && b + c > a && a + c > b
}

pub fn part1(input: Input) -> impl Display {
    input
        .iter()
        .copied()
        .filter(check_triangle)
        .count()
}

pub fn part2(input: Input) -> impl Display {
    input
        .chunks_exact(3)
        .map(|chunk| {
            let g = Grid2::new_rows_copied(chunk);
            g.cols()
                .filter(|col| check_triangle(
                    &col.iter().copied().collect_array::<3>().unwrap()))
                .count()
        })
        .sum::<usize>()
}
