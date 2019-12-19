#[macro_use]
extern crate itertools;
use derive_more::{Add, Neg, Sub};
use itertools::Itertools;
use ndarray::{Array1, Array2};

use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::From;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Cell {
    pub position: Position,
    pub kind: Kind,
}

impl Cell {
    pub fn new(position: Position, kind: Kind) -> Self {
        Self { position, kind }
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Add, Sub, Neg)]
pub struct Position {
    pub i: i64,
    pub j: i64,
}

impl Position {
    pub fn new(i: i64, j: i64) -> Self {
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

pub fn parse_map(input: &str) -> Array2<Kind> {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pois {
    pub entrance: Cell,
    pub keys: Vec<Cell>,
    pub doors: Vec<Cell>,
}

pub fn get_pois(map: &Array2<Kind>) -> Pois {
    Pois {
        entrance: iproduct!(0..map.nrows(), 0..map.ncols())
            .map(|x| Cell::new(Position::new(x.0 as i64, x.1 as i64), map[x]))
            .filter(|c| c.kind == Kind::Entrance)
            .next()
            .unwrap(),
        keys: iproduct!(0..map.nrows(), 0..map.ncols())
            .map(|x| Cell::new(Position::new(x.0 as i64, x.1 as i64), map[x]))
            .filter_map(|c| match c.kind {
                Kind::Key(_) => Some(c),
                _ => None,
            })
            .collect(),
        doors: iproduct!(0..map.nrows(), 0..map.ncols())
            .map(|x| Cell::new(Position::new(x.0 as i64, x.1 as i64), map[x]))
            .filter_map(|c| match c.kind {
                Kind::Door(_) => Some(c),
                _ => None,
            })
            .collect(),
    }
}

fn is_valid(v: &Position, map: &Array2<Kind>) -> bool {
    v.i >= 0 && (v.i as usize) < map.nrows() && v.j >= 0 && (v.j as usize) < map.ncols()
}

pub fn get_traveling_salesman_path(
    map: &Array2<Kind>,
    start: &Position,
    stops: &[Position],
) -> Option<u32> {
    //println!("Stops: {:?}", stops);
    let mut keys = HashSet::new();

    let mut total_distance = 0;

    let mut current_start = start.clone();
    for stop in stops {
        let mut used = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((current_start, 0));
        used.insert(current_start);

        let mut found = false;
        while !queue.is_empty() {
            let (position, distance) = queue.pop_front().unwrap();

            match map[(position.i as usize, position.j as usize)] {
                Kind::Key(k) => {
                    keys.insert(k);
                }
                _ => (),
            };

            if position == *stop {
                total_distance += distance;
                found = true;
                break;
            }

            for dir in [(0, 1), (1, 0), (0, -1), (-1, 0)]
                .into_iter()
                .map(|&(di, dj)| Position::new(di, dj))
            {
                let new_position = position + dir;
                if is_valid(&new_position, map)
                    && !used.contains(&new_position)
                    && match map[(new_position.i as usize, new_position.j as usize)] {
                        Kind::Empty => true,
                        Kind::Key(_) => true,
                        Kind::Entrance => true,
                        Kind::Door(w) => keys.contains(&w),
                        Kind::Wall => false,
                    }
                {
                    used.insert(new_position);
                    queue.push_back((new_position, distance + 1));
                }
            }
        }

        if !found {
            return None;
        }

        current_start = *stop;
    }

    Some(total_distance)
}

pub fn get_traveling_salesman(map: &Array2<Kind>) -> Option<u32> {
    let pois = get_pois(&map);
    let mut result = None;
    for keys in pois.keys.iter().permutations(pois.keys.len()) {
        let distance_opt = get_traveling_salesman_path(
            &map,
            &pois.entrance.position,
            &keys.iter().map(|x| x.position).collect::<Vec<_>>(),
        );
        if let Some(distance) = distance_opt {
            match result {
                Some(res) => {
                    if distance < res {
                        result = distance_opt;
                    }
                }
                None => {
                    result = distance_opt;
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test_1() {
        let map = parse_map(
            "#########
#b.A.@.a#
#########",
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
}
