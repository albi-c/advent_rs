use advent::prelude::*;
use z3::ast::Int;
use z3::Optimize;

parse!(Vec<(Vec<bool>, Vec<Vec<usize>>, Vec<u32>)>, lines(
    '[' { '.' => false, '#' => true }+ ']'
    { " (" repeat_sep(usize, ',') ')' }+
    " {" repeat_sep(u32, ',') '}'
));

pub fn part1(input: Input) -> impl Display {
    let mut result = 0;
    'outer: for (target_state, buttons, _) in input {
        for (dist, state) in bfs(vec![false; target_state.len()], |state| {
            buttons.iter()
                .map(move |btn| {
                    let mut state = state.clone();
                    for &i in btn {
                        state[i] = !state[i];
                    }
                    state
                })
        }) {
            if state == target_state {
                result += dist;
                continue 'outer;
            }
        }
        panic!("no solution");
    }
    result
}

pub fn part2(input: Input) -> impl Display {
    let mut result = 0;
    for (_, buttons, target_state) in input {
        let ctx = Optimize::new();
        let presses = (0..buttons.len())
            .map(|i| {
                let p = Int::fresh_const(&format!("presses_{i}"));
                ctx.assert(&p.ge(Int::from(0)));
                p
            })
            .collect_vec();
        for (i, &target) in target_state.iter().enumerate() {
            let mut s = Int::from(0);
            for (j, btn) in buttons.iter().enumerate() {
                if btn.contains(&i) {
                    s += &presses[j];
                }
            }
            ctx.assert(&s.eq(Int::from(target)));
        }
        let mut all_presses = Int::from(0);
        for p in &presses {
            all_presses += p;
        }
        ctx.minimize(&all_presses);
        ctx.check(&[]);
        let model = ctx.get_model().unwrap();
        result += model.eval(&all_presses, true).unwrap().as_u64().unwrap();
    }
    result
}
