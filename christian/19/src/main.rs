use itertools::*;

fn program() -> Vec<i64> {
    vec![
        109, 424, 203, 1, 21101, 11, 0, 0, 1106, 0, 282, 21101, 0, 18, 0, 1105, 1, 259, 1201, 1, 0,
        221, 203, 1, 21102, 1, 31, 0, 1105, 1, 282, 21102, 38, 1, 0, 1106, 0, 259, 21002, 23, 1, 2,
        22101, 0, 1, 3, 21102, 1, 1, 1, 21102, 1, 57, 0, 1106, 0, 303, 1201, 1, 0, 222, 21002, 221,
        1, 3, 20101, 0, 221, 2, 21101, 0, 259, 1, 21101, 80, 0, 0, 1106, 0, 225, 21102, 1, 33, 2,
        21102, 91, 1, 0, 1106, 0, 303, 1202, 1, 1, 223, 20101, 0, 222, 4, 21101, 0, 259, 3, 21101,
        0, 225, 2, 21102, 225, 1, 1, 21101, 0, 118, 0, 1106, 0, 225, 20102, 1, 222, 3, 21101, 0,
        22, 2, 21101, 133, 0, 0, 1105, 1, 303, 21202, 1, -1, 1, 22001, 223, 1, 1, 21102, 148, 1, 0,
        1106, 0, 259, 1201, 1, 0, 223, 21002, 221, 1, 4, 20102, 1, 222, 3, 21102, 5, 1, 2, 1001,
        132, -2, 224, 1002, 224, 2, 224, 1001, 224, 3, 224, 1002, 132, -1, 132, 1, 224, 132, 224,
        21001, 224, 1, 1, 21102, 1, 195, 0, 105, 1, 108, 20207, 1, 223, 2, 20101, 0, 23, 1, 21102,
        -1, 1, 3, 21101, 0, 214, 0, 1106, 0, 303, 22101, 1, 1, 1, 204, 1, 99, 0, 0, 0, 0, 109, 5,
        2102, 1, -4, 249, 22101, 0, -3, 1, 21201, -2, 0, 2, 21202, -1, 1, 3, 21101, 250, 0, 0,
        1105, 1, 225, 22101, 0, 1, -4, 109, -5, 2105, 1, 0, 109, 3, 22107, 0, -2, -1, 21202, -1, 2,
        -1, 21201, -1, -1, -1, 22202, -1, -2, -2, 109, -3, 2105, 1, 0, 109, 3, 21207, -2, 0, -1,
        1206, -1, 294, 104, 0, 99, 22101, 0, -2, -2, 109, -3, 2106, 0, 0, 109, 5, 22207, -3, -4,
        -1, 1206, -1, 346, 22201, -4, -3, -4, 21202, -3, -1, -1, 22201, -4, -1, 2, 21202, 2, -1,
        -1, 22201, -4, -1, 1, 22102, 1, -2, 3, 21102, 1, 343, 0, 1106, 0, 303, 1105, 1, 415, 22207,
        -2, -3, -1, 1206, -1, 387, 22201, -3, -2, -3, 21202, -2, -1, -1, 22201, -3, -1, 3, 21202,
        3, -1, -1, 22201, -3, -1, 2, 21201, -4, 0, 1, 21101, 0, 384, 0, 1105, 1, 303, 1106, 0, 415,
        21202, -4, -1, -4, 22201, -4, -3, -4, 22202, -3, -2, -2, 22202, -2, -4, -4, 22202, -3, -2,
        -3, 21202, -4, -1, -2, 22201, -3, -2, 1, 22101, 0, 1, -4, 109, -5, 2106, 0, 0,
    ]
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

fn test((x, y): (i64, i64)) -> i64 {
    Vm::new(program(), [x, y].iter().copied()).next().unwrap()
}

fn solve1() -> i64 {
    iproduct!(0..50, 0..50).map(test).sum()
}

fn solve2(spread_50_50: i64) -> (i64, i64) {
    let max = 50 * 100 / (spread_50_50 / 50);
    let first_y = |x| (max / 2..).filter(|y| test((x, *y)) == 1).next().unwrap();
    let (mut base, mut size) = (max / 2, max - max / 2);
    while size > 1 {
        let (half, mid) = (size / 2, base + size / 2);
        if test((mid - 99, first_y(mid) + 99)) == 0 {
            size -= half + 1;
            base = mid + 1;
        } else {
            size -= half;
        };
    }
    while test((base - 99, first_y(base) + 99)) == 0 {
        base -= 1; // correct for non-sorted too_small test;
    }
    (base - 99, first_y(base))
}

fn main() {
    let p1 = solve1();
    println!("Part1:\n{}", p1);
    println!("Part2:\n{:?}", solve2(p1));
}
