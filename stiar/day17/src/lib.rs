mod intcode;

use intcode::*;

#[macro_use]
extern crate itertools;
use derive_more::{Add, Neg, Sub};
use itertools::EitherOrBoth::Both;
use itertools::Itertools;
use ndarray::{Array1, Array2};
use rand::seq::SliceRandom;

use std::collections::HashSet;
use std::fmt::{self, Write};

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

#[derive(Debug, Copy, Clone)]
enum Turn {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
struct Command {
    pub turn: Turn,
    pub amount: u32,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{},{}",
            match self.turn {
                Turn::Left => "L",
                Turn::Right => "R",
            },
            self.amount
        )
    }
}

pub fn extract_map(program: Vec<i64>, print: bool) -> Array2<char> {
    let mut s = String::new();
    for output in Program::new(program).run(std::iter::empty()) {
        write!(&mut s, "{}", output as u8 as char).unwrap();
    }
    if print {
        println!("{}", s);
    }

    let vec = s
        .trim()
        .split('\n')
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let width = vec.len();
    let flattened: Array1<_> = vec.into_iter().flatten().collect();
    let height = flattened.len() / width;
    flattened.into_shape((width, height)).unwrap()
}

pub fn calculate_alignment(map: &Array2<char>) -> usize {
    iproduct!(1..map.nrows() - 1, 1..map.ncols() - 1)
        .filter(|(i, j)| {
            [(0, 0), (1, 0), (0, 1), (-1, 0), (0, -1)]
                .into_iter()
                .map(|(di, dj)| map[((*i as i64 + di) as usize, (*j as i64 + dj) as usize)] == '#')
                .all(|x| x)
        })
        .map(|(i, j)| i * j)
        .sum()
}

fn is_valid(v: Position, map: &Array2<char>) -> bool {
    v.i >= 0 && (v.i as usize) < map.nrows() && v.j >= 0 && (v.j as usize) < map.ncols()
}

fn dfs(
    v: Position,
    dir: Position,
    map: &Array2<char>,
    used: &mut HashSet<(Position, Position)>,
    path: &mut Vec<Position>,
    mut rng: &mut impl rand::RngCore,
) {
    let mut dirs = [(0, -1), (-1, 0), (0, 1), (1, 0)];
    dirs.shuffle(&mut rng);
    for dir in std::iter::once(dir).chain(dirs.into_iter().map(|&(di, dj)| Position::new(di, dj))) {
        let u = v + dir;
        if is_valid(u, map)
            && map[(u.i as usize, u.j as usize)] == '#'
            && !used.contains(&(v, dir))
            && !used.contains(&(u, -dir))
        {
            used.insert((v, dir));
            used.insert((u, -dir));
            dfs(u, dir, map, used, path, rng);
            path.push(v);
        }
    }
}

fn get_turn(old_dir: Position, new_dir: Position) -> Turn {
    if new_dir == Position::new(-old_dir.j, old_dir.i) {
        Turn::Left
    } else {
        assert!(old_dir == Position::new(-new_dir.j, new_dir.i));
        Turn::Right
    }
}

pub fn get_euler_path(
    map: &Array2<char>,
    mut rng: &mut impl rand::RngCore,
) -> (Vec<Position>, Vec<String>) {
    let start_pos = iproduct!(0..map.nrows(), 0..map.ncols())
        .find(|&pos| map[pos] != '.' && map[pos] != '#')
        .unwrap();

    let mut current_pos = Position::new(start_pos.0 as i64, start_pos.1 as i64);
    let mut used = HashSet::new();
    let mut path = vec![];
    dfs(
        current_pos,
        Position::new(0, 1),
        &map,
        &mut used,
        &mut path,
        &mut rng,
    );
    path.reverse();

    let mut current_dir = match map[start_pos] {
        '^' => Position::new(-1, 0),
        '>' => Position::new(0, 1),
        'v' => Position::new(1, 0),
        '<' => Position::new(0, -1),
        _ => panic!("Invalid robot"),
    };
    let mut current_command = Command {
        turn: Turn::Left,
        amount: 0,
    };
    let mut commands = vec![];
    for &position in path.iter().skip(1) {
        let new_dir = position - current_pos;
        if new_dir != current_dir {
            if current_command.amount > 0 {
                commands.push(current_command);
            }
            current_command = Command {
                turn: get_turn(current_dir, new_dir),
                amount: 0,
            };
            current_dir = new_dir;
        }
        current_command.amount += 1;
        current_pos = current_pos + current_dir;
    }
    current_command.amount += 1;
    commands.push(current_command);

    (
        path,
        commands.into_iter().map(|cmd| cmd.to_string()).collect(),
    )
}

pub fn pack_strings(strings: &[String]) -> Option<(String, String, String, String)> {
    for (l1, r1) in iproduct!(0..=strings.len(), 0..=strings.len()) {
        for (l2, r2) in iproduct!(0..=strings.len(), 0..=strings.len()) {
            for (l3, r3) in iproduct!(0..=strings.len(), 0..=strings.len()) {
                if l1 < r1 && r1 <= l2 && l2 < r2 && r2 <= l3 && l3 < r3 {
                    let patterns = [
                        strings[l1..r1].to_vec(),
                        strings[l2..r2].to_vec(),
                        strings[l3..r3].to_vec(),
                    ];
                    if !patterns.iter().all(|x| x.join(",").len() <= 20) {
                        continue;
                    }
                    let mut result = vec![];
                    let mut index = 0;
                    while index < strings.len() {
                        let mut advanced = false;
                        for (number, pattern) in patterns.iter().enumerate() {
                            if strings[index..]
                                .iter()
                                .zip_longest(pattern.iter())
                                .filter_map(|pair| match pair {
                                    Both(x, y) => Some((x, y)),
                                    _ => None,
                                })
                                .all(|(x, y)| x == y)
                                && index + pattern.len() <= strings.len()
                            {
                                result.push(match number {
                                    0 => "A",
                                    1 => "B",
                                    2 => "C",
                                    _ => panic!("WAT"),
                                });
                                index += pattern.len();
                                advanced = true;
                            }
                        }
                        if !advanced {
                            break;
                        }
                    }
                    if index == strings.len() && result.join(",").len() <= 20 {
                        return Some((
                            result.join(","),
                            patterns[0].join(","),
                            patterns[1].join(","),
                            patterns[2].join(","),
                        ));
                    }
                }
            }
        }
    }
    None
}
