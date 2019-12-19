use std::collections::HashMap;
use std::collections::VecDeque;
use vm::Vm;
mod program;

fn solve() -> (usize, usize) {
    let mut seen = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back(((0, 0), Vec::new()));
    seen.insert((0, 0), std::usize::MAX);
    let mut destination = None;
    while let Some((pos, dirs)) = queue.pop_front() {
        'moving: for (dir, new_pos) in &[
            (1, (pos.0, pos.1 - 1)),
            (2, (pos.0, pos.1 + 1)),
            (3, (pos.0 - 1, pos.1)),
            (4, (pos.0 + 1, pos.1)),
        ] {
            if let Some(_) = seen.get(&new_pos) {
                continue;
            }
            match Vm::new(
                program::data(),
                dirs.iter().chain(std::iter::once(dir)).cloned(),
            )
            .last()
            {
                Some(0) => continue 'moving,
                Some(1) => (),
                Some(2) => destination = Some(*new_pos),

                e => panic!("Unknown output {:?}", e),
            }

            seen.insert(*new_pos, std::usize::MAX);
            let mut new_dirs = dirs.clone();
            new_dirs.push(*dir);
            queue.push_back((*new_pos, new_dirs));
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back((0, destination.unwrap()));
    seen.insert(destination.unwrap(), 0);
    while let Some((dist, pos)) = queue.pop_front() {
        for new_pos in &[
            (pos.0, pos.1 - 1),
            (pos.0, pos.1 + 1),
            (pos.0 - 1, pos.1),
            (pos.0 + 1, pos.1),
        ] {
            seen.entry(*new_pos).and_modify(|v| {
                if *v > dist + 1 {
                    *v = dist + 1;
                    queue.push_back((dist + 1, *new_pos));
                }
            });
        }
    }

    (*seen.get(&(0, 0)).unwrap(), *seen.values().max().unwrap())
}

fn main() {
    println!("Part1:\n{:?}", solve());
}
