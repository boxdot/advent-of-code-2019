fn fetch_operands(modes: i32, ip: usize, data: &[i32]) -> (i32, i32) {
    let x_mod = modes % 10;
    let y_mod = (modes / 10) % 10;
    assert!(x_mod < 2);
    assert!(y_mod < 2);
    let mut x = data[ip + 1];
    let mut y = data[ip + 2];
    if x_mod == 0 {
        assert!(x >= 0);
        x = data[x as usize]
    }
    if y_mod == 0 {
        assert!(y >= 0);
        y = data[y as usize]
    }
    (x, y)
}

fn parse_program(contents: &str) -> Vec<i32> {
    contents
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn execute(mut mem: Vec<i32>, input: &[i32]) -> Vec<i32> {
    let mut output = Vec::new();
    let mut ip: usize = 0; // instruction pointer
    let mut dp: usize = 0; // input data pointer
    loop {
        let instr = mem[ip];
        let opcode = instr % 100;
        let modes = instr / 100;
        match opcode {
            99 => break,
            1 | 2 | 7 | 8 => {
                assert!(modes < 100);
                let (x, y) = fetch_operands(modes, ip, &mem);
                let target = mem[ip + 3] as usize;
                mem[target] = match opcode {
                    1 => x + y,
                    2 => x * y,
                    7 => {
                        if x < y {
                            1
                        } else {
                            0
                        }
                    }
                    8 => {
                        if x == y {
                            1
                        } else {
                            0
                        }
                    }
                    _ => 0,
                };
                ip += 4;
            }
            3 => {
                let target = mem[ip + 1] as usize;
                mem[target] = input[dp];
                dp += 1;
                ip += 2;
            }
            4 => {
                output.push(mem[mem[ip + 1] as usize]);
                ip += 2;
            }
            5 | 6 => {
                let (x, y) = fetch_operands(modes, ip, &mem);
                if (opcode == 5 && x != 0) || (opcode == 6 && x == 0) {
                    ip = y as usize;
                    continue;
                }
                ip += 3;
            }
            _ => panic!(),
        }
    }
    output
}

use permutohedron::Heap;

fn main() {
    let contents = include_str!("day07.txt");
    let program: Vec<i32> = parse_program(&contents);
    let mut phases = (0..5).collect::<Vec<i32>>();
    let perms = Heap::new(&mut phases);
    let mut signals = Vec::new();
    for perm in perms {
        let mut output = vec![0i32];
        for phase in perm {
            let mut input = vec![phase];
            input.append(&mut output);
            let mem = program.clone();
            output = execute(mem, &input);
            assert!(output.len() == 1);
        }
        signals.push(output[0]);
    }
    println!("{}", signals.iter().max().unwrap());
}
