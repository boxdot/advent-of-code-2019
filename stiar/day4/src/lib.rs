use itertools::Itertools;

use std::convert::TryInto;

fn is_valid_password(number: u32, should_contain_exact_two: bool) -> bool {
    let digits: Vec<_> = format!("{:0>6}", number.to_string())
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();
    digits.len() == 6
        && digits.windows(2).all(|slice| slice[0] <= slice[1])
        && digits
            .iter()
            .group_by(|&&d| d)
            .into_iter()
            .any(|(_, group)| {
                if should_contain_exact_two {
                    group.count() == 2
                } else {
                    group.count() >= 2
                }
            })
}

pub fn numbers_in_range((min, max): (u32, u32), should_contain_exact_two: bool) -> u32 {
    (min..=max)
        .filter(|&number| is_valid_password(number, should_contain_exact_two))
        .count()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(numbers_in_range((0, 10), false), 10);
        assert_eq!(numbers_in_range((100000, 111112), false), 2);
        assert_eq!(numbers_in_range((100000, 111112), true), 0);
        assert_eq!(numbers_in_range((100000, 111122), true), 1);
        assert_eq!(numbers_in_range((1000000, 1111122), false), 0);
    }
}
