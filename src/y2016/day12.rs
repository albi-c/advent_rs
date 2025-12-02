use advent::prelude::*;

#[derive(Debug)]
pub enum Value {
    Reg(usize),
    Imm(isize),
}

#[derive(Debug)]
pub enum Ins {
    Cpy(Value, usize),
    Inc(usize),
    Dec(usize),
    Jnz(Value, isize),
}

impl Value {
    fn eval(&self, regs: &[isize]) -> isize {
        match self {
            Value::Reg(r) => regs[*r],
            Value::Imm(i) => *i,
        }
    }
}

fn parse_reg() -> impl Parser<Output = usize> {
    parser!({
        r:alpha => (r as u8 - b'a') as usize,
    })
}

fn parse_value() -> impl Parser<Output = Value> {
    parser!({
        i:isize => Value::Imm(i),
        r:alpha => Value::Reg((r as u8 - b'a') as usize),
    })
}

parse!(Vec<Ins>, lines({
    "cpy " v:parse_value() " " r:parse_reg() => Ins::Cpy(v, r),
    "inc " r:parse_reg() => Ins::Inc(r),
    "dec " r:parse_reg() => Ins::Dec(r),
    "jnz " v:parse_value() " " i:isize => Ins::Jnz(v, i),
}));

fn run(input: &[Ins], reg_c: isize) -> isize {
    let mut regs = [0isize, 0isize, reg_c, 0isize];
    let mut pc = 0;
    while pc < input.len() {
        match &input[pc] {
            Ins::Cpy(v, r) => regs[*r] = v.eval(&regs),
            Ins::Inc(r) => regs[*r] += 1,
            Ins::Dec(r) => regs[*r] -= 1,
            Ins::Jnz(v, i) => if v.eval(&regs) != 0 {
                pc = (pc as isize + i) as usize;
                continue;
            },
        }
        pc += 1;
    }
    regs[0]
}

pub fn part1(input: Input) -> impl Display {
    run(&input, 0)
}

pub fn part2(input: Input) -> impl Display {
    run(&input, 1)
}
