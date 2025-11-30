use std::collections::HashSet;
use advent::prelude::*;

parse!(Vec<(isize, i32)>,
    repeat_sep({
        "R" => 1,
        "L" => -1,
    } i32, ", ")
);

pub fn part1(input: Input) -> impl Display {
    input.into_iter().fold((i32x2::from([1, 0]), i32x2::splat(0)), |(dir, pos), (rot, dist)| {
        let dir = dir.rotate(rot);
        (dir, pos + dir * i32x2::splat(dist))
    }).1.manhattan_len()
}

pub fn part2(input: Input) -> impl Display {
    let mut dir = i32x2::from([1, 0]);
    let mut pos = i32x2::splat(0);
    let mut locations = HashSet::new();
    locations.insert(pos);
    for (rot, dist) in input {
        dir = dir.rotate(rot);
        for _ in 0..dist {
            pos += dir;
            if locations.contains(&pos) {
                return pos.manhattan_len();
            }
            locations.insert(pos);
        }
    }
    0
}
