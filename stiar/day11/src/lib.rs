use derive_more::AddAssign;
use ndarray::prelude::*;

use std::collections::HashMap;

mod intcode;

use intcode::*;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, AddAssign)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

impl Position {
    pub fn turn_left(&mut self) {
        *self = Position {
            x: -self.y,
            y: self.x,
        };
    }

    pub fn turn_right(&mut self) {
        *self = Position {
            x: self.y,
            y: -self.x,
        };
    }
}

pub fn get_covered_panels(program_vec: Vec<i64>, initial_color: i64) -> HashMap<Position, i64> {
    let mut program = Program::new(program_vec);

    let mut position = Position { x: 0, y: 0 };
    let mut direction = Position { x: 0, y: 1 };

    let mut colored = HashMap::new();
    colored.insert(position, initial_color);

    loop {
        let mut it =
            program.run(vec![colored.get(&position).cloned().unwrap_or(0)].into_iter());
        match it.next() {
            Some(color) => {
                if color < 0 || color > 1 {
                    panic!("Unknown color");
                }
                colored.insert(position, color);
            }
            None => {
                break;
            }
        };
        let rotate = it.next().unwrap();
        match rotate {
            0 => direction.turn_left(),
            1 => direction.turn_right(),
            _ => panic!("Unknown rotation"),
        };
        position += direction;
    }

    colored
}

pub fn get_grid(panels: &HashMap<Position, i64>) -> Array2<i64> {
    let minx = panels.iter().map(|(pos, _)| pos.x).min().unwrap();
    let maxx = panels.iter().map(|(pos, _)| pos.x).max().unwrap();
    let miny = panels.iter().map(|(pos, _)| pos.y).min().unwrap();
    let maxy = panels.iter().map(|(pos, _)| pos.y).max().unwrap();

    let mut res = Array2::zeros(((maxy - miny + 1) as usize, (maxx - minx + 1) as usize));

    for (panel, &color) in panels.iter() {
        res[[(maxy - panel.y) as usize, (panel.x - minx) as usize]] = color;
    }

    res
}
