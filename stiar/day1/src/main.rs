use day1::*;

use std::process;
use std::io;

fn read_input(in_stream: impl io::BufRead) -> io::Result<Vec<u32>> {
    in_stream
        .lines()
        .filter_map(io::Result::ok)
        .map(|line| {
            line.trim()
                .parse()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        })
        .collect()
}

fn main() {
    let input = read_input(io::stdin().lock());
    match input {
        Ok(modules) => {
            println!("Fuel without recursion: {}", calculate_fuel_for_modules(&modules, false));
            println!("Fuel with recursion: {}", calculate_fuel_for_modules(&modules, true));
        },
        Err(e) => {
            eprintln!("Error while parsing input: {}", e);
            process::exit(1);
        }
    }
}
