/*! # Exercise 4.1-2 - Maximum Subarray Naive Algorithm
    
 **Input:** An array `A = [a_1, a_2, ..., a_n]` of `n` numbers.

 **Output:** A subarray `A'= [a_i, ..., a_j]` of `A` such that `a_i + ... + a_j` is maximal.

 **Time complexity:** `O(n^2)`

 The brute-force solution to the maximum subarray problem, with a single optimization: Instead of
 computing the sum over a given subarray from scratch on every iteration, we build our sums
 successively from one iteration to the next, allowing us to compute them in `O(1)` time. Thus,
 since the algorithm traverses `n * (n + 1) / 2` subarrays, it has time complexity in `O(n^2)`.

 Note that the behavior differs from Shamo's algorithm in the case that the array only contains
 negative numbers: in this case, Shamo's algorithm returns the one-element range comprising the
 maximum element, whereas the naive algorithm returns the empty range.
*/
use std::ops::AddAssign;

use super::MaximumSubarray;

#[cfg(test)]
mod test;

pub fn naive_algorithm<T>(data: &[T]) -> MaximumSubarray<T>
where
    T: AddAssign + PartialOrd + Default + Clone,
{
    let mut left = 0;
    let mut right = 0;
    let mut max_sum = T::default();

    for i in 0..data.len() {
        let mut last_sum = T::default();
        for j in i..data.len() {         
            last_sum += data[j].clone();
            if last_sum > max_sum {
                left = i;
                right = j + 1;
                max_sum = last_sum.clone();
            }
        }
    }

    MaximumSubarray {
        range: left..right,
        value: max_sum,
    }
}
