use std::io::{self, prelude::*};

fn parse(input: &Vec<String>) -> Mem {
    let ints = input.iter().map(|x| x.split(',')).flatten();
    Mem {
        data: ints.filter_map(|x| x.parse().ok()).collect(),
        ip: 0,
    }
}

#[derive(Clone, Debug)]
struct Mem {
    data: Vec<i64>,
    ip: usize,
}

impl Mem {
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
}

fn solve1(mem: &mut Mem, input: Vec<i64>) -> Result<Vec<i64>, ()> {
    let mut stdin = input.iter();
    let mut result = Vec::new();
    loop {
        mem.ip += match mem.data[mem.ip] % 100 {
            01 => (mem.set(3, mem.get(1) + mem.get(2)), 4),
            02 => (mem.set(3, mem.get(1) * mem.get(2)), 4),
            03 => (mem.set(1, *stdin.next().unwrap()), 2),
            04 => (result.push(mem.get(1)), 2),
            05 => (mem.jmp_if(mem.get(1) != 0, mem.get(2), 3), 0),
            06 => (mem.jmp_if(mem.get(1) == 0, mem.get(2), 3), 0),
            07 => (mem.set(3, (mem.get(1) < mem.get(2)) as i64), 4),
            08 => (mem.set(3, (mem.get(1) == mem.get(2)) as i64), 4),
            99 => return Ok(result),
            _ => return Err(()),
        }
        .1;
    }
}

fn main() {
    let input = parse(&io::stdin().lock().lines().map(|x| x.unwrap()).collect());
    println!("Part1: {:?}", solve1(&mut input.clone(), vec![1]));
    println!("Part2: {:?}", solve1(&mut input.clone(), vec![5]));
}
