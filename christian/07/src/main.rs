use permutator::Permutation;
use std::io::{self, prelude::*};
use std::sync::mpsc::channel;

fn parse(input: &Vec<String>) -> Vec<i64> {
    let ints = input.iter().map(|x| x.split(',')).flatten();
    ints.filter_map(|x| x.parse().ok()).collect()
}

#[derive(Clone, Debug)]
struct Vm<T: Iterator<Item = i64>> {
    data: Vec<i64>,
    ip: usize,
    input: T,
}

impl<T: Iterator<Item = i64>> Vm<T> {
    fn new(data: Vec<i64>, input: T) -> Self {
        Self { data, ip: 0, input }
    }
    fn get(&self, arg: u32) -> i64 {
        if self.data[self.ip] / 10_i64.pow(arg + 1) % 10 == 0 {
            self.data[self.data[self.ip + arg as usize] as usize]
        } else {
            self.data[self.ip + arg as usize]
        }
    }

    fn set(&mut self, arg: usize, value: i64) {
        let pos = self.data[self.ip + arg] as usize;
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
                99 => return None,
                xx => panic!("Unexptected opcode {}", xx),
            };
        }
    }
}

fn solve1(mem: &Vec<i64>) -> i64 {
    let mut result = 0;
    vec![0, 1, 2, 3, 4].permutation().for_each(|perm| {
        let vm = Vm::new(mem.clone(), vec![perm[0], 0].into_iter());
        let vm = Vm::new(mem.clone(), std::iter::once(perm[1]).chain(vm));
        let vm = Vm::new(mem.clone(), std::iter::once(perm[2]).chain(vm));
        let vm = Vm::new(mem.clone(), std::iter::once(perm[3]).chain(vm));
        let mut vm = Vm::new(mem.clone(), std::iter::once(perm[4]).chain(vm));
        result = result.max(vm.next().unwrap());
    });
    result
}

fn solve2(mem: &Vec<i64>) -> i64 {
    let mut result = 0;
    vec![5, 6, 7, 8, 9].permutation().for_each(|perm| {
        let (sender, receiver) = channel();
        let vm = Vm::new(
            mem.clone(),
            vec![perm[0], 0].into_iter().chain(receiver.into_iter()),
        );
        let vm = Vm::new(mem.clone(), std::iter::once(perm[1]).chain(vm));
        let vm = Vm::new(mem.clone(), std::iter::once(perm[2]).chain(vm));
        let vm = Vm::new(mem.clone(), std::iter::once(perm[3]).chain(vm));
        let vm = Vm::new(mem.clone(), std::iter::once(perm[4]).chain(vm));
        let vm = vm.map(|item| {
            sender.send(item).unwrap();
            item
        });
        result = result.max(vm.last().unwrap());
    });
    result
}

fn main() {
    let input = parse(&io::stdin().lock().lines().map(|x| x.unwrap()).collect());
    println!("Part1: {:?}", solve1(&input.clone()));
    println!("Part2: {:?}", solve2(&input.clone()));
}
