use quickcheck::TestResult;
use quickcheck_macros::quickcheck;

use super::*;

#[test]
#[should_panic]
fn short_iterator() {
    Matrix::from_iterator(
        MatrixDimensions {
            width: 10,
            height: 10,
        },
        (0..2).into_iter(),
    );
}

#[quickcheck]
fn from_generator(width: u8, height: u8) -> TestResult {
    let width = width as usize;
    let height = height as usize;

    if width == 0 || height == 0 {
        return TestResult::discard();
    }

    let dimensions = MatrixDimensions { width, height };

    let generator = |MatrixIndex { x, y }| x + width * y;
    let matrix = Matrix::from_generator(dimensions, generator);

    TestResult::from_bool(
        matrix
            == Matrix {
                dimensions,
                data: (0..width * height).into_iter().collect(),
            },
    )
}

#[quickcheck]
fn from_iterator(width: u8, height: u8) -> TestResult {
    let width = width as usize;
    let height = height as usize;

    if width == 0 || height == 0 {
        return TestResult::discard();
    }

    let dimensions = MatrixDimensions { width, height };
    let matrix = Matrix::from_iterator(dimensions, (0..width * height).into_iter());

    TestResult::from_bool(
        matrix
            == Matrix {
                dimensions,
                data: (0..width * height).into_iter().collect(),
            },
    )
}
