use ndarray::prelude::*;

type LayeredImage = Array3<u32>;
type Image = Array2<u32>;

pub fn checksum(image: &LayeredImage) -> usize {
    image
        .outer_iter()
        .into_iter()
        .map(|layer| {
            (
                layer.iter().filter(|&&v| v == 0).count(),
                layer.iter().filter(|&&v| v == 1).count()
                    * layer.iter().filter(|&&v| v == 2).count(),
            )
        })
        .min_by(|x, y| x.0.cmp(&y.0))
        .unwrap()
        .1
}

pub fn decode(image: &LayeredImage) -> Image {
    Array::from_shape_vec(
        (image.shape()[1], image.shape()[2]),
        image
            .gencolumns()
            .into_iter()
            .map(|column| {
                column
                    .into_iter()
                    .filter(|&&v| v != 2)
                    .next()
                    .cloned()
                    .unwrap_or(2)
            })
            .collect(),
    )
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test_1() {
        assert_eq!(
            checksum(
                &Array::from_shape_vec((2, 2, 3), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2])
                    .unwrap()
            ),
            1
        );
    }

    #[test]
    fn sample_test_2() {
        assert_eq!(
            decode(
                &Array::from_shape_vec(
                    (4, 2, 2),
                    vec![0, 2, 2, 2, 1, 1, 2, 2, 2, 2, 1, 2, 0, 0, 0, 0],
                )
                .unwrap(),
            ),
            Array::from_shape_vec((2, 2), vec![0, 1, 1, 0]).unwrap()
        );
    }
}
