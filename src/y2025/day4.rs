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
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                if grid[(x, y)] != b'@' {
                    continue;
                }
                if !accessible(&grid, (x, y)) {
                    continue;
                }
                removed = true;
                result += 1;
                grid[(x, y)] = b'.';
            }
        }
    }
    result
}
