use itertools::iproduct;

pub fn solve(input: &str) -> (i64, Option<i64>) {
    let state = parse(input);

    let part1 = {
        let mut state = state.clone();
        state[1] = 12;
        state[2] = 2;
        execute(state)
    };

    let part2 = iproduct!(0..100, 0..100)
        .find(|&(noun, verb)| {
            let mut state = state.clone();
            state[1] = noun;
            state[2] = verb;
            execute(state) == 19_690_720
        })
        .map(|(noun, verb)| 100 * noun + verb);

    (part1, part2)
}

fn parse(input: &str) -> Vec<i64> {
    input.split(',').filter_map(|s| s.parse().ok()).collect()
}

fn execute(mut state: Vec<i64>) -> i64 {
    let mut ip = 0; // instruction pointer
    loop {
        match state[ip] {
            1 => {
                let a = state[state[ip + 1] as usize];
                let b = state[state[ip + 2] as usize];
                let addr = state[ip + 3] as usize;
                state[addr] = a + b;
                ip += 4;
            }
            2 => {
                let a = state[state[ip + 1] as usize];
                let b = state[state[ip + 2] as usize];
                let addr = state[ip + 3] as usize;
                state[addr] = a * b;
                ip += 4;
            }
            99 => break,
            other => panic!("invalid opcode: {}", other),
        }
    }

    state[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute() {
        // assert_eq!(
        //     execute(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]),
        //     3500
        // );
        // assert_eq!(execute(vec![1, 0, 0, 0, 99]), 2);
        // assert_eq!(execute(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]), 30);

        execute(vec![2, 3, 0, 3, 99]);
        execute(vec![2, 4, 4, 5, 99, 0]);
    }
}
