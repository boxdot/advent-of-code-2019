use itertools::*;
use std::sync::mpsc::channel;
use vm::Vm;
mod program;

fn solve(break_on_first_nat: bool) {
    let mut vms: Vec<(_, _)> = (0..50)
        .map(|addr| {
            let (sender, receiver) = channel();
            let vm = Vm::new(
                program::data(),
                vec![addr].into_iter().chain(std::iter::from_fn(move || {
                    Some(receiver.try_iter().next().unwrap_or(-1))
                })),
            );
            (sender, vm)
        })
        .collect();

    let mut last_nat = (0, 0);
    let mut last_send = None;
    let mut time_since_last_package = 0;
    loop {
        let mut some_data = false;
        for i in 0..vms.len() {
            if let Some(output) = vms[i].1.single_step() {
                if let Some(target) = output {
                    let (x, y) = vms[i].1.next_tuple().unwrap();
                    if target as usize >= vms.len() {
                        last_nat = (x, y);
                        if break_on_first_nat {
                            println!("Payload to NAT: {},{} -> {}", x, y, target);
                            return;
                        }
                    } else {
                        vms[target as usize].0.send(x).unwrap();
                        vms[target as usize].0.send(y).unwrap();
                        some_data = true;
                    }
                }
            }
        }
        if some_data {
            time_since_last_package = 0;
        } else {
            time_since_last_package += 1;
        }
        if time_since_last_package > 2000 {
            // Too lazy to check input, just check output + timer
            time_since_last_package = 0;
            if last_send == Some(last_nat) {
                println!("2nd Package: {}, {}", last_nat.0, last_nat.1);
                return;
            }
            last_send = Some(last_nat);
            vms[0].0.send(last_nat.0).unwrap();
            vms[0].0.send(last_nat.1).unwrap();
        }
    }
}

fn main() {
    solve(true);
    solve(false);
}
