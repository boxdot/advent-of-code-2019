use day14::*;

use std::io;
use std::process;

fn read_input(mut in_stream: impl io::BufRead) -> io::Result<ReactionsTable> {
    let mut buffer = String::new();
    in_stream.read_to_string(&mut buffer)?;
    Ok(parse_reactions(&buffer))
}

fn main() {
    let input = read_input(io::stdin().lock());
    match input {
        Ok(reactions) => {
            println!(
                "Needed ore: {}",
                calculate_needed_source(&reactions, "ORE", "FUEL", 1)
            );
            println!(
                "Max fuel: {}",
                calculate_max_fuel(&reactions, "ORE", "FUEL", 1000000000000)
            );
        }
        Err(e) => {
            eprintln!("Error while parsing input: {}", e);
            process::exit(1);
        }
    }
}
