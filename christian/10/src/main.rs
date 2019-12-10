use itertools::*;
use std::io::{self, prelude::*};

type Pt = cgmath::Point2<i64>;
// A direction type with fractional components (instead of inprecise f64)
#[derive(Debug, Clone, Copy)]
struct Dir(cgmath::Vector2<i64>);

fn parse(input: &Vec<String>) -> Vec<Pt> {
    input
        .iter()
        .enumerate()
        .flat_map(|(y, s)| {
            s.bytes()
                .enumerate()
                .filter_map(move |(x, c)| Some(Pt::new(x as i64, y as i64)).filter(|_| c == b'#'))
        })
        .collect()
}

impl PartialEq for Dir {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl Eq for Dir {}

impl PartialOrd for Dir {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Dir {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let quadrant = |dir: &Dir| {
            let x = match dir.0.x.signum() {
                0 => dir.0.y.signum(),
                s => -s,
            };
            let y = match dir.0.y.signum() {
                0 => 1,
                s => s * -x,
            };
            (x, y)
        };
        (quadrant(self), other.0.x * self.0.y).cmp(&(quadrant(other), self.0.x * other.0.y))
    }
}

fn len2(a: Dir) -> i64 {
    (a.0.x * a.0.x) + (a.0.y * a.0.y)
}

fn solve1(coords: &[Pt]) -> Option<(Pt, usize)> {
    let num_visible = |&origin| {
        let angles = coords.iter().map(|p| Dir(p - origin)).sorted();
        (origin, angles.dedup().count() - 1)
    };
    coords.iter().map(num_visible).max_by_key(|(_, c)| *c)
}

fn solve2(origin: Pt, coords: &[Pt]) -> Option<(usize, Dir, Pt)> {
    let angles = coords.iter().map(|p| Dir(p - origin));
    let angles = angles.sorted_by_key(|&a| (a, len2(a)));
    let angles = angles.scan((None, 0), |(prev, count), angle| {
        *count = if Some(angle) != *prev { 1 } else { *count + 1 };
        *prev = Some(angle);
        return Some((*count, angle));
    });
    angles.sorted().nth(200).map(|(c, a)| (c, a, origin + a.0))
}

fn main() {
    let input = parse(&io::stdin().lock().lines().map(|x| x.unwrap()).collect());
    let result1 = solve1(&input);
    println!("Part1:\n{:?}", result1);
    println!("Part2:\n{:?}", solve2(result1.unwrap().0, &input));
}
