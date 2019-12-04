use itertools::*;

fn digits(x: usize) -> impl Iterator<Item = usize> + Clone {
    std::iter::successors(Some(x), |x| Some(x / 10).filter(|x| *x != 0)).map(|x| x % 10)
}

fn filter1(x: &(impl Iterator<Item = usize> + Clone)) -> bool {
    x.clone().tuple_windows().filter(|(a, b)| a == b).count() != 0
        && x.clone().tuple_windows().filter(|(a, b)| a < b).count() == 0
}

fn solve1(range: std::ops::RangeInclusive<usize>) -> usize {
    range.map(digits).filter(filter1).count()
}

fn filter2(x: &(impl Iterator<Item = usize> + Clone)) -> bool {
    for (_, group) in &x.clone().group_by(|d| *d) {
        if group.count() == 2 {
            return x.clone().tuple_windows().filter(|(a, b)| a < b).count() == 0;
        }
    }
    false
}

fn solve2(range: std::ops::RangeInclusive<usize>) -> usize {
    range.map(digits).filter(filter2).count()
}

fn main() {
    println!("Part1: {:?}", solve1(124075..=580769));
    println!("Part2: {:?}", solve2(124075..=580769));
}
