pub fn solve(input: &str) -> (i64, i64) {
    let masses = input.lines().filter_map(|l| l.parse().ok());

    let part1 = masses.clone().filter_map(fuel).sum();
    let part2 = masses
        .map(|mass| std::iter::successors(fuel(mass), |m| fuel(*m)))
        .flatten()
        .sum();

    (part1, part2)
}

fn fuel(mass: i64) -> Option<i64> {
    let fuel = mass / 3 - 2;
    if fuel > 0 {
        Some(fuel)
    } else {
        None
    }
}
