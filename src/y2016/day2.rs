use advent::prelude::*;

parse!(Vec<Vec<i32x2>>, lines(direction_ch()+));

pub fn part1(input: Input) -> impl Display {
    input.into_iter().fold((i32x2::from([1, 1]), "".to_string()), |(start_pos, code), steps| {
        let pos = steps.into_iter()
            .fold(start_pos, |pos, step| (pos + step)
                .simd_clamp(i32x2::splat(0), i32x2::splat(2)));
        (pos, code + &(1 + pos[0] + 3 * pos[1]).to_string())
    }).1
}

pub fn part2(input: Input) -> impl Display {
    let keypad = Grid2::<char>::new_str("  1  \n 234 \n56789\n ABC \n  D  ");
    let mut result = String::new();
    for steps in input {
        let mut pos = i32x2::from_array([0, 2]);
        for step in steps {
            let np = pos + step;
            if keypad.get(np).copied().unwrap_or(' ') != ' ' {
                pos = np;
            }
        }
        result.push(*keypad.at(pos));
    }
    result
}
