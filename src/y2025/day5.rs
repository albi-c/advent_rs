use advent::intervals::IntervalInclusive;
use advent::prelude::*;

parse!((Vec<IntervalInclusive>, Vec<usize>), section(lines(interval_inclusive_us('-'))) section(lines(usize)));

pub fn part1(input: Input) -> impl Display {
    let (ranges, ids) = input;
    ids
        .into_iter()
        .filter(|&i| ranges.iter()
            .any(|&interval| interval.contains(i)))
        .count()
}

pub fn part2(input: Input) -> impl Display {
    let (mut ranges, _) = input;
    IntervalInclusive::optimize_vec(&mut ranges);
    ranges
        .into_iter()
        .map(IntervalInclusive::len)
        .sum::<usize>()
}
