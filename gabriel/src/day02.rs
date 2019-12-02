fn run_intcode_computer(mut program: Vec<usize>, noun: usize, verb: usize) -> usize {
    let mut ptr = 0;
    program[1] = noun;
    program[2] = verb;
    loop {
        match program[ptr] {
            1 => {
                let (un, deux, trois) = (program[ptr + 1], program[ptr + 2], program[ptr + 3]);
                program[trois] = program[un] + program[deux];
                ptr += 4;
            }
            2 => {
                let (un, deux, trois) = (program[ptr + 1], program[ptr + 2], program[ptr + 3]);
                program[trois] = program[un] * program[deux];
                ptr += 4;
            }
            99 => break,
            invalid_opcode => {
                panic!(format!("unexpected opcode {}, go away!", invalid_opcode));
            }
        }
    }
    return program[0];
}

fn bruteforce_part2(program: Vec<usize>) -> (usize, usize) {
    for noun in 0..100 {
        for verb in 0..100 {
            if run_intcode_computer(program.clone(), noun, verb) == 19690720 {
                return (noun, verb);
            }
        }
    }
    return (0, 0);
}

pub fn unlock(input: &str) -> Result<(usize, usize), Box<dyn std::error::Error>> {
    let program: Vec<usize> = input.split(',').filter_map(|i| i.parse().ok()).collect();

    let part1 = run_intcode_computer(program.clone(), 12, 2);
    let part2 = bruteforce_part2(program);

    Ok((part1, 100 * part2.0 + part2.1))
}
