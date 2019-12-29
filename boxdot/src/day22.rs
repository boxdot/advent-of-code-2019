#![allow(clippy::unreadable_literal)]

use num_integer::Integer;

pub fn solve(input: &str) -> Result<(i64, i128)> {
    let ops = parse(input)?;

    let part1 = {
        const N: i64 = 10007;
        let (a, b) = ops.iter().fold((1, 0), |(a, b), op| op.coeff(a, b, N));
        ((a * 2019).rem_euclid(N) + b).rem_euclid(N)
    };

    let part2 = {
        const N: i128 = 119315717514047;
        let (a, b) = ops
            .iter()
            .fold((1, 0), |(a, b), op| op.coeff(a, b, N as i64));
        let a = a as i128;
        let b = b as i128;

        const P: i128 = 101741582076661;
        // P-times a*x + b applied yields:
        // 2020 = a^p * x + b * (a^p - 1)/(a - 1), since (a_(p-1) + ... + 1)(a - 1) = (a^p - 1).

        let a_p = mod_pow(a, P, N);
        let e = (a - 1).extended_gcd(&N);
        let a_min_one_inv = e.x;

        let e = a_p.extended_gcd(&N);
        let a_p_inv = e.x;

        let b_prime = ((b * (a_p - 1)).rem_euclid(N) * a_min_one_inv).rem_euclid(N);
        ((2020 - b_prime).rem_euclid(N) * a_p_inv).rem_euclid(N)
    };

    Ok((part1, part2))
}

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

fn parse(input: &str) -> Result<Vec<Op>> {
    input
        .lines()
        .map(|l| {
            let op = if l.starts_with("deal into new stack") {
                Op::DealIntoNewStack
            } else if l.starts_with("deal with increment") {
                let n = l.trim_start_matches("deal with increment ").parse()?;
                Op::DealWithIncrement(n)
            } else if l.starts_with("cut") {
                let n = l.trim_start_matches("cut ").parse()?;
                Op::Cut(n)
            } else {
                panic!("unexpected op: {}", l);
            };
            Ok(op)
        })
        .collect()
}

#[derive(Debug, Clone, Copy)]
enum Op {
    DealWithIncrement(usize),
    DealIntoNewStack,
    Cut(isize),
}

impl Op {
    // transforms function a*x + b by applying op to it
    fn coeff(self, a: i64, b: i64, n: i64) -> (i64, i64) {
        match self {
            Self::DealIntoNewStack => {
                let a = (-a).rem_euclid(n);
                let b = (-1_i64 - b).rem_euclid(n);
                (a, b)
            }
            Self::Cut(i) => {
                let b = (b - i as i64).rem_euclid(n);
                (a, b)
            }
            Self::DealWithIncrement(i) => {
                let a = (a * i as i64).rem_euclid(n);
                let b = (b * i as i64).rem_euclid(n);
                (a, b)
            }
        }
    }
}

fn mod_pow<T: Copy + Integer>(mut base: T, mut exp: T, modulus: T) -> T {
    let two = T::one() + T::one();
    if modulus == T::one() {
        return T::zero();
    }
    let mut result = T::one();
    base = base % modulus;
    while exp > T::zero() {
        if exp % two == T::one() {
            result = result * base % modulus;
        }
        exp = exp / two;
        base = base * base % modulus
    }
    result
}
