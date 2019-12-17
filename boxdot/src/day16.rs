pub fn solve(input: &str) -> Vec<i8> {
    let init = parse(input);
    let mut phases = std::iter::successors(Some(init), |seq| Some(phase(&seq)));
    let part1 = phases.nth(100).unwrap()[0..8].to_vec();
    part1
}

fn parse(input: &str) -> Vec<i8> {
    input
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as i8))
        .collect()
}

fn pattern(n: usize) -> impl Iterator<Item = i8> {
    use std::iter::repeat;
    repeat(0)
        .take(n)
        .chain(repeat(1).take(n))
        .chain(repeat(0).take(n))
        .chain(repeat(-1).take(n))
        .cycle()
}

fn phase(input: &[i8]) -> Vec<i8> {
    (0..input.len())
        .map(|i| {
            let x: i64 = input
                .iter()
                .zip(pattern(i + 1).skip(1))
                .map(|(&x, y)| x as i64 * y as i64)
                .sum();
            (x % 10).abs() as i8
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern() {
        assert_eq!(
            pattern(1).take(8).collect::<Vec<_>>(),
            &[0, 1, 0, -1, 0, 1, 0, -1]
        );
        assert_eq!(
            pattern(2).take(16).collect::<Vec<_>>(),
            &[0, 0, 1, 1, 0, 0, -1, -1, 0, 0, 1, 1, 0, 0, -1, -1]
        );
        assert_eq!(
            pattern(3).take(24).collect::<Vec<_>>(),
            &[0, 0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1, 0, 0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1]
        );
    }

    #[test]
    fn test_phase() {
        let ph1 = phase(&vec![1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(ph1, vec![4, 8, 2, 2, 6, 1, 5, 8]);

        let ph2 = phase(&ph1);
        assert_eq!(ph2, vec![3, 4, 0, 4, 0, 4, 3, 8]);
    }
}
