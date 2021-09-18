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
#[cfg(test)]
mod test;

pub fn merge_sort<T>(mut data: &mut Vec<T>)
where
    T: PartialOrd + Clone,
{
    if data.len() <= 1 {
        return;
    }

    let end = data.len() - 1;
    merge_sort_recursion(&mut data, 0, end);
}

// This is the actual main sorting function. For usability reasons, the actual `merge_sort`
// function is a simple facade that also does some setup.
// Note that the `end` index is _inclusive_, that is it is the index of the last element
// considered, not that index + 1, as is commonly used. This is done simply for ease of keeping
// track.
fn merge_sort_recursion<T>(mut data: &mut Vec<T>, begin: usize, end: usize)
where
    T: PartialOrd + Clone,
{
    if begin < end {
        let middle = (begin + end) / 2;
        merge_sort_recursion(&mut data, begin, middle);
        merge_sort_recursion(&mut data, middle + 1, end);

        merge(&mut data, begin, middle, end);
    }
}

// This function is where the actual work happens: It merges two sorted subarrays of `data`, namely
// the slices at `data[begin..=middle]` and `data[(middle + 1)..=end]` into a whole, residing in
// the same memory space. Unfortunately, this operation requires `O(n)` additional memory.
fn merge<T>(data: &mut Vec<T>, begin: usize, middle: usize, end: usize)
where
    T: PartialOrd + Clone,
{
    let left = Vec::from(&data[begin..=middle]);
    let right = Vec::from(&data[(middle + 1)..=end]);

    let mut left_i = 0;
    let mut right_i = 0;

    for j in begin..=end {
        // As opposed to the merge routine given in the book, we cannot cheat our way out of proper
        // bookkeeping by setting the last element of `left` and `right` to a sentinel value, since
        // we don't control the input type. There's two possible solutions to this: either by
        // wrapping the input type in an enum with a special "infinity" state, implementing
        // comparisons etc., at the cost of some additional memory, or by explicitly keeping track
        // of when we have reached the end of one collection and then copying all the left items in
        // the other collection in one go. We have here decided to go with the second variant.
        if left_i == left.len() {
            data.splice(j..=end, right[right_i..].iter().cloned());
            return;
        } else if right_i == right.len() {
            data.splice(j..=end, left[left_i..].iter().cloned());
            return;
        }

        if left[left_i] <= right[right_i] {
            data[j] = left[left_i].clone();
            left_i += 1;
        } else {
            data[j] = right[right_i].clone();
            right_i += 1;
        }
    }
}
