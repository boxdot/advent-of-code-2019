fn main() {
    let contents = include_str!("day01.txt");
    println!(
        "{}",
        contents
            .lines()
            .map(|x| (x.parse::<f64>().unwrap() / 3.0).floor() as u64 - 2)
            .sum::<u64>()
    );
}
