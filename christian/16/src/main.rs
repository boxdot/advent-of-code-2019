use std::io::{self, prelude::*};

fn parse(input: &Vec<String>) -> Vec<i32> {
    (input.first().unwrap().bytes().map(|x| (x - b'0') as i32)).collect()
}

fn solve(mut number: Vec<i32>, skip: usize) -> String {
    number.resize(number.len() + 1, 0); // make space for prefix sum's last element
    for _ in 0..100 {
        (number.iter_mut()).fold(0, |sum, x| sum + std::mem::replace(x, sum));
        let last = *number.last().unwrap();
        for i in 0..number.len() {
            let mut sum = 0;
            for pos in (i..number.len()).step_by(2 * (i + 1 + skip)) {
                sum = -sum + number.get(pos + i + 1 + skip).unwrap_or(&last) - number[pos];
            }
            number[i] = sum.abs() % 10;
        }
    }
    (number.iter().take(8).map(|x| (*x as u8 + b'0') as char)).collect()
}

fn main() {
    let input = parse(&io::stdin().lock().lines().map(|x| x.unwrap()).collect());
    println!("{:?}", solve(input.clone(), 0));
    let pos = input.iter().take(7).fold(0, |x, d| x * 10 + *d as usize);
    let offset_input = input.iter().cycle().skip(pos % input.len());
    let big = (offset_input.copied().take(input.len() * 10000 - pos)).collect();
    println!("{:?}", solve(big, pos));
}
