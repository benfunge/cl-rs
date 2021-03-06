use super::linear_search;
use quickcheck_macros::quickcheck;

use crate::testing::vector::{ElementOf, NoElementOf};

#[quickcheck]
fn finds_members(v: ElementOf<i32>) -> bool {
    linear_search(&v.data, v.element).is_some()
}

#[quickcheck]
fn doesnt_find_unmember(v: NoElementOf<i32>) -> bool {
    linear_search(&v.data, v.element).is_none()
}

// Edge cases

#[test]
fn no_data() {
    let nothing: Vec<i32> = Vec::new();
    if let Some(v) = linear_search(&nothing, 0) {
        assert!(false, "Expected None, got Some({})", v);
    }
}

#[test]
fn only_element() {
    let value = 12;
    let data = vec![value];
    match linear_search(&data, value) {
        Some(v) => assert_eq!(
            v, 0,
            "Got value {}, which does not match the search parameter {}!",
            v, 0
        ),
        None => assert!(false, "Expected value {}, got nothing!", value),
    }
}
