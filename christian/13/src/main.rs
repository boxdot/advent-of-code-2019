use itertools::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::*;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use vm::Vm;
mod program;

fn solve1() -> usize {
    let mut screen = HashMap::new();
    for (x, y, tile) in Vm::new(program::data(), 0..0).tuples() {
        screen.insert((x, y), tile);
    }

    screen.values().filter(|&&tile| tile == 2).count()
}

fn solve2() {
    let size = termion::terminal_size().unwrap();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let screen = RefCell::new(HashMap::new());
    let mut mem = program::data();
    mem[0] = 2;
    let score = RefCell::new(0);
    write!(
        stdout,
        "{}{}\n\r",
        termion::clear::All,
        termion::cursor::Hide
    )
    .unwrap();
    let input = || {
        print!("{}", termion::cursor::Goto(1, 1));
        print!("Score: {}\n\r", score.borrow());
        let screen = screen.borrow();
        for y in 1..size.1 - 10 {
            let row: String = (1..size.0 - 2)
                .map(|x| match screen.get(&(x as i64, y as i64)).unwrap_or(&0) {
                    1 => '█',
                    2 => '🝙',
                    3 => '▂',
                    4 => '●',
                    _ => ' ',
                })
                .collect();
            write!(stdout, "{}\n\r", row).unwrap();
        }
        let mut ball_pos = 0;
        let mut paddle_pos = 0;
        for ((x, _), tile) in screen.iter() {
            match tile {
                4 => ball_pos = *x,
                3 => paddle_pos = *x,
                _ => (),
            }
        }
        Some((ball_pos - paddle_pos).signum())
    };
    for (x, y, data) in Vm::new(mem.clone(), std::iter::from_fn(input)).tuples() {
        if (x, y) == (-1, 0) {
            *score.borrow_mut() = data;
        }
        screen.borrow_mut().insert((x, y), data);
    }

    write!(
        stdout,
        "Final score {}\n\r{}",
        score.into_inner(),
        termion::cursor::Show
    )
    .unwrap();
}

fn main() {
    println!("Part1:\n{}", solve1());
    stdin().events().next();
    solve2()
}
