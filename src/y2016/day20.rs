use advent::intervals::IntervalInclusive;
use advent::prelude::*;

parse!(Vec<IntervalInclusive<u32>>, lines(interval_inclusive(u32 '-' u32)));

pub fn part1(mut input: Input) -> impl Display {
    IntervalInclusive::optimize_vec(&mut input);
    input[0].1 + 1
}

pub fn part2(mut input: Input) -> impl Display {
    IntervalInclusive::optimize_vec(&mut input);
    let mut start = Some(0);
    let mut result = 0;
    for interval in input {
        if let Some(start) = start {
            result += interval.0 - start;
        }
        start = interval.1.checked_add(1);
    }
    if let Some(start) = start {
        result += u32::MAX - start;
    }
    result
}
