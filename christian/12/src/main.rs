use itertools::*;
use std::io::{self, prelude::*};
#[macro_use]
extern crate text_io;

type Pt = cgmath::Point3<i64>;
type Dir = cgmath::Vector3<i64>;

fn parse(input: &Vec<String>) -> Vec<(Pt, Dir)> {
    let to_point = |x: &String| {
        let mut bytes = x.bytes();
        let (x, y, z);
        scan!(bytes => "<x={}, y={}, z={}>", x, y, z);
        (Pt::new(x, y, z), Dir::new(0, 0, 0))
    };
    input.iter().map(to_point).collect()
}

fn solve1(mut points: Vec<(Pt, Dir)>) -> i64 {
    for _i in 0..1000 {
        for (a, b) in iproduct!(0..points.len(), 0..points.len()) {
            let diff = points[b].0 - points[a].0;
            points[a].1 += Dir::new(diff.x.signum(), diff.y.signum(), diff.z.signum());
        }
        for p in &mut points {
            p.0 += p.1;
        }
    }
    points
        .iter()
        .map(|p| {
            (p.0.x.abs() + p.0.y.abs() + p.0.z.abs()) * (p.1.x.abs() + p.1.y.abs() + p.1.z.abs())
        })
        .sum()
}

fn dim1cycle(data: Vec<(i64, i64)>) -> i64 {
    let mut points = data.clone();
    for i in 0.. {
        if i != 0 && points == data {
            return i;
        }
        for (a, b) in iproduct!(0..points.len(), 0..points.len()) {
            points[a].1 += (points[b].0 - points[a].0).signum();
        }
        for p in &mut points {
            p.0 += p.1;
        }
    }
    panic!("Not found -> endless loop");
}

fn lcm(mut a: i64, mut b: i64) -> i64 {
    let result = a * b;
    while a != 0 {
        b %= a;
        std::mem::swap(&mut a, &mut b);
    }
    result / b
}

fn solve2(data: Vec<(Pt, Dir)>) -> i64 {
    let x_cycle = dim1cycle(data.iter().map(|p| (p.0.x, p.1.x)).collect());
    let y_cycle = dim1cycle(data.iter().map(|p| (p.0.y, p.1.y)).collect());
    let z_cycle = dim1cycle(data.iter().map(|p| (p.0.z, p.1.z)).collect());
    lcm(lcm(x_cycle, y_cycle), z_cycle)
}

fn main() {
    let input = parse(&io::stdin().lock().lines().map(|x| x.unwrap()).collect());
    println!("Part1:\n{}", solve1(input.clone()));
    println!("Part2:\n{}", solve2(input.clone()));
}
