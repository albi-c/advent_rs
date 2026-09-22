use std::hint::{assert_unchecked, likely};
use advent::prelude::*;

parse!(Vec<(usize, usize)>, repeat_sep(usize '-' usize, ','));

fn is_invalid_str(n: usize, s: u64) -> bool {
    if n % 2 == 0 {
        let h = n / 2;
        s >> (4 * h) == s & mask(4 * h)
    } else {
        false
    }
}

pub fn part1(input: Input) -> impl Display {
    input
        .into_iter()
        .map(|(a, b)| invalid_in_interval(a, b, is_invalid_str))
        .sum::<usize>()
}

#[inline(always)]
const fn mask(b: usize) -> u64 {
    (1 << b) - 1
}

#[inline(always)]
fn check_many_chunks<const S: usize, const N: usize>(s: u64) -> bool {
    let first = s & mask(S);
    for i in 1..N {
        if (s >> (S * i)) & mask(S) != first {
            return false;
        }
    }
    true
}

#[inline(always)]
fn is_invalid_str_2(n: usize, s: u64) -> bool {
    unsafe {
        assert_unchecked(n >= 2);
        assert_unchecked(n <= 16);
    }
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
    for i in 1..n as u32 {
        if unsafe { s.unchecked_shr(i << 2) } & mask(4) != first {
            return false;
        }
    }
    true
}

#[inline(always)]
const fn mask_4b<const N: u64>() -> u64 {
    const {
        assert!(N < 0xf);
        let x = N | (N << 4);
        let x = x | (x << 8);
        let x = x | (x << 16);
        let x = x | (x << 32);
        x
    }
}

#[inline(always)]
fn spread_or_4b(x: u64) -> u64 {
    let x = x | (x >> 1);
    let x = x | (x >> 2);
    let x = x & mask_4b::<1>();
    unsafe { (x << 4).unchecked_sub(x) }
}

#[inline(always)]
fn increment_integer(mut n: usize, mut s: u64) -> (usize, u64) {
    unsafe {
        assert_unchecked(s >= 10);
    }
    if likely(s & 0xf < 9) {
        return (n, unsafe { s.unchecked_add(1) });
    }

    let is_9_4b = !spread_or_4b(s ^ mask_4b::<9>());
    let add_shift = is_9_4b.trailing_ones();
    unsafe {
        assert_unchecked(add_shift < 64);
        assert_unchecked(add_shift != 0);
    }
    s &= !(unsafe { 1u64.unchecked_shl(add_shift).unchecked_sub(1) });
    s += unsafe { 1u64.unchecked_shl(add_shift) };
    n = unsafe { 16u32.unchecked_sub(spread_or_4b(s).leading_zeros() >> 2) } as usize;

    (n, s)
}

#[inline(always)]
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

#[inline(always)]
fn invalid_in_interval(a: usize, b: usize, check: impl Fn(usize, u64) -> bool) -> usize {
    let mut result = 0;
    with_strings_in_interval(a, b, |i, n, s| {
        if check(n, s) {
            result += i;
        }
    });
    result
}

pub fn part2(input: Input) -> impl Display {
    input
        .into_iter()
        .map(|(a, b)| invalid_in_interval(a, b, is_invalid_str_2))
        .sum::<usize>()
}
