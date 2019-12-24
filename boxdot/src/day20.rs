use std::cmp::Reverse;
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

pub fn solve(input: &str) -> Option<(usize, usize)> {
    let map = parse(input);
    let part1 = shortest_path_len(&map)?;
    let part2 = shortest_path_len_with_levels(&map)?;
    Some((part1, part2))
}

type Coord = (usize, usize);
type Portal = (char, char);

#[derive(Debug)]
struct Vault {
    map: Vec<Vec<u8>>,
}

impl Vault {
    fn iter(&self) -> impl Iterator<Item = (Coord, char)> + Clone + '_ {
        self.map
            .iter()
            .enumerate()
            .flat_map(move |(y, l)| l.iter().enumerate().map(move |(x, c)| ((x, y), *c as char)))
    }

    fn get(&self, (x, y): Coord) -> Option<char> {
        Some(*self.map.get(y)?.get(x)? as char)
    }

    fn is_outer(&self, (x, y): Coord) -> bool {
        let width = self.map[0].len();
        let height = self.map.len();
        x == 2 || x + 3 == width || y == 2 || y + 3 == height
    }
}

fn parse(input: &str) -> Vault {
    let map = input.lines().map(|l| l.bytes().collect()).collect();
    Vault { map }
}

fn portals(vault: &Vault) -> HashMap<Coord, Portal> {
    let letters = vault.iter().filter(|&(_, c)| c.is_ascii_uppercase());
    let horiz = letters.clone().filter_map(|((x, y), c)| {
        vault
            .get((x + 1, y))
            .filter(char::is_ascii_uppercase)
            .map(|d| {
                if vault.get((x + 2, y)) == Some('.') {
                    ((x + 2, y), (c, d))
                } else if vault.get((x.saturating_sub(1), y)) == Some('.') {
                    ((x - 1, y), (c, d))
                } else {
                    panic!("invalid portal");
                }
            })
    });
    let vert = letters.clone().filter_map(|((x, y), c)| {
        vault
            .get((x, y + 1))
            .filter(char::is_ascii_uppercase)
            .map(|d| {
                if vault.get((x, y + 2)) == Some('.') {
                    ((x, y + 2), (c, d))
                } else if vault.get((x, y.saturating_sub(1))) == Some('.') {
                    ((x, y - 1), (c, d))
                } else {
                    panic!("invalid portal");
                }
            })
    });
    horiz.chain(vert).collect()
}

fn bfs(orig: Coord, vault: &Vault, portals: &HashMap<Coord, Portal>) -> Vec<(usize, Coord, bool)> {
    let mut q = VecDeque::new();
    q.push_back((0, orig));
    let mut seen = HashSet::new();
    seen.insert(orig);

    let mut paths = Vec::new();

    while let Some((dist, pos)) = q.pop_front() {
        if let Some(portal) = portals.get(&pos) {
            if dist != 0 {
                let jump_pos = portals
                    .iter()
                    .find(|&(&p_pos, p)| p_pos != pos && p == portal)
                    .map(|(&pos, _)| pos);
                let dist = dist + jump_pos.map(|_| 1).unwrap_or(0);
                let pos = jump_pos.unwrap_or(pos);
                paths.push((dist, pos, vault.is_outer(pos)));
            }
        }

        let (x, y) = pos;
        let neighbors = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
        for &pos in neighbors.iter() {
            if seen.insert(pos) {
                match vault.get(pos) {
                    Some('.') => q.push_back((dist + 1, pos)),
                    Some(a) if a.is_ascii_uppercase() && portals.get(&pos).is_some() => {
                        q.push_back((dist + 1, pos))
                    }
                    _ => {}
                }
            }
        }
    }

    paths
}

fn shortest_path_len(vault: &Vault) -> Option<usize> {
    let portals = portals(vault);
    let (&orig, _) = portals.iter().find(|&(_, &p)| p == ('A', 'A'))?;
    let (&dest, _) = portals.iter().find(|&(_, &p)| p == ('Z', 'Z'))?;

    let paths: HashMap<Coord, _> = portals
        .keys()
        .map(|&orig| (orig, bfs(orig, vault, &portals)))
        .collect();

    dijkstra(orig, dest, &paths)
}

fn shortest_path_len_with_levels(vault: &Vault) -> Option<usize> {
    let portals = portals(vault);
    let (&orig, _) = portals.iter().find(|&(_, &p)| p == ('A', 'A'))?;
    let (&dest, _) = portals.iter().find(|&(_, &p)| p == ('Z', 'Z'))?;

    let paths: HashMap<Coord, _> = portals
        .keys()
        .map(|&orig| (orig, bfs(orig, vault, &portals)))
        .collect();

    multilevel_dijksta(orig, dest, &paths)
}

fn dijkstra(
    orig: Coord,
    dest: Coord,
    paths: &HashMap<Coord, Vec<(usize, Coord, bool)>>,
) -> Option<usize> {
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, orig)));

    let mut distances = HashMap::new();
    distances.insert(orig, 0);

    while let Some(Reverse((dist, pos))) = heap.pop() {
        if pos == dest {
            return Some(dist);
        }

        for &(portal_dist, pos, _is_outer) in &paths[&pos] {
            let dist = dist + portal_dist;
            match distances.entry(pos) {
                Entry::Vacant(entry) => {
                    entry.insert(dist);
                    heap.push(Reverse((dist, pos)));
                }
                Entry::Occupied(mut entry) if dist < *entry.get() => {
                    entry.insert(dist);
                    heap.push(Reverse((dist, pos)));
                }
                _ => {}
            }
        }
    }

    None
}

fn multilevel_dijksta(
    orig: Coord,
    dest: Coord,
    paths: &HashMap<Coord, Vec<(usize, Coord, bool)>>,
) -> Option<usize> {
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, orig, 0_isize))); // (dist, pos, level)

    let mut distances = HashMap::new();
    distances.insert((orig, 0_isize), 0); // (pos, level) -> dist

    while let Some(Reverse((dist, pos, level))) = heap.pop() {
        for &(portal_dist, pos, is_outer) in &paths[&pos] {
            if pos == orig {
                continue;
            } else if pos == dest {
                if level == 0 {
                    return Some(dist + portal_dist);
                } else {
                    continue;
                }
            }

            let dist = dist + portal_dist;
            let level = level + if is_outer { 1 } else { -1 };
            if level < 0 {
                continue;
            }

            match distances.entry((pos, level)) {
                Entry::Vacant(entry) => {
                    entry.insert(dist);
                    heap.push(Reverse((dist, pos, level)));
                }
                Entry::Occupied(mut entry) if dist < *entry.get() => {
                    entry.insert(dist);
                    heap.push(Reverse((dist, pos, level)));
                }
                _ => {}
            }
        }
    }

    None
}
