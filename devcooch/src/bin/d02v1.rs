fn main() {
    let contents = include_str!("day02.txt");
    let mut data: Vec<usize> = contents
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    data[1] = 12;
    data[2] = 2;
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
    println!("{}", data[0]);
}
