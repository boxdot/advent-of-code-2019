use aoc2019::run_intcode_computer;

pub fn unlock(program: &str) -> Result<(usize, usize), Box<dyn std::error::Error>> {
    let part1 = permutohedron::Heap::new(&mut [0, 1, 2, 3, 4])
        .map(|phase_settings| thruster_signal(program, phase_settings))
        .max()
        .unwrap();
    Ok((part1 as usize, 0))
}

fn thruster_signal(program: &str, phase_settings: [isize; 5]) -> isize {
    phase_settings
        .iter()
        .fold(0, |previous_signal, &phase_setting| {
            run_intcode_computer(program, vec![phase_setting, previous_signal])[0]
        })
}

#[test]
fn test_phase_settings() {
    assert_eq!(
        thruster_signal(
            "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0",
            [4, 3, 2, 1, 0]
        ),
        43210
    );
    assert_eq!(
        thruster_signal(
            "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0",
            [5, 4, 3, 2, 1]
        ),
        01234
    );
    assert_eq!(thruster_signal("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0", [1,0,4,3,2]), 65210);
}
