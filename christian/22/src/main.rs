use std::io::{self, prelude::*};
use text_io::*;

#[derive(Debug, Clone)]
enum Instruction {
    Inc(isize),
    New,
    Cut(isize),
}

fn parse(input: &Vec<String>) -> Vec<Instruction> {
    let result: Result<Vec<Instruction>, Error> = input
        .iter()
        .map(|line| {
            let as_inc = || -> Result<Instruction, Error> {
                let i: isize;
                try_scan!(line.bytes() => "deal with increment {}!", i);
                Ok(Instruction::Inc(i))
            };
            let as_new = || -> Result<Instruction, Error> {
                match line.as_str() {
                    "deal into new stack" => Ok(Instruction::New),
                    _ => Err(Error::MissingMatch),
                }
            };
            let as_cut = || -> Result<Instruction, Error> {
                let i: isize;
                try_scan!(line.bytes() => "cut {}!", i);
                Ok(Instruction::Cut(i))
            };
            as_inc().or(as_new()).or(as_cut())
        })
        .collect();
    result.unwrap()
}

fn solve1((n, instructions): (isize, Vec<Instruction>)) -> Option<isize> {
    let mut pos = 2019;
    for instruction in &instructions {
        match instruction {
            Instruction::New => pos = n - 1 - pos,
            Instruction::Cut(i) => pos = (pos - i).rem_euclid(n),
            Instruction::Inc(i) => pos = (pos * i).rem_euclid(n),
        }
    }
    Some(pos)
}

fn inverse(a: isize, n: isize) -> isize {
    let mut t = (0, 1);
    let mut r = (n, a);
    while r.1 != 0 {
        let quotient = r.0 / r.1;
        t = (t.1, t.0 - quotient * t.1);
        r = (r.1, r.0 - quotient * r.1);
    }
    assert!(r.0 <= 1);
    t.0.rem_euclid(n)
}

fn apply(coefficients: &mut (i128, i128), formula_coeffience: (i128, i128), n: isize) {
    *coefficients = (
        (coefficients.0 * formula_coeffience.0).rem_euclid(n as i128),
        (coefficients.1 * formula_coeffience.0 + formula_coeffience.1).rem_euclid(n as i128),
    );
}

fn solve2((n, instructions): (isize, Vec<Instruction>)) -> Option<i128> {
    let mut coefficients = (1_i128, 0_i128); // x * {} + {}
    for instruction in instructions.iter().rev() {
        match instruction {
            Instruction::New => apply(&mut coefficients, (-1, -1), n),
            Instruction::Cut(i) => apply(&mut coefficients, (1, *i as i128), n),
            Instruction::Inc(i) => apply(&mut coefficients, (inverse(*i, n) as i128, 0), n),
        }
    }
    let mut repetitions = 101_741_582_076_661_isize;
    let mut final_coefficients = (1_i128, 0_i128);
    while repetitions != 0 {
        if repetitions % 2 == 1 {
            apply(&mut final_coefficients, coefficients, n);
        }
        let formula = coefficients;
        apply(&mut coefficients, formula, n);
        repetitions /= 2;
    }
    Some((2020 * final_coefficients.0 + final_coefficients.1).rem_euclid(n as i128))
}

fn main() {
    let input = parse(&io::stdin().lock().lines().map(|x| x.unwrap()).collect());
    println!("{:?}", solve1((10007, input.clone())));
    println!("{:?}", solve2((119315717514047, input.clone())));
}
