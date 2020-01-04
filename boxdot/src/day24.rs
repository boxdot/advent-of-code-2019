use std::collections::HashSet;

pub fn solve(input: &str) -> u32 {
    let mut state = parse(input);
    let mut prev = HashSet::new();
    while prev.insert(state) {
        state = next_gen(state);
    }
    state
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

fn next_gen(gen: u32) -> u32 {
    (0..5 * 5)
        .map(|i| {
            let is_alive = (gen >> i) & 1 == 1;

            let x = (i % 5) as isize;
            let y = (i / 5) as isize;
            let num_neighbours = [(1, 0), (-1, 0), (0, 1), (0, -1)]
                .iter()
                .filter_map(|&(dx, dy)| {
                    let x = x + dx;
                    let y = y + dy;
                    if 0 <= x && x < 5 && 0 <= y && y < 5 {
                        Some((x, y))
                    } else {
                        None
                    }
                })
                .filter(|(x, y)| (gen >> (x + 5 * y)) & 1 == 1)
                .count();

            ((is_alive && num_neighbours == 1)
                || (!is_alive && (num_neighbours == 1 || num_neighbours == 2))) as u32
        })
        .rev()
        .fold(0, |acc, bit| (acc << 1) | bit)
}
