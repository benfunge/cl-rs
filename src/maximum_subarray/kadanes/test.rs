use crate::maximum_subarray::kadanes::MaximumSubarray;

use super::kadanes_algorithm;

#[test]
fn empty() {
    assert_eq!(
        kadanes_algorithm(&[]),
        MaximumSubarray {
            range: 0..0,
            value: 0,
        }
    );
}

#[test]
fn all_positive() {
    let data = [1, 5, 7, 3, 20];
    let solution = MaximumSubarray {
        range: 0..data.len(),
        value: data.iter().sum(),
    };

    assert_eq!(kadanes_algorithm(&data), solution);
}

#[test]
fn all_negative() {
    let data = [-1, -2, -5, -10];
    let solution = MaximumSubarray {
        range: 0..0,
        value: 0,
    };

    assert_eq!(kadanes_algorithm(&data), solution);
}

#[test]
fn example_1() {
    assert_eq!(
        kadanes_algorithm(&[1, -4, 3, -4]),
        MaximumSubarray {
            range: 2..3,
            value: 3
        }
    );
}

#[test]
fn example_2() {}
