mod day01;
mod day02;

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let day: u8 = std::env::args()
        .nth(1)
        .ok_or("Usage: advent-of-code-2019 <day>")?
        .parse()
        .unwrap();

    let input = fs::read_to_string(format!("day{:02}.txt", day));

    let result = match day {
        1 => day01::unlock(&input?),
        2 => day02::unlock(&input?),
        n if n >= 1 || n <= 25 => panic!("wow there, you have to be more patient!"),
        _ => panic!("sadly, what you are trying to ask for is impossible!"),
    }?;

    print!("Day {:02} solution: {:?}", day, result);

    Ok(())
}
