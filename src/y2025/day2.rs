use advent::prelude::*;

parse!(Vec<(usize, usize)>, repeat_sep(usize '-' usize, ','));

fn is_invalid(n: usize) -> bool {
    let digits = n.ilog10() + 1;
    if digits % 2 == 0 {
        let s = n.to_string().into_bytes();
        let (a, b) = s.split_at(digits as usize / 2);
        a == b
    } else {
        false
    }
}

pub fn part1(input: Input) -> impl Display {
    input
        .into_iter()
        .map(|(a, b)| (a..=b).filter(|&n| is_invalid(n)).sum::<usize>())
        .sum::<usize>()
}

fn is_invalid_2(n: usize) -> bool {
    let digits = n.ilog10() + 1;
    for d in 1..=digits/2 {
        if digits % d == 0 && digits / d >= 2 {
            let s = n.to_string().into_bytes();
            if s.chunks(d as usize).all_equal() {
                return true;
            }
        }
    }
    false
}

pub fn part2(input: Input) -> impl Display {
    input
        .into_iter()
        .map(|(a, b)| (a..=b).filter(|&n| is_invalid_2(n)).sum::<usize>())
        .sum::<usize>()
}
