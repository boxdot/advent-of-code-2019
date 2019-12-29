mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args();
    let input_file = args
        .nth(1)
        .ok_or("Usage: advent-of-code-2019 <dayYY.txt>")?;
    if let Some(arg) = args.next() {
        return Err(format!("unexpected argument: {}", arg).into());
    }

    let re = regex::Regex::new(r"^.*day(\d+).*\.txt$")?;
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
        10 => println!("{:?}", day10::solve(&input)),
        11 => println!("{:?}", day11::solve(&input)),
        12 => println!("{:?}", day12::solve(&input)),
        13 => println!("{:?}", day13::solve(&input)),
        14 => println!("{:?}", day14::solve(&input)),
        15 => println!("{:?}", day15::solve(&input)),
        16 => println!("{:?}", day16::solve(&input)),
        17 => println!("{:?}", day17::solve(&input)),
        18 => println!("{:?}", day18::solve(&input)),
        19 => println!("{:?}", day19::solve(&input)),
        20 => println!("{:?}", day20::solve(&input)),
        21 => println!("{:?}", day21::solve(&input)),
        22 => println!("{:?}", day22::solve(&input)),
        _ => eprintln!("invalid day: {}", day),
    }

    Ok(())
}
