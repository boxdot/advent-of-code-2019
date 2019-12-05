use std::convert::{TryFrom, TryInto};

pub fn solve(input: &str) -> Result<Vec<i64>, String> {
    let program = parse(input);
    execute(program, 5)
}

fn parse(input: &str) -> Vec<i64> {
    input.split(',').filter_map(|s| s.parse().ok()).collect()
}

#[derive(Debug)]
enum Op {
    Add(Param, Param, Param),
    Mul(Param, Param, Param),
    Input(Param),
    Output(Param),
    JumpIfTrue(Param, Param),
    JumpIfFalse(Param, Param),
    LessThan(Param, Param, Param),
    Equals(Param, Param, Param),
    Stop,
}

#[derive(Debug)]
struct Param {
    value: i64,
    mode: Mode,
}

impl Param {
    fn value(&self, mem: &[i64]) -> i64 {
        match self.mode {
            Mode::Position => {
                assert!(self.value >= 0);
                mem[self.value as usize]
            }
            Mode::Immediate => self.value,
        }
    }

    fn addr(&self) -> usize {
        assert_eq!(self.mode, Mode::Position);
        assert!(self.value >= 0);
        self.value as usize
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Mode {
    Position,
    Immediate,
}

impl TryFrom<i64> for Mode {
    type Error = String;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Mode::Position),
            1 => Ok(Mode::Immediate),
            otherwise => Err(format!("invalid mode: {}", otherwise)),
        }
    }
}

fn fetch(mem: &[i64], ip: usize) -> Result<Op, String> {
    let get_param = |pos| -> Result<Param, String> {
        let shift = 10_i64.pow(2 + pos as u32);
        Ok(Param {
            value: mem[ip + pos + 1],
            mode: Mode::try_from(mem[ip] % (10 * shift) / shift)?,
        })
    };

    let op = match mem[ip] % 100 {
        1 => Op::Add(get_param(0)?, get_param(1)?, get_param(2)?),
        2 => Op::Mul(get_param(0)?, get_param(1)?, get_param(2)?),
        3 => Op::Input(get_param(0)?),
        4 => Op::Output(get_param(0)?),
        5 => Op::JumpIfTrue(get_param(0)?, get_param(1)?),
        6 => Op::JumpIfFalse(get_param(0)?, get_param(1)?),
        7 => Op::LessThan(get_param(0)?, get_param(1)?, get_param(2)?),
        8 => Op::Equals(get_param(0)?, get_param(1)?, get_param(2)?),
        99 => Op::Stop,
        opcode => return Err(format!("invalid instruction: {}", opcode)),
    };
    Ok(op)
}

fn execute(mut mem: Vec<i64>, input: i64) -> Result<Vec<i64>, String> {
    let mut outputs = Vec::new();
    let mut ip = 0;
    loop {
        let op = fetch(&mem, ip)?;
        println!("{:?}", op);
        match op {
            Op::Add(a, b, c) => {
                let value_a = a.value(&mem);
                let value_b = b.value(&mem);
                mem[c.addr()] = value_a + value_b;
                ip += 4;
            }
            Op::Mul(a, b, c) => {
                let value_a = a.value(&mem);
                let value_b = b.value(&mem);
                mem[c.addr()] = value_a * value_b;
                ip += 4;
            }
            Op::Input(a) => {
                mem[a.addr()] = input;
                ip += 2;
            }
            Op::Output(a) => {
                outputs.push(a.value(&mem));
                ip += 2;
            }
            Op::JumpIfTrue(a, b) => {
                if a.value(&mem) != 0 {
                    ip = b.value(&mem).try_into().map_err(|e| format!("{}", e))?;
                } else {
                    ip += 3;
                }
            }
            Op::JumpIfFalse(a, b) => {
                if a.value(&mem) == 0 {
                    ip = b.value(&mem).try_into().map_err(|e| format!("{}", e))?;
                } else {
                    ip += 3;
                }
            }
            Op::LessThan(a, b, c) => {
                mem[c.addr()] = if a.value(&mem) < b.value(&mem) { 1 } else { 0 };
                ip += 4;
            }
            Op::Equals(a, b, c) => {
                mem[c.addr()] = if a.value(&mem) == b.value(&mem) { 1 } else { 0 };
                ip += 4;
            }
            Op::Stop => break,
        }
    }
    Ok(outputs)
}
