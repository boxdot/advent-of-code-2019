fn main() {
    let contents: Vec<char> = include_str!("day08.txt").chars().collect();
    let w = 25;
    let h = 6;
    let mut result = vec!['2'; w * h];
    let layers = contents.chunks_exact(w * h);
    for layer in layers {
        for x in result.iter_mut().zip(layer.iter()) {
            let r = x.0;
            let l = x.1;
            if *r == '2' {
                *r = *l;
            }
        }
    }
    for line in result.chunks_exact(w) {
        for c in line {
            print!("{}", if *c == '0' { '.' } else { '█' });
        }
        println!();
    }
}
