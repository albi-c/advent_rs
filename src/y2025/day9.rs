use advent::prelude::*;
use advent::prelude::ordered_compression::CoordinateCompression;

parse!(Vec<usizex2>, lines({ x:usize ',' y:usize => usizex2::from_array([x, y]) }));

pub fn part1(input: Input) -> impl Display {
    input.iter()
        .cartesian_product(input.iter())
        .map(|(&a, &b)| (a.abs_diff(b) + usizex2::splat(1)).reduce_product())
        .max()
        .expect("no input")
}

pub fn part2(input: Input) -> impl Display {
    let coord_comp = CoordinateCompression::from_iter(input.iter().copied());

    let compressed = input.iter()
        .map(|&p| usizex2::from_array(coord_comp.compress(p)))
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
            usizex2::from_array(coord_comp.decompress(a)),
            usizex2::from_array(coord_comp.decompress(b)),
        ) })
        .map(|(a, b)| (a.abs_diff(b) + usizex2::splat(1)).reduce_product())
        .max()
        .expect("no input")
}
