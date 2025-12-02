use advent::prelude::*;

parse!(Vec<bool>, { '0' => false, '1' => true }+);

fn step(data: &mut Vec<bool>) {
    let length = data.len();
    data.push(false);
    for i in 1..=length {
        data.push(!data[length - i]);
    }
}

fn checksum(mut data: Vec<bool>) -> Vec<bool> {
    if data.len() % 2 == 0 {
        for i in 0..data.len() / 2 {
            data[i] = data[i*2] == data[i*2+1];
        }
        data.truncate(data.len() / 2);
        checksum(data)
    } else {
        data
    }
}

fn solve(mut data: Vec<bool>, target: usize) -> String {
    data.reserve(target - data.len());
    while data.len() < target {
        step(&mut data);
    }
    data.truncate(target);
    assert_eq!(data.len(), target);
    checksum(data).into_iter().map(|x| if x { '1' } else { '0' }).collect::<String>()
}

pub fn part1(input: Input) -> impl Display {
    solve(input, 272)
}

pub fn part2(input: Input) -> impl Display {
    solve(input, 35651584)
}
