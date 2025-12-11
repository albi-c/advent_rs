use std::collections::HashMap;
use advent::prelude::*;
use memoize::memoize;

parse!(Vec<(String, Vec<String>)>, lines(string(lower+) ':' { " " string(lower+) }+));

#[memoize(Ignore: map)]
fn get_paths(pos: String, map: &HashMap<String, Vec<String>>) -> usize {
    if pos == "out" {
        1
    } else {
        map[&pos].iter().map(|p| get_paths(p.clone(), map)).sum()
    }
}

pub fn part1(input: Input) -> impl Display {
    let map = input.into_iter().collect::<HashMap<_, _>>();
    get_paths("you".to_owned(), &map)
}

#[memoize(Ignore: map)]
fn get_paths_2(pos: String, map: &HashMap<String, Vec<String>>, dac: bool, fft: bool) -> usize {
    if pos == "out" {
        (dac && fft) as usize
    } else {
        let dac = dac || pos == "dac";
        let fft = fft || pos == "fft";
        map[&pos].iter().map(|p| get_paths_2(p.clone(), map, dac, fft)).sum()
    }
}

pub fn part2(input: Input) -> impl Display {
    let map = input.into_iter().collect::<HashMap<_, _>>();
    get_paths_2("svr".to_owned(), &map, false, false)
}
