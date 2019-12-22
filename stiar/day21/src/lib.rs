mod intcode;
use intcode::*;

#[macro_use]
extern crate itertools;

use itertools::Itertools;

fn into_ascii(cmd: &str) -> Vec<i64> {
    cmd.chars().map(|c| (c as u8) as i64).collect()
}

pub fn generate_walking_robot_program() -> Vec<i64> {
    ["NOT A J\n", "NOT C T\n", "AND D T\n", "OR T J\n", "WALK\n"]
        .into_iter()
        .map(|x| into_ascii(x))
        .flatten()
        .collect()
}

pub fn generate_running_robot_program() -> Vec<i64> {
    [
        "NOT B T\n",
        "NOT C J\n",
        "OR T J\n",
        "AND D J\n",
        "AND H J\n",
        "NOT A T\n",
        "OR T J\n",
        "RUN\n",
    ]
    .into_iter()
    .map(|x| into_ascii(x))
    .flatten()
    .collect()
}

pub fn run_robot(emulator: Vec<i64>, commands: Vec<i64>, print: bool) -> Option<i64> {
    let mut program = Program::new(emulator);
    for c in program.run(commands.into_iter()) {
        if c <= 256 {
            if print {
                print!("{}", (c as u8) as char);
            }
        } else {
            return Some(c);
        }
    }
    None
}

pub fn find_running_program(emulator: Vec<i64>) {
    let operators = ["NOT", "OR", "AND"];
    let input_registers = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "T", "J"];
    let output_registers = ["T", "J"];
    let combinations: Vec<_> = iproduct!(
        operators.iter(),
        input_registers.iter(),
        output_registers.iter()
    )
    .filter(|(&op, &i, &o)| {
        i != o || (op == "NOT" && o == "J" && i != "A")
    })
    .map(|(op, i, o)| format!("{} {} {}\n", op, i, o))
    .collect();
    println!("Number of combinations {}", combinations.len());
    let mut index = 0;
    for size in 1..10 {
        for commands in (0..size).map(|_| combinations.iter().cloned()).multi_cartesian_product() {
            index += 1;
            if index % 1000 == 0 {
                println!("Processed {}", index);
            }
            if commands.iter().unique().count() < 3 {
                continue;
            }
            let mut commands_copy = commands.clone();
            commands_copy.push("AND D J\n".into());
            commands_copy.push("AND H J\n".into());
            commands_copy.push("NOT A T\n".into());
            commands_copy.push("OR T J\n".into());
            commands_copy.push("RUN\n".into());
            if run_robot(
                emulator.clone(),
                commands_copy.clone()
                .into_iter()
                .map(|x| into_ascii(&x))
                .flatten()
                .collect(),
                false,
            )
            .is_some()
            {
                println!("{:?}", commands_copy);
                return;
            }
        }
    }
    unimplemented!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
