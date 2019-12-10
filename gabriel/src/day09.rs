use aoc2019::run_intcode_computer;

pub fn unlock(input: &str) -> Result<(usize, usize), Box<dyn std::error::Error>> {
    let boost = run_intcode_computer(input, vec![1]);
    assert_eq!(boost.len(), 1);

    let distress_signal = run_intcode_computer(input, vec![2]);
    assert_eq!(distress_signal.len(), 1);

    Ok((boost[0] as usize, distress_signal[0] as usize))
}
