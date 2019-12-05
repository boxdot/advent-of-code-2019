use std::ops::RangeInclusive;

pub fn solve(input: &str) -> Option<(usize, usize)> {
    let range = parse(input)?;
    Some((part1(range.clone()), part2(range)))
}

fn parse(input: &str) -> Option<RangeInclusive<usize>> {
    let mut parts = input.split('-');
    let start = parts.next()?.parse().ok()?;
    let end: usize = parts.next()?.parse().ok()?;
    Some(start..=end)
}

type Password = [u8; 6];

fn part1(range: RangeInclusive<usize>) -> usize {
    let adjacent_digits_are_the_same = |digits: &Password| digits.windows(2).any(|w| w[0] == w[1]);
    let never_decrease = |digits: &Password| digits.windows(2).all(|w| w[0] <= w[1]);

    range
        .map(digits)
        .filter(adjacent_digits_are_the_same)
        .filter(never_decrease)
        .count()
}

fn part2(range: RangeInclusive<usize>) -> usize {
    let adjacent_digits_are_the_same_prime = |digits: &Password| {
        let mut extended = [255; 8];
        extended[1..7].copy_from_slice(digits);
        extended
            .windows(4)
            .any(|x| x[0] != x[1] && x[1] == x[2] && x[2] != x[3])
    };
    let never_decrease = |digits: &Password| digits.windows(2).all(|w| w[0] <= w[1]);

    range
        .map(digits)
        .filter(adjacent_digits_are_the_same_prime)
        .filter(never_decrease)
        .count()
}

fn digits(n: usize) -> Password {
    [
        (n % 1000000 / 100000) as u8,
        (n % 100000 / 10000) as u8,
        (n % 10000 / 1000) as u8,
        (n % 1000 / 100) as u8,
        (n % 100 / 10) as u8,
        (n % 10 / 1) as u8,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(111111..=111111), 1);
        assert_eq!(part1(223450..=223450), 0);
        assert_eq!(part1(123789..=123789), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(112233..=112233), 1);
        assert_eq!(part2(123444..=123444), 0);
        assert_eq!(part2(111122..=111122), 1);
    }
}
