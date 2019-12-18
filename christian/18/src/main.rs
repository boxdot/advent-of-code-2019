use std::collections::*;
use std::io::{self, prelude::*};

fn parse(input: &Vec<String>) -> (Vec<u8>, u16) {
    (
        input.iter().flat_map(|x| x.bytes()).collect(),
        input[0].len() as u16,
    )
}

fn num_keys(map: &Vec<u8>) -> usize {
    map.iter().filter(|&&x| b'a' <= x && x <= b'z').count()
}

fn find_start(map: &Vec<u8>) -> u16 {
    map.iter().enumerate().find(|(_, &c)| c == b'@').unwrap().0 as u16
}

fn neighbours(pos: u16, width: u16) -> impl Iterator<Item = u16> {
    (0..4).map(move |i| [pos + 1, pos - 1, pos + width, pos - width][i])
}

fn solve1((map, width): (Vec<u8>, u16)) -> Option<usize> {
    let num_needed = num_keys(&map);
    let start = find_start(&map);

    let mut seen = HashSet::new();
    let mut stack = VecDeque::new();
    stack.push_back((start, 0_u32, 0));
    while let Some((pos, keys, dist)) = stack.pop_front() {
        if keys.count_ones() as usize == num_needed {
            return Some(dist);
        }
        for new_pos in neighbours(pos, width) {
            let mut new_keys = keys;
            let c = map[new_pos as usize];
            if c == b'#' {
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

fn solve2((mut map, width): (Vec<u8>, u16)) -> Option<usize> {
    let num_needed = num_keys(&map);
    let start = find_start(&map);

    for pos in neighbours(start, width) {
        map[pos as usize] = b'#';
    }

    let mut seen = HashSet::new();
    let mut stack = VecDeque::new();
    let starts = [
        start + 1 + width,
        start + 1 - width,
        start - 1 + width,
        start - 1 - width,
    ];
    for current in 0..4_u8 {
        stack.push_back((current, starts, 0_u32, 0));
        seen.insert((current, starts, 0));
    }

    while let Some((i, pos, keys, dist)) = stack.pop_front() {
        if keys.count_ones() as usize == num_needed {
            return Some(dist);
        }
        for coord in neighbours(pos[i as usize], width) {
            let mut new_keys = keys;
            let c = map[coord as usize];
            if c == b'#' {
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
            new_pos[i as usize] = coord;
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
    let now = std::time::Instant::now();
    println!("{:?}", solve1(input.clone()));
    println!("{}", now.elapsed().as_secs_f64());
    let now = std::time::Instant::now();
    println!("{:?}", solve2(input.clone()));
    println!("{}", now.elapsed().as_secs_f64());
}
