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
    let get_index = |digit: Option<usize>, left: usize, had_run: bool, below_bound: bool| {
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

    state[get_index(None, num_digits, false, false).unwrap()] = 1;
    for digit in 0..=9 {
        for left in 0..=num_digits {
            for &had_run in [false, true].into_iter() {
                for &below_bound in [false, true].into_iter() {
                    // how many used digits do we want to put into current digit?
                    let mut sum = 0;
                    let mut max_this_digit = num_digits - left;
                    let mut min_this_digit = 0;
                    if !below_bound {
                        min_this_digit = bound_digit_count[digit];
                        max_this_digit = bound_digit_count[digit];
                    }
                    for count in min_this_digit..=max_this_digit {
                        if !had_run {
                            if (part1 && count > 1) || count == 2 {
                                continue;
                            }
                        }
                        if !below_bound || count > bound_digit_count[digit] {
                            if had_run {
                                if let Some(index) =
                                    get_index(digit.checked_sub(1), left + count, true, false)
                                {
                                    sum += state[index];
                                }
                            }
                            if !had_run || (part1 && count >= 2) || count == 2 {
                                if let Some(index) =
                                    get_index(digit.checked_sub(1), left + count, false, false)
                                {
                                    sum += state[index];
                                }
                            }
                        }
                        if below_bound {
                            if had_run {
                                if let Some(index) =
                                    get_index(digit.checked_sub(1), left + count, true, true)
                                {
                                    sum += state[index];
                                }
                            }
                            if !had_run || (part1 && count >= 2) || count == 2 {
                                if let Some(index) =
                                    get_index(digit.checked_sub(1), left + count, false, true)
                                {
                                    sum += state[index];
                                }
                            }
                        }
                    }
                    state[get_index(Some(digit), left, had_run, below_bound).unwrap()] = sum;
                }
            }
        }
    }

    state[get_index(Some(9), 0, true, true).unwrap()]
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
