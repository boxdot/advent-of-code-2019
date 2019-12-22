//use itertools::*;
use vm::Vm;
mod program;
use rayon::prelude::*;

fn run(script: &[u8]) -> Option<i64> {
    for x in Vm::new(program::data(), script.iter().map(|x| *x as i64)) {
        if x < 255 {
            if x == b'D' as i64 {
                return None;
            }
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

fn try1() -> Option<String> {
    for i in 1..12 {
        let pb = std::sync::Mutex::new(pbr::ProgressBar::new((3_u64 * 10 * 2).pow(i) / 2));
        pb.lock().unwrap().message(&format!("Length {}: ", i));
        (0..(3_usize * 10 * 2).pow(i) / 2)
            .into_par_iter()
            .find_map_first(|mut seed| {
                let mut script = String::new();
                for j in 0..i {
                    script += ["OR ", "AND ", "NOT "][seed % 3];
                    seed /= 3;
                    script +=
                        ["A ", "B ", "C ", "D ", "E ", "F ", "G ", "H ", "J ", "T "][seed % 10];
                    seed /= 10;
                    if j == i - 1 {
                        script += "J\n";
                    } else {
                        script += ["J\n", "T\n"][seed % 2];
                        seed /= 2;
                    }
                }
                script += "RUN\n";
                if let Some(_) = run(script.as_bytes()) {
                    return Some(script);
                }
                pb.lock().unwrap().inc();
                None
            });
    }
    None
}

fn main() {
    println!("Part 1: {:?}", run(&SCRIPT1[1..]));
    println!("Part 2: {:?}", run(&SCRIPT2[1..]));

    println!("Shortest 1:\n{}", try1().unwrap());
}
