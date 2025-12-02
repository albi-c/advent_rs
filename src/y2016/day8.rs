use advent::prelude::*;

#[derive(Debug)]
pub enum Ins {
    Rect(usize, usize),
    RotRow(usize, usize),
    RotCol(usize, usize),
}

parse!(Vec<Ins>, lines({
    "rect " w:usize "x" h:usize => Ins::Rect(w, h),
    "rotate row y=" y:usize " by " c:usize => Ins::RotRow(y, c),
    "rotate column x=" x:usize " by " c:usize => Ins::RotCol(x, c),
}));

fn rotate(count: usize, mut row: impl Grid2RowMut<Item = char>) {
    let mut v = row.iter().copied().collect_vec();
    v.rotate_right(count);
    row.iter_mut().zip(v.into_iter()).for_each(|(a, b)| *a = b);
}

fn get_display(input: Input) -> Grid2<char> {
    let mut display = Grid2::new_value((50, 6), '.');
    for ins in input {
        match ins {
            Ins::Rect(w, h) => display.slice_mut((0, 0), (w, h)).fill('#'),
            Ins::RotRow(y, c) => rotate(c, display.row_mut(y)),
            Ins::RotCol(x, c) => rotate(c, display.col_mut(x)),
        }
    }
    display
}

pub fn part1(input: Input) -> impl Display {
    get_display(input).into_iter().filter(|&c| c == '#').count()
}

pub fn part2(input: Input) -> impl Display {
    get_display(input).print();
    ""
}
