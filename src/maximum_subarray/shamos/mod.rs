/*! # Chapter 4.1 - Shamo's Algorithm

 **Input:** An array `A = [a_1, a_2, ..., a_n]` of `n` numbers.

 **Output:** A subarray `A'= [a_i, ..., a_j]` of `A` such that `a_i + ... + a_j` is maximal.

 **Time complexity:** `O(n lg n)`

 Shamo's algorithm, unnamed in the book, is the second divide-and-conquer algorithm we're
 introduced to after merge sort. It solves the maximum subarray problem, and does so in `O(n lg n)`
 time compared to the `O(n^2)` time of the semi-naive algorithm. The improvement is based on the
 observation that the problem can be considered in terms of finding maximum arrays overstepping a
 midpoint in the array, recursively.
*/
use std::ops::{AddAssign, Range};

use super::MaximumSubarray;

#[cfg(test)]
mod test;

pub fn shamos_algorithm<T>(data: &[T]) -> MaximumSubarray<T>
where
    T: AddAssign + PartialOrd + Default + Clone,
{
    if data.is_empty() {
        return MaximumSubarray {
            range: 0..0,
            value: T::default(),
        };
    }

    shamos_algorithm_recursion(data, 0..data.len())
}

fn shamos_algorithm_recursion<T>(data: &[T], range: Range<usize>) -> MaximumSubarray<T>
where
    T: AddAssign + PartialOrd + Default + Clone,
{
    if range.end - range.start <= 1 {
        return MaximumSubarray {
            range: range.clone(),
            value: data[range.start].clone(),
        };
    }

    let mid = (range.start + range.end) / 2;
    let left_range = range.start..mid;
    let right_range = mid..range.end;

    // The algorithm is based on the observation, that either (a) a maximum subarray can be found
    // in the left half, or (b) it can be found in the right half, or (c) it lies partly in both
    // parts and hence passes the midpoint.
    let left = shamos_algorithm_recursion(data, left_range.clone());
    let right = shamos_algorithm_recursion(data, right_range.clone());
    let cross = find_max_crossing_subarray(data, left_range, right_range);

    if left.value >= right.value && left.value >= cross.value {
        return left;
    } else if right.value >= left.value && right.value >= cross.value {
        return right;
    } else {
        return cross;
    }
}

// This is where all the interesting stuff happens.
fn find_max_crossing_subarray<T>(
    data: &[T],
    left: Range<usize>,
    right: Range<usize>,
) -> MaximumSubarray<T>
where
    T: AddAssign + PartialOrd + Clone,
{
    assert!(!left.is_empty(), "Unexpected empty left range {:?}", left);
    assert!(
        !right.is_empty(),
        "Unexpected empty right range {:?}",
        right
    );

    // Since we don't control the input type, instead of letting `left_sum` start at `-\infty`, we
    // pull the first assignment that always happens anyways out of the loop. By assertion above,
    // this is well defined.
    let mut left_sum = data[left.end - 1].clone();
    let mut sum = left_sum.clone();
    let mut max_left = left.end - 1;

    for i in left.rev().skip(1) {
        sum += data[i].clone();

        if sum > left_sum {
            left_sum = sum.clone();
            max_left = i;
        }
    }

    let mut right_sum = data[right.start].clone();
    let mut sum = right_sum.clone();
    let mut max_right = right.start + 1;

    for i in right.skip(1) {
        sum += data[i].clone();

        if sum > right_sum {
            right_sum = sum.clone();
            max_right = i;
        }
    }

    left_sum += right_sum;

    // It's importantly, do not forget to add 1 to `max_right` because `max_right` is the
    // *inclusive* index of the last element.
    return MaximumSubarray {
        range: max_left..(max_right + 1),
        value: left_sum,
    };
}
