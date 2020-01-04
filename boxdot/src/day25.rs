use crate::day09::{execute, parse, Memory};

/// Just play the game! Its fun! :)
pub fn solve(input: &str) {
    let mem = parse(input);
    run(mem)
}

fn run(mut mem: Memory) {
    let mut buffer = String::new();
    let mut index = 0;

    let mut ip = Some(0);
    while let Some(next_ip) = ip {
        ip = execute(
            &mut mem,
            next_ip,
            || {
                if index >= buffer.len() {
                    index = 0;
                    buffer.clear();
                    std::io::stdin().read_line(&mut buffer).unwrap();
                }

                let value = buffer.as_bytes()[index] as i64;
                index += 1;
                value
            },
            |value| {
                print!("{}", value as u8 as char);
            },
        )
        .unwrap();
    }
}
