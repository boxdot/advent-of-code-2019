use std::collections::*;
use std::io::{self, prelude::*};

fn parse(input: &Vec<String>) -> Vec<Vec<u8>> {
    input.iter().map(|x| x.clone().into_bytes()).collect()
}

fn num_keys(map: &Vec<Vec<u8>>) -> usize {
    map.iter()
        .flat_map(|r| r)
        .filter(|&x| (b'a'..=b'z').contains(x))
        .count()
}

fn find_start(map: &Vec<Vec<u8>>) -> (usize, usize) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == b'@' {
                return (x, y);
            }
        }
    }
    panic!("No start found");
}

fn neighbours(pos: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    (0..4).map(move |i| {
        [
            (pos.0 + 1, pos.1),
            (pos.0 - 1, pos.1),
            (pos.0, pos.1 + 1),
            (pos.0, pos.1 - 1),
        ][i]
    })
}

fn solve1(map: Vec<Vec<u8>>) -> Option<usize> {
    let num_needed = num_keys(&map);
    let start = find_start(&map);

    let mut seen = HashSet::new();
    let mut stack = VecDeque::new();
    stack.push_back((start, 0_u32, 0));
    while let Some((pos, keys, dist)) = stack.pop_front() {
        if keys.count_ones() as usize == num_needed {
            return Some(dist);
        }
        for new_pos in neighbours(pos) {
            let mut new_keys = keys;
            let c = map[new_pos.1][new_pos.0];
            if map[new_pos.1][new_pos.0] == b'#' {
                continue;
            }
            if (b'A'..=b'Z').contains(&c) && (keys & (1 << (c - b'A')) == 0) {
                continue;
            }
            if (b'a'..=b'z').contains(&c) {
                new_keys |= 1 << (c - b'a');
            }
            if seen.insert((new_pos, new_keys)) {
                stack.push_back((new_pos, new_keys, dist + 1));
            }
        }
    }
    None
}

fn solve2(mut map: Vec<Vec<u8>>) -> Option<usize> {
    let num_needed = num_keys(&map);
    let start = find_start(&map);

    for pos in neighbours(start) {
        map[pos.1][pos.0] = b'#';
    }

    let mut seen = HashSet::new();
    let mut stack = VecDeque::new();
    let starts = [
        (start.0 + 1, start.1 + 1),
        (start.0 + 1, start.1 - 1),
        (start.0 - 1, start.1 + 1),
        (start.0 - 1, start.1 - 1),
    ];
    for current in 0..4 {
        stack.push_back((current, starts, 0_u32, 0));
        seen.insert((current, starts, 0));
    }

    while let Some((i, pos, keys, dist)) = stack.pop_front() {
        if keys.count_ones() as usize == num_needed {
            return Some(dist);
        }
        for coord in neighbours(pos[i]) {
            let mut new_keys = keys;
            let c = map[coord.1][coord.0];
            if map[coord.1][coord.0] == b'#' {
                continue;
            }
            if (b'A'..=b'Z').contains(&c) && (keys & (1 << (c - b'A')) == 0) {
                continue;
            }
            let has_key = (b'a'..=b'z').contains(&c) && (new_keys & 1 << (c - b'a')) == 0;
            if has_key {
                new_keys |= 1 << (c - b'a');
            }
            let mut new_pos = pos.clone();
            new_pos[i] = coord;
            for j in 0..4 {
                if j == i || has_key {
                    if seen.insert((j, new_pos, new_keys)) {
                        stack.push_back((j, new_pos, new_keys, dist + 1));
                    }
                }
            }
        }
    }
    None
}

fn main() {
    let input = parse(&io::stdin().lock().lines().map(|x| x.unwrap()).collect());
    println!("{:?}", solve1(input.clone()));
    println!("{:?}", solve2(input.clone()));
}
