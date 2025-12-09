use advent::prelude::*;
use advent::prelude::ordered_compression::{OrderedCompression, OrderedCompressionKey};

parse!(Vec<usizex2>, lines({ x:usize ',' y:usize => usizex2::from_array([x, y]) }));

pub fn part1(input: Input) -> impl Display {
    input.iter()
        .cartesian_product(input.iter())
        .map(|(&a, &b)| (a.abs_diff(b) + usizex2::splat(1)).reduce_product())
        .max()
        .expect("no input")
}

fn flood_fill(grid: &mut Grid2<bool>, pos: (usize, usize)) {
    let mut queue = vec![pos];
    while let Some(pos) = queue.pop() {
        if *grid.at(pos) {
            continue;
        }
        *grid.at_mut(pos) = true;
        queue.extend(grid.neighbor_positions(pos, false));
    }
}

pub fn part2(input: Input) -> impl Display {
    let (x_coords, y_coords) = input.iter().map(|&p| (p[0], p[1])).unzip_to_vec();
    let x_coords = OrderedCompression::from_vec(x_coords);
    let y_coords = OrderedCompression::from_vec(y_coords);

    let compressed = input.iter()
        .map(|&p| usizex2::from_array([x_coords.to_key(&p[0]).0, y_coords.to_key(&p[1]).0]))
        .collect_vec();
    let size = compressed.iter().copied().reduce(|a, b| a.simd_max(b)).unwrap() + usizex2::splat(1);

    let mut grid = Grid2::new_value(size, false);
    for (&a, &b) in compressed.iter().circular_tuple_windows() {
        let lo = a.simd_min(b);
        let hi = a.simd_max(b);
        grid.slice_mut(lo, hi - lo + usizex2::splat(1)).fill(true);
    }
    flood_fill(&mut grid, (size[0] / 2, size[1] / 2));

    compressed.iter()
        .cartesian_product(compressed.iter())
        .filter(|&(&a, &b)| {
            let lo = a.simd_min(b);
            let hi = a.simd_max(b);
            grid.slice(lo, hi - lo + usizex2::splat(1)).iter().all(|&x| x)
        })
        .map(|(&a, &b)| { (
            usizex2::from_array([
                *x_coords.to_val(OrderedCompressionKey(a[0])),
                *y_coords.to_val(OrderedCompressionKey(a[1])),
            ]),
            usizex2::from_array([
                *x_coords.to_val(OrderedCompressionKey(b[0])),
                *y_coords.to_val(OrderedCompressionKey(b[1])),
            ]),
        ) })
        .map(|(a, b)| (a.abs_diff(b) + usizex2::splat(1)).reduce_product())
        .max()
        .expect("no input")
}
