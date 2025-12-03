use advent::prelude::*;

parse!();

fn find_distances(grid: &Grid2<u8>, source: (usize, usize),
                  targets: &[(usize, usize)]) -> Vec<usize> {
    let mut result = targets.iter().map(|&target| (target, usize::MAX)).collect_vec();
    for (dist, pos) in bfs(source, |pos| {
        grid
            .neighbors(pos, false)
            .filter(|(_, v)| **v == b'.')
            .map(|(p, _)| p)
    }) {
        result.iter_mut().find(|(p, _)| *p == pos).map(|(_, d)| *d = dist);
    }
    result.into_iter().map(|(_, d)| d).collect_vec()
}

fn solve(input: Input, go_back: bool) -> usize {
    let mut grid = Grid2::<u8>::new_str(input);
    let mut targets = grid.enumerate()
        .filter(|(_, v)| (**v as char).is_digit(10))
        .map(|(p, v)| (*v - b'0', p))
        .sorted_by_key(|(v, _)| *v)
        .map(|(_, p)| p)
        .collect_vec();
    let start_pos = targets.remove(0);
    grid[start_pos] = b'.';
    for target in &targets {
        grid[*target] = b'.';
    }
    let start_distances = find_distances(&grid, start_pos, &targets);
    let all_distances = targets.iter()
        .map(|&pos| find_distances(&grid, pos, &targets))
        .collect_vec();
    let mut best = usize::MAX;
    for perm in (0..targets.len()).permutations(targets.len()) {
        let mut prev_distances = &start_distances;
        let mut total_dist = 0;
        let mut last_index = None;
        for index in perm {
            total_dist += prev_distances[index];
            prev_distances = &all_distances[index];
            last_index = Some(index);
        }
        if go_back {
            total_dist += start_distances[last_index.unwrap()];
        }
        best = best.min(total_dist);
    }
    best
}

pub fn part1(input: Input) -> impl Display {
    solve(input, false)
}

pub fn part2(input: Input) -> impl Display {
    solve(input, true)
}
