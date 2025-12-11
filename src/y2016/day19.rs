use advent::prelude::*;

parse!(u32, u32);

pub fn part1(input: Input) -> impl Display {
    let mut list = (1..=input).collect_vec();
    let mut i = 0;
    while list.len() > 1 {
        list.retain(|_| {
            i += 1;
            i % 2 == 1
        });
    }
    list[0]
}

pub fn part2(input: Input) -> impl Display {
    let mut list = (1..=input).collect::<indexset::BTreeSet<_>>();
    let mut cur = 0;
    while list.len() > 1 {
        let Some(&n) = list.lower_bound(&(cur + 1)) else {
            cur = 0;
            continue;
        };
        let idx = list.rank(&n);
        list.pop_index((idx + list.len() / 2) % list.len());
        if idx >= list.len() {
            cur = 0;
        } else {
            cur = n;
        }
    }
    list.pop_first().unwrap()
}
