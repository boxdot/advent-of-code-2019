use itertools::*;
use vm::Vm;
mod program;

fn test((x, y): (i64, i64)) -> i64 {
    Vm::new(program::data(), [x, y].iter().copied())
        .next()
        .unwrap()
}

fn solve1() -> i64 {
    iproduct!(0..50, 0..50).map(test).sum()
}

fn solve2(spread_50_50: i64) -> (i64, i64) {
    let max = 50 * 100 / (spread_50_50 / 50);
    let first_y = |x| (max / 2..).filter(|y| test((x, *y)) == 1).next().unwrap();
    let (mut base, mut size) = (max / 2, max - max / 2);
    while size > 1 {
        let (half, mid) = (size / 2, base + size / 2);
        if test((mid - 99, first_y(mid) + 99)) == 0 {
            size -= half + 1;
            base = mid + 1;
        } else {
            size -= half;
        };
    }
    while test((base - 99, first_y(base) + 99)) == 0 {
        base -= 1; // correct for non-sorted too_small test;
    }
    (base - 99, first_y(base))
}

fn main() {
    let p1 = solve1();
    println!("Part1:\n{}", p1);
    println!("Part2:\n{:?}", solve2(p1));
}
