use std::convert::{TryFrom, TryInto};
use std::sync::mpsc::{channel, Receiver, Sender};

pub type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<Vec<i64>, Error> {
    let program = parse(input);
    run(program, &[5])
}

pub fn parse(input: &str) -> Vec<i64> {
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
    type Error = Error;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Mode::Position),
            1 => Ok(Mode::Immediate),
            otherwise => Err(format!("invalid mode: {}", otherwise).into()),
        }
    }
}

fn fetch(mem: &[i64], ip: usize) -> Result<Op, Error> {
    let get_param = |pos| -> Result<Param, Error> {
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
        opcode => return Err(format!("invalid instruction: {}", opcode).into()),
    };
    Ok(op)
}

pub fn execute(
    mem: &mut [i64],
    ip: usize,
    mut input: impl FnMut() -> i64,
    mut output: impl FnMut(i64),
) -> Result<Option<usize>, Error> {
    let op = fetch(&mem, ip)?;
    let ip = match op {
        Op::Add(a, b, c) => {
            let value_a = a.value(mem);
            let value_b = b.value(mem);
            mem[c.addr()] = value_a + value_b;
            ip + 4
        }
        Op::Mul(a, b, c) => {
            let value_a = a.value(mem);
            let value_b = b.value(mem);
            mem[c.addr()] = value_a * value_b;
            ip + 4
        }
        Op::Input(a) => {
            mem[a.addr()] = input();
            ip + 2
        }
        Op::Output(a) => {
            output(a.value(mem));
            ip + 2
        }
        Op::JumpIfTrue(a, b) => {
            if a.value(mem) != 0 {
                b.value(mem).try_into().map_err(|e| format!("{}", e))?
            } else {
                ip + 3
            }
        }
        Op::JumpIfFalse(a, b) => {
            if a.value(mem) == 0 {
                b.value(mem).try_into().map_err(|e| format!("{}", e))?
            } else {
                ip + 3
            }
        }
        Op::LessThan(a, b, c) => {
            mem[c.addr()] = if a.value(&mem) < b.value(&mem) { 1 } else { 0 };
            ip + 4
        }
        Op::Equals(a, b, c) => {
            mem[c.addr()] = if a.value(&mem) == b.value(&mem) { 1 } else { 0 };
            ip + 4
        }
        Op::Stop => return Ok(None),
    };

    Ok(Some(ip))
}

pub fn run(mut mem: Vec<i64>, input: &[i64]) -> Result<Vec<i64>, Error> {
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

pub fn run_async(mut mem: Vec<i64>) -> (Sender<i64>, Receiver<i64>) {
    let (input_sender, input_receiver) = channel();
    let (output_sender, output_receiver) = channel();

    std::thread::spawn(move || {
        let mut ip = Some(0);
        while let Some(next_ip) = ip {
            ip = execute(
                &mut mem,
                next_ip,
                || input_receiver.recv().unwrap(),
                |value| {
                    output_sender.send(value).unwrap();
                },
            )
            .unwrap();
        }
    }); // detached thread

    (input_sender, output_receiver)
}
