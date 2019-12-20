use crate::day09::{execute, parse, Memory};
use itertools::iproduct;
use std::ops::Index;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;
type Coord = (usize, usize);

pub fn solve(input: &str) -> Result<usize> {
    let mem = parse(input);
    let field = run(mem)?;
    let part1 = alignment(&field);
    Ok(part1)
}

struct Field {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Index<Coord> for Field {
    type Output = u8;
    fn index(&self, (x, y): Coord) -> &Self::Output {
        &self.data[x + self.width * y]
    }
}

fn run(mut mem: Memory) -> Result<Field> {
    let mut field = Vec::new();
    let mut width = None;

    let mut ip = Some(0);
    while let Some(next_ip) = ip {
        ip = execute(
            &mut mem,
            next_ip,
            || 0,
            |value| {
                let c = value as u8;
                if c != b'\n' {
                    field.push(c);
                } else if width.is_none() {
                    width = Some(field.len());
                }
            },
        )?;
    }

    let width = width.unwrap_or_else(|| field.len());
    let height = field.len() / width;
    Ok(Field {
        data: field,
        width,
        height,
    })
}

fn alignment(field: &Field) -> usize {
    iproduct!(1..field.width - 1, 1..field.height - 1)
        .filter_map(|(x, y)| {
            if field[(x, y)] == b'#' {
                let neighbors = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
                let is_intersection = neighbors.iter().all(|&pos| field[pos] == b'#');
                if is_intersection {
                    return Some(x * y);
                }
            }
            None
        })
        .sum()
}
