use aoc2019::run_intcode_computer;

pub fn unlock(input: &str) -> Result<(usize, usize), Box<dyn std::error::Error>> {
    let part1 = run_intcode_computer(input, vec![1]);
    let part2 = run_intcode_computer(input, vec![5]);
    Ok((part1 as usize, part2 as usize))
}
