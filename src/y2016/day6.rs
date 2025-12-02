use advent::prelude::*;

parse!();

pub fn part1(input: Input) -> impl Display {
    Grid2::<char>::new_str(input)
        .cols()
        .map(|col| col.iter().copied().counts().into_iter().max_by_key(|&(_, c)| c).unwrap().0)
        .collect::<String>()
}

pub fn part2(input: Input) -> impl Display {
    Grid2::<char>::new_str(input)
        .cols()
        .map(|col| col.iter().copied().counts().into_iter().min_by_key(|&(_, c)| c).unwrap().0)
        .collect::<String>()
}
