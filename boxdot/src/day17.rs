use crate::day09::{execute, parse, Memory};
use itertools::iproduct;
use std::ops::Index;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;
type Coord = (isize, isize);

pub fn solve(input: &str) -> Result<(isize, i64)> {
    let mem = parse(input);
    let field = run(mem.clone())?;
    let part1 = alignment(&field);

    let ops = find_sequence(&field)?;
    println!("{}", ops.join(","));

    // manually partitioned the graph
    let part2 = run_robot(
        mem,
        "A,B,A,C,A,B,C,C,A,B",
        "R,8,L,10,R,8",
        "R,12,R,8,L,8,L,12",
        "L,12,L,10,L,8",
    )?;

    Ok((part1, part2))
}

struct Field {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Index<Coord> for Field {
    type Output = u8;
    fn index(&self, (x, y): Coord) -> &Self::Output {
        let idx = (x + self.width as isize * y) as usize;
        &self.data[idx]
    }
}

impl Field {
    fn is_scaffold(&self, (x, y): Coord) -> bool {
        if 0 <= x && x < self.width as isize && 0 <= y && y < self.height as isize {
            self[(x, y)] == b'#'
        } else {
            false
        }
    }

    fn iter(&self) -> impl Iterator<Item = (Coord, u8)> + '_ {
        iproduct!(0..self.width as isize, 0..self.height as isize).map(move |pos| (pos, self[pos]))
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
                print!("{}", c as char);
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

fn alignment(field: &Field) -> isize {
    field
        .iter()
        .filter_map(|((x, y), _)| {
            let is_intersection = [(x, y), (x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
                .iter()
                .all(|&pos| field.is_scaffold(pos));
            if is_intersection {
                Some(x * y)
            } else {
                None
            }
        })
        .sum()
}

fn find_sequence(field: &Field) -> Result<Vec<String>> {
    let mut ops = Vec::new();

    let (mut robot_pos, mut robot_state) = field
        .iter()
        .find(|(_pos, value)| match value {
            b'^' | b'v' | b'<' | b'>' => true,
            _ => false,
        })
        .ok_or_else(|| "no robot found")?;

    loop {
        // move
        let mut pos = robot_pos;
        while let Some(coord) = walk(field, pos, robot_state) {
            pos = coord;
        }
        let dist = (robot_pos.0 - pos.0).abs() + (robot_pos.1 - pos.1).abs();
        if dist != 0 {
            ops.push(dist.to_string());
            robot_pos = pos;
        }

        // turn
        let (x, y) = robot_pos;
        let (op, new_state) = match robot_state {
            b'^' if field.is_scaffold((x + 1, y)) => ("R", b'>'),
            b'^' if field.is_scaffold((x - 1, y)) => ("L", b'<'),

            b'v' if field.is_scaffold((x + 1, y)) => ("L", b'>'),
            b'v' if field.is_scaffold((x - 1, y)) => ("R", b'<'),

            b'<' if field.is_scaffold((x, y - 1)) => ("R", b'^'),
            b'<' if field.is_scaffold((x, y + 1)) => ("L", b'v'),

            b'>' if field.is_scaffold((x, y - 1)) => ("L", b'^'),
            b'>' if field.is_scaffold((x, y + 1)) => ("R", b'v'),

            _ => break,
        };
        ops.push(op.to_string());
        robot_state = new_state;
    }

    Ok(ops)
}

fn walk(field: &Field, (x, y): Coord, state: u8) -> Option<Coord> {
    let coord = match state {
        b'^' => (x, y - 1),
        b'v' => (x, y + 1),
        b'<' => (x - 1, y),
        b'>' => (x + 1, y),
        _ => unreachable!(),
    };
    if field.is_scaffold(coord) {
        Some(coord)
    } else {
        None
    }
}

fn run_robot(mut mem: Memory, main: &str, a: &str, b: &str, c: &str) -> Result<i64> {
    mem.write(0, 2);

    let input = format!("{}\n{}\n{}\n{}\nn\n", main, a, b, c).into_bytes();
    let mut input_pos = 0;

    let mut out = 0;

    let mut ip = Some(0);
    while let Some(next_ip) = ip {
        ip = execute(
            &mut mem,
            next_ip,
            || {
                let value = input[input_pos];
                input_pos += 1;
                value as i64
            },
            |value| {
                out = value;
            },
        )?;
    }

    Ok(out)
}
