const ADD: isize = 1;
const MUL: isize = 2;
const INPUT: isize = 3;
const OUTPUT: isize = 4;
const JUMP_IF_TRUE: isize = 5;
const JUMP_IF_FALSE: isize = 6;
const LESS_THAN: isize = 7;
const EQUALS: isize = 8;
const RELATIVE_BASE: isize = 9;

struct Program {
    mem: Vec<isize>,
    ptr: usize,
    relbase: isize,
}

impl Program {
    fn opcode(&self) -> isize {
        self.mem[self.ptr] % 100
    }

    fn get_addr(&self, arg_idx: usize) -> usize {
        let mode = self.mem[self.ptr] / 10_isize.pow(arg_idx as u32 + 1) % 10;
        if mode == 0 {
            self.mem[self.ptr + arg_idx as usize] as usize
        } else if mode == 1 {
            self.ptr + arg_idx as usize
        } else if mode == 2 {
            (self.mem[self.ptr + arg_idx as usize] + self.relbase) as usize
        } else {
            panic!("impossibru!");
        }
    }

    fn load(&self, arg_idx: usize) -> isize {
        self.mem[self.get_addr(arg_idx)]
    }

    fn store(&mut self, arg_idx: usize, value: isize) {
        let addr = self.get_addr(arg_idx);
        // auto-resize on demand
        if self.mem.len() <= addr {
            self.mem.resize(addr + 1, 0);
        }
        self.mem[addr] = value;
    }

    fn jump(&mut self, offset: usize) {
        self.ptr += offset;
    }
}

pub fn run_intcode_computer(program: &str, input: Vec<isize>) -> Vec<isize> {
    let data: Vec<isize> = program.split(',').filter_map(|i| i.parse().ok()).collect();
    let mut program = Program {
        mem: data,
        ptr: 0,
        relbase: 0,
    };
    let mut input = input.iter();
    let mut output = vec![];
    loop {
        match program.opcode() {
            ADD => {
                program.store(3, program.load(1) + program.load(2));
                program.jump(4);
            }
            MUL => {
                program.store(3, program.load(1) * program.load(2));
                program.jump(4);
            }
            INPUT => {
                program.store(1, *input.next().expect("no more input!"));
                program.jump(2);
            }
            OUTPUT => {
                output.push(program.load(1));
                program.jump(2);
            }
            JUMP_IF_TRUE => {
                let (cond, addr) = (program.load(1), program.load(2));
                if cond != 0 {
                    program.ptr = addr as usize;
                } else {
                    program.jump(3);
                }
            }
            JUMP_IF_FALSE => {
                let (cond, addr) = (program.load(1), program.load(2));
                if cond == 0 {
                    program.ptr = addr as usize;
                } else {
                    program.jump(3);
                }
            }
            LESS_THAN => {
                program.store(3, (program.load(1) < program.load(2)) as isize);
                program.jump(4);
            }
            EQUALS => {
                program.store(3, (program.load(1) == program.load(2)) as isize);
                program.jump(4);
            }
            RELATIVE_BASE => {
                program.relbase += program.load(1);
                program.jump(2);
            }
            99 => break,
            value => unimplemented!("{}", value),
        }
    }
    output
}

#[test]
fn test_input_output() {
    assert_eq!(run_intcode_computer("3,0,4,0,99", vec![666]), &[666]);
}

#[test]
fn test_negative_values() {
    assert_eq!(run_intcode_computer("1101,100,-1,4,0", vec![1]), &[0]);
}

#[test]
fn test_conditions() {
    // Using position mode, consider whether the input is equal to 8
    assert_eq!(
        run_intcode_computer("3,9,8,9,10,9,4,9,99,-1,8", vec![8]),
        &[1]
    );

    // Using position mode, consider whether the input is less than 8
    assert_eq!(
        run_intcode_computer("3,9,7,9,10,9,4,9,99,-1,8", vec![5]),
        &[1]
    );

    // Using immediate mode, consider whether the input is equal to 8
    assert_eq!(
        run_intcode_computer("3,3,1108,-1,8,3,4,3,99", vec![8]),
        &[1]
    );

    // Using immediate mode, consider whether the input is less than 8
    assert_eq!(
        run_intcode_computer("3,3,1107,-1,8,3,4,3,99", vec![5]),
        &[1]
    );

    // Using jumps, take an input, then output 0 if the input was zero or 1 if the input was non-zero
    // Using position mode
    assert_eq!(
        run_intcode_computer("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", vec![0]),
        &[0]
    );
    assert_eq!(
        run_intcode_computer("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", vec![6]),
        &[1]
    );
    // Using immediate mode
    assert_eq!(
        run_intcode_computer("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", vec![0]),
        &[0]
    );
    assert_eq!(
        run_intcode_computer("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", vec![3]),
        &[1]
    );

    let mut big_program = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
    assert_eq!(run_intcode_computer(&mut big_program, vec![5]), &[999]);
    assert_eq!(run_intcode_computer(&mut big_program, vec![10]), &[1001]);
}

#[test]
fn test_relative_base() {
    assert_eq!(
        run_intcode_computer(
            "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99",
            vec![]
        ),
        &[109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
    );
    assert_eq!(
        run_intcode_computer("1102,34915192,34915192,7,4,7,99,0", vec![]),
        &[1219070632396864]
    );
    assert_eq!(
        run_intcode_computer("104,1125899906842624,99", vec![]),
        &[1125899906842624],
    );
}
