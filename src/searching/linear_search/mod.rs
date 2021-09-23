/*! # Exercise 2.1-3 - Linear Search

 **Input:** A sequence of `n` numbers `A = (a_1, ..., a_n)` and a value `v`

 **Output:** An index `i` such that `v == a_i` or `NIL` if `v` does not appear in `A`

 **Time complexity:** `O(n)`

 Linear search is the obvious solution to the question: How do you find something in an array? The
 answer is, of course, that you simply traverse the array from left to right (or right to left,
 whatever direction you prefer) and check if your value is among the elements encountered on the
 way.
*/
#[cfg(test)]
mod test;

pub fn linear_search<T>(data: &[T], value: T) -> Option<usize>
where
    T: PartialEq,
{
    for i in 0..data.len() {
        if value == data[i] {
            return Some(i);
        }
    }

    None
}
