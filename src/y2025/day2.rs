use advent::prelude::*;

parse!(Vec<(usize, usize)>, repeat_sep(usize '-' usize, ','));

fn is_invalid(n: usize) -> bool {
    let digits = n.ilog10() + 1;
    if digits % 2 == 0 {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(n).as_bytes();
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

const fn mask(b: usize) -> u64 {
    (1 << b) - 1
}

fn check_many_chunks<const S: usize, const N: usize>(s: u64) -> bool {
    let first = s & mask(S);
    for i in 1..N {
        if (s >> (S * i)) & mask(S) != first {
            return false;
        }
    }
    true
}

fn is_invalid_str_2(n: usize, s: u64) -> bool {
    if match n {
        2 | 3 | 5 | 7 | 11 | 13 => false,
        4 => s & mask(8) == s >> 8,
        6 => s & mask(12) == s >> 12 || check_many_chunks::<8, 3>(s),
        8 => s & mask(16) == s >> 16,
        9 => check_many_chunks::<12, 3>(s),
        10 => s & mask(20) == s >> 20 || check_many_chunks::<8, 5>(s),
        12 => s & mask(24) == s >> 24 || check_many_chunks::<16, 3>(s),
        14 => s & mask(28) == s >> 28 || check_many_chunks::<8, 7>(s),
        15 => check_many_chunks::<20, 3>(s) || check_many_chunks::<12, 5>(s),
        16 => s & mask(32) == s >> 32,
        _ => unreachable!(),
    } {
        return true;
    }

    let first = s & mask(4);
    for i in 1..n {
        if (s >> (4 * i)) & mask(4) != first {
            return false;
        }
    }
    true
}

fn increment_integer(mut n: usize, mut s: u64) -> (usize, u64) {
    s += 1;
    if s & 0xf > 9 {
        s &= !0xf;
        let mut i = 1;
        loop {
            let mask = 0xf << (4 * i);
            s += 1 << (4 * i);
            if (s & mask) >> (4 * i) > 9 {
                s &= !mask;
                n = n.max(i + 2);
            } else {
                break;
            }
            i += 1;
        }
    }
    (n, s)
}

fn with_strings_in_interval(a: usize, b: usize, mut func: impl FnMut(usize, usize, u64)) {
    let a = a.max(10);
    let (mut s, mut len) = {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(a).as_bytes();
        let mut st = 0u64;
        for i in 0..s.len() {
            st |= ((s[s.len() - i - 1] - b'0') as u64) << (4 * i);
        }
        (st, s.len())
    };
    for i in a..=b {
        func(i, len, s);
        let pair = increment_integer(len, s);
        len = pair.0;
        s = pair.1;
    }
}

fn invalid_in_interval(a: usize, b: usize) -> usize {
    let mut result = 0;
    with_strings_in_interval(a, b, |i, n, s| {
        if is_invalid_str_2(n, s) {
            result += i;
        }
    });
    result
}

pub fn part2(input: Input) -> impl Display {
    input
        .into_iter()
        .map(|(a, b)| invalid_in_interval(a, b))
        .sum::<usize>()
}
