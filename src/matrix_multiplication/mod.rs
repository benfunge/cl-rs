/*! # Matrix Multiplication Algorithms

 This module contains all the different matrix multiplication algorithms introduced in the book, as
 well as a `Matrix` type at the top level.
*/
use std::ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign};

pub mod naive;

#[cfg(test)]
mod test;

/// The size of a matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MatrixDimensions {
    pub width: usize,
    pub height: usize,
}

/// Index type into a matrix
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MatrixIndex {
    pub x: usize,
    pub y: usize,
}

/// A matrix of `dimensions.width * dimensions.height` elements of type `T`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix<T> {
    dimensions: MatrixDimensions,
    data: Vec<T>,
}

impl<T> Matrix<T> {
    /// Get the dimensions of a matrix.
    pub fn dimensions(&self) -> MatrixDimensions {
        self.dimensions
    }

    /// Get the underlying storage of the matrix.
    pub fn data(&self) -> &[T] {
        &self.data
    }
}

impl<T> Matrix<T>
where
    T: Default + Clone,
{
    /// Construct a new matrix of given dimensions filled with default elements.
    pub fn new(dimensions: MatrixDimensions) -> Matrix<T> {
        Matrix::from_value(dimensions, T::default())
    }
}

impl<T> Matrix<T>
where
    T: Clone,
{
    /// Construct a new matrix of given dimensions filled with copies of `value`.
    pub fn from_value(dimensions: MatrixDimensions, value: T) -> Matrix<T> {
        Matrix {
            dimensions,
            data: vec![value; dimensions.width * dimensions.height],
        }
    }
}

impl<T> Matrix<T> {
    /// Construct a new matrix of given dimensions from an iterator.
    ///
    /// ## Panics
    /// This function panics if `iter` yields fewer elements than are required to fill the matrix.
    pub fn from_iterator<I>(dimensions: MatrixDimensions, iter: I) -> Matrix<T>
    where
        I: IntoIterator<Item = T>,
    {
        let data_size = dimensions.width * dimensions.height;
        let mut data = Vec::with_capacity(data_size);

        let mut iter = iter.into_iter();

        for _i in 0..data_size {
            let element = iter.next().unwrap();
            data.push(element);
        }

        Matrix { dimensions, data }
    }

    /// Constructs a new matrix of given dimensions from successive calls to a function.
    pub fn from_fn<F>(dimensions: MatrixDimensions, mut f: F) -> Matrix<T>
    where
        F: FnMut() -> T,
    {
        let data_size = dimensions.width * dimensions.height;
        let mut data = Vec::with_capacity(data_size);

        for _i in 0..data_size {
            let element = f();
            data.push(element);
        }

        Matrix { dimensions, data }
    }

    /// Constructs a new matrix of given dimensions from a function mapping each coordinate in the
    /// matrix to a value.
    pub fn from_generator<G>(dimensions: MatrixDimensions, g: G) -> Matrix<T>
    where
        G: Fn(MatrixIndex) -> T,
    {
        let data_size = dimensions.width * dimensions.height;
        let mut data = Vec::with_capacity(data_size);

        for y in 0..dimensions.height {
            for x in 0..dimensions.width {
                let element = g(MatrixIndex { x, y });
                data.push(element);
            }
        }

        Matrix { dimensions, data }
    }
}

impl<T> Index<MatrixIndex> for Matrix<T> {
    type Output = T;

    fn index(&self, MatrixIndex { x, y }: MatrixIndex) -> &Self::Output {
        let MatrixDimensions { width, .. } = self.dimensions;
        &self.data[x + y * width]
    }
}

impl<T> IndexMut<MatrixIndex> for Matrix<T> {
    fn index_mut(&mut self, MatrixIndex { x, y }: MatrixIndex) -> &mut Self::Output {
        let MatrixDimensions { width, .. } = self.dimensions;
        &mut self.data[x + y * width]
    }
}

impl<T> Add for Matrix<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        assert_eq!(
            self.dimensions, rhs.dimensions,
            "cannot add matrices of different dimensions!"
        );

        let data = self
            .data
            .into_iter()
            .zip(rhs.data.into_iter())
            .map(|(a, b)| a + b)
            .collect();

        Matrix {
            data,
            dimensions: self.dimensions,
        }
    }
}

impl<T> AddAssign for Matrix<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        assert_eq!(
            self.dimensions, rhs.dimensions,
            "Cannot add matrices of different dimensions!"
        );

        let mut index = 0;
        rhs.data.into_iter().for_each(|e| {
            self.data[index] += e;
            index += 1;
        });
    }
}

impl<T> Sub for Matrix<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        assert_eq!(
            self.dimensions, rhs.dimensions,
            "cannot subtract matrices of different dimensions!"
        );

        let data = self
            .data
            .into_iter()
            .zip(rhs.data.into_iter())
            .map(|(a, b)| a - b)
            .collect();

        Matrix {
            data,
            dimensions: self.dimensions,
        }
    }
}

impl<T> SubAssign for Matrix<T>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        assert_eq!(
            self.dimensions, rhs.dimensions,
            "Cannot subtract matrices of different dimensions!"
        );

        let mut index = 0;
        rhs.data.into_iter().for_each(|e| {
            self.data[index] -= e;
            index += 1;
        });
    }
}
