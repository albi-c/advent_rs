use advent::prelude::*;
use memoize::memoize;

parse!();

#[memoize(Ignore: input)]
fn do_hash(i: usize, input: &str) -> [u8; 32] {
    let mut result = [0; 32];
    hex::encode_to_slice(md5::compute(format!("{input}{i}")).0, &mut result).unwrap();
    result
}

fn find_triple(hash: [u8; 32]) -> Option<u8> {
    hash.into_iter()
        .map_windows(|window: &[u8; 3]| (*window, window.iter().all_equal()))
        .find(|(_, m)| *m)
        .map(|(window, _)| window[0])
}

fn has_five(hash: [u8; 32], ch: u8) -> bool {
    hash.into_iter()
        .map_windows(|window: &[u8; 5]| window.iter().all(|&x| x == ch))
        .any(|x| x)
}

fn solve(input: &str, hash_func: impl Fn(usize, &str) -> [u8; 32]) -> usize {
    let mut n_key = 0;
    for i in 0usize.. {
        if let Some(ch) = find_triple(hash_func(i, input)) {
            for j in i+1..=i+1000 {
                if has_five(hash_func(j, input), ch) {
                    n_key += 1;
                    if n_key == 64 {
                        return i;
                    }
                }
            }
        }
    }
    unreachable!()
}

pub fn part1(input: Input) -> impl Display {
    solve(input, do_hash)
}

#[memoize(Ignore: input)]
fn do_hash_2(i: usize, input: &str) -> [u8; 32] {
    let mut result = [0; 32];
    hex::encode_to_slice(md5::compute(format!("{input}{i}")).0, &mut result).unwrap();
    for _ in 0..2016 {
        let res = md5::compute(result).0;
        hex::encode_to_slice(&res, &mut result).unwrap();
    }
    result
}

pub fn part2(input: Input) -> impl Display {
    solve(input, do_hash_2)
}
