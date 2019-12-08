use itertools::Itertools;

enum Mode {
    Position,
    Immediate,
}

struct Argument<'a> {
    pub value: i64,
    pub mode: Mode,
    program: &'a [i64],
}

impl<'a> Argument<'a> {
    pub fn new(program: &'a [i64], value: i64, mode_num: u32) -> Self {
        Self {
            value: value,
            mode: if mode_num == 0 {
                Mode::Position
            } else {
                Mode::Immediate
            },
            program: program,
        }
    }

    pub fn get(&self) -> i64 {
        match self.mode {
            Mode::Position => self.program[self.value as usize],
            Mode::Immediate => self.value,
        }
    }
}

struct Program {
    mem: Vec<i64>,
    index: usize,
}

impl Program {
    fn new(mem: Vec<i64>) -> Self {
        Self { mem, index: 0 }
    }

    pub fn run<InputIterator>(&mut self, input: InputIterator) -> ProgramIter<InputIterator>
    where
        InputIterator: Iterator<Item = i64>,
    {
        ProgramIter {
            program: &mut self.mem,
            index: &mut self.index,
            input: input,
        }
    }
}

struct ProgramIter<'a, InputIterator: Iterator<Item = i64>> {
    program: &'a mut [i64],
    index: &'a mut usize,
    input: InputIterator,
}

impl<'a, InputIterator> Iterator for ProgramIter<'a, InputIterator>
where
    InputIterator: Iterator<Item = i64>,
{
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        while *self.index < self.program.len() {
            let digits: Vec<_> = format!("{:0>5}", self.program[*self.index].to_string())
                .chars()
                .map(|d| d.to_digit(10).unwrap())
                .collect();
            let opcode = 10 * digits[3] + digits[4];
            let (mode1, mode2, _) = (digits[2], digits[1], digits[0]);

            match opcode {
                1 if *self.index + 3 < self.program.len()
                    && self.program[*self.index + 3] >= 0
                    && (self.program[*self.index + 3] as usize) < self.program.len() =>
                {
                    self.program[self.program[*self.index + 3] as usize] =
                        Argument::new(&self.program, self.program[*self.index + 1], mode1).get()
                            + Argument::new(&self.program, self.program[*self.index + 2], mode2)
                                .get();
                    *self.index += 4;
                }
                2 if *self.index + 3 < self.program.len()
                    && self.program[*self.index + 3] >= 0
                    && (self.program[*self.index + 3] as usize) < self.program.len() =>
                {
                    self.program[self.program[*self.index + 3] as usize] =
                        Argument::new(&self.program, self.program[*self.index + 1], mode1).get()
                            * Argument::new(&self.program, self.program[*self.index + 2], mode2)
                                .get();
                    *self.index += 4;
                }
                3 if *self.index + 1 < self.program.len() => match self.input.next() {
                    Some(value) => {
                        self.program[self.program[*self.index + 1] as usize] = value;
                        *self.index += 2;
                    }
                    None => {
                        return None;
                    }
                },
                4 if *self.index + 1 < self.program.len() => {
                    *self.index += 2;
                    return Some(
                        Argument::new(&self.program, self.program[*self.index - 1], mode1).get(),
                    );
                }
                5 if *self.index + 2 < self.program.len() => {
                    if Argument::new(&self.program, self.program[*self.index + 1], mode1).get() != 0
                    {
                        *self.index = Argument::new(
                            &self.program,
                            self.program[*self.index + 2],
                            mode2,
                        )
                        .get() as usize;
                    } else {
                        *self.index += 3;
                    }
                }
                6 if *self.index + 2 < self.program.len() => {
                    if Argument::new(&self.program, self.program[*self.index + 1], mode1).get() == 0
                    {
                        *self.index = Argument::new(
                            &self.program,
                            self.program[*self.index + 2],
                            mode2,
                        )
                        .get() as usize;
                    } else {
                        *self.index += 3;
                    }
                }
                7 if *self.index + 3 < self.program.len()
                    && self.program[*self.index + 3] >= 0
                    && (self.program[*self.index + 3] as usize) < self.program.len() =>
                {
                    self.program[self.program[*self.index + 3] as usize] =
                        if Argument::new(&self.program, self.program[*self.index + 1], mode1).get()
                            < Argument::new(&self.program, self.program[*self.index + 2], mode2)
                                .get()
                        {
                            1
                        } else {
                            0
                        };
                    *self.index += 4;
                }
                8 if *self.index + 3 < self.program.len()
                    && self.program[*self.index + 3] >= 0
                    && (self.program[*self.index + 3] as usize) < self.program.len() =>
                {
                    self.program[self.program[*self.index + 3] as usize] =
                        if Argument::new(&self.program, self.program[*self.index + 1], mode1).get()
                            == Argument::new(&self.program, self.program[*self.index + 2], mode2)
                                .get()
                        {
                            1
                        } else {
                            0
                        };
                    *self.index += 4;
                }
                99 => return None,
                _ => panic!("Unsupported opcode".to_string()),
            }
        }
        panic!("Something went wrong");
    }
}

pub fn maximize_output(program: &[i64], num_copies: usize) -> Option<i64> {
    (0..num_copies)
        .permutations(num_copies)
        .map(|permutation| {
            let mut input = vec![0];

            for phase_input in permutation.into_iter() {
                input = Program::new(program.into())
                    .run(
                        vec![phase_input as i64]
                            .into_iter()
                            .chain(input.into_iter()),
                    )
                    .collect();
            }

            input.last().cloned()
        })
        .flatten()
        .max()
}

pub fn maximize_output_with_feedback(
    program: &[i64],
    start_phase: usize,
    num_copies: usize,
) -> Option<i64> {
    (start_phase..start_phase + num_copies)
        .permutations(num_copies)
        .map(|permutation| {
            let mut programs = vec![];

            for phase_input in permutation.into_iter() {
                programs.push(Program::new(program.into()));
                let mut it = programs
                    .last_mut()
                    .unwrap()
                    .run(vec![phase_input as i64].into_iter());
                assert_eq!(it.next(), None);
            }

            let mut input = 0;
            let mut output = None;
            let mut program_index = 0;

            loop {
                match programs[program_index].run(vec![input].into_iter()).next() {
                    Some(value) => {
                        input = value;
                        if program_index + 1 == programs.len() {
                            output = Some(value);
                            program_index = 0;
                        } else {
                            program_index += 1;
                        }
                    }
                    None => {
                        return output;
                    }
                }
            }
        })
        .flatten()
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test_1() {
        assert_eq!(
            maximize_output(
                &vec![3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0],
                5
            ),
            Some(43210)
        );
    }

    #[test]
    fn sample_test_2() {
        assert_eq!(
            maximize_output(
                &vec![
                    3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23,
                    23, 4, 23, 99, 0, 0
                ],
                5
            ),
            Some(54321)
        );
    }

    #[test]
    fn sample_test_3() {
        assert_eq!(
            maximize_output(
                &vec![
                    3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7,
                    33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
                ],
                5
            ),
            Some(65210)
        );
    }

    #[test]
    fn sample_test_4() {
        assert_eq!(
            maximize_output_with_feedback(
                &vec![
                    3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001,
                    28, -1, 28, 1005, 28, 6, 99, 0, 0, 5
                ],
                5,
                5
            ),
            Some(139629729)
        );
    }

    #[test]
    fn sample_test_5() {
        assert_eq!(
            maximize_output_with_feedback(
                &vec![
                    3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26,
                    1001, 54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55,
                    2, 53, 55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10
                ],
                5,
                5
            ),
            Some(18216)
        );
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
