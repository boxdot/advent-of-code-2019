use itertools::*;
use std::io::{self, prelude::*};
use vm::Vm;

fn parse(input: &Vec<String>) -> Vec<i64> {
    let ints = input.iter().map(|x| x.split(',')).flatten();
    ints.filter_map(|x| x.parse().ok()).collect()
}

fn solve1(mem: &mut [i64], noun: i64, verb: i64) -> Option<i64> {
    mem[1] = noun;
    mem[2] = verb;
    Vm::new(mem.into(), [].iter().copied()).next()
}

fn solve2(input: Vec<i64>, result: i64) -> Option<(i64, i64)> {
    iproduct!(0..100, 0..100)
        .find(|&(noun, verb)| solve1(&mut input.clone(), noun, verb) == Some(result))
}

fn main() {
    let input = parse(&io::stdin().lock().lines().map(|x| x.unwrap()).collect());
    println!("Part1: {:?}", solve1(&mut input.clone(), 12, 02));
    println!("Part2: {:?}", solve2(input.clone(), 19690720));
}
