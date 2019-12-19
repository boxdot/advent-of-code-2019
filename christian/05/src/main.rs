use std::io::{self, prelude::*};
use vm::Vm;

fn parse(input: &Vec<String>) -> Vec<i64> {
    let ints = input.iter().map(|x| x.split(',')).flatten();
    ints.filter_map(|x| x.parse().ok()).collect()
}

fn main() {
    let input = parse(&io::stdin().lock().lines().map(|x| x.unwrap()).collect());
    println!(
        "Part1: {:?}",
        Vm::new(input.clone(), vec![1].into_iter()).next()
    );
    println!(
        "Part2: {:?}",
        Vm::new(input.clone(), vec![5].into_iter()).next()
    );
}
