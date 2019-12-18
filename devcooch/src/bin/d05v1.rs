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
        //println!("OP: {}", opcode);
        match opcode {
            99 => break,
            1 | 2 => {
                let (x, y) = fetch_operands(instr / 100, i, &data);
                let target = data[i + 3] as usize;
                if opcode == 1 {
                    data[target] = x + y;
                } else if opcode == 2 {
                    data[target] = x * y;
                } else {
                    panic!();
                }
                i += 4;
            }
            3 => {
                let target = data[i + 1] as usize;
                data[target] = 1; // air conditioner
                i += 2;
            }
            4 => {
                println!("OUT>{}", data[data[i + 1] as usize]);
                i += 2;
            }
            _ => panic!(),
        }
    }
}
