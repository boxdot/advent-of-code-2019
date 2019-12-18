use std::convert::From;

#[derive(Debug)]
struct Memory {
    data: Vec<i64>,
    rel_base: i64,
}

impl Memory {
    pub fn getm(&self, index: usize, mode: Mode) -> i64 {
        let value = self.get(index);
        match mode {
            Mode::Position => self.get(value as usize),
            Mode::Immediate => value,
            Mode::Relative => self.get((self.rel_base + value) as usize),
        }
    }

    pub fn setm(&mut self, index: usize, mode: Mode, value: i64) {
        match mode {
            Mode::Position => self.set(self.get(index) as usize, value),
            Mode::Relative => self.set((self.get(index) + self.rel_base) as usize, value),
            Mode::Immediate => panic!("Invalid mode for setting"),
        }
    }

    fn get(&self, index: usize) -> i64 {
        if index >= self.data.len() {
            0
        } else {
            self.data[index]
        }
    }

    fn set(&mut self, index: usize, value: i64) {
        if index >= self.data.len() {
            self.data.resize(index + 1, 0);
        }
        self.data[index] = value;
    }
}

#[derive(Debug)]
enum Mode {
    Position,
    Immediate,
    Relative,
}

impl From<u32> for Mode {
    fn from(value: u32) -> Self {
        match value {
            0 => Mode::Position,
            1 => Mode::Immediate,
            2 => Mode::Relative,
            _ => panic!("Unknown mode"),
        }
    }
}

pub struct Program {
    mem: Memory,
    index: usize,
}

impl Program {
    pub fn new(mem: Vec<i64>) -> Self {
        Self {
            mem: Memory {
                data: mem,
                rel_base: 0,
            },
            index: 0,
        }
    }

    pub fn run<InputIterator>(&mut self, input: InputIterator) -> ProgramIter<InputIterator>
    where
        InputIterator: Iterator<Item = i64>,
    {
        ProgramIter {
            memory: &mut self.mem,
            index: &mut self.index,
            input: input,
        }
    }
}

pub struct ProgramIter<'a, InputIterator: Iterator<Item = i64>> {
    memory: &'a mut Memory,
    index: &'a mut usize,
    input: InputIterator,
}

impl<'a, InputIterator> Iterator for ProgramIter<'a, InputIterator>
where
    InputIterator: Iterator<Item = i64>,
{
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        loop {
            let digits: Vec<_> = format!("{:0>5}", self.memory.get(*self.index).to_string())
                .chars()
                .map(|d| d.to_digit(10).unwrap())
                .collect();
            let opcode = 10 * digits[3] + digits[4];
            let (mode1, mode2, mode3) = (digits[2], digits[1], digits[0]);

            match opcode {
                1 => {
                    self.memory.setm(
                        *self.index + 3,
                        mode3.into(),
                        self.memory.getm(*self.index + 1, mode1.into())
                            + self.memory.getm(*self.index + 2, mode2.into()),
                    );
                    *self.index += 4;
                }
                2 => {
                    self.memory.setm(
                        *self.index + 3,
                        mode3.into(),
                        self.memory.getm(*self.index + 1, mode1.into())
                            * self.memory.getm(*self.index + 2, mode2.into()),
                    );
                    *self.index += 4;
                }
                3 => match self.input.next() {
                    Some(value) => {
                        self.memory
                            .setm(*self.index + 1, mode1.into(), value);
                        *self.index += 2;
                    }
                    None => {
                        return None;
                    }
                },
                4 => {
                    *self.index += 2;
                    return Some(self.memory.getm(*self.index - 1, mode1.into()));
                }
                5 => {
                    if self.memory.getm(*self.index + 1, mode1.into()) != 0 {
                        *self.index = self.memory.getm(*self.index + 2, mode2.into()) as usize;
                    } else {
                        *self.index += 3;
                    }
                }
                6 => {
                    if self.memory.getm(*self.index + 1, mode1.into()) == 0 {
                        *self.index = self.memory.getm(*self.index + 2, mode2.into()) as usize;
                    } else {
                        *self.index += 3;
                    }
                }
                7 => {
                    self.memory.setm(
                        *self.index + 3,
                        mode3.into(),
                        if self.memory.getm(*self.index + 1, mode1.into())
                            < self.memory.getm(*self.index + 2, mode2.into())
                        {
                            1
                        } else {
                            0
                        },
                    );
                    *self.index += 4;
                }
                8 => {
                    self.memory.setm(
                        *self.index + 3,
                        mode3.into(),
                        if self.memory.getm(*self.index + 1, mode1.into())
                            == self.memory.getm(*self.index + 2, mode2.into())
                        {
                            1
                        } else {
                            0
                        },
                    );
                    *self.index += 4;
                }
                9 => {
                    self.memory.rel_base += self.memory.getm(*self.index + 1, mode1.into());
                    *self.index += 2;
                }
                99 => return None,
                _ => panic!("Unsupported opcode".to_string()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test_1() {
        assert!(itertools::equal(
            Program::new(vec![
                109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99
            ])
            .run(vec![0].into_iter()),
            vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
        ));
    }

    #[test]
    fn sample_test_2() {
        assert!(itertools::equal(
            Program::new(vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0]).run(vec![0].into_iter()),
            vec![1219070632396864]
        ));
    }

    #[test]
    fn sample_test_3() {
        assert!(itertools::equal(
            Program::new(vec![104, 1125899906842624, 99]).run(vec![0].into_iter()),
            vec![1125899906842624]
        ));
    }

    #[test]
    fn equal_to_8() {
        assert!(itertools::equal(
            Program::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]).run(vec![8].into_iter()),
            vec![1]
        ));
        assert!(itertools::equal(
            Program::new(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]).run(vec![8].into_iter()),
            vec![1]
        ));
        assert!(itertools::equal(
            Program::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]).run(vec![42].into_iter()),
            vec![0]
        ));
        assert!(itertools::equal(
            Program::new(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]).run(vec![42].into_iter()),
            vec![0]
        ));
    }

    #[test]
    fn less_than_8() {
        assert!(itertools::equal(
            Program::new(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]).run(vec![8].into_iter()),
            vec![0]
        ));
        assert!(itertools::equal(
            Program::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]).run(vec![8].into_iter()),
            vec![0]
        ));

        assert!(itertools::equal(
            Program::new(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]).run(vec![42].into_iter()),
            vec![0]
        ));
        assert!(itertools::equal(
            Program::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]).run(vec![42].into_iter()),
            vec![0]
        ));

        assert!(itertools::equal(
            Program::new(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]).run(vec![4].into_iter()),
            vec![1]
        ));
        assert!(itertools::equal(
            Program::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]).run(vec![4].into_iter()),
            vec![1]
        ));
    }

    #[test]
    fn jump() {
        assert!(itertools::equal(
            Program::new(vec![
                3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9
            ])
            .run(vec![0].into_iter()),
            vec![0]
        ));
        assert!(itertools::equal(
            Program::new(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1])
                .run(vec![0].into_iter()),
            vec![0]
        ));

        assert!(itertools::equal(
            Program::new(vec![
                3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9
            ])
            .run(vec![42].into_iter()),
            vec![1]
        ));
        assert!(itertools::equal(
            Program::new(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1])
                .run(vec![42].into_iter()),
            vec![1]
        ));
    }
}
