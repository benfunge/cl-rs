/*! # Chapter 2.3 - Merge Sort

 **Input:** A sequence of `n` elements of a partial order `(a_1, a_2, ..., a_n)`

 **Output:** A permutation `(a'_1, a'_2, ..., a'_n)` of the input sequence such that `a'_1 <= a'_2
 <= ... <= a'_n`

 **Time complexity:** `O(n lg n)`

 Merge sort is the first sorting algorithm in the book with optimal time complexity. Using a
 divide-and-conquer approach, it attacks the sorting problem by reducing it to recombination of the
 solutions of two subproblems of half the original size, recursively. In fact, no work goes into
 "solving" the actual subproblems: instead, the algorithm first reduces to single-element arrays, so
 that all the work goes into recombining these into a sorted whole.

 The optimal time complexity comes at a cost: for one, the merge operation requires an additional
 `O(n)` memory at each step. Further, the algorithm is highly unkind to modern CPU caching and
 memory fetching, incurring non-negligible practical costs.
*/

use crate::util::slice;

use std::ops::{Range, RangeBounds};

#[cfg(test)]
mod test;

pub fn merge_sort<T>(mut data: &mut Vec<T>)
where
    T: PartialOrd + Clone,
{
    if data.len() <= 1 {
        return;
    }

    let end = data.len();
    merge_sort_recursion(&mut data, 0..end);
}

// This is the actual main sorting function. For usability reasons, the actual `merge_sort`
// function is a simple facade that also does some setup.
fn merge_sort_recursion<T>(mut data: &mut Vec<T>, range: Range<usize>)
where
    T: PartialOrd + Clone,
{
    let Range { start, end } = range;

    if start < end - 1 {
        let middle = (start + end) / 2;
        merge_sort_recursion(&mut data, start..middle);
        merge_sort_recursion(&mut data, middle..end);

        merge(&mut data, start..middle, middle..end);
    }
}

// This function is where the actual work happens: It merges two adjacent sorted subarrays of
// `data`, namely the slices `data[left]` and `data[right]` into a whole, residing in the same
// memory space. Unfortunately, this operation requires `O(n)` additional memory.
fn merge<T, L, R>(data: &mut Vec<T>, left: L, right: R)
where
    T: PartialOrd + Clone,
    L: RangeBounds<usize>,
    R: RangeBounds<usize>,
{
    let left = slice::range(left, ..data.len());
    let right = slice::range(right, ..data.len());

    let mut data_index = left.start;
    let data_end = right.end;

    let mut left_data = Vec::from(&data[left]);
    let mut right_data = Vec::from(&data[right]);

    // Instead of a literal translation of the algorithm as presented in the book, we have to do
    // some shenanigans to avoid unnecessary cloning, which may or may not be expensive, depending
    // on the data type. So, we make use of the handy draining iterators of `Vec`, which let us
    // move out of the respective elements.
    let mut drain_left = left_data.drain(..).peekable();
    let mut drain_right = right_data.drain(..).peekable();

    let mut assign_next = |item| {
        data[data_index] = item;
        data_index += 1;
    };

    // To make the algorithm work with the `drain_left` and `drain_right` iterators (see above), we
    // adapt the given procedure as follows: instead of treating `left` and `right` in an
    // egalitarian manner, we...
    loop {
        // (1) take an element `item_l` of `left`, if there are any; ...
        let item_l = if drain_left.peek().is_some() {
            drain_left.next().unwrap()
        } else {
            break;
        };

        // (2) take as many elements of `right` that are smaller than `item_l` as possible and
        // insert them into `data`; and...
        while matches!(drain_right.peek(), Some(i) if i < &item_l) {
            let item_r = drain_right.next().unwrap();
            assign_next(item_r);
        }

        assign_next(item_l);
    }

    // (3) after having exhausted all elements from `left`, move over the remaining elements from
    // `right`.
    data.splice(data_index..data_end, drain_right);
}
