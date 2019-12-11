use core::cell::RefCell;
use itertools::*;
use std::collections::HashMap;
use std::io::{self, prelude::*};

fn parse(input: &Vec<String>) -> Vec<i64> {
    let ints = input.iter().map(|x| x.split(',')).flatten();
    ints.filter_map(|x| x.parse().ok()).collect()
}

#[derive(Clone, Debug)]
struct Vm<T: Iterator<Item = i64>> {
    data: Vec<i64>,
    ip: usize,
    base: i64,
    input: T,
}

impl<T: Iterator<Item = i64>> Vm<T> {
    fn new(data: Vec<i64>, input: T) -> Self {
        Self {
            data,
            ip: 0,
            base: 0,
            input,
        }
    }

    fn arg_pos(&self, arg: u32) -> usize {
        match self.data[self.ip] / 10_i64.pow(arg + 1) % 10 {
            0 => self.data[self.ip + arg as usize] as usize,
            1 => self.ip + arg as usize,
            2 => (self.data[self.ip + arg as usize] + self.base) as usize,
            m => panic!("Unknown mode {}", m),
        }
    }

    fn get(&self, arg: u32) -> i64 {
        *self.data.get(self.arg_pos(arg)).unwrap_or(&0)
    }

    fn set(&mut self, arg: u32, value: i64) {
        let pos = self.arg_pos(arg);
        if pos >= self.data.len() {
            self.data.resize(pos + 1, 0);
        }
        self.data[pos] = value;
    }

    fn jmp_if(&mut self, cond: bool, a: i64, op_size: usize) {
        if cond {
            self.ip = a as usize;
        } else {
            self.ip += op_size;
        }
    }

    fn inc(&mut self, num: usize) {
        self.ip += num;
    }
}

impl<T: Iterator<Item = i64>> Iterator for Vm<T> {
    type Item = i64;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.data[self.ip] % 100 {
                01 => (self.set(3, self.get(1) + self.get(2)), self.inc(4)),
                02 => (self.set(3, self.get(1) * self.get(2)), self.inc(4)),
                03 => {
                    let input = self.input.next()?;
                    (self.set(1, input), self.inc(2))
                }
                04 => {
                    let res = self.get(1);
                    self.inc(2);
                    return Some(res);
                }
                05 => (self.jmp_if(self.get(1) != 0, self.get(2), 3), self.inc(0)),
                06 => (self.jmp_if(self.get(1) == 0, self.get(2), 3), self.inc(0)),
                07 => (self.set(3, (self.get(1) < self.get(2)) as i64), self.inc(4)),
                08 => (
                    self.set(3, (self.get(1) == self.get(2)) as i64),
                    self.inc(4),
                ),
                09 => (self.base += self.get(1), self.inc(2)),
                99 => return None,
                xx => panic!("Unexptected opcode {}", xx),
            };
        }
    }
}

fn solve(mem: &Vec<i64>, initial_value: i64) -> (usize, String) {
    let hull: RefCell<HashMap<(i64, i64), i64>> = RefCell::new(Default::default());
    hull.borrow_mut().insert((0, 0), initial_value);
    let pos = RefCell::new((0, 0));
    let mut dir = (0, -1);
    let input = || Some(*hull.borrow().get(&pos.borrow()).unwrap_or(&0));
    for (color, turn) in Vm::new(mem.clone(), std::iter::from_fn(input)).tuples() {
        let prev_pos = *pos.borrow();
        hull.borrow_mut().insert(prev_pos, color);
        match turn {
            0 => dir = (dir.1, -dir.0),
            1 => dir = (-dir.1, dir.0),
            e => panic!("{:?}", e),
        }
        *pos.borrow_mut() = (prev_pos.0 + dir.0, prev_pos.1 + dir.1);
    }

    let hull = hull.into_inner();
    let y_range = hull.keys().map(|(_, y)| *y).minmax().into_option().unwrap();
    let x_range = hull.keys().map(|(x, _)| *x).minmax().into_option().unwrap();
    let append = |mut res: String, (y, x)| {
        let value = *hull.get(&(x, y)).unwrap_or(&0);
        res.push(if value == 0 { ' ' } else { '*' });
        res.push(if x == x_range.1 { '\n' } else { '‎' });
        res
    };
    let pic = iproduct!(y_range.0..=y_range.1, (x_range.0..=x_range.1)).fold(String::new(), append);
    (hull.len(), pic)
}

fn main() {
    let input = parse(&io::stdin().lock().lines().map(|x| x.unwrap()).collect());
    println!("Part1:\n{}", solve(&input, 0).0);
    println!("Part2:\n{}", solve(&input, 1).1);
}
