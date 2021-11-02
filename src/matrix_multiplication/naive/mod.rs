/*! # Chapter 4.2 - Naive Matrix Multiplication Algorithm
  
 **Input:** An `m * n` matrix `A` and a `n * p` matrix `B`.

 **Output:** The matrix product `AB`.

 **Time complexity:** `O(mnp)`

 The basic, immediate matrix multiplication algorithm given by the definition of matrix
 multiplication. In the book, this algorithm doesn't get its own section, so we categorize it under
 chapter 4.2 where it is first shown, which is the chapter about Strassen's algorithm. Notably, the
 book shows pseudocode for square matrices only; we generalize this to allow varying dimensions.
*/
use crate::matrix_multiplication::{MatrixDimensions, MatrixIndex};

use super::Matrix;

#[cfg(test)]
mod test;

pub fn naive_matrix_multiply<T>(m_1: &Matrix<T>, m_2: &Matrix<T>) -> Matrix<T>
where
    T: Default + Clone + std::ops::Mul<Output = T> + std::ops::AddAssign,
{
    assert_eq!(
        m_1.dimensions.columns, m_2.dimensions.rows,
        "First matrix' column count must match second matrix' row count!"
    );

    let mut data = Vec::with_capacity(m_1.dimensions.rows * m_2.dimensions.columns);

    for y in 0..m_1.dimensions.rows {
        for x in 0..m_2.dimensions.columns {
            let mut sum = T::default();
            for i in 0..m_1.dimensions.columns {
                sum += m_1[MatrixIndex { x: i, y }].clone() * m_2[MatrixIndex { x, y: i }].clone();
            }

            data.push(sum);
        }
    }

    Matrix::from_data(
        MatrixDimensions {
            columns: m_2.dimensions.columns,
            rows: m_1.dimensions.rows,
        },
        data,
    )
}
