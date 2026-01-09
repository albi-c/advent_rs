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

#[repr(align(64))]
struct CacheAligned<T>(pub T);

const CHUNK_INFO: CacheAligned<[[u8; 4]; 16]> = const {
    let mut result = [[0; _]; _];
    let mut i = 0;
    while i < result.len() {
        let mut res = [0; _];
        let mut j = 0;
        let mut k = 1;
        while j < res.len() && k <= i {
            if i % k == 0 {
                let count = i / k;
                if count > 1 {
                    let size = i / count;
                    if size != 1 {
                        res[j] = (((count - 1) as u8) << 4) | size as u8;
                        j += 1;
                    }
                }
            }
            k += 1;
        }
        result[i] = res;
        i += 1;
    }
    CacheAligned(result)
};

fn is_invalid_str_2(s: &[u8]) -> bool {
    if s.iter().all_equal() {
        return true;
    }
    let digits = s.len();
    // SAFETY: there are never more than 15 digits in the input - limited by buffer size
    let mut ci = u32::from_le_bytes(unsafe { *CHUNK_INFO.0.get_unchecked(digits) });
    'outer: while ci != 0 {
        let chunks_m1 = ((ci & 0xf0) >> 4) as usize;
        let size = (ci & 0xf) as usize;
        ci >>= 8;

        let first = &s[..size];
        let mut offset = size;
        for _ in 0..chunks_m1 {
            if first != &s[offset..offset + size] {
                continue 'outer;
            }
            offset += size;
        }
        return true;
    }
    false
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
    let mut data = {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(a).as_bytes();
        let mut buf = [b'0'; 16];
        for i in 0..s.len() {
            buf[i] = s[s.len() - i - 1];
        }
        CacheAligned((buf, s.len()))
    };
    for i in a..=b {
        func(i, &data.0.0[..data.0.1]);
        increment_integer(&mut data.0.1, &mut data.0.0);
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
