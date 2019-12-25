use day25::*;

use std::io;
use std::fs;
use std::process;

fn read_input() -> io::Result<Vec<i64>> {
    let buffer = fs::read_to_string("resources/input.txt")?;
    buffer
        .trim()
        .split(',')
        .map(|x| {
            x.parse()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        })
        .collect()
}

fn main() {
    let input = read_input();
    match input {
        Ok(commands) => {
            try_pass(Program::new(commands));
        }
        Err(e) => {
            eprintln!("Error while parsing input: {}", e);
            process::exit(1);
        }
    }
}
