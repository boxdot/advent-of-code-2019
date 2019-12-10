fn image_checksum(input: &str, width: usize, height: usize) -> usize {
    input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|n| n as u8)
        .collect::<Vec<u8>>()
        .chunks(width * height)
        .min_by_key(|layer| layer.iter().filter(|&px| *px == 0).count())
        .map(|layer| {
            layer.iter().filter(|&d| *d == 1).count() * layer.iter().filter(|&d| *d == 2).count()
        })
        .unwrap_or(0)
}

fn decode_image(input: &str, width: usize, height: usize) -> Vec<u8> {
    let layers = input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|n| n as u8)
        .collect::<Vec<u8>>();

    let image = (0..width * height)
        .into_iter()
        .map(|px| {
            for layer in layers.chunks(width * height) {
                if layer[px] != 2 {
                    return layer[px];
                }
            }
            2
        })
        .collect();

    image
}

#[test]
fn test_everything() {
    assert_eq!(image_checksum("123456789012", 3, 2), 1);
    assert_eq!(decode_image("0222112222120000", 2, 2), &[0, 1, 1, 0]);
}

pub fn unlock(input: &str) -> Result<(usize, usize), Box<dyn std::error::Error>> {
    let width = 25;
    let height = 6;
    let part1 = image_checksum(input, width, height);
    let image = decode_image(input, width, height);
    for y in 0..height {
        for x in 0..width {
            print!(
                "{}",
                if image[x + y * width] == 0 {
                    " "
                } else {
                    "⭐"
                }
            );
        }
        println!("");
    }
    Ok((part1, 0))
}
