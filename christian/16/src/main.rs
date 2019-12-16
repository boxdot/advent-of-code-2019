// use itertools::*;
use std::io::{self, prelude::*};

fn parse(input: &Vec<String>) -> Vec<u8> {
    input.first().unwrap().bytes().map(|x| x - b'0').collect()
}

fn solve(mut number: Vec<u8>, skip: usize) -> String {
    let mut prefix = vec![0_i32; number.len() + 1];
    let mut next = number.clone();
    for _ in 0..100 {
        for (i, x) in number.iter().enumerate() {
            prefix[i + 1] = prefix[i] + *x as i32;
        }
        for i in 0..number.len() {
            let mut sum = 0;
            for pos in (i..prefix.len()).step_by(2 * (i + 1 + skip)) {
                sum = -sum + prefix[(pos + i + 1 + skip).min(number.len())] - prefix[pos];
            }
            next[i] = (sum.abs() % 10) as u8;
        }
        std::mem::swap(&mut next, &mut number);
    }
    (number.iter().take(8).map(|x| (x + b'0') as char)).collect()
}

fn main() {
    let input = parse(&io::stdin().lock().lines().map(|x| x.unwrap()).collect());
    println!("{:?}", solve(input.clone(), 0));
    let pos = input.iter().take(7).fold(0, |x, d| x * 10 + *d as usize);
    let offset_input = input.iter().cycle().skip(pos % input.len());
    let big = (offset_input.copied().take(input.len() * 10000 - pos)).collect();
    println!("{:?}", solve(big, pos));
}
