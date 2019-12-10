use day10::*;

use std::collections::HashSet;
use std::io;
use std::process;

fn read_input(mut in_stream: impl io::BufRead) -> io::Result<HashSet<Cell>> {
    let mut buffer = String::new();

    // Input numbers.
    in_stream.read_to_string(&mut buffer)?;
    Ok(parse_from_map(&buffer))
}

fn main() {
    let input = read_input(io::stdin().lock());
    match input {
        Ok(asteroids) => {
            println!(
                "Best monitoring station: {}",
                find_best_monitoring_station(&asteroids)
            );
        }
        Err(e) => {
            eprintln!("Error while parsing input: {}", e);
            process::exit(1);
        }
    }
}
