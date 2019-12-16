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
        for length in 1.max(skip)..=number.len() {
            let mut pos = length - 1;
            let mut sum = 0;
            for mul in [1, -1].iter().cycle() {
                sum += mul * (prefix[(pos + length).min(number.len())] - prefix[pos]);
                pos += 2 * length;
                if pos >= number.len() {
                    break;
                }
            }
            next[length - 1] = (sum.abs() % 10) as u8;
        }
        std::mem::swap(&mut next, &mut number);
    }
    (number.iter().skip(skip).take(8).map(|x| (x + b'0') as char)).collect()
}

fn main() {
    let input = parse(&io::stdin().lock().lines().map(|x| x.unwrap()).collect());
    println!("{:?}", solve(input.clone(), 0));
    let big = (input.iter().cycle().copied().take(input.len() * 10000)).collect();
    let pos = input.iter().take(7).fold(0, |x, d| x * 10 + *d as usize);
    println!("{:?}", solve(big, pos));
}
