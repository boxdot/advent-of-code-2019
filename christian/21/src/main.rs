//use itertools::*;
use vm::Vm;
mod program;

fn run(script: &[u8]) -> Option<i64> {
    for x in Vm::new(program::data(), script.iter().map(|x| *x as i64)) {
        if x < 255 {
            print!("{}", x as u8 as char);
        } else {
            return Some(x);
        }
    }
    None
}

const SCRIPT1: &[u8] = b"
NOT A J
NOT C T
AND D T
OR T J
WALK
";

const SCRIPT2: &[u8] = b"
NOT A J
OR B T
AND C T
NOT T T
AND D T
AND H T
OR T J
RUN
";

fn main() {
    println!("Part 1: {:?}", run(&SCRIPT1[1..]));
    println!("Part 2: {:?}", run(&SCRIPT2[1..]));
}
