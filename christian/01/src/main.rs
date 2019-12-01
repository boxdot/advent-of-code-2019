use std::io::{self, prelude::*};

fn parse(input: &Vec<String>) -> Vec<u64> {
    input.iter().map(|x| x.parse().unwrap()).collect()
}

fn solve1(input: &[u64]) -> u64 {
    input.iter().map(|x| (x / 3).saturating_sub(2)).sum()
}

fn solve2(input: &[u64]) -> u64 {
    let fuel = |x: &u64| (x / 3).checked_sub(2);
    let total_fuel = |x| -> u64 { std::iter::successors(fuel(x), fuel).sum() };
    input.iter().map(total_fuel).sum()
}

fn main() {
    let input = parse(&io::stdin().lock().lines().map(|x| x.unwrap()).collect());
    println!("Part1: {}", solve1(&input));
    println!("Part2: {}", solve2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input() {
        assert_eq!(solve2(&vec![14]), 2);
        assert_eq!(solve2(&vec![1969]), 966);
        assert_eq!(solve2(&vec![100756]), 50346);
    }
}
