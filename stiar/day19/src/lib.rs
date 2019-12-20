mod intcode;
use intcode::*;

#[macro_use]
extern crate itertools;

use std::collections::HashMap;

fn is_covered(
    mut program: Program,
    cache: &mut HashMap<(u32, u32), bool>,
    (i, j): (u32, u32),
) -> bool {
    if cache.get(&(i, j)).is_none() {
        cache.insert(
            (i, j),
            program
                .run(([i as i64, j as i64]).into_iter().cloned())
                .next()
                .unwrap()
                == 1,
        );
    }
    cache.get(&(i, j)).unwrap().clone()
}

pub fn count_affected(commands: Vec<i64>) -> usize {
    let program = Program::new(commands);

    let mut cache = HashMap::new();

    iproduct!(0..50, 0..50)
        .filter(|&x| is_covered(program.clone(), &mut cache, x))
        .count()
}

fn fit(program: Program, cache: &mut HashMap<(u32, u32), bool>, size: u32, i: u32) -> Option<u32> {
    let mut has_covered = false;
    for j in i..2 * i {
        if !is_covered(program.clone(), cache, (i, j)) {
            if has_covered {
                return None;
            }
            continue;
        } else {
            has_covered = true;
        }

        if iproduct!(i..i + size, j..j + size).all(|x| is_covered(program.clone(), cache, x)) {
            return Some(i * 10000 + j);
        }
    }
    None
}

pub fn print_beam(commands: Vec<i64>, size: u32) {
    let mut cache = HashMap::new();
    for i in 0..size {
        println!(
            "{}",
            (0..size)
                .map(
                    |j| match is_covered(Program::new(commands.clone()), &mut cache, (i, j)) {
                        true => "#".to_string(),
                        false => ".".to_string(),
                    }
                )
                .collect::<Vec<_>>()
                .join("")
        );
    }
}

pub fn find_closest_fitting_square(commands: Vec<i64>, size: u32) -> u32 {
    let program = Program::new(commands);
    let mut cache = HashMap::new();

    let mut left = 2;
    let mut right = 2;

    while fit(program.clone(), &mut cache, size, right).is_none() {
        left = right;
        right *= 2;
    }

    while left + 1 < right {
        let mid = (left + right) / 2;
        if fit(program.clone(), &mut cache, size, mid).is_some() {
            right = mid;
        } else {
            left = mid;
        }
    }

    if let Some(result) = fit(program.clone(), &mut cache, size, left) {
        result
    } else {
        fit(program.clone(), &mut cache, size, right).unwrap()
    }
}
