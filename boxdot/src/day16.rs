pub fn solve(input: &str) -> (i64, i64) {
    let mut seq = parse(input);
    let mut buf = vec![0; seq.len()];
    for _ in 0..100 {
        phase(&seq, &mut buf);
        std::mem::swap(&mut seq, &mut buf);
    }
    let part1 = to_number(seq[0..8].iter());

    (part1, part2(&parse(input)))
}

fn parse(input: &str) -> Vec<u8> {
    input
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as u8))
        .collect()
}

fn pattern_ones(n: usize) -> impl Iterator<Item = usize> {
    (0..)
        .map(move |i| n * (1 + 4 * i)..n * (2 + 4 * i))
        .flatten()
}

fn pattern_minus_ones(n: usize) -> impl Iterator<Item = usize> {
    (0..)
        .map(move |i| n * (3 + 4 * i)..n * (4 + 4 * i))
        .flatten()
}

fn phase(input: &[u8], output: &mut [u8]) {
    assert_eq!(input.len(), output.len());
    for (i, out) in output.iter_mut().enumerate() {
        let x: i64 = pattern_ones(i + 1)
            .take_while(|&pos| pos < input.len() + 1)
            .map(|pos| input[pos - 1] as i64)
            .sum();
        let y: i64 = pattern_minus_ones(i + 1)
            .take_while(|&pos| pos < input.len() + 1)
            .map(|pos| input[pos - 1] as i64)
            .sum();
        *out = ((x - y) % 10).abs() as u8;
    }
}

fn part2(seq: &[u8]) -> i64 {
    let mut seq = seq.repeat(10_000);
    let offset: usize = to_number(seq[0..7].iter()) as usize;
    let mut seq = seq.split_off(offset);

    // The pattern at x is the following when x > len / 2
    //
    // [0 .. x x + 1 .. len]
    // [0 .. 0 1     .. 1  ]
    //
    // That's why the calcuation is so simple, since the offset is big!
    let mut buf = vec![0; seq.len()];
    for _ in 0..100 {
        let mut sum = 0;
        for i in (0..seq.len()).rev() {
            sum = (sum + seq[i]) % 10;
            buf[i] = sum;
        }
        std::mem::swap(&mut seq, &mut buf);
    }

    to_number(seq[0..8].iter())
}

fn to_number<'a>(digits: impl Iterator<Item = &'a u8> + DoubleEndedIterator) -> i64 {
    digits
        .rev()
        .enumerate()
        .map(|(e, &d)| d as i64 * 10_i64.pow(e as u32))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase() {
        let mut out = vec![0; 8];

        phase(&vec![1, 2, 3, 4, 5, 6, 7, 8], &mut out);
        assert_eq!(out, vec![4, 8, 2, 2, 6, 1, 5, 8]);

        phase(&out.clone(), &mut out);
        assert_eq!(out, vec![3, 4, 0, 4, 0, 4, 3, 8]);
    }

    #[test]
    fn test_part2() {
        let res = part2(&parse("03036732577212944063491565474664"));
        assert_eq!(res, 84462026);

        let res = part2(&parse("02935109699940807407585447034323"));
        assert_eq!(res, 78725270);

        let res = part2(&parse("03081770884921959731165446850517"));
        assert_eq!(res, 53553731);
    }
}
