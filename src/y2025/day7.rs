use advent::prelude::*;
use memoize::memoize;

parse!();

fn solve_row(row: &[u8], prev_beams: &[bool]) -> (Vec<bool>, usize) {
    let mut new_beams = vec![false; row.len()];
    let mut result = 0;
    for (i, &ch) in row.iter().enumerate() {
        match ch {
            b'.' => new_beams[i] = new_beams[i] || prev_beams[i],
            b'S' => new_beams[i] = true,
            b'^' => if prev_beams[i] {
                new_beams[i - 1] = true;
                new_beams[i + 1] = true;
                result += 1;
            },
            _ => panic!("unexpected character in input {}", ch as char),
        }
    }
    (new_beams, result)
}

pub fn part1(input: Input) -> impl Display {
    let grid = Grid2::<u8>::new_str(input);
    let mut beams = vec![false; grid.width()];
    let mut result = 0usize;
    for row in grid.rows() {
        let (new_beams, res) = solve_row(row, &beams);
        beams = new_beams;
        result += res;
    }
    result
}

#[memoize(Ignore: grid)]
fn get_timelines(grid: &Grid2<u8>, particle: (usize, usize)) -> usize {
    if particle.1 >= grid.height() {
        return 1;
    }
    let ch = grid[particle];
    match ch {
        b'.' => get_timelines(grid, (particle.0, particle.1 + 1)),
        b'^' => get_timelines(grid, (particle.0 - 1, particle.1 + 1)) + get_timelines(grid, (particle.0 + 1, particle.1 + 1)),
        _ => panic!("unexpected character in input {}", ch as char),
    }
}

pub fn part2(input: Input) -> impl Display {
    let grid = Grid2::<u8>::new_str(input);
    let start = grid.row(0).iter()
        .find_position(|&&ch| ch == b'S')
        .expect("no starting position").0;
    get_timelines(&grid, (start, 1))
}
