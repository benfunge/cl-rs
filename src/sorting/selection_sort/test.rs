use quickcheck_macros::quickcheck;

use super::selection_sort;

// If only `is_sorted` was already stabilized...
fn is_sorted<T: PartialOrd>(data: &Vec<T>) -> bool {
    if data.len() == 0 {
        return true;
    }

    for i in 0..data.len() - 1 {
        if !(data[i] <= data[i + 1]) {
            return false;
        }
    }

    true
}

#[quickcheck]
fn sortedness(mut data: Vec<i32>) -> bool {
    selection_sort(&mut data);
    is_sorted(&data)
}

#[quickcheck]
fn sortedness_preservation(mut data: Vec<i32>) -> bool {
    let mut data_copy = data.clone();
    selection_sort(&mut data);
    data_copy.sort_unstable();
    data == data_copy
}

// Edge case(s)

#[test]
fn no_data() {
    let mut nothing: Vec<i32> = Vec::new();
    selection_sort(&mut nothing);
    assert!(nothing == Vec::new());
}
