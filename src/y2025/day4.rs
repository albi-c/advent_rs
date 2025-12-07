use advent::prelude::*;

parse!();

fn accessible(grid: &Grid2<u8>, pos: (usize, usize)) -> bool {
    grid.neighbors(pos, true)
        .filter(|&(_, v)| *v == b'@')
        .count() < 4
}

pub fn part1(input: Input) -> impl Display {
    let grid = Grid2::<u8>::new_str(input);
    grid
        .enumerate()
        .filter(|&(p, v)| *v == b'@' && accessible(&grid, p))
        .count()
}

pub fn part2(input: Input) -> impl Display {
    let mut grid = Grid2::<u8>::new_str(input);
    let mut removed = true;
    let mut result = 0;
    while removed {
        removed = false;
        grid.with_enumerate_modify(|pos, val, grid| {
            if val == b'@' && accessible(grid, pos) {
                removed = true;
                result += 1;
                b'.'
            } else {
                val
            }
        }, 0);
    }
    result
}
