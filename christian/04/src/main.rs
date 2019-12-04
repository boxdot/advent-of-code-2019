use itertools::*;
use std::cmp::Ordering as Ord;
use std::iter::successors;

fn valid(x: usize, run_filter: &[Ord]) -> bool {
    let digits = successors(Some(x), |x| Some(x / 10).filter(|x| *x != 0)).map(|x| x % 10);
    !digits.clone().tuple_windows().any(|(a, b)| a < b)
        && (digits.group_by(|d| *d).into_iter())
            .any(|(_, run)| run_filter.contains(&run.count().cmp(&2)))
}

fn solve(range: std::ops::RangeInclusive<usize>, run_filter: &[Ord]) -> usize {
    range.filter(|&x| valid(x, run_filter)).count()
}

fn main() {
    let r = 124075..=580769;
    println!("Part1: {:?}", solve(r.clone(), &[Ord::Equal, Ord::Greater]));
    println!("Part2: {:?}", solve(r, &[Ord::Equal]));
}
