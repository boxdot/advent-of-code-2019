mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args();
    let input_file = args
        .nth(1)
        .ok_or("Usage: advent-of-code-2019 <dayYY.txt>")?;
    if let Some(arg) = args.next() {
        return Err(format!("unexpected argument: {}", arg).into());
    }

    let re = regex::Regex::new(r"^.*day(\d+).txt$")?;
    let day: u8 = re
        .captures(&input_file)
        .and_then(|c| c.get(1).map(|g| g.as_str()))
        .ok_or_else(|| format!("can't deduce day from: {}", input_file))?
        .parse()?;

    let input = std::fs::read_to_string(input_file)?;
    match day {
        1 => println!("{:?}", day01::solve(&input)),
        2 => println!("{:?}", day02::solve(&input)),
        3 => println!("{:?}", day03::solve(&input)),
        4 => println!("{:?}", day04::solve(&input)),
        5 => println!("{:?}", day05::solve(&input)),
        6 => println!("{:?}", day06::solve(&input)),
        7 => println!("{:?}", day07::solve(&input)),
        8 => println!("{:?}", day08::solve(&input)),
        9 => println!("{:?}", day09::solve(&input)),
        _ => eprintln!("invalid day: {}", day),
    }

    Ok(())
}
