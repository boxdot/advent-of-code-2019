use itertools::*;
use std::cmp::Ordering as Ord;
use std::iter::successors;

fn digits(x: u64) -> impl Iterator<Item = u64> {
    successors(Some(x), |x| Some(x / 10).filter(|x| *x != 0)).map(|x| x % 10)
}

fn valid(x: u64, run_filter: &[Ord]) -> bool {
    !digits(x).tuple_windows().any(|(a, b)| a < b)
        && (digits(x).group_by(|d| *d).into_iter())
            .any(|(_, run)| run_filter.contains(&run.count().cmp(&2)))
}

fn solve(range: std::ops::RangeInclusive<u64>, run_filter: &[Ord]) -> usize {
    range.filter(|&x| valid(x, run_filter)).count()
}

// Compure the result directly based on combinatorics and dynamic programming
// 1) non-decreasing sequence means ordere of digits does not matter, just digit histogram
// 2) sorted sequence less than upper bound == histogram bigger than histogram of upper bound
fn compute(upper_bound: u64, part1: bool) -> usize {
    //next bigger sorted bound -> min digit hist count
    let digits: Vec<_> = digits(upper_bound).collect();
    let mut min_digit_count = [0; 10];
    for i in (0..=digits.len() - 1).rev() {
        if Some(digits[i]) < digits.get(i + 1).cloned() {
            min_digit_count[digits[i + 1] as usize] += i + 1;
            break;
        }
        min_digit_count[digits[i] as usize] += 1;
    }
    let num_digits: usize = min_digit_count.iter().sum();

    // state space is: digit, digits left, had_run, is_less
    let mut state = vec![0; 2 * 2 * 10 * (num_digits + 1)];
    let index = |digit: usize, left: usize, had_run: bool, is_less: bool| {
        ((digit * (num_digits + 1) + left) * 2 + had_run as usize) * 2 + is_less as usize
    };

    // start point: No `0` assigned, no valid runs, not yet below bound
    state[index(0, num_digits, false, false)] = 1;

    for (next, left, &had_run, &is_less) in
        iproduct!(1..=9, 0..=num_digits, &[false, true], &[false, true])
    {
        // how many used left digits do we want to put into current digit?
        for count in 0.max((!is_less) as usize * min_digit_count[next])..=left {
            let next_is_less = is_less || count > min_digit_count[next];
            let next_had_run = had_run || ((part1 && count >= 2) || count == 2);
            state[index(next, left - count, next_had_run, next_is_less)] +=
                state[index(next - 1, left, had_run, is_less)];
        }
    }

    // Extract result from final state: 9, all assigned, valid run, below bound
    state[index(9, 0, true, true)]
}

fn main() {
    let r = 124075..=580769;

    println!("Part1: {}", compute(580769, true) - compute(124075, true));
    println!(
        "Check Part1: {:?}",
        solve(r.clone(), &[Ord::Equal, Ord::Greater])
    );
    println!("Part2: {}", compute(580769, false) - compute(124075, false));
    println!("Check Part2: {:?}", solve(r, &[Ord::Equal]));
}
