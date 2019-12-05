enum Mode {
    Position,
    Immediate,
}

struct Position<'a> {
    pub value: i64,
    pub mode: Mode,
    program: &'a [i64],
}

impl<'a> Position<'a> {
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

pub fn apply_program(program: &mut [i64], input: i64) -> Result<i64, String> {
    let mut index = 0;
    let mut output = None;
    while index < program.len() {
        let digits: Vec<_> = format!("{:0>5}", program[index].to_string())
            .chars()
            .map(|d| d.to_digit(10).unwrap())
            .collect();
        let opcode = 10 * digits[3] + digits[4];
        let (mode1, mode2, _) = (digits[2], digits[1], digits[0]);

        match opcode {
            1 if index + 3 < program.len()
                && program[index + 3] >= 0
                && (program[index + 3] as usize) < program.len() =>
            {
                program[program[index + 3] as usize] =
                    Position::new(&program, program[index + 1], mode1).get()
                        + Position::new(&program, program[index + 2], mode2).get();
                index += 4;
            }
            2 if index + 3 < program.len()
                && program[index + 3] >= 0
                && (program[index + 3] as usize) < program.len() =>
            {
                program[program[index + 3] as usize] =
                    Position::new(&program, program[index + 1], mode1).get()
                        * Position::new(&program, program[index + 2], mode2).get();
                index += 4;
            }
            3 if index + 1 < program.len() => {
                program[program[index + 1] as usize] = input;
                index += 2;
            }
            4 if index + 1 < program.len() => {
                match output {
                    Some(value) if value > 0 => {
                        return Err("Diagnostic error".to_string());
                    }
                    _ => (),
                };
                output = Some(Position::new(&program, program[index + 1], mode1).get());
                index += 2;
            }
            5 if index + 2 < program.len() => {
                if Position::new(&program, program[index + 1], mode1).get() != 0 {
                    index = Position::new(&program, program[index + 2], mode2).get() as usize;
                } else {
                    index += 3;
                }
            }
            6 if index + 2 < program.len() => {
                if Position::new(&program, program[index + 1], mode1).get() == 0 {
                    index = Position::new(&program, program[index + 2], mode2).get() as usize;
                } else {
                    index += 3;
                }
            }
            7 if index + 3 < program.len()
                && program[index + 3] >= 0
                && (program[index + 3] as usize) < program.len() =>
            {
                program[program[index + 3] as usize] =
                    if Position::new(&program, program[index + 1], mode1).get()
                        < Position::new(&program, program[index + 2], mode2).get()
                    {
                        1
                    } else {
                        0
                    };
                index += 4;
            }
            8 if index + 3 < program.len()
                && program[index + 3] >= 0
                && (program[index + 3] as usize) < program.len() =>
            {
                program[program[index + 3] as usize] =
                    if Position::new(&program, program[index + 1], mode1).get()
                        == Position::new(&program, program[index + 2], mode2).get()
                    {
                        1
                    } else {
                        0
                    };
                index += 4;
            }
            99 if output.is_some() => return Ok(output.unwrap()),
            _ => return Err("Unsupported opcode".to_string()),
        }
    }
    Err("No halt".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn io_params() {
        assert_eq!(apply_program(&mut vec![3, 0, 4, 0, 99], 42), Ok(42));
    }

    #[test]
    fn param_mode() {
        let mut program = vec![1002, 4, 3, 4, 33];
        assert!(apply_program(&mut program, 0).is_err());
        assert_eq!(&program, &[1002, 4, 3, 4, 99]);
    }

    #[test]
    fn negative_input() {
        let mut program = vec![1101, 100, -1, 4, 0];
        assert!(apply_program(&mut program, 0).is_err());
        assert_eq!(&program, &[1101, 100, -1, 4, 99]);
    }

    #[test]
    fn equal_to_8() {
        assert_eq!(
            apply_program(&mut vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 8),
            Ok(1)
        );
        assert_eq!(
            apply_program(&mut vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], 8),
            Ok(1)
        );
        assert_eq!(
            apply_program(&mut vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 42),
            Ok(0)
        );
        assert_eq!(
            apply_program(&mut vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], 42),
            Ok(0)
        );
    }

    #[test]
    fn less_than_8() {
        assert_eq!(
            apply_program(&mut vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 8),
            Ok(0)
        );
        assert_eq!(
            apply_program(&mut vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], 8),
            Ok(0)
        );
        assert_eq!(
            apply_program(&mut vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 42),
            Ok(0)
        );
        assert_eq!(
            apply_program(&mut vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], 42),
            Ok(0)
        );
        assert_eq!(
            apply_program(&mut vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 4),
            Ok(1)
        );
        assert_eq!(
            apply_program(&mut vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], 4),
            Ok(1)
        );
    }

    #[test]
    fn jump() {
        assert_eq!(
            apply_program(
                &mut vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
                0
            ),
            Ok(0)
        );
        assert_eq!(
            apply_program(
                &mut vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
                0
            ),
            Ok(0)
        );
        assert_eq!(
            apply_program(
                &mut vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
                42
            ),
            Ok(1)
        );
        assert_eq!(
            apply_program(
                &mut vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
                42
            ),
            Ok(1)
        );
    }
}
