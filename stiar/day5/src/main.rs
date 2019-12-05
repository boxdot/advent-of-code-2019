use day5::*;

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
            let mut mut_program_1: Vec<_> = program.iter().cloned().collect();
            println!("Output for 1: {:?}", apply_program(&mut mut_program_1, 1));
            let mut mut_program_5: Vec<_> = program.iter().cloned().collect();
            println!("Output for 5: {:?}", apply_program(&mut mut_program_5, 5));
        }
        Err(e) => {
            eprintln!("Error while parsing input: {}", e);
            process::exit(1);
        }
    }
}
