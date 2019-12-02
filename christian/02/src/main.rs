use itertools::*;
use std::convert::TryInto;
use std::io::{self, prelude::*};

fn parse(input: &Vec<String>) -> Vec<usize> {
    let ints = input.iter().map(|x| x.split(',')).flatten();
    let mut result: Vec<_> = ints.filter_map(|x| x.parse().ok()).collect();
    result.resize((result.len() + 3) / 4 * 4, 0);
    result
}

fn solve1(mut input: Vec<usize>, noun: usize, verb: usize) -> Result<usize, ()> {
    input[1] = noun;
    input[2] = verb;
    for pos in (0..input.len()).step_by(4) {
        let data: [usize; 4] = input[pos..pos + 4].try_into().unwrap();
        match data[0] {
            01 => input[data[3]] = input[data[1]] + input[data[2]],
            02 => input[data[3]] = input[data[1]] * input[data[2]],
            99 => break,
            _ => return Err(()),
        };
    }
    Ok(input[0])
}

fn solve2(input: Vec<usize>, result: usize) -> Result<usize, ()> {
    for (noun, verb) in iproduct!(0..100, 0..100) {
        if solve1(input.clone(), noun, verb) == Ok(result) {
            return Ok(noun * 100 + verb);
        }
    }
    Err(())
}

fn main() {
    let input = parse(&io::stdin().lock().lines().map(|x| x.unwrap()).collect());
    println!("Part1: {:?}", solve1(input.clone(), 12, 02));
    println!("Part1: {:?}", solve2(input.clone(), 19690720));
}
