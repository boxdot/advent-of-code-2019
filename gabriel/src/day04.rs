use itertools::*;

fn get_digits(number: usize) -> [u8; 6] {
    // works only for 6 digits, but ultra fast
    [
        (number / 100000 % 10) as u8,
        (number / 10000 % 10) as u8,
        (number / 1000 % 10) as u8,
        (number / 100 % 10) as u8,
        (number / 10 % 10) as u8,
        (number % 10) as u8,
    ]
}

fn meets_criteria_part1(number: usize) -> bool {
    let digits = get_digits(number);
    let consecutive_digits = digits.iter().tuple_windows().any(|(a, b)| a == b);
    let decreasing_digits = digits.iter().tuple_windows().any(|(a, b)| b < a);
    digits.len() == 6 && consecutive_digits && !decreasing_digits
}

fn meets_criteria_part2(number: usize) -> bool {
    let digits = get_digits(number);
    let many_consecutive_digits = digits
        .iter()
        .group_by(|d| *d)
        .into_iter()
        .any(|(_, group)| {
            group.count() >= 2
        });

    let part1 = meets_criteria_part1(number);
    part1 && many_consecutive_digits
}

#[test]
pub fn test_meets_criterias() {
    // part 1
    assert_eq!(meets_criteria_part1(111111), true);
    assert_eq!(meets_criteria_part1(223450), false);
    assert_eq!(meets_criteria_part1(123789), false);
    // part 2
    assert_eq!(meets_criteria_part2(112233), true);
    assert_eq!(meets_criteria_part2(123444), false);
    assert_eq!(meets_criteria_part2(111122), false);
}

pub fn unlock(input: &str) -> Result<(usize, usize), Box<dyn std::error::Error>> {
    let (min, max) = input
        .split('-')
        .filter_map(|n| n.parse::<usize>().ok())
        .next_tuple()
        .unwrap();

    let part1 = (min..=max).filter(|n| meets_criteria_part1(*n)).count();
    let part2 = (min..=max).filter(|n| meets_criteria_part2(*n)).count();

    Ok((part1, part2))
}
