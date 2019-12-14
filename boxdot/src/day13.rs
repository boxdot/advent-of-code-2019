use crate::day09::{execute, parse, Memory};
use std::collections::HashMap;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

pub fn solve(input: &str) -> Result<usize> {
    let mem = parse(input);
    let tiles = collect_tiles(mem.clone())?;
    print_tiles(&tiles);

    let num_block_tiles = tiles.values().filter(|&&id| id == 2).count();

    play(mem)?;

    Ok(num_block_tiles)
}

type Coord = (usize, usize);

fn collect_tiles(mut mem: Memory) -> Result<HashMap<Coord, i64>> {
    let mut tiles: HashMap<Coord, _> = HashMap::new();

    let mut ip = Some(0);
    let mut output = Vec::new();
    while let Some(next_ip) = ip {
        ip = execute(&mut mem, next_ip, || 0, |value| output.push(value))?;
        if output.len() == 3 {
            let pos = (output[0] as usize, output[1] as usize);
            let tile_id = output[2];
            tiles.insert(pos, tile_id);
            output.clear();
        }
    }

    Ok(tiles)
}

fn play(mut mem: Memory) -> Result<()> {
    mem.write(0, 2);

    let mut ip = Some(0);
    let mut output = Vec::new();
    while let Some(next_ip) = ip {
        ip = execute(&mut mem, next_ip, || 1, |value| output.push(value))?;
    }

    Ok(())
}

fn print_tiles(tiles: &HashMap<Coord, i64>) {
    use std::usize::{MAX, MIN};
    let (min, max) = tiles
        .keys()
        .fold(((MAX, MAX), (MIN, MIN)), |(min, max), pos| {
            let min = (min.0.min(pos.0), min.1.min(pos.1));
            let max = (max.0.max(pos.0), max.1.max(pos.1));
            (min, max)
        });
    let mut s = String::new();
    for y in min.1..max.1 {
        for x in min.0..max.0 {
            let c = match tiles.get(&(x, y)) {
                Some(1) => 'x',
                Some(2) => 'o',
                Some(3) => '=',
                Some(4) => '.',
                Some(_) | None => ' ',
            };
            s.push(c);
        }
        s.push('\n');
    }
    println!("{}", s);
}
