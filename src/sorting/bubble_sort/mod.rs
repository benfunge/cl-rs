/*! # Problem 2-2 - Bubble Sort

 
 **Input:** A sequence of `n` elements of a partial order `(a_1, a_2, ..., a_n)`

 **Output:** A permutation `(a'_1, a'_2, ..., a'_n)` of the input sequence such that `a'_1 <= a'_2
 <= ... <= a'_n`

 **Time complexity:** `O(n^2)`

 Bubble sort is one of the simplest and most popular (as far as implementing algorithms goes)
 sorting algorithms. It always runs in `O(n^2)` time, even in the best case, and does `O(n^2)` swaps
 in the worst case. Don't use it for anything serious.
*/
#[cfg(test)]
mod test;

// This implementation is pretty much what is given in the book.
pub fn bubble_sort<T>(data: &mut [T])
where
    T: PartialOrd,
{
    if data.len() <= 1 {
        return;
    }

    for i in 0..(data.len() - 1) {
        for j in ((i + 1)..data.len()).rev() {
            if data[j] < data[j - 1] {
                data.swap(j, j - 1)
            }
        }
    }
}
