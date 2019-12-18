#[derive(Default)]
struct Operands {
    pub x: i32,
    pub y: i32,
    pub x_mode: i32,
    pub y_mode: i32,
    pub x_addr: usize,
    pub y_addr: usize,
}

fn fetch_operands(modes: i32, ip: usize, data: &[i32]) -> Operands {
    let mut ops: Operands = Default::default();
    ops.x_mode = modes % 10;
    ops.y_mode = (modes / 10) % 10;
    assert!(ops.x_mode < 2);
    assert!(ops.y_mode < 2);
    ops.x = data[ip + 1];
    ops.y = data[ip + 2];
    if ops.x_mode == 0 {
        assert!(ops.x >= 0);
        ops.x_addr = ops.x as usize;
        ops.x = data[ops.x_addr]
    }
    if ops.y_mode == 0 {
        assert!(ops.y >= 0);
        ops.y_addr = ops.y as usize;
        ops.y = data[ops.y_addr]
    }
    ops
}

fn parse_program(contents: &str) -> Vec<i32> {
    contents
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

use log::{error, info, warn};

fn execute_amp(amp: &mut Amplifier) -> bool {
    execute(
        &mut amp.mem,
        &mut amp.ip,
        &mut amp.dp,
        &mut amp.input,
        &mut amp.output,
    )
}

fn execute(
    mem: &mut Vec<i32>,
    ip: &mut usize,
    dp: &mut usize,
    input: &mut Vec<i32>,
    output: &mut Vec<i32>,
) -> bool {
    loop {
        let instr = mem[*ip];
        let opcode = instr % 100;
        let modes = instr / 100;
        match opcode {
            1 | 2 | 7 | 8 => {
                assert!(modes < 100);
                let ops = fetch_operands(modes, *ip, &mem);
                let target = mem[*ip + 3] as usize;
                mem[target] = match opcode {
                    1 => ops.x + ops.y,
                    2 => ops.x * ops.y,
                    7 => (ops.x < ops.y) as i32,
                    8 => (ops.x == ops.y) as i32,
                    _ => 0,
                };
                *ip += 4;
                match instr {
                    1 => info!(
                        "ADD [{:04}] [{:04}] -> [{:04}]",
                        ops.x_addr, ops.y_addr, target
                    ),
                    2 => info!(
                        "MUL [{:04}] [{:04}] -> [{:04}]",
                        ops.x_addr, ops.y_addr, target
                    ),
                    101 => info!("ADD {:5}  [{:04}]  -> [{:04}]", ops.x, ops.y_addr, target),
                    102 => info!("MUL {:5}  [{:04}]  -> [{:04}]", ops.x, ops.y_addr, target),
                    1001 => info!("ADD [{:04}] {:5}  -> [{:04}]", ops.x_addr, ops.y, target),
                    1002 => info!("MUL [{:04}] {:5}  -> [{:04}]", ops.x_addr, ops.y, target),
                    1007 => info!("X<Y [{:04}] {:5}  -> [{:04}]", ops.x_addr, ops.y, target),
                    1008 => info!("X=Y [{:04}] {:5}  -> [{:04}]", ops.x_addr, ops.y, target),
                    _ => error!("Unknown instruction: {}", instr),
                }
            }
            3 => {
                let target = mem[*ip + 1] as usize;
                info!("GET [{:04}]", target);
                mem[target] = input[*dp];
                *dp += 1;
                *ip += 2;
            }
            4 => {
                let target = mem[*ip + 1] as usize;
                info!("OUT [{:04}]", target);
                output.push(mem[target]);
                *ip += 2;
                return false;
            }
            5 | 6 => {
                let ops = fetch_operands(modes, *ip, &mem);
                match instr {
                    5 => info!("JNZ [{:04}]        -> [{}]", ops.x_addr, ops.y),
                    6 => info!("JEZ [{:04}]        -> [{}]", ops.x_addr, ops.y),
                    105 => info!("JNX {:5}        -> [{}]", ops.x, ops.y_addr),
                    1005 => info!("JNZ [{:04}]        -> {}", ops.x_addr, ops.y),
                    1105 => info!("JNZ {:5}         -> {}", ops.x, ops.y),
                    _ => error!("Unknown instruction: {}", instr),
                }
                *ip += 3;
                if (opcode == 5 && ops.x != 0) || (opcode == 6 && ops.x == 0) {
                    *ip = ops.y as usize;
                    continue;
                }
            }
            99 => {
                info!("HLT");
                return true;
            }
            _ => panic!(),
        }
    }
}

use permutohedron::Heap;
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
struct Amplifier {
    pub mem: Vec<i32>,
    pub input: Vec<i32>,
    pub output: Vec<i32>,
    pub ip: usize, // instruction pointer
    pub dp: usize, // data pointer
}

fn make_amp(program: Vec<i32>) -> Amplifier {
    let mut result: Amplifier = Default::default();
    result.mem = program;
    result
}

fn prepare_amps(phases: Vec<i32>, program: &Vec<i32>) -> Vec<Amplifier> {
    let mut amps = Vec::new();
    for i in 0..5 {
        amps.push(make_amp(program.clone()));
        amps[i].input.push(phases[i]);
    }
    amps[0].input.push(0);
    amps
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
    let contents = include_str!("day07.txt");
    let program: Vec<i32> = parse_program(&contents);
    let mut five_to_nine = (5..10).collect::<Vec<i32>>();
    let permutations = Heap::new(&mut five_to_nine);
    let mut signals = Vec::new();
    for phases in permutations {
        let mut amps = prepare_amps(phases, &program);
        let mut ap = 0; // amplifier pointer
        let mut halt = false;
        while !halt {
            warn!("Amplifier {}", ap);
            halt = execute_amp(&mut amps[ap]);
            let old_ap = ap;
            ap += 1;
            ap %= 5;
            let out: i32 = *amps[old_ap].output.last().unwrap();
            amps[ap].input.push(out);
        }
        signals.push(*amps[4].output.last().unwrap());
    }
    println!("{}", signals.iter().max().unwrap());
}
