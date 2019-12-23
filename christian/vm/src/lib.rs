#[derive(Clone, Debug)]
pub struct Vm<T: Iterator<Item = i64>> {
    data: Vec<i64>,
    ip: usize,
    base: i64,
    input: T,
}

impl<T: Iterator<Item = i64>> Vm<T> {
    pub fn new(data: Vec<i64>, input: T) -> Self {
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

    pub fn single_step(&mut self) -> Option<Option<i64>> {
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
                return Some(Some(res));
            }
            05 => (self.jmp_if(self.get(1) != 0, self.get(2), 3), self.inc(0)),
            06 => (self.jmp_if(self.get(1) == 0, self.get(2), 3), self.inc(0)),
            07 => (self.set(3, (self.get(1) < self.get(2)) as i64), self.inc(4)),
            08 => (
                self.set(3, (self.get(1) == self.get(2)) as i64),
                self.inc(4),
            ),
            09 => (self.base += self.get(1), self.inc(2)),
            99 => return Some(None),
            xx => panic!("Unexptected opcode {}", xx),
        };
        None
    }
}

impl<T: Iterator<Item = i64>> Iterator for Vm<T> {
    type Item = i64;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(x) = self.single_step() {
                return x;
            }
        }
    }
}
