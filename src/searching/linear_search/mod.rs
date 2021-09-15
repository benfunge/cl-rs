/*! # Exercise 2.1-3 - Linear Search

 **Input**: A sequence of `n` numbers `A = (a_1, ..., a_n)` and a value `v`

 **Output**: An index `i` such that `v == a_i` or `NIL` if `v` does not appear in `A`

 **Time complexity**: `O(n)`

 Linear search is the obvious solution to the question: How do you find something in an array? The
 answer is, of course, that you simply traverse the array from left to right (or right to left,
 whatever direction you prefer) and check if your value is among the elements encountered on the
 way. We choose here to implement the algorithm on iterators for some added generality, and we also
 allow the item type to be anything that can be compared for equality.
*/
#[cfg(test)]
mod test;

pub fn linear_search<T, I>(data: I, value: T) -> Option<T>
where
    I: Iterator<Item = T>,
    T: PartialEq,
{
    for element in data {
        if element == value {
            return Some(value);
        }
    }

    None
}
