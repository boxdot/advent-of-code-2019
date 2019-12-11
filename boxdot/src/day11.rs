use crate::day09::{execute, parse, Memory};
use std::collections::HashMap;
use std::isize::{MAX, MIN};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

pub fn solve(input: &str) -> Result<(usize, String)> {
    let mem = parse(input);
    let field = paint(mem.clone(), 0)?;
    let part1 = field.len();

    let field = paint(mem, 1)?;
    let (l, r, t, b) = field
        .iter()
        .fold((MAX, MIN, MIN, MAX), |(l, r, t, b), (&(x, y), _)| {
            (l.min(x), r.max(x), t.max(y), b.min(y))
        });
    let message: String = (b..=t)
        .rev()
        .map(|y| {
            (l..=r)
                .map(|x| {
                    if field.get(&(x, y)) == Some(&1) {
                        'X'
                    } else {
                        ' '
                    }
                })
                .collect()
        })
        .collect::<Vec<String>>()
        .join("\n");
    println!("{}", message);

    Ok((part1, message))
}

type Coord = (isize, isize);

fn paint(mut mem: Memory, init_color: i64) -> Result<HashMap<Coord, i64>> {
    let mut field: HashMap<Coord, _> = HashMap::new();
    let mut pos = (0, 0);
    field.insert(pos, init_color);
    let mut direction = (0, 1);

    let mut ip = Some(0);
    let mut output = Vec::new();
    while let Some(next_ip) = ip {
        ip = execute(
            &mut mem,
            next_ip,
            || field.get(&pos).cloned().unwrap_or(0),
            |value| output.push(value),
        )?;
        if output.len() == 2 {
            let color = output[0];
            field.insert(pos, color);
            direction = match output[1] {
                0 => (-direction.1, direction.0), // 90 deg left turn matrix
                1 => (direction.1, -direction.0), // 90 deg right turn matrix
                otherwise => panic!("unexpected turn direction: {}", otherwise),
            };
            pos = (pos.0 + direction.0, pos.1 + direction.1);
            output.clear();
        }
    }

    Ok(field)
}
