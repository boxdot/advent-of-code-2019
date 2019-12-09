use std::convert::{TryFrom, TryInto};

pub type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<(i64, i64), Error> {
    let mem = parse(input);
    let produce_output = |input| -> Result<i64, Error> {
        let output = run(mem.clone(), &[input])?;
        let value = output.first().ok_or_else(|| "missing output")?;
        Ok(*value)
    };

    Ok((produce_output(1)?, produce_output(2)?))
}

fn parse(input: &str) -> Memory {
    let data = input.split(',').filter_map(|s| s.parse().ok()).collect();
    Memory {
        data,
        relative_base: 0,
    }
}

#[derive(Debug, Clone)]
struct Memory {
    data: Vec<i64>,
    relative_base: usize,
}

impl Memory {
    fn value(&self, param: Param) -> i64 {
        match param.mode {
            Mode::Immediate => param.value,
            Mode::Position | Mode::Relative => self.read(self.addr(param)),
        }
    }

    fn addr(&self, param: Param) -> usize {
        match param.mode {
            Mode::Position => {
                assert!(param.value >= 0);
                param.value as usize
            }
            Mode::Relative => {
                let addr = self.relative_base as i64 + param.value;
                assert!(addr >= 0);
                addr as usize
            }
            otherwise => panic!("unexpected mode: {:?}", otherwise),
        }
    }

    fn read(&self, addr: usize) -> i64 {
        self.data.get(addr).cloned().unwrap_or(0)
    }

    fn write(&mut self, addr: usize, value: i64) {
        if addr >= self.data.len() {
            self.data.resize(addr + 1, 0);
        }
        self.data[addr] = value;
    }
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
    RelativeBase(Param),
    Stop,
}

#[derive(Debug, Clone, Copy)]
struct Param {
    value: i64,
    mode: Mode,
}

impl Param {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Position,
    Immediate,
    Relative,
}

impl TryFrom<i64> for Mode {
    type Error = Error;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Mode::Position),
            1 => Ok(Mode::Immediate),
            2 => Ok(Mode::Relative),
            otherwise => Err(format!("invalid mode: {}", otherwise).into()),
        }
    }
}

fn fetch(mem: &Memory, ip: usize) -> Result<Op, Error> {
    let get_param = |pos| -> Result<Param, Error> {
        let shift = 10_i64.pow(2 + pos as u32);
        Ok(Param {
            value: mem.read(ip + pos + 1),
            mode: Mode::try_from(mem.read(ip) % (10 * shift) / shift)?,
        })
    };

    let op = match mem.read(ip) % 100 {
        1 => Op::Add(get_param(0)?, get_param(1)?, get_param(2)?),
        2 => Op::Mul(get_param(0)?, get_param(1)?, get_param(2)?),
        3 => Op::Input(get_param(0)?),
        4 => Op::Output(get_param(0)?),
        5 => Op::JumpIfTrue(get_param(0)?, get_param(1)?),
        6 => Op::JumpIfFalse(get_param(0)?, get_param(1)?),
        7 => Op::LessThan(get_param(0)?, get_param(1)?, get_param(2)?),
        8 => Op::Equals(get_param(0)?, get_param(1)?, get_param(2)?),
        9 => Op::RelativeBase(get_param(0)?),
        99 => Op::Stop,
        opcode => return Err(format!("invalid instruction: {}", opcode).into()),
    };
    Ok(op)
}

fn execute(
    mem: &mut Memory,
    ip: usize,
    mut input: impl FnMut() -> i64,
    mut output: impl FnMut(i64),
) -> Result<Option<usize>, Error> {
    let op = fetch(&mem, ip)?;
    let ip = match op {
        Op::Add(a, b, c) => {
            let value_a = mem.value(a);
            let value_b = mem.value(b);
            mem.write(mem.addr(c), value_a + value_b);
            ip + 4
        }
        Op::Mul(a, b, c) => {
            let value_a = mem.value(a);
            let value_b = mem.value(b);
            mem.write(mem.addr(c), value_a * value_b);
            ip + 4
        }
        Op::Input(a) => {
            mem.write(mem.addr(a), input());
            ip + 2
        }
        Op::Output(a) => {
            output(mem.value(a));
            ip + 2
        }
        Op::JumpIfTrue(a, b) => {
            if mem.value(a) != 0 {
                mem.value(b).try_into().map_err(|e| format!("{}", e))?
            } else {
                ip + 3
            }
        }
        Op::JumpIfFalse(a, b) => {
            if mem.value(a) == 0 {
                mem.value(b).try_into().map_err(|e| format!("{}", e))?
            } else {
                ip + 3
            }
        }
        Op::LessThan(a, b, c) => {
            mem.write(mem.addr(c), if mem.value(a) < mem.value(b) { 1 } else { 0 });
            ip + 4
        }
        Op::Equals(a, b, c) => {
            mem.write(
                mem.addr(c),
                if mem.value(a) == mem.value(b) { 1 } else { 0 },
            );
            ip + 4
        }
        Op::RelativeBase(a) => {
            let base = mem.relative_base as i64 + mem.value(a);
            assert!(base >= 0);
            mem.relative_base = base as usize;
            ip + 2
        }
        Op::Stop => return Ok(None),
    };

    Ok(Some(ip))
}

fn run(mut mem: Memory, input: &[i64]) -> Result<Vec<i64>, Error> {
    let mut input_pos = 0;
    let mut outputs = Vec::new();

    let mut ip = Some(0);
    while let Some(next_ip) = ip {
        ip = execute(
            &mut mem,
            next_ip,
            || {
                let value = input[input_pos];
                input_pos += 1;
                value
            },
            |value| {
                outputs.push(value);
            },
        )?;
    }

    Ok(outputs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let mem = parse("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
        let output = run(mem.clone(), &[]).expect("run failed");
        assert_eq!(output, mem.data);

        let mem = parse("1102,34915192,34915192,7,4,7,99,0");
        let output = run(mem.clone(), &[]).expect("run failed");
        assert_eq!(output[0], 1219070632396864);

        let mem = parse("104,1125899906842624,99");
        let output = run(mem.clone(), &[]).expect("run failed");
        assert_eq!(output[0], 1125899906842624);
    }
}
