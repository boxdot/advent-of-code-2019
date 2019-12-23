#[macro_use]
extern crate scan_fmt;
use modinverse::modinverse;

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    DealWithIncrement(u64),
    DealIntoNewStack,
    Cut(i64),
}

pub fn parse_commands(input: &str) -> Vec<Command> {
    input
        .trim()
        .split('\n')
        .map(|line| {
            if let Ok(x) = scan_fmt!(line, "deal with increment {d}", u64) {
                Command::DealWithIncrement(x)
            } else if let Ok(x) = scan_fmt!(line, "cut {d}", i64) {
                Command::Cut(x)
            } else if line == "deal into new stack" {
                Command::DealIntoNewStack
            } else {
                panic!("Unknown command")
            }
        })
        .collect()
}

pub fn apply_commands(deck_size: u64, commands: &[Command]) -> Vec<u64> {
    let mut deck: Vec<_> = (0..deck_size).collect();

    for command in commands {
        match command {
            Command::DealWithIncrement(x) => {
                let mut new_deck = vec![0; deck_size as usize];
                for i in 0..deck_size {
                    new_deck[((i * x) % deck_size) as usize] = deck[i as usize];
                }
                deck = new_deck;
            }
            Command::DealIntoNewStack => {
                deck.reverse();
            }
            Command::Cut(x) => {
                if *x > 0 {
                    deck.rotate_left(*x as usize);
                } else {
                    deck.rotate_right(-*x as usize);
                }
            }
        }
    }

    deck
}

fn modmult(x: u64, mut y: u64, m: u64) -> u64 {
    if y == 0 {
        return 0;
    }

    let mut result = x;
    let mut remainder = 0;
    while y > 1 {
        if y % 2 == 1 {
            remainder = (remainder + result) % m;
            y -= 1;
        }
        result = (2 * result) % m;
        y = y / 2;
    }
    (result + remainder) % m
}

#[derive(Debug, Clone)]
struct Transformation(u64, u64);

fn transmodmult(x: &Transformation, y: &Transformation, m: u64) -> Transformation {
    Transformation(modmult(x.0, y.0, m), (modmult(x.0, y.1, m) + x.1) % m)
}

fn transmodpow(x: &Transformation, pow: u64, m: u64) -> Transformation {
    if pow == 0 {
        return Transformation(1, 0);
    }

    if pow == 1 {
        return x.clone();
    }

    if pow % 2 == 1 {
        transmodmult(x, &transmodpow(x, pow - 1, m), m)
    } else {
        let r = transmodpow(x, pow / 2, m);
        transmodmult(&r, &r, m)
    }
}

fn evaluate(transformation: &Transformation, value: u64, deck_size: u64) -> u64 {
    (modmult(transformation.0, value, deck_size) + transformation.1) % deck_size
}

fn inverse_command(deck_size: u64, command: &Command, x: &Transformation) -> Transformation {
    match *command {
        Command::DealIntoNewStack => Transformation(deck_size - x.0, deck_size - x.1 - 1),
        Command::Cut(k) => Transformation(x.0, (x.1 + (k + deck_size as i64) as u64) % deck_size),
        Command::DealWithIncrement(k) => Transformation(
            modmult(
                x.0,
                modinverse(k as i64, deck_size as i64).unwrap() as u64,
                deck_size,
            ),
            modmult(
                x.1,
                modinverse(k as i64, deck_size as i64).unwrap() as u64,
                deck_size,
            ),
        ),
    }
}

fn inverse_commands(deck_size: u64, commands: &[Command]) -> Transformation {
    let mut transformation = Transformation(1, 0);
    for command in commands.iter().rev() {
        transformation = inverse_command(deck_size, command, &transformation);
    }
    transformation
}

pub fn find_original_position(
    deck_size: u64,
    commands: &[Command],
    iterations: u64,
    position: u64,
) -> u64 {
    let mut x = inverse_commands(deck_size, commands);
    x = transmodpow(&x, iterations, deck_size);
    evaluate(&x, position, deck_size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test_1() {
        let input = "deal with increment 7
deal into new stack
deal into new stack";
        let result = vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7];
        assert_eq!(apply_commands(10, &parse_commands(input)), result,);
        for (index, &value) in result.iter().enumerate() {
            assert_eq!(
                find_original_position(10, &parse_commands(input), 2, index as u64),
                result[value as usize]
            );
        }
    }

    #[test]
    fn sample_test_2() {
        let input = "cut 6
deal with increment 7
deal into new stack";
        let result = vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6];
        assert_eq!(apply_commands(10, &parse_commands(input)), result);
        for (index, &value) in result.iter().enumerate() {
            assert_eq!(
                find_original_position(10, &parse_commands(input), 1, index as u64),
                value
            );
        }
        for (index, &value) in result.iter().enumerate() {
            assert_eq!(
                find_original_position(10, &parse_commands(input), 2, index as u64),
                result[value as usize]
            );
        }
    }

    #[test]
    fn sample_test_3() {
        let input = "deal with increment 7
deal with increment 9
cut -2";
        let result = vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9];
        assert_eq!(apply_commands(10, &parse_commands(input)), result);
        for (index, &value) in result.iter().enumerate() {
            assert_eq!(
                find_original_position(10, &parse_commands(input), 1, index as u64),
                value
            );
        }
        for (index, &value) in result.iter().enumerate() {
            assert_eq!(
                find_original_position(10, &parse_commands(input), 2, index as u64),
                result[value as usize]
            );
        }
    }

    #[test]
    fn sample_test_4() {
        let input = "deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1";
        let result = vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6];
        assert_eq!(apply_commands(10, &parse_commands(input)), result);
        for (index, &value) in result.iter().enumerate() {
            assert_eq!(
                find_original_position(10, &parse_commands(input), 1, index as u64),
                value
            );
        }
        for (index, &value) in result.iter().enumerate() {
            assert_eq!(
                find_original_position(10, &parse_commands(input), 2, index as u64),
                result[value as usize]
            );
        }
    }

    #[test]
    fn sample_test_5() {
        let input = "cut 2
deal into new stack
deal into new stack";
        assert_eq!(
            apply_commands(10, &parse_commands(input)),
            vec![2, 3, 4, 5, 6, 7, 8, 9, 0, 1]
        );
        for iter in 0..100 {
            assert_eq!(
                find_original_position(10, &parse_commands(input), iter, 0),
                (2 * iter) % 10
            );
        }
    }
}
