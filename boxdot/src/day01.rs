pub fn solve(input: &str) -> (u64, u64) {
    let masses = input.lines().filter_map(|l| l.parse().ok());

    let part1 = masses.clone().filter_map(fuel).sum();
    let part2 = masses
        .map(|mass| std::iter::successors(fuel(mass), |m| fuel(*m)))
        .flatten()
        .sum();

    (part1, part2)
}

fn fuel(mass: u64) -> Option<u64> {
    (mass / 3).checked_sub(2)
}
