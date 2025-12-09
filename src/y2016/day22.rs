use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
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

fn available_moves(grid: &Grid2<Node>) -> impl Iterator<Item = ((usize, usize), (usize, usize))> {
    grid
        .enumerate()
        .filter(|&(_, a)| a.used != 0)
        .flat_map(|(ap, a)| grid
            .neighbors(ap, false)
            .filter(move |&(bp, b)| ap != bp && a.used + b.used <= b.size)
            .map(move |(bp, _)| (ap, bp)))
}

// type DfsState = (Grid2<Node>, (usize, usize));
type DfsState = u64;

fn make_dfs_state(grid: &Grid2<Node>, data_pos: (usize, usize)) -> DfsState {
    let mut hasher = DefaultHasher::new();
    grid.hash(&mut hasher);
    data_pos.hash(&mut hasher);
    hasher.finish()
    // (grid.cloned(), data_pos)
}

fn dfs(grid: &mut Grid2<Node>, data_pos: (usize, usize), depth: usize, seen: &mut HashMap<DfsState, usize>,
       last_move: Option<((usize, usize), (usize, usize))>) -> usize {
    let state = make_dfs_state(grid, data_pos);
    if depth > 300 {
        usize::MAX
    } else if data_pos == (0, 0) {
        println!("target {}", depth);
        depth
    } else if let Some(d) = seen.get(&state) && *d < depth {
        *d
    } else {
        let res = available_moves(grid)
            .collect_vec()
            .into_iter()
            .filter(|&m| Some(m) != last_move)
            .map(|(ap, bp)| {
                let data_n = grid[ap].used;
                grid[bp].used += data_n;
                grid[ap].used = 0;
                let result = if ap == data_pos {
                    dfs(grid, bp, depth + 1, seen, Some((bp, ap)))
                } else {
                    dfs(grid, data_pos, depth + 1, seen, Some((bp, ap)))
                };
                grid[bp].used -= data_n;
                grid[ap].used = data_n;
                result
            })
            .min()
            .unwrap_or(usize::MAX);
        if res != usize::MAX {
            seen.entry(state).and_modify(|d| *d = res.min(*d)).or_insert(res);
        }
        res
    }
}

// const INP: &'static str = r"
// /dev/grid/node-x0-y0   10T    8T     2T   80%
// /dev/grid/node-x0-y1   11T    6T     5T   54%
// /dev/grid/node-x0-y2   32T   28T     4T   87%
// /dev/grid/node-x1-y0    9T    7T     2T   77%
// /dev/grid/node-x1-y1    8T    0T     8T    0%
// /dev/grid/node-x1-y2   11T    7T     4T   63%
// /dev/grid/node-x2-y0   10T    6T     4T   60%
// /dev/grid/node-x2-y1    9T    8T     1T   88%
// /dev/grid/node-x2-y2    9T    6T     3T   66%
// ";

pub fn part2(input: Input) -> impl Display {
    // let input = parse(INP.trim());

    // let grid = make_grid(input);
    // available_moves(&grid).count()

    let mut grid = make_grid(input);
    let data_pos = (grid.width() - 1, 0usize);
    let mut seen = HashMap::new();
    dfs(&mut grid, data_pos, 0, &mut seen, None)
    // let mut best = usize::MAX;
    // for (dist, (_, pos)) in bfs((grid, data_pos), |(grid, data_pos)| {
    //     available_moves(&grid)
    //         .collect_vec()
    //         .into_iter()
    //         .map(move |(ap, bp)| {
    //             let mut grid = grid.clone();
    //             grid[bp].used += grid[ap].used;
    //             grid[ap].used = 0;
    //             if ap == data_pos {
    //                 (grid, bp)
    //             } else {
    //                 (grid, data_pos)
    //             }
    //         })
    // }) {
    //     if pos == (0, 0) {
    //         best = best.min(dist);
    //     }
    // }
    // best
}
