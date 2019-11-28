fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = std::env::args()
        .nth(1)
        .ok_or("Usage: advent-of-code-2019 <dayYY.txt>")?;

    let re = regex::Regex::new(r"^.*day(\d+).txt$")?;
    let day: u8 = re
        .captures(&input_file)
        .and_then(|c| c.get(1).map(|g| g.as_str()))
        .ok_or_else(|| format!("can't deduce day from: {}", input_file))?
        .parse()?;

    let _input = std::fs::read_to_string(input_file)?;
    match day {
        _ => eprintln!("invalid day: {}", day),
    }

    Ok(())
}