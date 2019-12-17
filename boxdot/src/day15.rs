use crate::day09::{execute, parse, Memory};
use rand::{thread_rng, Rng};

use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;
type Coord = (i64, i64);

pub fn solve(input: &str) -> Result<usize> {
    let mem = parse(input);

    let tiles = collect_tiles(mem)?;
    print_tiles(&tiles, (0, 0));

    let oxygen_system_pos = tiles
        .iter()
        .find_map(|(&pos, &(id, _))| if id == 2 { Some(pos) } else { None })
        .ok_or_else(|| "no oxygen system found")?;
    let part1 = shortest_path((0, 0), oxygen_system_pos, tiles)
        .ok_or_else(|| "no path to oxygen system found")?;

    Ok(part1)
}

fn walk(pos: Coord, cmd: i64) -> Coord {
    match cmd {
        1 => (pos.0, pos.1 + 1),
        2 => (pos.0, pos.1 - 1),
        3 => (pos.0 - 1, pos.1),
        4 => (pos.0 + 1, pos.1),
        _ => panic!("unexpected move command"),
    }
}

fn neighbors(pos: Coord) -> impl Iterator<Item = Coord> {
    let mut cmd = 0;
    std::iter::from_fn(move || {
        if cmd < 4 {
            cmd += 1;
            Some(walk(pos, cmd))
        } else {
            None
        }
    })
}

type Tiles = HashMap<Coord, (i64, bool)>;

#[derive(Debug, Default)]
struct State {
    pos: Coord,
    move_cmd: i64,
    tiles: Tiles, // mapping tile pos -> (id, visited)
}

// We are lazy so we just do a random walk, since we know that it
// will converge to the solution eventually.
fn collect_tiles(mut mem: Memory) -> Result<Tiles> {
    let mut tiles: HashMap<Coord, _> = HashMap::new();
    tiles.insert((0, 0), (1, false));
    for pos in neighbors((0, 0)) {
        tiles.insert(pos, (1, false));
    }
    let state = RefCell::new(State {
        pos: (0, 0),
        move_cmd: 1,
        tiles,
    });

    let mut rng = thread_rng();
    let mut cmds = Vec::with_capacity(4);
    let mut guess_input = |state: &mut State| {
        cmds.clear();
        for (move_cmd, pos) in neighbors(state.pos)
            .enumerate()
            .map(|(i, pos)| ((i + 1) as i64, pos))
        {
            let &(id, visited) = state.tiles.get(&pos).unwrap_or(&(1, false));
            if id != 0 {
                if !visited {
                    state.move_cmd = move_cmd;
                    return move_cmd;
                } else {
                    cmds.push(move_cmd);
                }
            }
        }
        state.move_cmd = cmds[rng.gen_range(0, cmds.len())];
        state.move_cmd
    };

    let process_output = |value, state: &mut State| {
        let (_, visited) = state.tiles.get_mut(&state.pos).unwrap();
        *visited = true;

        match value {
            0 => {
                let wall_pos = walk(state.pos, state.move_cmd);
                state.tiles.insert(wall_pos, (0, true));
            }
            1 | 2 => {
                state.pos = walk(state.pos, state.move_cmd);
                let (id, _) = state.tiles.get_mut(&state.pos).unwrap();
                *id = value;
                for pos in neighbors(state.pos) {
                    state.tiles.entry(pos).or_insert((value, false));
                }
            }
            _ => panic!("unexpected output"),
        };
    };

    let mut ip = Some(0);
    while let Some(next_ip) = ip {
        ip = execute(
            &mut mem,
            next_ip,
            || guess_input(&mut state.borrow_mut()),
            |value| process_output(value, &mut state.borrow_mut()),
        )?;
        let all_visited = state
            .borrow()
            .tiles
            .values()
            .all(|(id, visited)| *id == 0 || *visited);
        if all_visited {
            break;
        }
    }

    let tiles = state.into_inner().tiles;
    Ok(tiles)
}

fn print_tiles(tiles: &Tiles, pos: Coord) {
    use std::i64::{MAX, MIN};
    let (min, max) = tiles
        .keys()
        .fold(((MAX, MAX), (MIN, MIN)), |(min, max), pos| {
            let min = (min.0.min(pos.0), min.1.min(pos.1));
            let max = (max.0.max(pos.0), max.1.max(pos.1));
            (min, max)
        });
    let mut s = String::new();
    for y in min.1..=max.1 {
        for x in min.0..=max.0 {
            if (x, y) == pos {
                s.push('o');
            } else {
                let c = match tiles.get(&(x, y)).map(|v| v.0) {
                    Some(0) => 'x',
                    Some(2) => '.',
                    Some(_) | None => ' ',
                };
                s.push(c);
            }
        }
        s.push('\n');
    }
    println!("{}", s);
}

fn shortest_path(orig: Coord, dest: Coord, mut tiles: Tiles) -> Option<usize> {
    // reset visited tiles
    for (_, visited) in tiles.values_mut() {
        *visited = false;
    }

    let mut q = VecDeque::new();
    q.push_back((orig, 0));
    tiles.get_mut(&orig)?.1 = true;

    while let Some((pos, dist)) = q.pop_front() {
        if pos == dest {
            return Some(dist);
        }
        for neighbor in neighbors(pos) {
            let (id, seen) = tiles.get_mut(&neighbor)?;
            if *id != 0 && !*seen {
                *seen = true;
                q.push_back((neighbor, dist + 1));
            }
        }
    }

    None
}
