/*! # Exercise 4.1-5 - Kadane's Algorithm

 **Input:** An array `A = [a_1, a_2, ..., a_n]` of `n` numbers.

 **Output:** A subarray `A'= [a_i, ..., a_j]` of `A` such that `a_i + ... + a_j` is maximal.

 **Time complexity:** `O(n)`

 Kadane's algorithm is the optimal solution to the maximum subarray problem: by observing that one
 can reduce the problem to keeping track of the best sum seen during linear iteration, one can
 reduce the time complexity of the algorithm to `O(n)`.
*/
use std::ops::AddAssign;

use super::MaximumSubarray;

#[cfg(test)]
mod test;

pub fn kadanes_algorithm<T>(data: &[T]) -> MaximumSubarray<T>
where
    T: AddAssign + PartialOrd + Default + Clone,
{
    let mut best_sum = T::default();
    let mut current_sum = best_sum.clone();

    let mut best_start = 0;
    let mut best_end = 0;

    let mut current_start = 0;

    for i in 0..data.len() {
        let current_element = data[i].clone();

        if current_sum <= T::default() {
            current_start = i;
            current_sum = current_element;
        } else {
            current_sum += current_element;
        }

        if current_sum > best_sum {
            best_sum = current_sum.clone();
            best_start = current_start;
            best_end = i + 1;
        }
    }

    MaximumSubarray {
        range: best_start..best_end,
        value: best_sum,
    }
}
