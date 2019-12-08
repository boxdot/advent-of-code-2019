use itertools::*;
use std::io::{self, prelude::*};

fn parse(input: &Vec<String>) -> Vec<u8> {
    let ints = input.iter().flat_map(|x| x.bytes());
    ints.map(|x| x - b'0').collect()
}

fn solve1(input: &Vec<u8>) -> usize {
    let layers = input.chunks(25 * 6);
    let count = |x: u8| move |l: &&[u8]| l.iter().filter(move |&&y| y == x).count();
    let best = layers.min_by_key(count(0)).unwrap();
    count(1)(&best) * count(2)(&best)
}

fn solve2(input: &Vec<u8>) -> String {
    let mut result = String::new();
    for y in 0..6 {
        for x in 0..25 {
            let pixels = input.iter().skip(x + 25 * y).step_by(25 * 6);
            result.push((b' ' + pixels.filter(|&&p| p != 2).next().unwrap()) as char);
        }
        result.push('\n');
    }
    result
}

fn main() {
    let input = parse(&io::stdin().lock().lines().map(|x| x.unwrap()).collect());
    println!("Part1:\n{}", solve1(&input.clone()));
    println!("Part2:\n{}", solve2(&input.clone()));
}
