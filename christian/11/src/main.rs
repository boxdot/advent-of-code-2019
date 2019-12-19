use core::cell::RefCell;
use itertools::*;
use std::collections::HashMap;
use std::io::{self, prelude::*};
use vm::Vm;

fn parse(input: &Vec<String>) -> Vec<i64> {
    let ints = input.iter().map(|x| x.split(',')).flatten();
    ints.filter_map(|x| x.parse().ok()).collect()
}

fn solve(mem: &Vec<i64>, start: i64) -> (usize, String) {
    let hull: RefCell<HashMap<_, _>> = RefCell::new([((0, 0), start)].iter().cloned().collect());
    let (pos, mut dir) = (RefCell::new((0, 0)), (0, -1));
    let input = || Some(*hull.borrow().get(&pos.borrow()).unwrap_or(&0));
    for (color, turn) in Vm::new(mem.clone(), std::iter::from_fn(input)).tuples() {
        let prev_pos = *pos.borrow();
        hull.borrow_mut().insert(prev_pos, color);
        match turn {
            0 => dir = (dir.1, -dir.0),
            1 => dir = (-dir.1, dir.0),
            e => panic!("{:?}", e),
        }
        *pos.borrow_mut() = (prev_pos.0 + dir.0, prev_pos.1 + dir.1);
    }

    let hull = hull.into_inner();
    let y_range = hull.keys().map(|(_, y)| *y).minmax().into_option().unwrap();
    let x_range = hull.keys().map(|(x, _)| *x).minmax().into_option().unwrap();
    let append = |mut res: String, (y, x)| {
        let value = *hull.get(&(x, y)).unwrap_or(&0);
        res.push(if value == 0 { ' ' } else { '█' });
        res.push(if x == x_range.1 { '\n' } else { '‎' });
        res
    };
    let pic = iproduct!(y_range.0..=y_range.1, (x_range.0..=x_range.1)).fold(String::new(), append);
    (hull.len(), pic)
}

fn main() {
    let input = parse(&io::stdin().lock().lines().map(|x| x.unwrap()).collect());
    println!("Part1:\n{}", solve(&input, 0).0);
    println!("Part2:\n{}", solve(&input, 1).1);
}
