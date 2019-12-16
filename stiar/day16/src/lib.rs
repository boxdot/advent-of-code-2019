use itertools::Itertools;

fn round_to_even(value: usize) -> usize {
    if value % 2 == 0 {
        value
    } else {
        value + 1
    }
}

fn perform_phase(input: &[u32], offset: usize) -> Vec<u32> {
    let mut partial_sum = vec![0];
    for (index, element) in input.iter().enumerate() {
        partial_sum.push(partial_sum[index] + element);
    }

    let last_index = input.len();
    let capped_index = |index| std::cmp::min(last_index, index);

    (offset..offset + input.len())
        .map(|i| {
            ((1..=round_to_even((offset + input.len()) / (i + 1)))
                .map(|x| x * (i + 1) - 1)
                .tuples()
                .map(|(left, right)| {
                    let mut value = 
                        (partial_sum[capped_index(right - offset)]
                            - partial_sum[capped_index(left - offset)])
                            as i32;
                    if ((left + 1) / (i + 1)) % 4 == 3 {
                        value = -value;
                    }
                    value
                })
                .sum::<i32>()
                .abs()
                % 10) as u32
        })
        .collect()
}

pub fn calculate_fft(mut input: Vec<u32>, phases: u32, offset: usize) -> u32 {
    input = input.drain(offset..).collect();

    for _ in 0..phases {
        input = perform_phase(&input, offset);
    }

    input[..8]
        .iter()
        .map(|digit| digit.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test_1() {
        assert_eq!(calculate_fft(vec![1, 2, 3, 4, 5, 6, 7, 8], 1, 0), 48226158);
        assert_eq!(calculate_fft(vec![1, 2, 3, 4, 5, 6, 7, 8], 4, 0), 1029498);
    }

    #[test]
    fn sample_test_2() {
        assert_eq!(
            calculate_fft(
                vec![
                    8, 0, 8, 7, 1, 2, 2, 4, 5, 8, 5, 9, 1, 4, 5, 4, 6, 6, 1, 9, 0, 8, 3, 2, 1, 8,
                    6, 4, 5, 5, 9, 5
                ],
                100,
                0
            ),
            24176176
        );
    }

    #[test]
    fn sample_test_3() {
        assert_eq!(
            calculate_fft(
                std::iter::repeat(vec![
                    0, 3, 0, 3, 6, 7, 3, 2, 5, 7, 7, 2, 1, 2, 9, 4, 4, 0, 6, 3, 4, 9, 1, 5, 6, 5,
                    4, 7, 4, 6, 6, 4
                ])
                .take(10000)
                .flatten()
                .collect(),
                100,
                0303673
            ),
            84462026
        );
    }
}
