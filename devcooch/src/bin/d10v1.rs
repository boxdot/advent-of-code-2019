fn main() {
    let data = include_str!("day10.txt");
    let map: Vec<Vec<bool>> = data
        .lines()
        .map(|x| x.chars().map(|y| y == '#').collect())
        .collect();
    let asteroids: Vec<(usize, usize)> = map
        .iter()
        .enumerate()
        .map(|x| x.1.iter().enumerate().map(|y| y.1))
        .collect();
    println!("{:?}", asteroids);
}
