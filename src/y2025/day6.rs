use advent::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum Op {
    Add,
    Mul,
}

impl Op {
    fn perform(self, a: usize, b: usize) -> usize {
        match self {
            Op::Add => a + b,
            Op::Mul => a * b,
        }
    }
}

parse!();

fn do_parse(input: Input) -> (Grid2<usize>, Vec<Op>) {
    let mut lines = input.lines().collect_vec();
    let ops_line = lines.pop().expect("no input");
    let grid = Grid2::new_rows(lines.into_iter()
        .map(|line| line.split_ascii_whitespace()
            .filter(|&s| !s.is_empty())
            .map(|s| s.parse().unwrap())));
    let ops = ops_line
        .split_ascii_whitespace()
        .filter(|&s| !s.is_empty())
        .map(|s| match s {
            "+" => Op::Add,
            "*" => Op::Mul,
            _ => panic!("invalid op: {}", s),
        })
        .collect_vec();
    (grid, ops)
}

pub fn part1(input: Input) -> impl Display {
    let (grid, ops) = do_parse(input);
    grid
        .cols()
        .zip(ops.into_iter())
        .map(|(col, op)| col.iter()
            .copied()
            .reduce(|a, b| op.perform(a, b))
            .expect("no data"))
        .sum::<usize>()
}

fn make_number(chars: impl Iterator<Item = u8>) -> Option<usize> {
    let mut result = None;
    for c in chars {
        if c == b' ' {
            continue;
        }
        result = Some(result.unwrap_or(0) * 10 + (c - b'0') as usize);
    }
    result
}

pub fn part2(input: Input) -> impl Display {
    let grid = Grid2::<u8>::new_rows_padded(input.lines().map(|line| line.bytes()), || b' ');
    let mut last_op = Op::Add;
    let mut result = 0;
    let mut acc = 0;
    for col in grid.cols() {
        let changed = match col.get(col.len() - 1).unwrap() {
            b' ' => false,
            b'+' => {
                last_op = Op::Add;
                true
            },
            b'*' => {
                last_op = Op::Mul;
                true
            },
            _ => panic!("invalid char: {}", col.get(col.len() - 1).unwrap()),
        };
        let Some(n) = make_number(col.iter().take(col.len() - 1).copied()) else { continue };
        if changed {
            result += acc;
            acc = n;
        } else {
            acc = last_op.perform(acc, n);
        }
    }
    result += acc;
    result
}
