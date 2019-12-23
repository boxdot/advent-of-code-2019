mod intcode;
use intcode::*;

use itertools::Itertools;

use std::collections::{VecDeque, HashSet};

#[derive(Debug, Copy, Clone)]
struct Message {
    pub to: usize,
    pub x: i64,
    pub y: i64,
}

fn run_computer(
    iter: impl Iterator<Item = i64>,
    queue: &mut Vec<VecDeque<Message>>,
    nat: &mut Option<Message>,
) {
    for (to, x, y) in iter.tuples() {
        if to == 255 {
            *nat = Some(Message { to: 0, x, y });
        } else {
            queue[to as usize].push_back(Message {
                to: to as usize,
                x,
                y,
            });
        }
    }
}

pub fn run_computers(program: Vec<i64>, size: usize, nat: bool) -> i64 {
    let mut computers: Vec<_> = (0..size).map(|_| Program::new(program.clone())).collect();
    let mut queue: Vec<VecDeque<Message>> = vec![VecDeque::new(); size];
    let mut nat_message = None;

    for (index, computer) in computers.iter_mut().enumerate() {
        run_computer(
            computer.run(std::iter::once(index as i64)),
            &mut queue,
            &mut nat_message,
        );
        if !nat && nat_message.is_some() {
            return nat_message.unwrap().y;
        }
    }

    let mut nat_sent = HashSet::new();

    loop {
        let mut advanced = false;
        for computer_index in 0..size {
            if let Some(message) = queue[computer_index].pop_front() {
                advanced = true;
                assert_eq!(computer_index, message.to);
                run_computer(
                    computers[computer_index].run([message.x, message.y].iter().cloned()),
                    &mut queue,
                    &mut nat_message,
                );
                if !nat && nat_message.is_some() {
                    return nat_message.unwrap().y;
                }
            } else {
                run_computer(
                    computers[computer_index].run(std::iter::once(-1)),
                    &mut queue,
                    &mut nat_message,
                );
                if !nat && nat_message.is_some() {
                    return nat_message.unwrap().y;
                }
            }
        }
        if nat && !advanced {
            assert!(nat_message.is_some());
            let message = nat_message.unwrap();
            if nat_sent.contains(&message.y) {
                return message.y;
            }
            nat_sent.insert(message.y);
            queue[message.to].push_back(message);
        }
    }
}
