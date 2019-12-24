#[macro_use]
extern crate itertools;
use derive_more::Add;
use ndarray::{Array1, Array2};

use std::collections::{BTreeMap, HashSet};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Bug,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Bug,
            _ => panic!("Unknown cell"),
        }
    }
}

pub fn parse_map(input: &str) -> Array2<Cell> {
    let vec: Vec<Vec<Cell>> = input
        .trim()
        .split('\n')
        .map(|x| x.chars().map(|c| c.into()).collect())
        .collect();

    vec.into_iter()
        .flatten()
        .collect::<Array1<_>>()
        .into_shape((5, 5))
        .unwrap()
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Add)]
pub struct Position {
    pub i: i32,
    pub j: i32,
    pub level: i32,
}

impl Position {
    pub fn new(i: i32, j: i32, level: i32) -> Self {
        Self { i, j, level }
    }
}

pub fn get_code(map: &Array2<Cell>) -> u32 {
    let mut result = 0;
    for (index, &x) in map.iter().enumerate() {
        if x == Cell::Bug {
            result += 1 << index;
        }
    }
    result
}

fn is_valid(v: &Position, map: &Array2<Cell>) -> bool {
    v.i >= 0 && (v.i as usize) < map.nrows() && v.j >= 0 && (v.j as usize) < map.ncols()
}

fn get_adjacent_count(map: &Array2<Cell>, position: Position, kind: Cell) -> u32 {
    [(0, 1), (1, 0), (0, -1), (-1, 0)]
        .into_iter()
        .map(|&(di, dj)| position + Position::new(di, dj, 0))
        .filter(|x| is_valid(x, map) && map[(x.i as usize, x.j as usize)] == kind)
        .count() as u32
}

pub fn iterate(map: &Array2<Cell>) -> Array2<Cell> {
    iproduct!(0..5, 0..5)
        .map(|(i, j)| match map[(i, j)] {
            Cell::Bug => {
                if get_adjacent_count(map, Position::new(i as i32, j as i32, 0), Cell::Bug) == 1 {
                    Cell::Bug
                } else {
                    Cell::Empty
                }
            }
            Cell::Empty => {
                let bugs = get_adjacent_count(map, Position::new(i as i32, j as i32, 0), Cell::Bug);
                if bugs == 1 || bugs == 2 {
                    Cell::Bug
                } else {
                    Cell::Empty
                }
            }
        })
        .collect::<Array1<_>>()
        .into_shape((5, 5))
        .unwrap()
}

pub fn first_equal(mut map: Array2<Cell>) -> Array2<Cell> {
    let mut seen = HashSet::new();

    while !seen.contains(&get_code(&map)) {
        seen.insert(get_code(&map));
        map = iterate(&map);
    }

    map
}

fn get(multimap: &BTreeMap<i32, Array2<Cell>>, position: Position) -> Cell {
    if let Some(map) = multimap.get(&position.level) {
        map[(position.i as usize, position.j as usize)]
    } else {
        Cell::Empty
    }
}

fn get_multiadjacent_count(
    multimap: &BTreeMap<i32, Array2<Cell>>,
    position: Position,
    kind: Cell,
) -> u32 {
    [
        (0, 1, position.level),
        (1, 0, position.level),
        (0, -1, position.level),
        (-1, 0, position.level),
    ]
    .into_iter()
    .map(|&(di, dj, level)| (position.i + di, position.j + dj, level))
    .map(|x| match x {
        (-1, _, level) => vec![get(multimap, Position::new(1, 2, level - 1))],
        (5, _, level) => vec![get(multimap, Position::new(3, 2, level - 1))],
        (_, -1, level) => vec![get(multimap, Position::new(2, 1, level - 1))],
        (_, 5, level) => vec![get(multimap, Position::new(2, 3, level - 1))],
        (2, 2, level) => match (position.i, position.j) {
            (1, 2) => (0..5)
                .map(|j| get(multimap, Position::new(0, j, level + 1)))
                .collect(),
            (3, 2) => (0..5)
                .map(|j| get(multimap, Position::new(4, j, level + 1)))
                .collect(),
            (2, 1) => (0..5)
                .map(|i| get(multimap, Position::new(i, 0, level + 1)))
                .collect(),
            (2, 3) => (0..5)
                .map(|i| get(multimap, Position::new(i, 4, level + 1)))
                .collect(),
            _ => panic!("Logic error"),
        },
        (i, j, level) => vec![get(multimap, Position::new(i, j, level))],
    })
    .flatten()
    .filter(|&x| x == kind)
    .count() as u32
}

fn iterate_once_multimap(multimap: &BTreeMap<i32, Array2<Cell>>) -> BTreeMap<i32, Array2<Cell>> {
    let min = multimap.iter().next().unwrap().0;
    let max = multimap.iter().next_back().unwrap().0;
    (min - 1..=max + 1)
        .map(|level| {
            (
                level,
                iproduct!(0..5, 0..5)
                    .map(|(i, j)| {
                        if (i, j) == (2, 2) {
                            Cell::Empty
                        } else {
                            match get(multimap, Position::new(i, j, level)) {
                                Cell::Bug => {
                                    if get_multiadjacent_count(
                                        multimap,
                                        Position::new(i, j, level),
                                        Cell::Bug,
                                    ) == 1
                                    {
                                        Cell::Bug
                                    } else {
                                        Cell::Empty
                                    }
                                }
                                Cell::Empty => {
                                    let bugs = get_multiadjacent_count(
                                        multimap,
                                        Position::new(i, j, level),
                                        Cell::Bug,
                                    );
                                    if bugs == 1 || bugs == 2 {
                                        Cell::Bug
                                    } else {
                                        Cell::Empty
                                    }
                                }
                            }
                        }
                    })
                    .collect::<Array1<_>>()
                    .into_shape((5, 5))
                    .unwrap(),
            )
        })
        .collect()
}

pub fn iterate_multimap(
    mut multimap: BTreeMap<i32, Array2<Cell>>,
    iterations: u32,
) -> BTreeMap<i32, Array2<Cell>> {
    for _ in 0..iterations {
        multimap = iterate_once_multimap(&multimap);
    }
    multimap
}

pub fn count_bugs(multimap: &BTreeMap<i32, Array2<Cell>>) -> u32 {
    multimap
        .values()
        .map(|map| map.iter().filter(|&&x| x == Cell::Bug).count())
        .sum::<usize>() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test_1() {
        let map = parse_map(
            "....#
#..#.
#..##
..#..
#....",
        );
        assert_eq!(get_code(&first_equal(map)), 2129920);
    }

    #[test]
    fn sample_test_2() {
        let map = parse_map(
            "....#
#..#.
#..##
..#..
#....",
        );
        let mut multimap = BTreeMap::new();
        multimap.insert(0, map);
        let after_10 = iterate_multimap(multimap, 10);
        assert_eq!(count_bugs(&after_10), 99);
    }
}
