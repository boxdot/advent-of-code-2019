use day11::*;

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
                "Num covered: {:?}",
                get_covered_panels(program.clone(), 0).len()
            );
            let panels = get_covered_panels(program.clone(), 1);
            let grid = get_grid(&panels);
            for row in grid.genrows() {
                println!(
                    "{}",
                    row.into_iter()
                        .map(|v| match v {
                            0 => " ".into(),
                            1 => "*".into(),
                            _ => panic!("Unknown color"),
                        })
                        .collect::<Vec<String>>()
                        .join(" ")
                );
            }
        }
        Err(e) => {
            eprintln!("Error while parsing input: {}", e);
            process::exit(1);
        }
    }
}
