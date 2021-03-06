use itertools::*;
use std::io::{self, prelude::*};

type Pt = cgmath::Point3<i64>; // x, y, and length from origin along line
type Dim = cgmath::Vector3<i64>; // x, y, length of dim
type Line = (Pt, Pt);

fn parse(input: &Vec<String>) -> Vec<Vec<Dim>> {
    let parse = |x: &str| -> Dim {
        let value = x[1..].parse().expect("Expected i64");
        match x.chars().next().unwrap() {
            'U' => Dim::new(0, value, value),
            'D' => Dim::new(0, -value, value),
            'R' => Dim::new(value, 0, value),
            'L' => Dim::new(-value, 0, value),
            d => panic!("Invalid direction {}", d),
        }
    };
    let parse_line = |x: &String| x.split(',').map(parse).collect();
    input.iter().map(parse_line).collect()
}

fn to_line(x: &Vec<Dim>) -> impl Iterator<Item = Line> + Clone + '_ {
    x.iter().scan(Pt::new(0, 0, 0), |pt, dim| {
        let prev = *pt;
        *pt += *dim;
        if (prev.x, prev.y) < (pt.x, pt.y) {
            Some((prev, *pt))
        } else {
            Some((*pt, prev))
        }
    })
}

fn intersect((l1, l2): (Line, Line)) -> Option<Pt> {
    let dist = |h: Line, v: Line| {
        (h.0.z + v.0.x - h.0.x).min(h.1.z + h.1.x - v.0.x)
            + (v.0.z + h.0.y - v.0.y).min(v.1.z + v.1.y - h.0.y)
    };
    if (l1.0.x..=l1.1.x).contains(&l2.0.x) && (l2.0.y..=l2.1.y).contains(&l1.0.y) {
        Some(Pt::new(l2.0.x, l1.0.y, dist(l1, l2))).filter(|pt| (pt.x, pt.y) != (0, 0))
    } else if (l2.0.x..=l2.1.x).contains(&l1.0.x) && (l1.0.y..=l1.1.y).contains(&l2.0.y) {
        Some(Pt::new(l1.0.x, l2.0.y, dist(l2, l1))).filter(|pt| (pt.x, pt.y) != (0, 0))
    } else {
        None
    }
}

fn solve(data: &Vec<Vec<Dim>>, min_func: impl Fn(Pt) -> i64) -> Option<i64> {
    let combos = data.iter().map(|x| to_line(x)).tuple_combinations();
    // No need for sweep line, input is too small... O(N²) works fine here
    let pairs = combos.map(|(left, right)| iproduct!(left, right)).flatten();
    pairs.filter_map(intersect).map(min_func).min()
}

fn main() {
    let input = parse(&io::stdin().lock().lines().map(|x| x.unwrap()).collect());
    println!("Part1: {:?}", solve(&input, |pt| pt.x.abs() + pt.y.abs()));
    println!("Part2: {:?}", solve(&input, |pt| pt.z));
}
