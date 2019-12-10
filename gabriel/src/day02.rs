use aoc2019::run_intcode_computer;

fn bruteforce_part2(program: &str) -> (usize, usize) {
    for noun in 0..100 {
        for verb in 0..100 {
            if run_intcode_computer(program, vec![noun, verb]) == &[19690720] {
                return (noun as usize, verb as usize);
            }
        }
    }
    return (0, 0);
}

pub fn unlock(program: &str) -> Result<(usize, usize), Box<dyn std::error::Error>> {
    let part1 = run_intcode_computer(program, vec![12, 2]);
    let part2 = bruteforce_part2(program);

    Ok((part1[0] as usize, 100 * part2.0 + part2.1))
}
