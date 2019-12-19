use vm::Vm;
mod program;

fn solve1() -> (usize, Vec<Vec<u8>>) {
    let mut data: Vec<Vec<u8>> = vec![Vec::new()];
    for x in Vm::new(program::data(), [].iter().copied()) {
        if x == b'\n' as i64 {
            data.push(Vec::new());
        } else {
            data.last_mut().unwrap().push(x as u8);
        }
    }

    for x in &data {
        println!("{}", std::str::from_utf8(&x).unwrap());
    }

    let mut result = 0;
    for y in 1..data.len().saturating_sub(1) {
        for x in 1..data[y].len().saturating_sub(1) {
            if ![(x, y), (x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                .iter()
                .any(|(x, y)| data[*y][*x] != b'#')
            {
                result += x * y;
            }
        }
    }

    (result, data)
}

fn prune<'a>(
    mut s: &'a str,
    a: &'a str,
    b: &'a str,
    c: &'a str,
) -> impl Iterator<Item = (&'a str, &'a str)> + 'a {
    std::iter::from_fn(move || {
        if s.starts_with(a) {
            s = &s[a.len()..];
            Some(("A,", s))
        } else if s.starts_with(b) {
            s = &s[b.len()..];
            Some(("B,", s))
        } else if s.starts_with(c) {
            s = &s[c.len()..];
            Some(("C,", s))
        } else {
            None
        }
    })
}

fn solve2(data: Vec<Vec<u8>>) -> usize {
    let mut pos = data
        .iter()
        .enumerate()
        .map(|(y, x)| x.iter().enumerate().map(move |(x, c)| (x, y, *c)))
        .flatten()
        .find(|(_, _, c)| *c == b'^')
        .map(|(x, y, _)| (x as isize, y as isize))
        .unwrap();
    let mut dir = (0, -1);

    let mut instructions = String::new();
    loop {
        let mut try_move = |new_dir: (isize, isize), name: char| {
            let mut length = 0;
            for i in 1.. {
                match data
                    .get((pos.1 + i * new_dir.1) as usize)
                    .and_then(|r| r.get((pos.0 + i * new_dir.0) as usize))
                {
                    Some(c) if *c == b'#' => length = i,
                    _ => break,
                }
            }
            if length > 0 {
                instructions += &format!("{},{},", name, length);
                pos = (pos.0 + length * new_dir.0, pos.1 + length * new_dir.1);
                return Some(new_dir);
            }
            None
        };
        if let Some(new_dir) = try_move((dir.1, -dir.0), 'L') {
            dir = new_dir;
        } else if let Some(new_dir) = try_move((-dir.1, dir.0), 'R') {
            dir = new_dir;
        } else {
            break;
        }
    }
    println!("{}", instructions);

    // Try to find 3 substrings making up the whole
    for i in 1..instructions.len().min(22) {
        let a = &instructions[0..i];
        let a_rest = prune(&instructions[..], a, "Z", "Z").last().unwrap().1;
        for j in 1..a_rest.len().min(22) {
            let b = &a_rest[0..j];
            let b_rest = prune(&instructions[..], a, b, "Z").last().unwrap().1;
            for k in 1..b_rest.len().min(22) {
                let c = &b_rest[0..k];
                let c_rest = prune(&instructions[..], a, b, c).last().unwrap().1;
                if c_rest.is_empty() {
                    let main: String = prune(&instructions[..], a, b, c)
                        .flat_map(|(p, _)| p.chars())
                        .collect();
                    let input = format!(
                        "{}\n{}\n{}\n{}\nn\n",
                        &main[..main.len() - 1],
                        &a[..a.len() - 1],
                        &b[..b.len() - 1],
                        &c[..c.len() - 1]
                    );
                    println!("{}", input);
                    let mut program = program();
                    program[0] = 2;
                    for x in Vm::new(program, input.chars().map(|x| x as i64)) {
                        if x < 255 {
                            print!("{}", x as u8 as char);
                        } else {
                            println!("Result: {}", x);
                        }
                    }
                    break;
                }
            }
        }
    }

    unimplemented!()
}

fn main() {
    let p1 = solve1();
    println!("Part1:\n{}", p1.0);
    println!("Part2:\n{}", solve2(p1.1));
}
