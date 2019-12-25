mod intcode;
pub use intcode::*;

pub fn run_command(program: &mut Program, iter: impl Iterator<Item = i64>) -> String {
    program.run(iter).map(|c| (c as u8) as char).collect()
}

pub fn to_ascii(input: &str) -> Vec<i64> {
    input.chars().map(|c| (c as u8) as i64).collect()
}

pub fn collect_inventory(program: &mut Program) {
    let commands = [
        "south",
        "west",
        "north",
        "take fuel cell",
        "south",
        "east",
        "north",
        "west",
        "west",
        "east",
        "east",
        "north",
        "east",
        "take candy cane",
        "south",
        "take hypercube",
        "east",
        "west",
        "north",
        "north",
        "south",
        "west",
        "north",
        "take coin",
        "east",
        "take tambourine",
        "west",
        "west",
        "take spool of cat6",
        "north",
        "take weather machine",
        "west",
        "take mutex",
        "west",
    ];

    for command in commands.iter() {
        println!(
            "{}",
            run_command(program, to_ascii(&(command.to_string() + "\n")).into_iter())
        );
    }
}

pub fn try_pass(program: Program) {
    let inventory = [
        "spool of cat6",
        "hypercube",
        "weather machine",
        "coin",
        "candy cane",
        "tambourine",
        "fuel cell",
        "mutex",
    ];

    for subset in (0..1 << inventory.len()).map(|mask| {
        inventory
            .iter()
            .enumerate()
            .filter_map(|(i, x)| if (mask >> i) % 2 == 1 { Some(x) } else { None })
            .collect::<Vec<_>>()
    }) {
        let mut p = program.clone();
        println!("{}", run_command(&mut p, std::iter::empty()));
        collect_inventory(&mut p);
        for element in subset.iter() {
            println!(
                "{}",
                run_command(&mut p, to_ascii(&format!("drop {}\n", element)).into_iter())
            );
        }
        let output = run_command(&mut p, to_ascii("west\n").into_iter());
        if !output.contains("Alert! Droids on this ship") {
            println!("Subset to drop: {:?}", subset);
            println!("{}", output);
            return;
        }
    }
}
