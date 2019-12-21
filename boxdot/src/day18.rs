use std::cmp::Reverse;
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

pub fn solve(input: &str) -> Option<(usize, usize)> {
    let vault = parse(input);
    let part1 = shortest_path_len(&vault)?;
    let part2 = shortest_path_len4(vault)?;
    Some((part1, part2))
}

struct Vault {
    map: Vec<u8>,
    width: usize,
}

fn parse(input: &str) -> Vault {
    let width = input.lines().next().unwrap().len();
    let map: Vec<_> = input.lines().flat_map(|l| l.bytes()).collect();
    Vault { map, width }
}

type Coord = (u8, u8);

struct PathFinder<'a> {
    vault: &'a Vault,
    q: VecDeque<(Coord, usize, u32)>,
    seen: HashSet<Coord>,
}

impl<'a> PathFinder<'a> {
    fn new(vault: &'a Vault) -> Self {
        Self {
            vault,
            q: Default::default(),
            seen: Default::default(),
        }
    }

    /// Finds all paths from `orig` to keys.
    ///
    /// Returns the list of (dest, dist, key, doors on the path).
    fn bfs(&mut self, orig: Coord, keys: &HashMap<Coord, u32>) -> Vec<(Coord, usize, u32, u32)> {
        let width = self.vault.width;
        let index = |(x, y)| x as usize + width * y as usize;

        self.q.clear();
        self.q.push_back((orig, 0, 0));
        self.seen.clear();
        self.seen.insert(orig);

        let mut paths = Vec::new();
        while let Some((pos, dist, doors)) = self.q.pop_front() {
            if let Some(&key) = keys.get(&pos) {
                paths.push((pos, dist, key, doors));
            }

            let (x, y) = pos;
            let neighbors = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
            for &pos in neighbors.iter() {
                if self.seen.insert(pos) {
                    let field = self.vault.map[index(pos)];
                    match field {
                        b'#' => continue,
                        b'.' | b'@' => {
                            self.q.push_back((pos, dist + 1, doors));
                        }
                        a if (a as char).is_lowercase() => {
                            self.q.push_back((pos, dist + 1, doors));
                        }
                        a if (a as char).is_uppercase() => {
                            let door = 1 << (a - b'A');
                            self.q.push_back((pos, dist + 1, doors | door))
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }

        paths
    }
}

fn shortest_path_len(vault: &Vault) -> Option<usize> {
    let width = vault.width;
    let coord = |i| ((i % width) as u8, (i / width) as u8);

    let orig = vault.map.iter().position(|&v| v == b'@').map(coord)?;
    let keys: HashMap<_, _> = vault
        .map
        .iter()
        .enumerate()
        .filter(|(_, &v)| (v as char).is_lowercase())
        .map(|(i, &v)| (coord(i), 1_u32 << (v - b'a')))
        .collect();

    // precompute all distances from each key to each key with doors on the way
    let mut path_finder = PathFinder::new(vault);
    let paths: HashMap<_, _> = keys
        .keys()
        .chain([orig].iter())
        .map(|&from| (from, path_finder.bfs(from, &keys)))
        .collect();

    let keys_bitset = (1 << keys.len()) - 1;
    dijkstra([orig], &paths, keys_bitset)
}

fn shortest_path_len4(mut vault: Vault) -> Option<usize> {
    let width = vault.width;
    let coord = |i| ((i % width) as u8, (i / width) as u8);
    let index = |(x, y)| x as usize + width * y as usize;

    let orig = vault.map.iter().position(|&v| v == b'@').map(coord)?;
    let keys: HashMap<_, _> = vault
        .map
        .iter()
        .enumerate()
        .filter(|(_, &v)| (v as char).is_lowercase())
        .map(|(i, &v)| (coord(i), 1_u32 << (v - b'a')))
        .collect();

    // add 4 robots and walls around them
    let (x, y) = orig;
    for &pos in [(x, y), (x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)].iter() {
        vault.map[index(pos)] = b'#';
    }
    let origins = [
        (x - 1, y - 1),
        (x + 1, y - 1),
        (x + 1, y + 1),
        (x - 1, y + 1),
    ];

    // precompute all distances from each key to each key with doors on the way
    let mut path_finder = PathFinder::new(&vault);
    let paths: HashMap<_, _> = keys
        .keys()
        .chain(origins.iter())
        .map(|&from| (from, path_finder.bfs(from, &keys)))
        .collect();

    let keys_bitset = (1 << keys.len()) - 1;
    dijkstra(origins, &paths, keys_bitset)
}

// Length of the shortest path to collect all keys and open all doors.
//
// We run Dijkstra on the set of all precomputed paths from origins + keys to all keys and check if
// the path if traversable by looking at the doors along it and the keys we've already collected.
fn dijkstra<Ar>(
    orig: Ar,
    paths: &HashMap<Coord, Vec<(Coord, usize, u32, u32)>>,
    keys: u32,
) -> Option<usize>
where
    Ar: AsRef<[Coord]> + AsMut<[Coord]> + Copy + Eq + Ord + std::hash::Hash,
{
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, orig, 0)));

    let mut distances = HashMap::new();
    distances.insert((orig, 0), 0);

    while let Some(Reverse((dist, positions, collected_keys))) = heap.pop() {
        if collected_keys == keys {
            return Some(dist);
        }

        for (i, pos) in positions.as_ref().iter().enumerate() {
            let reachabled_keys =
                paths[&pos]
                    .iter()
                    .copied()
                    .filter(|&(_dest, _dist, key, doors)| {
                        collected_keys & key == 0 && !collected_keys & doors == 0
                    });

            for (dest, key_dist, key, _) in reachabled_keys {
                let collected_keys = collected_keys | key;
                let dist = dist + key_dist;
                let mut positions = positions;
                positions.as_mut()[i] = dest;

                match distances.entry((positions, collected_keys)) {
                    Entry::Vacant(entry) => {
                        entry.insert(dist);
                        heap.push(Reverse((dist, positions, collected_keys)));
                    }
                    Entry::Occupied(mut entry) if dist < *entry.get() => {
                        entry.insert(dist);
                        heap.push(Reverse((dist, positions, collected_keys)));
                    }
                    _ => {}
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortest_path_len() {
        let vault = parse(
            r#"#########
#b.A.@.a#
#########"#,
        );
        assert_eq!(shortest_path_len(&vault), Some(8));

        let vault = parse(
            r#"########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################"#,
        );
        assert_eq!(shortest_path_len(&vault), Some(86));

        let vault = parse(
            r#"########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################"#,
        );
        assert_eq!(shortest_path_len(&vault), Some(132));

        let vault = parse(
            r#"#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################"#,
        );
        assert_eq!(shortest_path_len(&vault), Some(136));

        let vault = parse(
            r#"########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################"#,
        );
        assert_eq!(shortest_path_len(&vault), Some(81));
    }

    #[test]
    fn test_shortest_path_len4() {
        let vault = parse(
            r#"#######
#a.#Cd#
##...##
##.@.##
##...##
#cB#Ab#
#######"#,
        );
        assert_eq!(shortest_path_len4(vault), Some(8));
    }
}
