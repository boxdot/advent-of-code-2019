use itertools::*;
use std::io::{self, prelude::*};
use vm::Vm;
mod program;

fn force() {
    let mut to_checkpoint: String = "south
east
take whirled peas
west
north
north
east
take ornament
north
east
west
north
take dark matter
south
south
west
west
west
take candy cane
west
south
north
west
take tambourine
east
east
east
north
take astrolabe
west
east
east
take hologram
east
take klein bottle
west
south
west
"
    .into();

    let items = [
        "ornament",
        "astrolabe",
        "klein bottle",
        "dark matter",
        "candy cane",
        "hologram",
        "whirled peas",
        "tambourine",
    ];
    for item in &items {
        to_checkpoint += &format!("drop {}\n", item)
    }
    for i in 1..items.len() {
        for inventory in items.iter().combinations(i) {
            let mut input = to_checkpoint.clone();
            for item in inventory {
                input += &format!("take {}\n", item);
            }
            input += "north\n";
            let output: String = Vm::new(program::data(), input.bytes().map(|x| x as i64))
                .map(|x| x as u8 as char)
                .collect();
            if !output.contains("Alert! Droids on this ship are lighter than the detected value!")
                && !output
                    .contains("Alert! Droids on this ship are heavier than the detected value!")
            {
                println!("{}", output);
                break;
            }
        }
    }
}

fn run() {
    for x in Vm::new(
        program::data(),
        io::stdin().lock().bytes().map(|x| x.unwrap() as i64),
    ) {
        print!("{}", x as u8 as char);
    }
}

fn main() {
    run();
    force();
}
