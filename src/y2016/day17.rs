use advent::prelude::*;

parse!();

fn move_options(salt: &str, (x, y): (usize, usize), path: &str) -> impl Iterator<Item = (u8, (usize, usize))> + use<> {
    let mut hash = [0; 32];
    hex::encode_to_slice(md5::compute(format!("{salt}{path}")).0, &mut hash).unwrap();
    [
        (b'U', (0isize, -1isize)),
        (b'D', (0, 1)),
        (b'L', (-1, 0)),
        (b'R', (1, 0)),
    ].into_iter()
        .zip([hash[0], hash[1], hash[2], hash[3]].into_iter())
        .filter_map(move |((d, (dx, dy)), h)| (h >= b'b' && h <= b'f')
            .then(move || x.checked_add_signed(dx)
                .and_then(move |x| y.checked_add_signed(dy)
                    .map(move |y| (d, (x, y)))))
            .flatten())
        .filter(move |(_, (x, y))| *x < 4 && *y < 4)
}

pub fn part1(input: Input) -> impl Display {
    for (_, (path, pos)) in bfs(("".to_owned(), (0, 0)), move |(path, pos)| {
        move_options(input, pos, &path)
            .map(move |(ch, new_pos)| {
                let mut new_path = path.clone();
                new_path.push(ch as char);
                (new_path, new_pos)
            })
    }) {
        if pos == (3, 3) {
            return path;
        }
    }
    "".to_string()
}

pub fn part2(_input: Input) -> impl Display {
    0
    // let mut longest = 0;
    // for (dist, (_, pos)) in bfs(("".to_owned(), (0, 0)), move |(path, pos)| {
    //     move_options(input, pos, &path)
    //         .map(move |(ch, new_pos)| {
    //             let mut new_path = path.clone();
    //             new_path.push(ch as char);
    //             (new_path, new_pos)
    //         })
    // }) {
    //     if pos == (3, 3) {
    //         let prev = longest;
    //         longest = longest.max(dist);
    //         if longest != prev {
    //             println!("17:2 - longest {longest}");
    //         }
    //     }
    // }
    // unreachable!()
}
