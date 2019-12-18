fn main() {
    let contents = include_str!("day02.txt");
    let original: Vec<usize> = contents
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    'outer: for noun in 0..100 {
        for verb in 0..100 {
            let mut data = original.clone();
            data[1] = noun;
            data[2] = verb;
            let mut i = 0;
            loop {
                let instr = data[i];
                if instr == 99 {
                    break;
                }
                let x = data[i + 1];
                let y = data[i + 2];
                let target = data[i + 3];
                i += 4;
                if instr == 1 {
                    data[target] = data[x] + data[y];
                } else if instr == 2 {
                    data[target] = data[x] * data[y];
                } else {
                    panic!();
                }
            }
            if data[0] == 19_690_720 {
                println!("{}", 100 * noun + verb);
                break 'outer;
            }
        }
    }
}
