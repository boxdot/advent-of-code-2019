fn digits(mut x: u32) -> Vec<u32> {
    let mut result = Vec::new();
    while x > 0 {
        result.push(x % 10);
        x /= 10;
    }
    result.reverse();
    result
}

fn test_number(x: u32) -> bool {
    let ds = digits(x);
    assert!(ds.len() == 6);
    let mut last = 0;
    let mut same_count = 1;
    let mut same_group2 = 0;
    for d in ds {
        if d < last {
            return false;
        }
        if d == last {
            same_count += 1;
        } else {
            if same_count == 2 {
                same_group2 += 1;
            }
            same_count = 1;
        }
        last = d;
    }
    if same_count == 2 {
        same_group2 += 1;
    }
    if same_group2 == 0 {
        return false;
    }
    return true;
}

fn main() {
    assert_eq!(test_number(112233), true);
    assert_eq!(test_number(123444), false);
    assert_eq!(test_number(111122), true);
    let range = 273025..767253;
    let mut n = 0;
    for x in range {
        if test_number(x) {
            n += 1;
        }
    }
    println!("{}", n);
}
