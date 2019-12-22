use crate::day09::{execute, parse, Memory};
use itertools::iproduct;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;
type Coord = (usize, usize);

pub fn solve(input: &str) -> Result<(usize, usize)> {
    let mem = parse(input);
    let shape: Result<Vec<bool>> = iproduct!(0..50, 0..50)
        .map(|pos| scan(mem.clone(), pos))
        .collect();
    let shape = shape?;
    println!("{}", to_string(&shape, 50));

    let part1 = shape.iter().filter(|&&x| x).count();

    let (x, y) = find_fitting_shape(&mem);
    let part2 = x * 10000 + y;

    Ok((part1, part2))
}

// false - not pulled (i.e. stationary), true - pulled
fn scan(mut mem: Memory, pos: Coord) -> Result<bool> {
    let mut index = 0;
    let input = [pos.0 as i64, pos.1 as i64];

    let mut retval = false;
    let mut ip = Some(0);
    while let Some(next_ip) = ip {
        ip = execute(
            &mut mem,
            next_ip,
            || {
                let value = input[index];
                index += 1;
                value
            },
            |value| retval = value == 1,
        )?;
    }

    Ok(retval)
}

fn find_fitting_shape(mem: &Memory) -> (usize, usize) {
    let mut x = 0;
    for y in 100.. {
        x += (x..)
            .position(|x| scan(mem.clone(), (x, y)).unwrap())
            .unwrap();
        let corner = (x + 100 - 1, y - 100 + 1);
        if scan(mem.clone(), corner).unwrap() {
            return (x, y - 100 + 1);
        }
    }
    unreachable!();
}

fn to_string(shape: &[bool], width: usize) -> String {
    use itertools::Itertools;
    shape
        .chunks(width)
        .map(|l| {
            l.iter()
                .map(|&pulled| if pulled { '#' } else { '.' })
                .collect::<String>()
        })
        .intersperse("\n".into())
        .collect()
}
