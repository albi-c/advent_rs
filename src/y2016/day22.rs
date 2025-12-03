use advent::prelude::*;

// input is edited to remove first two lines
// root@ebhq-gridcenter# df -h
// Filesystem              Size  Used  Avail  Use%
parse!(Vec<(usize, usize, usize, usize)>, lines( "/dev/grid/node-x" x:usize "-y" y:usize " "+ s:usize "T" " "+ u:usize "T" " "+ usize "T" " "+ usize "%" => (x, y, s, u)));

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Node {
    pub size: usize,
    pub used: usize,
}

fn make_grid(input: Input) -> Grid2<Node> {
    let width = input.iter().map(|n| n.0).max().unwrap() + 1;
    let height = input.iter().map(|n| n.1).max().unwrap() + 1;
    let mut grid = Grid2::new_value((width, height), None);
    for (x, y, size, used) in input {
        let prev = grid[(x, y)].replace(Node { size, used });
        assert!(prev.is_none());
    }
    grid.into_map(|node| node.expect("node not found"))
}

pub fn part1(input: Input) -> impl Display {
    let grid = make_grid(input);
    grid
        .enumerate()
        .filter(|&(_, a)| a.used != 0)
        .flat_map(|(ap, a)| grid
            .enumerate()
            .filter(move |&(bp, b)| ap != bp && a.used + b.used <= b.size))
        .count()
}

pub fn part2(input: Input) -> impl Display {
    let grid = make_grid(input);
    grid
        .enumerate()
        .filter(|&(_, a)| a.used != 0)
        .flat_map(|(ap, a)| grid
            .neighbors(ap, false)
            .filter(move |&(bp, b)| ap != bp && a.used + b.used <= b.size))
        .count()
    // let grid = make_grid(input);
    // let data_pos = (grid.width() - 1, 0);
    // for (dist, (_, pos)) in bfs((grid, data_pos), |(grid, data_pos)| {}) {
    //     if pos == data_pos {
    //
    //     }
    // }
    // 0
}
