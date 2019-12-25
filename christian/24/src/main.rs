use itertools::*;
use std::collections::*;
use std::io::{self, prelude::*};

fn parse1(input: &Vec<String>) -> u32 {
    let mut result = 0;
    for (i, x) in input.iter().flat_map(|x| x.bytes()).enumerate() {
        if x == b'#' {
            result |= 1 << i;
        }
    }
    result
}

fn neighbours(pos: u32) -> u32 {
    let mut result = 0;
    if pos % 5 != 0 {
        result |= 1 << (pos - 1);
    }
    if pos % 5 != 4 {
        result |= 1 << (pos + 1);
    }
    if pos >= 5 {
        result |= 1 << (pos - 5);
    }
    if pos < 20 {
        result |= 1 << (pos + 5);
    }
    result
}

fn solve1(mut map: u32) -> u32 {
    let mut seen = HashSet::new();
    seen.insert(map);
    loop {
        let mut next_map = 0;
        for pos in 0..25 {
            next_map |= match (map & (1 << pos) != 0, (map & neighbours(pos)).count_ones()) {
                (true, 1) | (false, 1..=2) => 1 << pos,
                _ => 0,
            };
        }
        map = next_map;
        if !seen.insert(map) {
            return map;
        }
    }
}

fn parse2(input: &Vec<String>) -> HashSet<(i32, i8, i8)> {
    input
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes()
                .enumerate()
                .filter_map(move |(x, c)| Some((0, x as i8, y as i8)).filter(|_| c == b'#'))
        })
        .collect()
}

fn solve2(mut map: HashSet<(i32, i8, i8)>) -> usize {
    for i in 1..201 {
        let mut next = HashSet::new();
        for (level, x, y) in iproduct!(-i..=i, 0..5, 0..5).filter(|(_, x, y)| (*x, *y) != (2, 2)) {
            let mut num_neighbours = 0;
            if x == 0 {
                num_neighbours += map.contains(&(level - 1, 1, 2)) as u32;
            } else {
                num_neighbours += map.contains(&(level, x - 1, y)) as u32;
            }
            if x == 4 {
                num_neighbours += map.contains(&(level - 1, 3, 2)) as u32;
            } else {
                num_neighbours += map.contains(&(level, x + 1, y)) as u32;
            }
            if y == 0 {
                num_neighbours += map.contains(&(level - 1, 2, 1)) as u32;
            } else {
                num_neighbours += map.contains(&(level, x, y - 1)) as u32;
            }
            if y == 4 {
                num_neighbours += map.contains(&(level - 1, 2, 3)) as u32;
            } else {
                num_neighbours += map.contains(&(level, x, y + 1)) as u32;
            }
            if x == 2 && (y == 1 || y == 3) {
                for x_inner in 0..5 {
                    num_neighbours += map.contains(&(level + 1, x_inner, 2 * y - 2)) as u32;
                }
            }
            if y == 2 && (x == 1 || x == 3) {
                for y_inner in 0..5 {
                    num_neighbours += map.contains(&(level + 1, 2 * x - 2, y_inner)) as u32;
                }
            }
            match (map.contains(&(level, x, y)), num_neighbours) {
                (true, 1) | (false, 1..=2) => next.insert((level, x, y)),
                _ => false,
            };
        }
        map = next;
    }
    map.len()
}

fn main() {
    let input = io::stdin().lock().lines().map(|x| x.unwrap()).collect();
    println!("{:?}", solve1(parse1(&input)));
    println!("{:?}", solve2(parse2(&input)));
}
