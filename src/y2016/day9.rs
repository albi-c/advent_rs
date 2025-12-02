use std::iter::Peekable;
use std::str::Chars;
use advent::prelude::*;

parse!();

fn parse_int(chars: &mut Peekable<Chars>) -> usize {
    let mut n = 0;
    while let Some(ch) = chars.peek() {
        if ch.is_ascii_digit() {
            n = n * 10 + ch.to_digit(10).unwrap();
            chars.next();
        } else {
            break;
        }
    }
    n as usize
}

fn decompress(string: &str, recurse: bool) -> usize {
    let mut chars = string.chars().peekable();
    let mut length = 0;
    while let Some(ch) = chars.next() {
        match ch {
            '(' => {
                let n = parse_int(chars.by_ref());
                assert_eq!(chars.next(), Some('x'));
                let r = parse_int(chars.by_ref());
                assert_eq!(chars.next(), Some(')'));
                if !recurse {
                    length += r * n;
                    chars.by_ref().take(n).for_each(drop);
                } else {
                    let s = chars.by_ref().take(n).collect::<String>();
                    length += r * decompress(&s, true);
                }
            },
            _ => length += 1,
        }
    }
    length
}

pub fn part1(input: Input) -> impl Display {
    decompress(input, false)
}

pub fn part2(input: Input) -> impl Display {
    decompress(input, true)
}
