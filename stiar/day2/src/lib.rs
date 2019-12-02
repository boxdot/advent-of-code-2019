use itertools::Itertools;

pub fn apply_program(program: &mut [usize]) {
    for index in (0..program.len()).step_by(4) {
        match program[index] {
            1 if index + 3 < program.len() => {
                program[program[index + 3]] =
                    program[program[index + 1]] + program[program[index + 2]];
            }
            2 if index + 3 < program.len() => {
                program[program[index + 3]] =
                    program[program[index + 1]] * program[program[index + 2]];
            }
            _ => break,
        }
    }
}

pub fn find_verb_and_noun(program: &[usize], desired_value: usize) -> Option<(usize, usize)> {
    (0..100).cartesian_product(0..100).find(|(verb, noun)| {
        let mut mut_program: Vec<_> = program.iter().cloned().collect();
        mut_program[1] = *verb as usize;
        mut_program[2] = *noun as usize;
        apply_program(&mut mut_program);
        mut_program[0] == desired_value
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test_1() {
        let mut program = vec![1, 0, 0, 0, 99];
        apply_program(&mut program);
        assert_eq!(program, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn sample_test_2() {
        let mut program = vec![2, 3, 0, 3, 99];
        apply_program(&mut program);
        assert_eq!(program, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn sample_test_3() {
        let mut program = vec![2, 4, 4, 5, 99, 0];
        apply_program(&mut program);
        assert_eq!(program, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn sample_test_4() {
        let mut program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        apply_program(&mut program);
        assert_eq!(program, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
