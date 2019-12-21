use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};

pub fn solve(input: &str) -> Option<usize> {
    let vault = parse(input);
    shortest_path_len(&vault)
}

struct Vault {
    map: Vec<char>,
    width: usize,
}

fn parse(input: &str) -> Vault {
    let width = input.lines().next().unwrap().len();
    let map: Vec<_> = input.lines().flat_map(|l| l.chars()).collect();
    Vault { map, width }
}

type Coord = (usize, usize);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
struct State {
    pos: Coord,
    missing_keys: u32,
}

impl State {
    fn can_open(&self, a: char) -> bool {
        let bit = 1 << (a as u8 - b'A') as usize;
        self.missing_keys & bit == 0
    }

    fn add_key(&mut self, a: char) {
        let bit = !(1 << (a as u8 - b'a') as usize);
        self.missing_keys &= bit;
    }

    fn add_missing_key(&mut self, a: char) {
        let bit = 1 << (a as u8 - b'a') as usize;
        self.missing_keys |= bit;
    }
}

// length of the shortest path to collect all keys and open all doors
fn shortest_path_len(vault: &Vault) -> Option<usize> {
    let index = |(x, y)| x + vault.width * y;

    let mut state = State::default();
    for (i, field) in vault.map.iter().copied().enumerate() {
        match field {
            '@' => state.pos = (i % vault.width, i / vault.width),
            '#' | '.' => (),
            a if a.is_lowercase() => state.add_missing_key(a),
            _ => (),
        };
    }

    let mut q = VecDeque::new();
    q.push_back(state);

    let mut distances = HashMap::new();
    distances.insert(state, 0);

    while let Some(state) = q.pop_front() {
        let dist = distances[&state];
        if state.missing_keys == 0 {
            return Some(dist);
        }

        let (x, y) = state.pos;
        let neightbors = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
        for pos in neightbors.iter().copied() {
            let field = vault.map[index(pos)];
            match field {
                '#' => continue,
                a => {
                    let mut next_state = State {
                        pos,
                        missing_keys: state.missing_keys,
                    };
                    if a != '.' && a != '@' {
                        if a.is_uppercase() && !state.can_open(a) {
                            // we don't have the key yet
                            continue;
                        } else if a.is_lowercase() {
                            next_state.add_key(a);
                        }
                    }
                    if let Entry::Vacant(entry) = distances.entry(next_state) {
                        entry.insert(dist + 1);
                        q.push_back(next_state);
                    }
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
}
