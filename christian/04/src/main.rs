use itertools::*;
use std::cmp::Ordering as Ord;

fn valid(x: usize, run_filter: &[Ord]) -> bool {
    let digits =
        std::iter::successors(Some(x), |x| Some(x / 10).filter(|x| *x != 0)).map(|x| x % 10);
    for (_, group) in &digits.clone().group_by(|d| *d) {
        if run_filter.contains(&group.count().cmp(&2)) {
            return digits.tuple_windows().filter(|(a, b)| a < b).count() == 0;
        }
    }
    false
}

fn solve(range: std::ops::RangeInclusive<usize>, run_filter: &[Ord]) -> usize {
    range.filter(|&x| valid(x, run_filter)).count()
}

fn main() {
    let r = 124075..=580769;
    println!("Part1: {:?}", solve(r.clone(), &[Ord::Equal, Ord::Greater]));
    println!("Part2: {:?}", solve(r, &[Ord::Equal]));
}
