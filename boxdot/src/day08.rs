pub fn solve(input: &str) -> Option<(usize, String)> {
    const WIDTH: usize = 25;
    const HEIGHT: usize = 6;

    let layers: Vec<&[u8]> = input.as_bytes().chunks(WIDTH * HEIGHT).collect();

    let part1 = layers
        .iter()
        .map(|layer| {
            let num_zeroes = bytecount::count(layer, b'0');
            let num_ones = bytecount::count(layer, b'1');
            let num_twos = bytecount::count(layer, b'2');
            (num_zeroes, num_ones, num_twos)
        })
        .min_by_key(|&(num_zeroes, _, _)| num_zeroes)
        .map(|(_, num_ones, num_twos)| num_ones * num_twos)?;

    let mut img = [b'2'; WIDTH * HEIGHT];
    for layer in layers.into_iter() {
        for i in 0..WIDTH * HEIGHT {
            let color = match (img[i], layer[i]) {
                (b'0', _) => b'0',
                (b'1', _) => b'1',
                (b'2', x) => x,
                _ => unreachable!(),
            };
            img[i] = color;
        }
    }

    let part2 = img.chunks(WIDTH).fold(String::new(), |mut msg, line| {
        msg.push_str(std::str::from_utf8(line).unwrap());
        msg.push('\n');
        msg
    });
    println!("{}", part2);

    Some((part1, part2))
}
