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

fn part1(range: RangeInclusive<usize>) -> usize {
    let adjacent_digits_are_the_same = |digits: &Vec<u32>| digits.windows(2).any(|w| w[0] == w[1]);
    let never_decrease = |digits: &Vec<u32>| digits.windows(2).all(|w| w[0] <= w[1]);

    range
        .map(digits)
        .filter(adjacent_digits_are_the_same)
        .filter(never_decrease)
        .count()
}

fn part2(range: RangeInclusive<usize>) -> usize {
    let adjacent_digits_are_the_same = |digits: &Vec<u32>| {
        digits.windows(2).enumerate().any(|(pos, w)| {
            let left_pos = pos.saturating_sub(1);
            let right_pos = (pos + 1 + 1).min(5);
            w[0] == w[1]
                && (left_pos == pos || digits[left_pos] != w[0])
                && (right_pos == pos + 1 || digits[right_pos] != w[0])
        })
    };
    let never_decrease = |digits: &Vec<u32>| digits.windows(2).all(|w| w[0] <= w[1]);

    range
        .map(digits)
        .filter(adjacent_digits_are_the_same)
        .filter(never_decrease)
        .count()
}

fn digits(n: usize) -> Vec<u32> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
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
