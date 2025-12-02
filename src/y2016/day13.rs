use advent::prelude::*;

parse!(usize, usize);

fn create_grid(input: usize, size: (usize, usize)) -> Grid2<bool> {
    Grid2::new_pos_func(size, |(x, y)| {
        (x*x + 3*x + 2*x*y + y + y*y + input).count_ones() % 2 == 0
    })
}

pub fn part1(input: Input) -> impl Display {
    let size = (100, 100);
    let target = (31, 39);
    let grid = create_grid(input, size);
    let searchable = grid.searchable_weighted(
        |_, &b| b.then_some(1), false);
    for (dist, pos) in searchable.bfs((1, 1)) {
        if pos == target {
            return dist as isize;
        }
    }
    -1
}

pub fn part2(input: Input) -> impl Display {
    let size = (100, 100);
    let grid = create_grid(input, size);
    let searchable = grid.searchable_weighted(
        |_, &b| b.then_some(1), false);
    let mut result = 0;
    for (dist, _) in searchable.bfs((1, 1)) {
        if dist <= 50 {
            result += 1;
        }
    }
    result
}
