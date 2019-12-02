use itertools::iproduct;

pub fn solve(input: &str) -> (Option<usize>, Option<usize>) {
    let memory = parse(input);

    let part1 = execute(memory.clone(), 12, 2).ok();
    let part2 = iproduct!(0..100, 0..100)
        .find(|&(noun, verb)| execute(memory.clone(), noun, verb) == Ok(19_690_720))
        .map(|(noun, verb)| 100 * noun + verb);

    (part1, part2)
}

fn parse(input: &str) -> Vec<usize> {
    input.split(',').filter_map(|s| s.parse().ok()).collect()
}

fn execute(mut mem: Vec<usize>, noun: usize, verb: usize) -> Result<usize, ()> {
    mem[1] = noun;
    mem[2] = verb;

    let add = |a, b, c, mem: &mut [usize]| mem[c] = mem[a] + mem[b];
    let mul = |a, b, c, mem: &mut [usize]| mem[c] = mem[a] * mem[b];

    for ip in (0..).step_by(4) {
        match mem[ip] {
            1 => add(mem[ip + 1], mem[ip + 2], mem[ip + 3], &mut mem),
            2 => mul(mem[ip + 1], mem[ip + 2], mem[ip + 3], &mut mem),
            99 => return Ok(mem[0]),
            _ => break,
        }
    }
    Err(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute() {
        assert_eq!(
            execute(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50], 12, 2),
            Ok(3500)
        );
    }
}
