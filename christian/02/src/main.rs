use itertools::*;
use std::io::{self, prelude::*};

fn parse(input: &Vec<String>) -> Vec<usize> {
    let ints = input.iter().map(|x| x.split(',')).flatten();
    ints.filter_map(|x| x.parse().ok()).collect()
}

fn solve1(mem: &mut [usize], noun: usize, verb: usize) -> Result<usize, ()> {
    mem[1] = noun;
    mem[2] = verb;
    for pos in (0..mem.len()).step_by(4) {
        match mem[pos] {
            01 => mem[mem[pos + 3]] = mem[mem[pos + 1]] + mem[mem[pos + 2]],
            02 => mem[mem[pos + 3]] = mem[mem[pos + 1]] * mem[mem[pos + 2]],
            99 => return Ok(mem[0]),
            _ => return Err(()),
        };
    }
    Err(())
}

fn solve2(input: Vec<usize>, result: usize) -> Option<(usize, usize)> {
    iproduct!(0..100, 0..100)
        .find(|&(noun, verb)| solve1(&mut input.clone(), noun, verb) == Ok(result))
}

fn main() {
    let input = parse(&io::stdin().lock().lines().map(|x| x.unwrap()).collect());
    println!("Part1: {:?}", solve1(&mut input.clone(), 12, 02));
    println!("Part2: {:?}", solve2(input.clone(), 19690720));
}
