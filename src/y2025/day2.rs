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

fn is_invalid_str_2(s: &[u8]) -> bool {
    if s.iter().all_equal() {
        return true;
    }
    let digits = s.len();
    match digits {
        2 | 3 | 5 | 7 | 11 | 13 => false,
        4 => s[..2] == s[2..],
        6 => s[..3] == s[3..] || s.as_chunks::<2>().0.iter().all_equal(),
        8 => s[..4] == s[4..],
        9 => s.as_chunks::<3>().0.iter().all_equal(),
        10 => s[..5] == s[5..] || s.as_chunks::<2>().0.iter().all_equal(),
        12 => s[..6] == s[6..] || s.as_chunks::<4>().0.iter().all_equal(),
        14 => s[..7] == s[7..] || s.as_chunks::<2>().0.iter().all_equal(),
        15 => s.as_chunks::<5>().0.iter().all_equal() || s.as_chunks::<3>().0.iter().all_equal(),
        16 => s[..8] == s[8..],
        _ => unreachable!(),
    }
}

fn increment_integer(len: &mut usize, buf: &mut [u8; 16]) {
    let mut i = 0;
    loop {
        let f = buf[i] + 1;
        if f > b'9' {
            buf[i] = b'0';
            i += 1;
            *len = (*len).max(i + 1);
        } else {
            buf[i] = f;
            break;
        }
    }
}

fn with_strings_in_interval(a: usize, b: usize, mut func: impl FnMut(usize, &[u8])) {
    let a = a.max(10);
    let (mut buf, mut len) = {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(a).as_bytes();
        let mut buf = [b'0'; 16];
        for i in 0..s.len() {
            buf[i] = s[s.len() - i - 1];
        }
        (buf, s.len())
    };
    for i in a..=b {
        func(i, &buf[..len]);
        increment_integer(&mut len, &mut buf);
    }
}

fn invalid_in_interval(a: usize, b: usize) -> usize {
    let mut result = 0;
    with_strings_in_interval(a, b, |i, s| {
        if is_invalid_str_2(s) {
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
