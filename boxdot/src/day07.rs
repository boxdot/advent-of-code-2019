use crate::day05::{parse, run, run_async, Error};

use std::sync::mpsc::{Receiver, Sender};

pub fn solve(input: &str) -> Result<(i64, i64), Error> {
    let mem = parse(input);

    let mut init_phase = [0, 1, 2, 3, 4];
    let part1 = permutohedron::Heap::new(&mut init_phase)
        .filter_map(|phase| run_with_phase(&mem, phase).ok())
        .max()
        .ok_or_else(|| "all programs failed")?;

    let mut init_phase = [5, 6, 7, 8, 9];
    let part2 = permutohedron::Heap::new(&mut init_phase)
        .filter_map(|phase| run_with_feedback_loop(&mem, phase).ok())
        .max()
        .ok_or_else(|| "all programs failed")?;

    Ok((part1, part2))
}

fn run_with_phase(mem: &Vec<i64>, phase: [i64; 5]) -> Result<i64, Error> {
    phase.iter().try_fold(0, |out, &phase_value| {
        let output = run(mem.clone(), &[phase_value, out])?;
        assert_eq!(output.len(), 1);
        Ok(output[0])
    })
}

fn run_with_feedback_loop(mem: &Vec<i64>, phase: [i64; 5]) -> Result<i64, Error> {
    let (a_in, a_out) = run_async(mem.clone());
    a_in.send(phase[0]).unwrap();
    a_in.send(0).unwrap();

    let (b_in, b_out) = run_async(mem.clone());
    b_in.send(phase[1]).unwrap();

    let (c_in, c_out) = run_async(mem.clone());
    c_in.send(phase[2]).unwrap();

    let (d_in, d_out) = run_async(mem.clone());
    d_in.send(phase[3]).unwrap();

    let (e_in, e_out) = run_async(mem.clone());
    e_in.send(phase[4]).unwrap();

    let connect = |input: Receiver<i64>, output: Sender<i64>| {
        std::thread::spawn(move || {
            while let Ok(value) = input.recv() {
                if !output.send(value).is_ok() {
                    return Some(value);
                }
            }
            None
        })
    };

    connect(a_out, b_in);
    connect(b_out, c_in);
    connect(c_out, d_in);
    connect(d_out, e_in);
    let out = connect(e_out, a_in)
        .join()
        .map_err(|_| "E thread paniced")?
        .ok_or_else(|| "E stopped without output")?;

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let mem = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        assert_eq!(run_with_phase(&mem, [4, 3, 2, 1, 0]).unwrap(), 43210);

        let mem = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        assert_eq!(run_with_phase(&mem, [0, 1, 2, 3, 4]).unwrap(), 54321);

        let mem = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        assert_eq!(run_with_phase(&mem, [1, 0, 4, 3, 2]).unwrap(), 65210);
    }

    #[test]
    fn test_run_with_feedback_loop() {
        let mem = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        assert_eq!(
            run_with_feedback_loop(&mem, [9, 8, 7, 6, 5]).unwrap(),
            139629729
        );

        let mem = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];
        assert_eq!(
            run_with_feedback_loop(&mem, [9, 7, 8, 5, 6]).unwrap(),
            18216
        );
    }
}
