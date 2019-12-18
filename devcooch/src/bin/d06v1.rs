use std::collections::HashMap;

fn main() {
    let contents = include_str!("day06.txt");
    let pairs: Vec<_> = contents
        .lines()
        .map(|l| l.split(')'))
        .map(|mut s| (s.next().unwrap(), s.next().unwrap()))
        .collect();
    let rel: HashMap<&str, &str> = pairs.iter().cloned().map(|(x, y)| (y, x)).collect();
    let mut tot = 0;
    for mut k in rel.keys() {
        while rel.contains_key(k) {
            tot += 1;
            k = &rel[k];
        }
    }
    println!("{}", tot);
}
