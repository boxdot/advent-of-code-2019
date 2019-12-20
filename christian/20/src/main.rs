use std::collections::*;
use std::io::{self, prelude::*};

fn parse(input: &Vec<String>) -> (Vec<u8>, usize) {
    (
        input.iter().flat_map(|x| x.bytes()).collect(),
        input[0].len(),
    )
}

fn portals(map: &Vec<u8>, width: usize) -> HashMap<[u8; 2], Vec<usize>> {
    let is_portal = |pos: usize| (b'A'..=b'Z').contains(&map[pos]);
    let mut result: HashMap<[u8; 2], Vec<usize>> = HashMap::new();
    for pos in (0..map.len()).filter(|pos| map[*pos] == b'.') {
        let mut add_portal = |a, b| {
            if is_portal(a) && is_portal(b) {
                result.entry([map[a], map[b]]).or_default().push(pos);
            }
        };
        add_portal(pos + 1, pos + 2);
        add_portal(pos - 2, pos - 1);
        add_portal(pos + width, pos + 2 * width);
        add_portal(pos - 2 * width, pos - 1 * width);
    }
    result
}

fn neighbours(pos: usize, width: usize) -> impl Iterator<Item = usize> {
    (0..4).map(move |i| [pos + 1, pos - 1, pos + width, pos - width][i])
}

fn solve1((map, width): (Vec<u8>, usize), levels: bool) -> Option<usize> {
    let portals = portals(&map, width);
    let start = *portals.get(b"AA").unwrap().first().unwrap();

    let mut q: VecDeque<_> = [(start, 0, 0)].iter().copied().collect();
    let mut seen: HashSet<_> = [(start, 0)].iter().copied().collect();
    while let Some((pos, dist, level)) = q.pop_front() {
        for mut n in neighbours(pos, width).filter(|n| map[*n as usize] != b'#') {
            let mut new_level = level;
            if (b'A'..=b'Z').contains(&map[n]) {
                let id = [map[n.min(2 * n - pos)], map[n.max(2 * n - pos)]];
                if &id == b"ZZ" && level == 0 {
                    return Some(dist);
                }
                let inside = (4..width - 3).contains(&(n % width))
                    && (4..map.len() / width - 3).contains(&(n / width));
                new_level += if inside { 1 } else { -1 } * levels as i32;
                if let Some(target) = portals.get(&id).and_then(|x| x.iter().find(|n| **n != pos)) {
                    n = *target;
                } else {
                    continue;
                }
            }
            if new_level >= 0 && seen.insert((n, new_level)) {
                q.push_back((n, dist + 1, new_level));
            }
        }
    }
    None
}

fn main() {
    let input = parse(&io::stdin().lock().lines().map(|x| x.unwrap()).collect());
    println!("{:?}", solve1(input.clone(), false));
    println!("{:?}", solve1(input.clone(), true));
}
