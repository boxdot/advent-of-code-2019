use day18::*;

use ndarray::Array2;

use std::io;
use std::process;

fn read_input(mut in_stream: impl io::BufRead) -> io::Result<Array2<Kind>> {
    let mut buffer = String::new();
    in_stream.read_to_string(&mut buffer)?;
    Ok(parse_map(&buffer))
}

fn main() {
    let input = read_input(io::stdin().lock());
    match input {
        Ok(map) => {
            println!(
                "Shortest path: {:?}",
                get_traveling_salesman(&map)
            );
        }
        Err(e) => {
            eprintln!("Error while parsing input: {}", e);
            process::exit(1);
        }
    }
}
