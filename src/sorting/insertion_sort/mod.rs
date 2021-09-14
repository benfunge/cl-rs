/*! # Chapter 2.1 - Insertion Sort

 **Input**: A sequence of `n` elements of a partial order `(a_1, a_2, ..., a_n)`

 **Output**: A permutation `(a'_1, a'_2, ..., a'_n)` of the input sequence such that `a'_1 <= a'_2
 <= ... <= a'_n`

 **Time complexity**: `O(n^2)`

 Insertion sort is one of, if not _the_ simplest sorting algorithm, useful mainly for sorting small
 collections. The implementation presented here differs significantly from the pseudocode in the
 book, mainly due to having to circumnavigate borrowing rules.
*/

#[cfg(test)]
mod test;

// N. b. we limit ourselves to sorting vectors because there are currently no (?) useful
// abstractions around indexing and swapping elements.
pub fn insertion_sort<T>(data: &mut Vec<T>)
where
    T: PartialOrd,
{
    // Starting with the second element because a one-element array is trivially sorted.
    for j in 1..data.len() {
        let mut i = j;

        // Picture this as moving the element down in the container: at each step, our element
        // moves one step to the left until it has either hit the very beginning or is not smaller
        // than its left-next neighbor. In contrast, the book version seemingly "picks out" the
        // element, then shifts all the other elements right in its search for the right place, and
        // finally inserts the element.
        while i > 0 && data[i - 1] > data[i] {
            data.swap(i - 1, i);
            i -= 1;
        }
    }
}
