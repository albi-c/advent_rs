use advent::prelude::*;

parse!((Vec<Vec<Vec<bool>>>, Vec<(usize, usize, Vec<usize>)>), section({
    line(usize ':') r:lines({ '.' => false, '#' => true }+) => r,
})+ section({
    l:lines(usize 'x' usize ':' (' ' usize)+) => l,
}));

pub fn part1(input: Input) -> impl Display {
    let (items, grids) = input;
    let items = items.into_iter().map(Grid::from_rows).collect_vec();
    let mut result = 0;
    for (w, h, counts) in grids {
        let mut area = 0;
        for (i, &n) in counts.iter().enumerate() {
            area += n * items[i].iter().filter(|&&x| x).count();
        }
        if area <= w * h {
            result += 1;
        }
    }
    result
}

pub fn part2(_input: Input) -> impl Display {
    "*"
}
