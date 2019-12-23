use day22::*;

use std::io;
use std::process;

fn read_input(mut in_stream: impl io::BufRead) -> io::Result<Vec<Command>> {
    let mut buffer = String::new();
    in_stream.read_to_string(&mut buffer)?;
    Ok(parse_commands(&buffer))
}

fn main() {
    let input = read_input(io::stdin().lock());
    match input {
        Ok(commands) => {
            println!(
                "At position 2019: {:?}",
                apply_commands(10007, &commands)
                    .iter()
                    .position(|&x| x == 2019)
            );
            println!(
                "Ends at position 2020 in big shuffle: {}",
                find_original_position(119315717514047, &commands, 101741582076661, 2020)
            );
        }
        Err(e) => {
            eprintln!("Error while parsing input: {}", e);
            process::exit(1);
        }
    }
}
