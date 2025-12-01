use advent::prelude::*;

parse!(Vec<(char, isize)>, lines(upper isize));

pub fn part1(input: Input) -> impl Display {
    let mut pos = 50isize;
    let mut result = 0;
    for (direction, mut count) in input {
        if direction == 'L' {
            count = -count;
        }
        pos += count;
        pos %= 100;
        if pos == 0 {
            result += 1;
        }
    }
    result
}

pub fn part2(input: Input) -> impl Display {
    let mut pos = 50isize;
    let mut result = 0;
    for (direction, count) in input {
        let sign = if direction == 'L' {
            -1
        } else {
            1
        };
        for _ in 0..count {
            pos += sign;
            pos %= 100;
            if pos == 0 {
                result += 1;
            }
        }
    }
    result
}
