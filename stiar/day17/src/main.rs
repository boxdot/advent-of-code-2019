use day17::*;

use rand::{self, SeedableRng};

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
            let map = extract_map(program.clone(), true);
            println!("Aligment: {}", calculate_alignment(&map));
            let mut rng = rand::rngs::StdRng::from_seed([42; 32]);
            loop {
                let (_, commands) = get_euler_path(&map, &mut rng);
                if let Some(output) = pack_strings(&commands) {
                    println!(
                        "Robot result: {:?}",
                        run_robot(
                            program.clone(),
                            &output.0,
                            &output.1,
                            &output.2,
                            &output.3,
                            false
                        )
                    );
                    break;
                }
            }
        }
        Err(e) => {
            eprintln!("Error while parsing input: {}", e);
            process::exit(1);
        }
    }
}
