use std::collections::HashMap;

fn get_orbit<'a>(key: &'a str, rel: &HashMap<&'a str, &'a str>) -> Vec<&'a str> {
    let mut result = Vec::new();
    let mut it = key;
    while rel.contains_key(it) {
        result.push(it);
        it = &rel[it];
    }
    result.reverse();
    result
}

fn main() {
    let contents = include_str!("day06.txt");
    let pairs: Vec<_> = contents
        .lines()
        .map(|l| l.split(')'))
        .map(|mut s| (s.next().unwrap(), s.next().unwrap()))
        .collect();
    let rel: HashMap<&str, &str> = pairs.iter().cloned().map(|(x, y)| (y, x)).collect();
    let my = get_orbit("YOU", &rel);
    let santa = get_orbit("SAN", &rel);
    let same = my.iter().zip(&santa).filter(|p| *p.0 == *p.1).count();
    println!("{} {}", my.len(), santa.len());
    let jumps = my.len() + santa.len() - 2 * same - 2;
    println!("{}", jumps);
}
