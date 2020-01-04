use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> (u32, usize) {
    let mut state = parse(input);
    let mut prev = HashSet::new();
    while prev.insert(state) {
        state = next_gen(state);
    }
    let part1 = state;

    let state = parse(input);
    let mut gen = HashMap::new();
    gen.insert(0, state);
    let gen = (0..200).fold(gen, |gen, _| next_gen_rec(gen));
    let part2 = gen
        .values()
        .map(|&state| (0..5 * 5).filter(|&i| is_bug_at_index(state, i)).count())
        .sum();

    (part1, part2)
}

fn parse(input: &str) -> u32 {
    input
        .bytes()
        .filter_map(|c| match c {
            b'#' => Some(1),
            b'.' => Some(0),
            _ => None,
        })
        .rev()
        .fold(0_u32, |acc, bit| (acc << 1) | bit)
}

fn is_bug_at_index(state: u32, index: isize) -> bool {
    (state >> index) & 1 == 1
}

fn state_folder(state: u32, bit: u32) -> u32 {
    state << 1 | bit
}

fn next_gen(state: u32) -> u32 {
    (0..5 * 5)
        .map(|i| {
            let is_alive = is_bug_at_index(state, i);
            let num_neighbors = neighbors(i).filter(|&i| is_bug_at_index(state, i)).count();
            (!is_alive && num_neighbors == 2 || num_neighbors == 1) as u32
        })
        .rev()
        .fold(0, state_folder)
}

fn neighbors(index: isize) -> impl Iterator<Item = isize> {
    const NEIGHBORS: [&[isize]; 25] = [
        &[1, 5],
        &[2, 0, 6],
        &[3, 1, 7],
        &[4, 2, 8],
        &[3, 9],
        &[6, 10, 0],
        &[7, 5, 11, 1],
        &[8, 6, 12, 2],
        &[9, 7, 13, 3],
        &[8, 14, 4],
        &[11, 15, 5],
        &[12, 10, 16, 6],
        &[13, 11, 17, 7],
        &[14, 12, 18, 8],
        &[13, 19, 9],
        &[16, 20, 10],
        &[17, 15, 21, 11],
        &[18, 16, 22, 12],
        &[19, 17, 23, 13],
        &[18, 24, 14],
        &[21, 15],
        &[22, 20, 16],
        &[23, 21, 17],
        &[24, 22, 18],
        &[23, 19],
    ];
    NEIGHBORS[index as usize].iter().copied()
}

fn next_gen_rec(gen: HashMap<isize, u32>) -> HashMap<isize, u32> {
    let min_level = *gen.keys().min().unwrap();
    let max_level = *gen.keys().max().unwrap();

    let mut next_gen = HashMap::new();
    for level in min_level - 1..=max_level + 1 {
        let state = gen.get(&level).copied().unwrap_or(0);
        let next_state = (0..5 * 5)
            .map(|i| {
                let is_alive = is_bug_at_index(state, i);
                let num_neighbors = neighbors_rec(i)
                    .filter(|(i, dlevel)| {
                        gen.get(&(level + dlevel))
                            .map(|&state| is_bug_at_index(state, *i))
                            .unwrap_or(false)
                    })
                    .count();

                (!is_alive && num_neighbors == 2 || num_neighbors == 1) as u32
            })
            .rev()
            .fold(0, state_folder);
        if next_state != 0 {
            next_gen.insert(level, next_state);
        }
    }

    next_gen
}

fn neighbors_rec(index: isize) -> impl Iterator<Item = (isize, isize)> {
    const NEIGHBORS: [&[(isize, isize)]; 25] = [
        &[(1, 0), (11, -1), (5, 0), (7, -1)],
        &[(2, 0), (0, 0), (6, 0), (7, -1)],
        &[(3, 0), (1, 0), (7, 0), (7, -1)],
        &[(4, 0), (2, 0), (8, 0), (7, -1)],
        &[(13, -1), (3, 0), (9, 0), (7, -1)],
        &[(6, 0), (11, -1), (10, 0), (0, 0)],
        &[(7, 0), (5, 0), (11, 0), (1, 0)],
        &[
            (8, 0),
            (6, 0),
            (0, 1),
            (1, 1),
            (2, 1),
            (3, 1),
            (4, 1),
            (2, 0),
        ],
        &[(9, 0), (7, 0), (13, 0), (3, 0)],
        &[(13, -1), (8, 0), (14, 0), (4, 0)],
        &[(11, 0), (11, -1), (15, 0), (5, 0)],
        &[
            (0, 1),
            (5, 1),
            (10, 1),
            (15, 1),
            (20, 1),
            (10, 0),
            (16, 0),
            (6, 0),
        ],
        &[],
        &[
            (14, 0),
            (4, 1),
            (9, 1),
            (14, 1),
            (19, 1),
            (24, 1),
            (18, 0),
            (8, 0),
        ],
        &[(13, -1), (13, 0), (19, 0), (9, 0)],
        &[(16, 0), (11, -1), (20, 0), (10, 0)],
        &[(17, 0), (15, 0), (21, 0), (11, 0)],
        &[
            (18, 0),
            (16, 0),
            (22, 0),
            (20, 1),
            (21, 1),
            (22, 1),
            (23, 1),
            (24, 1),
        ],
        &[(19, 0), (17, 0), (23, 0), (13, 0)],
        &[(13, -1), (18, 0), (24, 0), (14, 0)],
        &[(21, 0), (11, -1), (17, -1), (15, 0)],
        &[(22, 0), (20, 0), (17, -1), (16, 0)],
        &[(23, 0), (21, 0), (17, -1), (17, 0)],
        &[(24, 0), (22, 0), (17, -1), (18, 0)],
        &[(13, -1), (23, 0), (17, -1), (19, 0)],
    ];
    NEIGHBORS[index as usize].iter().copied()
}
