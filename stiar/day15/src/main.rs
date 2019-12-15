use day15::*;

use std::io;
use std::process;

fn read_input(mut in_stream: impl io::BufRead) -> io::Result<Vec<i64>> {
    let mut buffer = String::new();

    // Input numbers.
    in_stream.read_line(&mut buffer)?;
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
    let input = read_input(io::stdin().lock());
    match input {
        Ok(program) => {
            let (distance_to_oxygen, path_to_oxygen) =
                get_distance_to_oxygen_system(program.clone());
            println!("Distance to oxygen system: {}", distance_to_oxygen);
            println!("Time to fill the oxygen: {}", fill_with_oxygen(program, &path_to_oxygen));
        }
        Err(e) => {
            eprintln!("Error while parsing input: {}", e);
            process::exit(1);
        }
    }
}
