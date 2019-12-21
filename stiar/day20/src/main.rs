use day20::*;

use ndarray::Array2;

use std::collections::HashMap;
use std::io;
use std::process;

fn read_input(
    mut in_stream: impl io::BufRead,
) -> io::Result<(Array2<Cell>, HashMap<Position, Position>)> {
    let mut buffer = String::new();
    in_stream.read_to_string(&mut buffer)?;
    println!("{}", buffer);
    Ok(parse_maze(&buffer))
}

fn main() {
    let input = read_input(io::stdin().lock());
    match input {
        Ok((maze, teleports)) => {
            println!("Shortest path: {:?}", calculate_shortest_path(&maze, &teleports, false));
            println!("Shortest path with levels: {:?}", calculate_shortest_path(&maze, &teleports, true));
        }
        Err(e) => {
            eprintln!("Error while parsing input: {}", e);
            process::exit(1);
        }
    }
}
