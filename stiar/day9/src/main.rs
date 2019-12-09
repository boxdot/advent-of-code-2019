use day9::*;

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
            println!(
                "Input 1: {:?}",
                Program::new(program.clone())
                    .run(vec![1].into_iter())
                    .collect::<Vec<_>>()
            );
            println!(
                "Input 2: {:?}",
                Program::new(program)
                    .run(vec![2].into_iter())
                    .collect::<Vec<_>>()
            );
        }
        Err(e) => {
            eprintln!("Error while parsing input: {}", e);
            process::exit(1);
        }
    }
}
