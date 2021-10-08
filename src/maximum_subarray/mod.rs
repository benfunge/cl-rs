use std::ops::Range;

pub mod shamos;
pub mod naive;

#[cfg(test)]
mod test;

/// A type encapsulating a maximal subarray of some slice.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaximumSubarray<T> {
    /// Indices of the array, as pertaining to the original slice.
    pub range: Range<usize>,
    /// The sum over all the elements of `slice[range]`.
    pub value: T,
}
