use crate::matrix_multiplication::{
    naive::naive_matrix_multiply, Matrix, MatrixDimensions, MatrixIndex,
};

#[test]
fn trivial_multiplications() {
    let m_1 = Matrix::from_data(
        MatrixDimensions {
            rows: 1,
            columns: 1,
        },
        vec![1],
    );
    let m_2 = Matrix::from_generator(
        MatrixDimensions {
            columns: 5,
            rows: 1,
        },
        |MatrixIndex { x, y }| x + y,
    );

    assert_eq!(naive_matrix_multiply(&m_1, &m_2), m_2);
}
