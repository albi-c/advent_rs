use advent::prelude::*;

#[derive(Debug)]
pub enum Target {
    Bot(usize),
    Output(usize),
}

#[derive(Debug)]
pub enum Ins {
    Val(usize, usize),
    Split(usize, Target, Target),
}

fn target_parser() -> impl Parser<Output = Target> {
    parser!({
        "bot " b:usize => Target::Bot(b),
        "output " o:usize => Target::Output(o),
    })
}

parse!(Vec<Ins>, lines({
    "value " v:usize " goes to bot " b:usize => Ins::Val(v, b),
    "bot " b:usize " gives low to " l:target_parser() " and high to " h:target_parser() => Ins::Split(b, l, h),
}));

pub fn part1(input: Input) -> impl Display {
    let count = input.iter()
        .map(|ins| match ins {
            Ins::Val(_, b) => *b,
            Ins::Split(s, Target::Bot(b), Target::Bot(b2)) => (*b).max(*b2).max(*s),
            Ins::Split(s, Target::Bot(b), _) => (*b).max(*s),
            Ins::Split(s, _, Target::Bot(b)) => (*b).max(*s),
            Ins::Split(s, _, _) => *s,
        })
        .max().expect("no input") + 1;
    count
}

pub fn part2(input: Input) -> impl Display {
    0
}
