use advent::prelude::*;

#[derive(Debug, Clone)]
pub enum Ins {
    Swap(usize, usize),
    SwapL(u8, u8),
    RotateLeft(usize),
    RotateRight(usize),
    RotatePos(u8),
    Reverse(usize, usize),
    Move(usize, usize),
}

parse!(Vec<Ins>, lines({
    "swap position " a:usize " with position " b:usize => Ins::Swap(a, b),
    "swap letter " a:lower " with letter " b:lower => Ins::SwapL(a as u8, b as u8),
    "rotate left " n:usize " step" 's'? => Ins::RotateLeft(n),
    "rotate right " n:usize " step" 's'? => Ins::RotateRight(n),
    "rotate based on position of letter " x:lower => Ins::RotatePos(x as u8),
    "reverse positions " a:usize " through " b:usize => Ins::Reverse(a, b),
    "move position " a:usize " to position " b:usize => Ins::Move(a, b),
}));

fn replace(string: &mut [u8], a: u8, b: u8) {
    for ch in string {
        if *ch == a {
            *ch = b;
        }
    }
}

fn swap_letters(string: &mut [u8], a: u8, b: u8) {
    replace(string, a, 0);
    replace(string, b, a);
    replace(string, 0, b);
}

impl Ins {
    fn perform(self, string: &mut Vec<u8>) {
        match self {
            Ins::Swap(a, b) => string.swap(a, b),
            Ins::SwapL(a, b) => swap_letters(string, a, b),
            Ins::RotateLeft(n) => string.rotate_left(n),
            Ins::RotateRight(n) => string.rotate_right(n),
            Ins::RotatePos(c) => {
                let index = string.iter().enumerate().find(|(_, ch)| **ch == c).unwrap().0;
                let length = string.len();
                string.rotate_right((index + 1 + if index >= 4 { 1 } else { 0 }) % length);
            },
            Ins::Reverse(a, b) => string[a..=b].reverse(),
            Ins::Move(a, b) => {
                let c = string.remove(a);
                string.insert(b, c);
            },
        }
    }

    fn reverse(self, string: &mut Vec<u8>) {
        match self {
            Ins::Swap(a, b) => string.swap(a, b),
            Ins::SwapL(a, b) => swap_letters(string, a, b),
            Ins::RotateLeft(n) => string.rotate_right(n),
            Ins::RotateRight(n) => string.rotate_left(n),
            Ins::RotatePos(c) => {
                let index = string.iter().enumerate().find(|(_, ch)| **ch == c).unwrap().0;
                let length = string.len();
                string.rotate_left((index + 1 + if index >= 4 { 1 } else { 0 }) % length);
                todo!()
            },
            Ins::Reverse(a, b) => string[a..=b].reverse(),
            Ins::Move(a, b) => {
                let c = string.remove(b);
                string.insert(a, c);
            }
        }
    }
}

static INS: &'static str = r"
swap position 4 with position 0
swap letter d with letter b
reverse positions 0 through 4
rotate left 1 step
move position 1 to position 4
move position 3 to position 0
rotate based on position of letter b
rotate based on position of letter d
";

pub fn part1(input: Input) -> impl Display {
    let input = parse(INS.trim());

    let i2 = input.clone();

    // let mut string = b"abcdefgh".into_iter().copied().collect_vec();
    let mut string = b"abcde".into_iter().copied().collect_vec();
    for ins in input {
        ins.perform(&mut string);
    }

    let mut res = string.clone();

    let r = string.into_iter().map(|c| c as char).join("");

    for ins in i2 {
        ins.reverse(&mut res);
        println!("R {:?}", res.iter().copied().map(|c| c as char).join(""));
    }

    println!("F {:?}", res.into_iter().map(|c| c as char).join(""));

    r
}

pub fn part2(input: Input) -> impl Display {
    let mut string = b"fbgdceah".into_iter().copied().collect_vec();
    for ins in input {
        ins.reverse(&mut string);
    }
    string.into_iter().map(|c| c as char).join("")
}
