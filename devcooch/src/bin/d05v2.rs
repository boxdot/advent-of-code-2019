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

fn main() {
    let contents = include_str!("day05.txt");
    let mut data: Vec<i32> = contents
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let mut i: usize = 0;
    loop {
        let instr = data[i];
        let opcode = instr % 100;
        let modes = instr / 100;
        match opcode {
            99 => break,
            1 | 2 | 7 | 8 => {
                assert!(modes < 100);
                let (x, y) = fetch_operands(modes, i, &data);
                let target = data[i + 3] as usize;
                data[target] = match opcode {
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
                i += 4;
            }
            3 => {
                let target = data[i + 1] as usize;
                data[target] = 5; // thermal radiator controller
                i += 2;
            }
            4 => {
                println!("OUT>{}", data[data[i + 1] as usize]);
                i += 2;
            }
            5 | 6 => {
                let (x, y) = fetch_operands(modes, i, &data);
                if (opcode == 5 && x != 0) || (opcode == 6 && x == 0) {
                    i = y as usize;
                    continue;
                }
                i += 3;
            }
            _ => panic!(),
        }
    }
}
