use day16::*;

use std::io;
use std::process;

fn read_input(mut in_stream: impl io::BufRead) -> io::Result<Vec<u32>> {
    let mut buffer = String::new();

    // Input numbers.
    in_stream.read_line(&mut buffer)?;
    Ok(buffer
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect())
}

fn main() {
    let input = read_input(io::stdin().lock());
    match input {
        Ok(numbers) => {
            println!(
                "First eight digits of 100 fft: {}",
                calculate_fft(numbers.clone(), 100, 0)
            );

            let bigger_input: Vec<_> = std::iter::repeat(numbers.clone())
                .take(10000)
                .flatten()
                .collect();
            println!(
                "First eight digits of big input shifted 100 fft: {}",
                calculate_fft(bigger_input.clone(), 100, 5975483)
            );
        }
        Err(e) => {
            eprintln!("Error while parsing input: {}", e);
            process::exit(1);
        }
    }
}
