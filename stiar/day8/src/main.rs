use day8::*;

use ndarray::prelude::*;

use std::io;
use std::process;

fn read_input(mut in_stream: impl io::BufRead) -> io::Result<Vec<u32>> {
    let mut buffer = String::new();

    // Input numbers.
    in_stream.read_line(&mut buffer)?;
    buffer
        .trim()
        .chars()
        .map(|x| {
            x.to_digit(10)
                .ok_or(io::Error::new(io::ErrorKind::InvalidData, "Invalid digit"))
        })
        .collect()
}

fn main() {
    let input = read_input(io::stdin().lock());
    match input {
        Ok(raw_image) => {
            let image =
                Array::from_shape_vec((raw_image.len() / (25 * 6), 6, 25), raw_image).unwrap();
            println!("Checksum {}", checksum(&image));
            let decoded = decode(&image);

            for row in decoded.genrows() {
                println!(
                    "{}",
                    row.into_iter()
                        .map(|v| v.to_string())
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
