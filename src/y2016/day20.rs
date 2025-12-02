use advent::prelude::*;

parse!(Vec<(u32, u32)>, lines(u32 '-' u32));

pub fn part1(input: Input) -> impl Display {
    (0u32..=u32::MAX)
        .into_par_iter()
        .find_first(move |&i| input.iter().all(|&(lo, hi)| i < lo || i > hi))
        .unwrap()
}

pub fn part2(input: Input) -> impl Display {
    (0u32..=u32::MAX)
        .into_par_iter()
        .filter(move |&i| input.iter().all(|&(lo, hi)| i < lo || i > hi))
        .count()
}
