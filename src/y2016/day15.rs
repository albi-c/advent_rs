use advent::prelude::*;

parse!(Vec<(usize, usize, usize)>, lines("Disc #" usize " has " usize " positions; at time=0, it is at position " usize "."));

fn falls_through(disc: &(usize, usize, usize), time: usize) -> bool {
    (disc.2 + disc.0 + time) % disc.1 == 0
}

pub fn part1(input: Input) -> impl Display {
    for time in 0usize.. {
        if input.iter().all(|disc| falls_through(disc, time)) {
            return time;
        }
    }
    unreachable!()
}

pub fn part2(mut input: Input) -> impl Display {
    input.push((input.len() + 1, 11, 0));
    part1(input)
}
