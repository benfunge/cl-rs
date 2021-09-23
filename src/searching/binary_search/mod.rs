/*! # Exercise 2.3-5 - Binary Search

 **Input:** A sequence of `n` numbers `A = (a_1, ..., a_n)` in sorted order, and a value `v`

 **Output:** An index `i` such that `v == a_i` or `NIL` if `v` does not appear in `A`

 **Time complexity:** `O(lg n)`

 Binary search is based on the observation that for a sorted input array, one can improve on the
 `O(n)` of linear search by dividing-and-conquering the array in halves, by simply comparing against
 the middle element to see which side you have to search in.
*/
use std::ops::{Range, RangeBounds};

use crate::util::slice;

#[cfg(test)]
mod test;

pub fn binary_search<T>(data: &[T], value: T) -> Option<usize>
where
    T: PartialOrd,
{
    if data.len() == 0 {
        return None;
    }

    binary_search_recursion(data, value, 0..data.len())
}

fn binary_search_recursion<T, B>(data: &[T], value: T, bounds: B) -> Option<usize>
where
    T: PartialOrd,
    B: RangeBounds<usize>,
{
    let Range { start, end } = slice::range(bounds, ..data.len());

    if start < end - 1 {
        let middle = (start + end) / 2;

        if value < data[middle] {
            return binary_search_recursion(data, value, start..middle);
        } else {
            return binary_search_recursion(data, value, middle..end);
        }
    }

    if data[start] == value {
        Some(start)
    } else {
        None
    }
}
