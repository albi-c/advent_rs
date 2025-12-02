use std::cell::Cell;
use advent::prelude::*;

#[derive(Debug, Copy, Clone)]
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

impl Target {
    fn give(&self, value: usize, bots: &[Bot]) -> Option<usize> {
        match self {
            Target::Bot(i) => bots[*i].give(value, bots),
            Target::Output(_o) => {
                // println!("output {} {}", o, value);
                None
            },
        }
    }
}

#[derive(Debug)]
struct Bot {
    index: usize,
    value: Cell<Option<usize>>,
    lo: Target,
    hi: Target,
}

impl Bot {
    fn give(&self, value: usize, bots: &[Bot]) -> Option<usize> {
        if let Some(v) = self.value.get() {
            self.value.set(None);
            let mi = v.min(value);
            let ma = v.max(value);
            self.lo.give(mi, bots);
            self.hi.give(ma, bots);
            (mi == 17 && ma == 61).then_some(self.index)
        } else {
            self.value.set(Some(value));
            None
        }
    }
}

pub fn part1(input: Input) -> impl Display {
    let count = input.iter()
        .map(|ins| match ins {
            Ins::Val(_, b) => *b,
            Ins::Split(s, _, _) => *s,
        })
        .max().expect("no input") + 1;
    let mut bots = (0..count)
        .map(|index| Bot {
            index,
            value: Cell::new(None),
            lo: Target::Bot(usize::MAX),
            hi: Target::Bot(usize::MAX) ,
        })
        .collect_vec();
    for ins in &input {
        if let Ins::Split(b, l, h) = ins {
            bots[*b].lo = *l;
            bots[*b].hi = *h;
        }
    }
    for ins in &input {
        if let Ins::Val(v, b) = ins {
            if let Some(res) = bots[*b].give(*v, &bots) {
                return res as isize;
            }
        }
    }
    -1
}

pub fn part2(_input: Input) -> impl Display {
    0
}
