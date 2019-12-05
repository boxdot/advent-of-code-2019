const ADD: isize = 1;
const MUL: isize = 2;
const INPUT: isize = 3;
const OUTPUT: isize = 4;
const JUMP_IF_TRUE: isize = 5;
const JUMP_IF_FALSE: isize = 6;
const LESS_THAN: isize = 7;
const EQUALS: isize = 8;

// see day02 for the original version of the IntCode computer
fn run_intcode_computer(program: &mut [isize], input: usize) -> Option<usize> {
    let mut ptr = 0;
    let mut output = None;
    loop {
        let opcode = program[ptr];
        let operation = (opcode / 10 % 10 * 10) + opcode % 10;
        let modes = [
            (opcode / 100 % 10) == 1,   // mode of 1st parameter
            (opcode / 1000 % 10) == 1,  // mode of 2nd parameter
            (opcode / 10000 % 10) == 1, // mode of 3rd parameter
        ];

        let val = |idx: usize| {
            let operand = program[ptr + idx + 1];
            if !modes[idx] && idx < 2 {
                // the write parameter is always position
                program[operand as usize]
            } else {
                operand
            }
        };

        match operation {
            ADD => {
                let (a, b, dst) = (val(0), val(1), val(2));
                program[dst as usize] = a + b;
                ptr += 4;
            }
            MUL => {
                let (a, b, dst) = (val(0), val(1), val(2));
                program[dst as usize] = a * b;
                ptr += 4;
            }
            INPUT => {
                let dst = program[ptr + 1];
                program[dst as usize] = input as isize;
                ptr += 2;
            }
            OUTPUT => {
                let src = program[ptr + 1];
                output = Some(program[src as usize] as usize);
                ptr += 2;
            }
            JUMP_IF_TRUE => {
                let (cond, addr) = (val(0), val(1));
                if cond != 0 {
                    ptr = addr as usize;
                } else {
                    ptr += 3;
                }
            }
            JUMP_IF_FALSE => {
                let (cond, addr) = (val(0), val(1));
                if cond == 0 {
                    ptr = addr as usize;
                } else {
                    ptr += 3;
                }
            }
            LESS_THAN => {
                let (a, b, dst) = (val(0), val(1), val(2));
                program[dst as usize] = if a < b { 1 } else { 0 };
                ptr += 4;
            }
            EQUALS => {
                let (a, b, dst) = (val(0), val(1), val(2));
                program[dst as usize] = if a == b { 1 } else { 0 };
                ptr += 4;
            }
            99 => break,
            value => unimplemented!("{}", value),
        }
    }
    output
}

#[test]
fn test_input_output() {
    assert_eq!(
        run_intcode_computer(&mut [3, 0, 4, 0, 99], 666),
        Some(666)
    );
}

#[test]
fn test_negative_values() {
    assert_eq!(run_intcode_computer(&mut [1101, 100, -1, 4, 0], 1), None);
}

#[test]
fn test_conditions() {
    // Using position mode, consider whether the input is equal to 8
    assert_eq!(
        run_intcode_computer(&mut [3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 8),
        Some(1)
    );

    // Using position mode, consider whether the input is less than 8
    assert_eq!(
        run_intcode_computer(&mut [3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 5),
        Some(1)
    );

    // Using immediate mode, consider whether the input is equal to 8
    assert_eq!(
        run_intcode_computer(&mut [3, 3, 1108, -1, 8, 3, 4, 3, 99], 8),
        Some(1)
    );

    // Using immediate mode, consider whether the input is less than 8
    assert_eq!(
        run_intcode_computer(&mut [3, 3, 1107, -1, 8, 3, 4, 3, 99], 5),
        Some(1)
    );

    // Using jumps, take an input, then output 0 if the input was zero or 1 if the input was non-zero
    // Using position mode
    assert_eq!(
        run_intcode_computer(
            &mut [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            0
        ),
        Some(0)
    );
    assert_eq!(
        run_intcode_computer(
            &mut [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            6
        ),
        Some(1)
    );
    // Using immediate mode
    assert_eq!(
        run_intcode_computer(&mut [3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], 0),
        Some(0)
    );
    assert_eq!(
        run_intcode_computer(&mut [3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], 3),
        Some(1)
    );

    let mut big_program = [
        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
        1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
        1105, 1, 46, 98, 99,
    ];
    // assert_eq!(run_intcode_computer(&mut big_program, 5), Some(999));
    assert_eq!(run_intcode_computer(&mut big_program, 10), Some(1001));
}

pub fn unlock(input: &str) -> Result<(usize, usize), Box<dyn std::error::Error>> {
    let mut program: Vec<isize> = input.split(',').filter_map(|i| i.parse().ok()).collect();
    let part1 = run_intcode_computer(&mut program, 1).unwrap();
    let part2 = run_intcode_computer(&mut program, 5).unwrap();
    Ok((part1, part2))
}
