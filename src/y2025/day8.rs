use std::cell::RefCell;
use std::rc::Rc;
use advent::prelude::*;
use advent::vector::prelude::*;

parse!(Vec<USVec3>, lines({ x:usize ',' y:usize ',' z:usize => USVec3::from_array([x, y, z]) }));

#[derive(Debug)]
struct Circuit {
    pub boxes: Vec<usize>,
    pub id: usize,
}

fn distance_squared(a: USVec3, b: USVec3) -> usize {
    let d = a - b;
    (d * d).reduce_sum()
}

fn prepare_input(input: &Input) -> (Vec<Rc<RefCell<Circuit>>>, Vec<(usize, usize, usize)>, Grid2<bool>) {
    let circuits = (0..input.len())
        .map(|i| Rc::new(RefCell::new(Circuit { boxes: vec![i], id: i })))
        .collect_vec();
    let pairs = (0..input.len())
        .cartesian_product(0..input.len())
        .filter(|&(a, b)| a != b)
        .map(|(a, b)| (a, b, distance_squared(input[a], input[b])))
        .sorted_unstable_by_key(|(_, _, d)| *d)
        .collect_vec();
    let direct = Grid2::new_value((input.len(), input.len()), false);
    (circuits, pairs, direct)
}

fn connect(a: usize, b: usize, circuits: &mut Vec<Rc<RefCell<Circuit>>>, direct: &mut Grid2<bool>) -> (bool, bool) {
    if direct[(a, b)] {
        return (false, false);
    }
    direct[(a, b)] = true;
    direct[(b, a)] = true;

    if circuits[a].borrow().id == circuits[b].borrow().id {
        return (true, false);
    }

    let ptr = &raw mut *circuits;
    let [ac, bc] = circuits.get_disjoint_mut([a, b]).expect("invalid pair");
    let bc = std::mem::replace(bc, ac.clone());
    for &i in bc.borrow().boxes.iter() {
        if i != a && i != b {
            *unsafe { (&mut *ptr).as_mut_ptr().add(i).as_mut().unwrap() } = ac.clone();
        }
    }
    ac.borrow_mut().boxes.extend(bc.borrow_mut().boxes.drain(..));
    (true, true)
}

pub fn part1(input: Input) -> impl Display {
    let (mut circuits, pairs, mut direct) = prepare_input(&input);
    let mut connected = 0usize;
    for (a, b, _) in pairs {
        if connected == 1000 {
            break;
        }
        if connect(a, b, &mut circuits, &mut direct).0 {
            connected += 1;
        }
    }
    let lengths = circuits.iter()
        .unique_by(|c| c.borrow().id)
        .map(|c| c.borrow().boxes.len())
        .sorted_unstable()
        .rev()
        .collect_vec();
    lengths[..3].iter().product::<usize>()
}

pub fn part2(input: Input) -> impl Display {
    let (mut circuits, pairs, mut direct) = prepare_input(&input);
    let mut diff_circuits = circuits.len();
    for (a, b, _) in pairs {
        if connect(a, b, &mut circuits, &mut direct).1 {
            diff_circuits -= 1;
            if diff_circuits == 1 {
                return input[a][0] * input[b][0];
            }
        }
    }
    panic!("no solution")
}
