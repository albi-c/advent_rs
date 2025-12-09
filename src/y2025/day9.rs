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
    grid.flood_fill(size / usizex2::splat(2), &false, &true, false);

    compressed.iter()
        .cartesian_product(compressed.iter())
        .filter(|&(&a, &b)| {
            let lo = a.simd_min(b);
            let hi = a.simd_max(b);
            let len = hi - lo + usizex2::splat(1);
            Grid2Range::new(lo, len)
                .edges()
                .all(|p| grid.slice_range(&p).iter().all(|&x| x))
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
