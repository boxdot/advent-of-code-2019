use std::io::{self, prelude::*};
use vm::Vm;

fn parse(input: &Vec<String>) -> Vec<i64> {
    let ints = input.iter().map(|x| x.split(',')).flatten();
    ints.filter_map(|x| x.parse().ok()).collect()
}

fn solve1(mem: &Vec<i64>) -> Vec<i64> {
    Vm::new(mem.clone(), vec![1].into_iter()).collect()
}

fn solve2(mem: &Vec<i64>) -> Vec<i64> {
    Vm::new(mem.clone(), vec![2].into_iter()).collect()
}

fn main() {
    let input = parse(&io::stdin().lock().lines().map(|x| x.unwrap()).collect());
    println!("Part1: {:?}", solve1(&input.clone()));
    println!("Part2: {:?}", solve2(&input.clone()));
}
