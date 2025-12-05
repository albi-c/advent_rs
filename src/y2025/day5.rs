use advent::prelude::*;

parse!((Vec<(usize, usize)>, Vec<usize>), section(lines(usize '-' usize)) section(lines(usize)));

pub fn part1(input: Input) -> impl Display {
    let (ranges, ids) = input;
    ids
        .into_iter()
        .filter(|&i| ranges.iter()
            .any(|&(lo, hi)| i >= lo && i <= hi))
        .count()
}

fn join_intervals(a: (usize, usize), b: (usize, usize)) -> Option<(usize, usize)> {
    let (al, ah) = a;
    let (bl, bh) = b;
    if ah < bl {
        None
    } else {
        Some((al.min(bl), ah.max(bh)))
    }
}

pub fn part2(input: Input) -> impl Display {
    let (mut ranges, _) = input;
    ranges.sort();
    for i in (1..ranges.len()).rev() {
        if let Some(interval) = join_intervals(ranges[i - 1], ranges[i]) {
            ranges[i - 1] = interval;
            ranges.remove(i);
        }
    }
    ranges
        .into_iter()
        .map(|(a, b)| b - a + 1)
        .sum::<usize>()
}
