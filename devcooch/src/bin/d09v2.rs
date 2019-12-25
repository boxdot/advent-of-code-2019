#[derive(Default)]
struct Operands {
    pub x: i64,
    pub y: i64,
    pub x_mode: usize,
    pub y_mode: usize,
    pub x_addr: usize,
    pub y_addr: usize,
}

fn fetch_value_by_mode(
    inp: i64,
    mode: usize,
    rb: usize,
    data: &[i64],
    out: &mut i64,
    addr: &mut usize,
) {
    assert!(mode < 3);
    *out = inp;
    match mode {
        0 => {
            assert!(inp >= 0);
            *addr = inp as usize;
            *out = data[*addr];
        }
        1 => *addr = inp as usize, // do nothing, already set
        2 => {
            assert!(rb as i64 + inp >= 0);
            *addr = (rb as i64 + inp) as usize;
            *out = data[*addr];
        }
        _ => panic!(),
    }
}

fn fetch_operands(modes: usize, ip: usize, rb: usize, data: &[i64]) -> Operands {
    let mut ops: Operands = Default::default();
    ops.x_mode = modes % 10;
    ops.y_mode = (modes / 10) % 10;
    fetch_value_by_mode(
        data[ip + 1],
        ops.x_mode,
        rb,
        data,
        &mut ops.x,
        &mut ops.x_addr,
    );
    fetch_value_by_mode(
        data[ip + 2],
        ops.y_mode,
        rb,
        data,
        &mut ops.y,
        &mut ops.y_addr,
    );
    ops
}

fn parse_program(contents: &str) -> Vec<i64> {
    let mut zeros = vec![0; 1000];
    let mut result = contents
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    result.append(&mut zeros);
    result
}

use log::{error, info};

fn execute_state(state: &mut State) -> bool {
    execute(
        &mut state.mem,
        &mut state.ip,
        &mut state.dp,
        &mut state.rb,
        &mut state.input,
        &mut state.output,
    )
}

fn get_mode_formatted(mode: usize, value: usize) -> String {
    match mode {
        0 => format!("[{:08}]", value),
        1 => format!("{:010}", value),
        2 => format!("[{:08}]/R", value),
        _ => panic!(),
    }
}

fn get_jump_string(jump: bool) -> &'static str {
    if jump {
        "YES"
    } else {
        " NO"
    }
}

fn log_3ops_instruction(
    ip: &usize,
    opcode: usize,
    modes: usize,
    ops: &Operands,
    target_addr: usize,
) {
    let instr_text = match opcode {
        1 => "ADD",
        2 => "MUL",
        7 => "X<Y",
        8 => "X=Y",
        _ => panic!(),
    };
    let op1_mode = modes % 10;
    let op2_mode = (modes / 10) % 10;
    let target_mode = modes / 100;
    info!(
        "{:04} {} {} {} -> {}",
        ip,
        instr_text,
        get_mode_formatted(op1_mode, ops.x_addr),
        get_mode_formatted(op2_mode, ops.y_addr),
        get_mode_formatted(target_mode, target_addr)
    );
}

fn log_2ops_instruction(ip: &usize, opcode: usize, modes: usize, ops: &Operands) {
    let instr_text = match opcode {
        5 => "JNZ",
        6 => "JEZ",
        _ => panic!(),
    };
    let op1_mode = modes % 10;
    let op2_mode = (modes / 10) % 10;
    info!(
        "{:04} {} {} -> {}",
        ip,
        instr_text,
        get_mode_formatted(op1_mode, ops.x_addr),
        get_mode_formatted(op2_mode, ops.y_addr),
    );
}

fn execute(
    mem: &mut Vec<i64>,
    ip: &mut usize,
    dp: &mut usize,
    rb: &mut usize,
    input: &mut Vec<i64>,
    output: &mut Vec<i64>,
) -> bool {
    loop {
        let instr = mem[*ip] as usize;
        let opcode = instr % 100;
        let modes = instr / 100;
        match opcode {
            1 | 2 | 7 | 8 => {
                assert!(
                    modes <= 222,
                    "Instruction {}, op {}, modes {}",
                    instr,
                    opcode,
                    modes
                );
                let ops = fetch_operands(modes % 100, *ip, *rb, &mem);
                let target_mode = modes / 100;
                let target_addr = match target_mode {
                    0 => *ip + 3,
                    2 => (*rb as i64 + mem[*ip + 3]) as usize,
                    _ => panic!(),
                };
                let target = match target_mode {
                    0 => mem[target_addr] as usize,
                    2 => target_addr,
                    _ => panic!(),
                };
                log_3ops_instruction(ip, opcode, modes, &ops, target_addr);
                match opcode {
                    1 => mem[target] = ops.x + ops.y,
                    2 => mem[target] = ops.x * ops.y,
                    7 => mem[target] = (ops.x < ops.y) as i64,

                    8 => mem[target] = (ops.x == ops.y) as i64,

                    _ => panic!(),
                };
                let op = match opcode {
                    1 => "+",
                    2 => "*",
                    7 => "<",
                    8 => "=",
                    _ => panic!(),
                };
                info!(
                    "         {:>10}{}{:>10} => [{:08}]",
                    ops.x, op, ops.y, target
                );
                *ip += 4;
            }
            3 => {
                let target = mem[*ip + 1];
                match instr {
                    3 => {
                        info!("{:04} GET [{:04}]", ip, target);
                        mem[target as usize] = input[*dp];
                    }
                    203 => {
                        info!("{:04} GET [RB + {:04}]", ip, target);
                        let addr = (*rb as i64 + target) as usize;
                        mem[addr] = input[*dp];
                        info!("         {} => {}", mem[addr], addr);
                    }
                    _ => panic!(),
                }
                *dp += 1;
                *ip += 2;
            }
            4 => {
                let target = mem[*ip + 1];
                match instr {
                    4 => {
                        info!("{:04} OUT [{:04}]", ip, target);
                        output.push(mem[target as usize]);
                    }
                    104 => {
                        info!("{:04} OUT {:05}", ip, target);
                        output.push(target);
                    }
                    204 => {
                        info!("{:04} OUT [RB{:04}]", ip, target);
                        output.push(mem[(*rb as i64 + target) as usize]);
                    }
                    _ => error!("Unknown instruction: {}", instr),
                }
                *ip += 2;
            }
            5 | 6 => {
                let ops = fetch_operands(modes, *ip, *rb, &mem);
                let jump = (opcode == 5 && ops.x != 0) || (opcode == 6 && ops.x == 0);
                log_2ops_instruction(ip, opcode, modes, &ops);
                match opcode {
                    5 => info!("         {:>10}!=0? {}", ops.x, get_jump_string(jump)),
                    6 => info!("         {:>10}==0? {}", ops.x, get_jump_string(jump)),
                    _ => panic!(),
                }
                *ip += 3;
                if jump {
                    *ip = ops.y as usize;
                    continue;
                }
            }
            9 => {
                let value = mem[*ip + 1];
                match instr {
                    9 => {
                        info!("{:04} RB+ [{:04}]", ip, value); // set relative RB
                        *rb += mem[value as usize] as usize;
                        info!("         RB = {}", *rb);
                    }
                    109 => {
                        info!("{:04} RB+ {:5}", ip, value); // set absolute RB
                        *rb = (*rb as i64 + value) as usize;
                        info!("         RB = {}", *rb);
                    }
                    209 => {
                        info!("{:04} RB+ RB[{:04}]", ip, value); // set RB from RB
                        *rb += mem[(*rb as i64 + value) as usize] as usize;
                        info!("         RB = {}", *rb);
                    }
                    _ => error!("Unknown instruction: {}", instr),
                }
                *ip += 2;
            }
            99 => {
                info!("{:04} HLT", ip);
                return true;
            }
            _ => {
                println!("OPCODE: {}", opcode);
                panic!();
            }
        }
    }
}

use structopt::StructOpt;
#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    /// Silence all output
    #[structopt(short = "q", long = "quiet")]
    quiet: bool,
    /// Verbose mode (-v, -vv, -vvv, etc)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: usize,
    /// Timestamp (sec, ms, ns, none)
    #[structopt(short = "t", long = "timestamp")]
    ts: Option<stderrlog::Timestamp>,
}

#[derive(Default)]
struct State {
    pub mem: Vec<i64>,
    pub input: Vec<i64>,
    pub output: Vec<i64>,
    pub ip: usize, // instruction pointer
    pub dp: usize, // data pointer
    pub rb: usize, // relative base
}

fn main() {
    let opt = Opt::from_args();
    stderrlog::new()
        .module(module_path!())
        .quiet(opt.quiet)
        .verbosity(opt.verbose)
        .timestamp(opt.ts.unwrap_or(stderrlog::Timestamp::Off))
        .show_level(false)
        .init()
        .unwrap();
    let contents = include_str!("day09.txt");
    let program: Vec<i64> = parse_program(&contents);
    let mut state = State {
        mem: program,
        input: vec![2],
        output: Vec::new(),
        ip: 0,
        dp: 0,
        rb: 0,
    };
    execute_state(&mut state);
    let output_as_str: Vec<_> = state.output.iter().map(|x| x.to_string()).collect();
    println!("{}", output_as_str.join(","));
}
