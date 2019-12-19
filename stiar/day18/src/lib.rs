#[macro_use]
extern crate itertools;
use derive_more::{Add, Neg, Sub};
use ndarray::{Array1, Array2};

use std::collections::{HashSet, VecDeque};
use std::convert::{From, Into};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Add, Sub, Neg)]
pub struct Position {
    pub i: i32,
    pub j: i32,
}

impl Position {
    pub fn new(i: i32, j: i32) -> Self {
        Self { i, j }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Kind {
    Empty,
    Entrance,
    Wall,
    Key(char),
    Door(char),
}

impl From<char> for Kind {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '@' => Self::Entrance,
            '#' => Self::Wall,
            k @ 'a'..='z' => Self::Key(k.to_uppercase().next().unwrap()),
            d @ 'A'..='Z' => Self::Door(d),
            _ => panic!("Unknown cell"),
        }
    }
}

impl Into<char> for Kind {
    fn into(self) -> char {
        match self {
            Self::Empty => '.',
            Self::Entrance => '@',
            Self::Wall => '#',
            Self::Key(k) => k.to_lowercase().next().unwrap(),
            Self::Door(d) => d,
        }
    }
}

pub fn parse_map(input: &str) -> Array2<Kind> {
    println!("{}", input);
    let vec: Vec<Vec<Kind>> = input
        .trim()
        .split('\n')
        .map(|x| x.chars().map(|c| c.into()).collect())
        .collect();
    let width = vec.len();
    let flattened: Array1<_> = vec.into_iter().flatten().collect();
    let height = flattened.len() / width;
    flattened.into_shape((width, height)).unwrap()
}

fn is_valid(v: &Position, map: &Array2<Kind>) -> bool {
    v.i >= 0 && (v.i as usize) < map.nrows() && v.j >= 0 && (v.j as usize) < map.ncols()
}

fn get_bit(input: u32, n: u8) -> bool {
    if n < 32 {
        input & (1 << n) != 0
    } else {
        false
    }
}

fn set_bit(input: u32, n: u8) -> u32 {
    if n < 32 {
        input | (1 << n)
    } else {
        input
    }
}

fn get_char_index(x: char) -> u8 {
    (x as u8) - ('A' as u8)
}

fn dfs(map: &mut Array2<Kind>, v: &Position, used: &mut HashSet<Position>) -> bool {
    used.insert(*v);

    let mut num_ways = 0;

    for dir in [(0, 1), (1, 0), (0, -1), (-1, 0)]
        .into_iter()
        .map(|&(di, dj)| Position::new(di, dj))
    {
        let u = *v + dir;
        if is_valid(&u, map)
            && !used.contains(&u)
            && match map[(u.i as usize, u.j as usize)] {
                Kind::Wall => false,
                _ => true,
            }
        {
            if !dfs(map, &u, used) {
                num_ways += 1;
            }
        }
    }

    if num_ways == 0
        && match map[(v.i as usize, v.j as usize)] {
            Kind::Empty => true,
            Kind::Door(_) => true,
            _ => false,
        }
    {
        map[(v.i as usize, v.j as usize)] = Kind::Wall;
        true
    } else {
        false
    }
}

fn prune_map(map: &Array2<Kind>, start: &Position) -> Array2<Kind> {
    let mut result = map.clone();
    let mut used = HashSet::new();
    dfs(&mut result, start, &mut used);
    result
}

fn bfs(map: &Array2<Kind>, start: &Position, keys_goal: usize) -> Option<u32> {
    let mut used = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((*start, 0, 0));
    used.insert((*start, 0));

    while !queue.is_empty() {
        let (position, keys, distance) = queue.pop_front().unwrap();
        if keys == (1 << keys_goal) - 1 {
            return Some(distance);
        }

        match map[(position.i as usize, position.j as usize)] {
            Kind::Key(k) if !get_bit(keys, get_char_index(k)) => {
                let keys_copy = set_bit(keys, get_char_index(k));
                queue.push_back((position, keys_copy, distance));
                used.insert((position, keys_copy));
            }
            _ => {
                for dir in [(0, 1), (1, 0), (0, -1), (-1, 0)]
                    .into_iter()
                    .map(|&(di, dj)| Position::new(di, dj))
                {
                    let new_position = position + dir;
                    if is_valid(&new_position, map)
                        && !used.contains(&(new_position, keys.clone()))
                        && match map[(new_position.i as usize, new_position.j as usize)] {
                            Kind::Empty => true,
                            Kind::Key(_) => true,
                            Kind::Entrance => true,
                            Kind::Door(w) => get_bit(keys, get_char_index(w)),
                            Kind::Wall => false,
                        }
                    {
                        used.insert((new_position, keys.clone()));
                        queue.push_back((new_position, keys.clone(), distance + 1));
                    }
                }
            }
        }
    }

    None
}

fn bfs4(
    map: &Array2<Kind>,
    starts: &(Position, Position, Position, Position),
    keys_goal: usize,
) -> Option<u32> {
    let mut used = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((*starts, 0, 0));
    used.insert((*starts, 0));

    while !queue.is_empty() {
        let (positions, keys, distance) = queue.pop_front().unwrap();
        if keys == (1 << keys_goal) - 1 {
            return Some(distance);
        }

        let mut useless = false;
        for position in &[positions.0, positions.1, positions.2, positions.3] {
            match map[(position.i as usize, position.j as usize)] {
                Kind::Key(k) if !get_bit(keys, get_char_index(k)) => {
                    let keys_copy = set_bit(keys, get_char_index(k));
                    queue.push_back((positions, keys_copy, distance));
                    used.insert((positions, keys_copy));
                    useless = true;
                    break;
                }
                _ => (),
            }
        }

        if useless {
            continue;
        }

        // TODO: Fix this monster.
        for dir in [(0, 1), (1, 0), (0, -1), (-1, 0)]
            .into_iter()
            .map(|&(di, dj)| Position::new(di, dj))
        {
            let new_positions = (positions.0 + dir, positions.1, positions.2, positions.3);
            if is_valid(&new_positions.0, map)
                && !used.contains(&(new_positions, keys.clone()))
                && match map[(new_positions.0.i as usize, new_positions.0.j as usize)] {
                    Kind::Empty => true,
                    Kind::Key(_) => true,
                    Kind::Entrance => true,
                    Kind::Door(w) => get_bit(keys, get_char_index(w)),
                    Kind::Wall => false,
                }
            {
                used.insert((new_positions, keys.clone()));
                queue.push_back((new_positions, keys.clone(), distance + 1));
            }
        }

        for dir in [(0, 1), (1, 0), (0, -1), (-1, 0)]
            .into_iter()
            .map(|&(di, dj)| Position::new(di, dj))
        {
            let new_positions = (positions.0, positions.1 + dir, positions.2, positions.3);
            if is_valid(&new_positions.1, map)
                && !used.contains(&(new_positions, keys.clone()))
                && match map[(new_positions.1.i as usize, new_positions.1.j as usize)] {
                    Kind::Empty => true,
                    Kind::Key(_) => true,
                    Kind::Entrance => true,
                    Kind::Door(w) => get_bit(keys, get_char_index(w)),
                    Kind::Wall => false,
                }
            {
                used.insert((new_positions, keys.clone()));
                queue.push_back((new_positions, keys.clone(), distance + 1));
            }
        }

        for dir in [(0, 1), (1, 0), (0, -1), (-1, 0)]
            .into_iter()
            .map(|&(di, dj)| Position::new(di, dj))
        {
            let new_positions = (positions.0, positions.1, positions.2 + dir, positions.3);
            if is_valid(&new_positions.2, map)
                && !used.contains(&(new_positions, keys.clone()))
                && match map[(new_positions.2.i as usize, new_positions.2.j as usize)] {
                    Kind::Empty => true,
                    Kind::Key(_) => true,
                    Kind::Entrance => true,
                    Kind::Door(w) => get_bit(keys, get_char_index(w)),
                    Kind::Wall => false,
                }
            {
                used.insert((new_positions, keys.clone()));
                queue.push_back((new_positions, keys.clone(), distance + 1));
            }
        }

        for dir in [(0, 1), (1, 0), (0, -1), (-1, 0)]
            .into_iter()
            .map(|&(di, dj)| Position::new(di, dj))
        {
            let new_positions = (positions.0, positions.1, positions.2, positions.3 + dir);
            if is_valid(&new_positions.3, map)
                && !used.contains(&(new_positions, keys.clone()))
                && match map[(new_positions.3.i as usize, new_positions.3.j as usize)] {
                    Kind::Empty => true,
                    Kind::Key(_) => true,
                    Kind::Entrance => true,
                    Kind::Door(w) => get_bit(keys, get_char_index(w)),
                    Kind::Wall => false,
                }
            {
                used.insert((new_positions, keys.clone()));
                queue.push_back((new_positions, keys.clone(), distance + 1));
            }
        }
    }

    None
}

pub fn get_traveling_salesman(map: &Array2<Kind>) -> Option<u32> {
    let start_position = iproduct!(0..map.nrows(), 0..map.ncols())
        .filter_map(|x| match map[x] {
            Kind::Entrance => Some(Position::new(x.0 as i32, x.1 as i32)),
            _ => None,
        })
        .next()
        .unwrap();

    let pruned = prune_map(map, &start_position);

    bfs(
        &pruned,
        &start_position,
        iproduct!(0..map.nrows(), 0..map.ncols())
            .filter(|&x| match map[x] {
                Kind::Key(_) => true,
                _ => false,
            })
            .count(),
    )
}

pub fn get_traveling_salesmans(map: &Array2<Kind>) -> Option<u32> {
    let entrances: Vec<_> = iproduct!(0..map.nrows(), 0..map.ncols())
        .filter_map(|x| match map[x] {
            Kind::Entrance => Some(Position::new(x.0 as i32, x.1 as i32)),
            _ => None,
        })
        .collect();
    let mut pruned = map.clone();
    for entrance in &entrances {
        pruned = prune_map(&pruned, &entrance);
    }

    for row in pruned.genrows() {
        println!(
            "{}",
            row.iter()
                .map(|&x| {
                    let c: char = x.into();
                    c.to_string()
                })
                .collect::<Vec<String>>()
                .join("")
        );
    }

    bfs4(
        &pruned,
        &(entrances[0], entrances[1], entrances[2], entrances[3]),
        iproduct!(0..map.nrows(), 0..map.ncols())
            .filter(|&x| match map[x] {
                Kind::Key(_) => true,
                _ => false,
            })
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test_1() {
        let map = parse_map(
            "##########
#.b.A.@.a#
#.########",
        );
        assert_eq!(get_traveling_salesman(&map), Some(8));
    }

    #[test]
    fn sample_test_2() {
        let map = parse_map(
            "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################",
        );
        assert_eq!(get_traveling_salesman(&map), Some(86));
    }

    #[test]
    fn sample_test_3() {
        let map = parse_map(
            "########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################",
        );
        assert_eq!(get_traveling_salesman(&map), Some(132));
    }

    #[test]
    fn sample_test_4() {
        let map = parse_map(
            "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################",
        );
        assert_eq!(get_traveling_salesman(&map), Some(136));
    }

    #[test]
    fn sample_test_5() {
        let map = parse_map(
            "########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################",
        );
        assert_eq!(get_traveling_salesman(&map), Some(81));
    }

    #[test]
    fn sample_test_6() {
        let map = parse_map(
            "#######
#a.#Cd#
##@#@##
#######
##@#@##
#cB#Ab#
#######",
        );
        assert_eq!(get_traveling_salesmans(&map), Some(8));
    }

    #[test]
    fn sample_test_7() {
        let map = parse_map(
            "#############
#DcBa.#.GhKl#
#.###@#@#I###
#e#d#####j#k#
###C#@#@###J#
#fEbA.#.FgHi#
#############",
        );
        assert_eq!(get_traveling_salesmans(&map), Some(32));
    }

    #[test]
    fn sample_test_8() {
        let map = parse_map(
            "#############
#g#f.D#..h#l#
#F###e#E###.#
#dCba@#@BcIJ#
#############
#nK.L@#@G...#
#M###N#H###.#
#o#m..#i#jk.#
#############",
        );
        assert_eq!(get_traveling_salesmans(&map), Some(72));
    }
}
