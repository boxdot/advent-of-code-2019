use day2::*;

use std::io;
use std::process;

fn read_input(mut in_stream: impl io::BufRead) -> io::Result<Vec<usize>> {
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
            let mut mut_program: Vec<_> = program.iter().cloned().collect();
            mut_program[1] = 12;
            mut_program[2] = 2;
            apply_program(&mut mut_program);
            println!("Initial value: {}", mut_program[0]);

            let noun_verb = find_verb_and_noun(&program, 19690720);
            match noun_verb {
                Some((noun, verb)) => {
                    println!("100*noun + verb: {}", 100*noun + verb);
                }
                None => {
                    println!("Couldn't find noun and verb!");
                }
            }

        }
        Err(e) => {
            eprintln!("Error while parsing input: {}", e);
            process::exit(1);
        }
    }
}
