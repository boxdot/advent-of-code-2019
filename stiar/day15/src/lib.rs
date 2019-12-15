use derive_more::Add;

use std::collections::{HashSet, VecDeque};

mod intcode;

use intcode::*;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Add)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

impl Position {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone)]
pub struct Direction {
    pub vector: Position,
    pub code: i64,
}

impl Direction {
    pub fn new(x: i64, y: i64, code: i64) -> Self {
        Self {
            vector: Position::new(x, y),
            code,
        }
    }

    pub fn flip(&self) -> Direction {
        Self::new(
            -self.vector.x,
            -self.vector.y,
            if self.code % 2 == 1 {
                self.code + 1
            } else {
                self.code - 1
            },
        )
    }
}

#[derive(Debug, Clone)]
struct SearchVertex {
    pub position: Position,
    pub path: Vec<Direction>,
}

impl SearchVertex {
    pub fn new(position: Position, path: Vec<Direction>) -> Self {
        Self { position, path }
    }
}

fn move_droid(program: &mut Program, direction: &Direction) -> i64 {
    program.run(std::iter::once(direction.code)).next().unwrap()
}

#[derive(Debug, PartialEq, Eq)]
enum StopCondition {
    FoundOxygen,
    FilledWithOxygen,
}

fn run_bfs(
    map_program: Vec<i64>,
    init_sequence: &[Direction],
    stop_condition: StopCondition,
) -> (u64, Vec<Direction>) {
    let mut program = Program::new(map_program);

    for direction in init_sequence {
        let outcome = move_droid(&mut program, &direction);
        assert!(outcome >= 1);
    }

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    let start_position = Position::new(0, 0);
    queue.push_back(SearchVertex::new(start_position, vec![]));
    visited.insert(start_position);

    let directions = [
        Direction::new(0, 1, 1),
        Direction::new(0, -1, 2),
        Direction::new(-1, 0, 3),
        Direction::new(1, 0, 4),
    ];

    let mut max_distance = 0;

    while !queue.is_empty() {
        let front = queue.pop_front().unwrap();
        max_distance = front.path.len();

        for direction in &front.path {
            let outcome = move_droid(&mut program, &direction);
            assert!(outcome >= 1);
        }

        for direction in &directions {
            let new_position = front.position + direction.vector;
            if visited.contains(&new_position) {
                continue;
            }

            visited.insert(new_position);

            let mut new_path = front.path.clone();
            new_path.push(direction.clone());

            match move_droid(&mut program, &direction) {
                0 => (),
                1 => {
                    queue.push_back(SearchVertex::new(new_position, new_path));

                    let outcome = move_droid(&mut program, &direction.flip());
                    assert!(outcome >= 1);
                }
                2 if stop_condition == StopCondition::FoundOxygen => {
                    return ((front.path.len() + 1) as u64, new_path)
                }
                _ => panic!("Unrecognized output"),
            };
        }

        for direction in front.path.iter().rev() {
            let outcome = move_droid(&mut program, &direction.flip());
            assert!(outcome >= 1);
        }
    }

    (max_distance as u64, vec![])
}

pub fn get_distance_to_oxygen_system(map_program: Vec<i64>) -> (u64, Vec<Direction>) {
    run_bfs(map_program, &[], StopCondition::FoundOxygen)
}

pub fn fill_with_oxygen(map_program: Vec<i64>, init_sequence: &[Direction]) -> u64 {
    run_bfs(map_program, init_sequence, StopCondition::FilledWithOxygen).0
}
