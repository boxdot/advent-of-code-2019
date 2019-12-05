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

// Compure the result directly based on combinatorics and dynamic programming
// 1) non-decreasing sequence means ordere of digits does not matter, just digit histogram
// 2) sorted sequence less than upper bound == histogram bigger than histogram of upper bound
fn compute(upper_bound: u64, part1: bool) -> usize {
    let mut bound_digit_count = [0; 10];
    let mut digits: Vec<_> = successors(Some(upper_bound), |x| Some(x / 10).filter(|x| *x != 0))
        .map(|x| x % 10)
        .collect();

    //next bigger sorted bound
    for i in (0..digits.len() - 1).rev() {
        if digits[i] < digits[i + 1] {
            for j in 0..=i {
                digits[j] = digits[i + 1];
            }
            break;
        }
    }

    for &d in digits.iter() {
        bound_digit_count[d as usize] += 1;
    }
    let num_digits: usize = bound_digit_count.iter().sum();

    // state space is: digit, digits left, had_run, below_bound
    let mut state = vec![0; 2 * 2 * 11 * (num_digits + 1)];
    let index = |digit: Option<usize>, left: usize, had_run: bool, below_bound: bool| {
        if left > num_digits {
            None
        } else {
            Some(
                digit.map(|x| x + 1).unwrap_or(0) * 2 * 2 * (num_digits + 1)
                    + left * 2 * 2
                    + 2 * had_run as usize
                    + below_bound as usize,
            )
        }
    };
    let try_get =
        |state: &[usize], index: Option<usize>| index.map(|index| state[index]).unwrap_or(0);

    state[index(None, num_digits, false, false).unwrap()] = 1;

    for digit in 0..=9_usize {
        for left in 0..=num_digits {
            for &had_run in [false, true].into_iter() {
                for &below_bound in [false, true].into_iter() {
                    let prev_digit = digit.checked_sub(1);
                    let mut sum = 0;

                    // how many used digits do we want to put into current digit?
                    for count in 0..=(num_digits - left) {
                        // if we do not want a run in this state, make sure we do not get one
                        if !had_run && ((part1 && count > 1) || count == 2) {
                            continue;
                        }
                        // if we do not want to be less than the bound, make sure we are not
                        if !below_bound && count != bound_digit_count[digit] {
                            continue;
                        }
                        // try all possible state transitions that are allowed
                        if !below_bound || count > bound_digit_count[digit] {
                            if had_run {
                                sum +=
                                    try_get(&state, index(prev_digit, left + count, true, false));
                            }
                            if !had_run || (part1 && count >= 2) || count == 2 {
                                sum +=
                                    try_get(&state, index(prev_digit, left + count, false, false));
                            }
                        }
                        if below_bound {
                            if had_run {
                                sum += try_get(&state, index(prev_digit, left + count, true, true));
                            }
                            if !had_run || (part1 && count >= 2) || count == 2 {
                                sum +=
                                    try_get(&state, index(prev_digit, left + count, false, true));
                            }
                        }
                    }
                    state[index(Some(digit), left, had_run, below_bound).unwrap()] = sum;
                }
            }
        }
    }

    state[index(Some(9), 0, true, true).unwrap()]
}

fn main() {
    let r = 124075..=580769;

    println!(
        "Compute Part1: {}",
        compute(580769, true) - compute(124075, true)
    );
    println!(
        "Check(slow) Part1: {:?}",
        solve(r.clone(), &[Ord::Equal, Ord::Greater])
    );
    println!(
        "Compute Part2: {}",
        compute(580769, false) - compute(124075, false)
    );
    println!("Check (slow) Part2: {:?}", solve(r, &[Ord::Equal]));
}
