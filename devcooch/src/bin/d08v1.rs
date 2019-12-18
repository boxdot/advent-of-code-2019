fn count_by_char<'a>(l: impl Iterator<Item = &'a char>, c: char) -> usize {
    l.filter(|&x| *x == c).count()
}

fn main() {
    let contents: Vec<char> = include_str!("day08.txt").chars().collect();
    let w = 25;
    let h = 6;
    let layers = contents.chunks_exact(w * h);
    let best = layers.min_by_key(|x| count_by_char(x.iter(), '0')).unwrap();
    let n = count_by_char(best.iter(), '1') * count_by_char(best.iter(), '2');
    println!("{}", n);
}
