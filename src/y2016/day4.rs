use advent::prelude::*;

parse!(Vec<(Vec<String>, usize, String)>, lines((string(alpha+) '-')+ usize '[' string(alpha+) ']'));

pub fn part1(input: Input) -> impl Display {
    let mut result = 0;
    for (names, sector, checksum) in input {
        let counts = names.join("").chars().counts();
        let best = counts.into_iter().map(|(a, b)| (b, -(a as i64), a)).sorted().rev().take(5).collect_vec();
        if best.into_iter().map(|(_, _, c)| c).eq(checksum.chars()) {
            result += sector;
        }
    }
    result
}

fn caesar(string: &str, n: usize) -> String {
    let n = (n % 26) as u8;
    string.chars().map(|c| ((c as u8 - 'a' as u8 + n) % 26 + 'a' as u8) as char).collect()
}

pub fn part2(input: Input) -> impl Display {
    for (names, sector, _) in input {
        if names.iter().any(|name| caesar(name, sector).contains("north")) {
            return sector;
        }
    }
    0
}
