fn digits(mut x: u32) -> Vec<u32> {
    let mut result = Vec::new();
    while x > 0 {
        result.push(x % 10);
        x /= 10;
    }
    result.reverse();
    result
}

fn main() {
    let range = 273_025..767_253;
    let mut n = 0;
    'outer: for x in range {
        let ds = digits(x);
        assert!(ds.len() == 6);
        let mut last = 0;
        let mut same = 0;
        for d in ds {
            if d < last {
                continue 'outer;
            }
            if d == last {
                same += 1;
            }
            last = d;
        }
        if same == 0 {
            continue 'outer;
        }
        n += 1;
    }
    println!("{}", n);
}
