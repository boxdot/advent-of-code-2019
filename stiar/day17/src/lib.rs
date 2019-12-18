mod intcode;

use intcode::*;

#[macro_use]
extern crate itertools;
use derive_more::{Add, Neg, Sub};
use ndarray::{Array1, Array2};

use std::collections::HashSet;
use std::fmt::Write;

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
pub enum Turn {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
pub struct Command {
    pub turn: Turn,
    pub amount: u32,
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
    map: &Array2<char>,
    used: &mut HashSet<(Position, Position)>,
    path: &mut Vec<Position>,
) {
    for dir in [(1, 0), (0, 1), (-1, 0), (0, -1)]
        .into_iter()
        .map(|&(di, dj)| Position::new(di, dj))
    {
        let u = v + dir;
        if is_valid(u, map)
            && map[(u.i as usize, u.j as usize)] == '#'
            && !used.contains(&(v, dir))
            && !used.contains(&(u, -dir))
        {
            used.insert((v, dir));
            used.insert((u, -dir));
            dfs(u, map, used, path);
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

pub fn get_euler_path(map: &Array2<char>) -> (Vec<Position>, Vec<Command>) {
    let start_pos = iproduct!(0..map.nrows(), 0..map.ncols())
        .find(|&pos| map[pos] != '.' && map[pos] != '#')
        .unwrap();

    let mut current_pos = Position::new(start_pos.0 as i64, start_pos.1 as i64);
    let mut used = HashSet::new();
    let mut path = vec![];
    dfs(current_pos, &map, &mut used, &mut path);
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
        //let mut map_copy = map.clone();
        //map_copy[(position.i as usize, position.j as usize)] = '*';
        //for row in map_copy.genrows().into_iter() {
        //    println!(
        //        "{}",
        //        row.iter()
        //            .map(|x| x.to_string())
        //            .collect::<Vec<String>>()
        //            .join("")
        //    );
        //}
        //println!("");

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
    commands.push(current_command);

    (path, commands)
}
