use derive_more::AddAssign;
use itertools::Itertools;
use num;

use std::collections::HashMap;
use std::convert::From;
use std::fmt;
use std::{thread, time};

mod intcode;

use intcode::*;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, AddAssign)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TileType {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

impl From<i64> for TileType {
    fn from(value: i64) -> Self {
        match value {
            0 => TileType::Empty,
            1 => TileType::Wall,
            2 => TileType::Block,
            3 => TileType::HorizontalPaddle,
            4 => TileType::Ball,
            _ => panic!("Unknown tile type"),
        }
    }
}

impl fmt::Display for TileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TileType::Empty => " ",
                TileType::Wall => "▩",
                TileType::Block => "x",
                TileType::HorizontalPaddle => "_",
                TileType::Ball => "o",
            }
        )
    }
}

pub fn get_tiles_count(program_vec: Vec<i64>) -> HashMap<TileType, u64> {
    let mut program = Program::new(program_vec);

    let mut result = HashMap::new();

    for (_, _, tile_type) in program.run(std::iter::empty()).tuples() {
        *result.entry(tile_type.into()).or_insert(0) += 1;
    }

    result
}

pub fn print_board(board: &Vec<Vec<TileType>>, score: i64) {
    print!("{}[2J", 27 as char);
    println!("Score: {}", score);
    for row in board.iter() {
        println!(
            "{}",
            row.iter()
                .map(|tile| tile.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
    thread::sleep(time::Duration::from_millis(100));
}

pub fn run_game(mut program_vec: Vec<i64>, print: bool) -> i64 {
    let mut board = vec![vec![TileType::Empty; 45]; 26];

    program_vec[0] = 2;
    let mut program = Program::new(program_vec);
    let mut ball_x = 0;
    let mut paddle_x = 0;
    let mut score = 0;

    loop {
        let mut game_iter = program.run(std::iter::once(num::signum(ball_x - paddle_x))).peekable();
        if game_iter.peek().is_none() {
            return score;
        }
        for (x, y, tile_type) in game_iter.tuples() {
            if x == -1 && y == 0 {
                score = tile_type;
            } else {
                board[y as usize][x as usize] = tile_type.into();
                if tile_type == 3 {
                    paddle_x = x;
                }
                if tile_type == 4 {
                    ball_x = x;
                }
                if tile_type >= 3 && print {
                    print_board(&board, score);
                }
            }
        }
    }
}
