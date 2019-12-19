use permutator::Permutation;
use std::io::{self, prelude::*};
use std::sync::mpsc::channel;
use vm::Vm;

fn parse(input: &Vec<String>) -> Vec<i64> {
    let ints = input.iter().map(|x| x.split(',')).flatten();
    ints.filter_map(|x| x.parse().ok()).collect()
}

fn solve1(mem: &Vec<i64>) -> i64 {
    let mut result = 0;
    vec![0, 1, 2, 3, 4].permutation().for_each(|perm| {
        let vm = Vm::new(mem.clone(), vec![perm[0], 0].into_iter());
        let vm = Vm::new(mem.clone(), std::iter::once(perm[1]).chain(vm));
        let vm = Vm::new(mem.clone(), std::iter::once(perm[2]).chain(vm));
        let vm = Vm::new(mem.clone(), std::iter::once(perm[3]).chain(vm));
        let mut vm = Vm::new(mem.clone(), std::iter::once(perm[4]).chain(vm));
        result = result.max(vm.next().unwrap());
    });
    result
}

fn solve2(mem: &Vec<i64>) -> i64 {
    let mut result = 0;
    vec![5, 6, 7, 8, 9].permutation().for_each(|perm| {
        let (sender, receiver) = channel();
        let vm = Vm::new(
            mem.clone(),
            vec![perm[0], 0].into_iter().chain(receiver.into_iter()),
        );
        let vm = Vm::new(mem.clone(), std::iter::once(perm[1]).chain(vm));
        let vm = Vm::new(mem.clone(), std::iter::once(perm[2]).chain(vm));
        let vm = Vm::new(mem.clone(), std::iter::once(perm[3]).chain(vm));
        let vm = Vm::new(mem.clone(), std::iter::once(perm[4]).chain(vm));
        let vm = vm.map(|item| {
            sender.send(item).unwrap();
            item
        });
        result = result.max(vm.last().unwrap());
    });
    result
}

fn main() {
    let input = parse(&io::stdin().lock().lines().map(|x| x.unwrap()).collect());
    println!("Part1: {:?}", solve1(&input.clone()));
    println!("Part2: {:?}", solve2(&input.clone()));
}
