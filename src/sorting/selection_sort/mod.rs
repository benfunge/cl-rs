/*! # Exercise 2.2-2 - Selection Sort
 
 **Input:** A sequence of `n` elements of a partial order `(a_1, a_2, ..., a_n)`

 **Output:** A permutation `(a'_1, a'_2, ..., a'_n)` of the input sequence such that `a'_1 <= a'_2
 <= ... <= a'_n`

 **Time complexity:** `O(n^2)`

 Selection sort is another simple sorting algorithm with suboptimal time complexity, based around
 the idea of finding the next smallest element in each iteration. 
*/

#[cfg(test)]
mod test;

// A simple function to find the index of the minimum of a given slice. I think this function
// doesn't exist in the standard library, but I might be wrong.
fn min_index<T>(data: &[T]) -> usize
where
    T: PartialOrd,
{
    let mut min_index = 0;
    for i in 0..data.len() {
        if data[i] < data[min_index] {
            min_index = i;
        }
    }

    min_index
}

pub fn selection_sort<T>(data: &mut [T])
where
    T: PartialOrd,
{
    if data.len() <= 1 {
        return;
    }

    for i in 0..(data.len() - 1) {
        // Do not forget to add `i`, as the result of `min_index` is relative to the passed slice
        let index = i + min_index(&data[i..]);
        data.swap(i, index);
    }
}
