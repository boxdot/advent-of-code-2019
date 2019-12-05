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
    //next bigger sorted bound
    let digits: Vec<_> = digits(upper_bound).collect();
    let mut bound_digit_count = [0; 10];
    for i in (0..=digits.len() - 1).rev() {
        if Some(digits[i]) < digits.get(i + 1).cloned() {
            bound_digit_count[digits[i + 1] as usize] += i + 1;
            break;
        }
        bound_digit_count[digits[i] as usize] += 1;
    }
    let num_digits: usize = bound_digit_count.iter().sum();

    // state space is: digit, digits left, had_run, is_less
    let mut state = vec![0; 2 * 2 * 11 * (num_digits + 1)];
    let index = |digit: Option<usize>, left: usize, had_run: bool, is_less: bool| {
        ((digit.unwrap_or(10) * (num_digits + 1) + left) * 2 + had_run as usize) * 2
            + is_less as usize
    };

    // start point: No digits assigned, no valid runs, not yet below bound
    state[index(None, num_digits, false, false)] = 1;

    for (digit, left, &had_run, &is_less) in
        iproduct!(0..=9, 0..=num_digits, &[false, true], &[false, true])
    {
        // how many used digits do we want to put into current digit?
        // if we do not want a run in this state, make sure we do not get one
        // if we do not want to be less than the bound, make sure we are not
        let counts = (0..=(num_digits - left)).filter(|&count| {
            (had_run || !((part1 && count >= 2) || count == 2))
                && (is_less || count == bound_digit_count[digit])
        });
        let sum_possibilites = |count| -> usize {
            // try all possible state transitions that are allowed
            let prev_is_less = [is_less, is_less && count <= bound_digit_count[digit]];
            let prev_had_run = [had_run, had_run && !((part1 && count >= 2) || count == 2)];
            iproduct!(prev_is_less.iter().dedup(), prev_had_run.iter().dedup())
                .map(|(&prev_is_less, &prev_had_run)| {
                    let prev = digit.checked_sub(1);
                    state[index(prev, left + count, prev_had_run, prev_is_less)]
                })
                .sum()
        };
        state[index(Some(digit), left, had_run, is_less)] = counts.map(sum_possibilites).sum();
    }

    // Extract result from final state: 9, all assigned, valid run, below bound
    state[index(Some(9), 0, true, true)]
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
