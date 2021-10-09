use crate::testing::vector::{AllNegative, AllPositive};

use super::{MaximumSubarray, kadanes::kadanes_algorithm, naive::naive_algorithm, shamos::shamos_algorithm};

use quickcheck_macros::quickcheck;

fn adapt_empty_subrange(m: MaximumSubarray<i32>) -> MaximumSubarray<i32> {
    if m.value > 0 {
        m
    } else {
        MaximumSubarray {
            range: 0..0,
            value: 0,
        }
    }
}

type SubrangeFn = fn(&[i32]) -> MaximumSubarray<i32>;

#[derive(Clone, Copy)]
struct Algorithm<'a> {
    algorithm: SubrangeFn,
    name: &'a str,
}

const ALGORITHMS: [Algorithm; 3] = [
    Algorithm {
        algorithm: naive_algorithm,
        name: "naive_algorithm",
    },
    Algorithm {
        algorithm: shamos_algorithm,
        name: "shamos_algorithm",
    },
    Algorithm {
        algorithm: kadanes_algorithm,
        name: "kadanes_algorithm",
    },
];

#[test]
fn empty_agreement() {
    let comparator_algorithm = ALGORITHMS[0];
    let solution = (comparator_algorithm.algorithm)(&[]);

    for a in ALGORITHMS.iter().skip(1) {
        assert_eq!(
            (a.algorithm)(&[]),
            solution,
            "{} doesn't agree with {} on empty slices!",
            a.name,
            comparator_algorithm.name,
        );
    }
}

#[quickcheck]
fn all_positive_agreement(data: AllPositive<i16>) -> bool {
    // This is a hack: since taking a Vec<i32> is liable to contain values of very large magnitude
    // which can overflow the internal counters of these algorithms (and we don't want to guard
    // against that), we force quickcheck to limit itself to 16 bit integers and then upcast.
    let data: Vec<_> = data.data.into_iter().map(|i| i as i32).collect();
    let comparator_algorithm = ALGORITHMS[0];
    let solution = (comparator_algorithm.algorithm)(&data);

    for a in ALGORITHMS.iter().skip(1) {
        let result = (a.algorithm)(&data);
        if result.value != solution.value {
            return false;
        }
    }

    true
}

#[quickcheck]
fn all_negative_agreement(data: AllNegative<i16>) -> bool {
    let data: Vec<_> = data.data.into_iter().map(|i| i as i32).collect();
    let comparator_algorithm = ALGORITHMS[0];
    let solution = adapt_empty_subrange((comparator_algorithm.algorithm)(&data));

    for a in ALGORITHMS.iter().skip(1) {
        let result = adapt_empty_subrange((a.algorithm)(&data));
        if result.value != solution.value {
            return false;
        }
    }

    true
}

#[quickcheck]
fn general_agreement(data: Vec<i16>) -> bool {
    let data: Vec<_> = data.into_iter().map(|i| i as i32).collect();

    let comparator_algorithm = ALGORITHMS[0];
    let solution = adapt_empty_subrange((comparator_algorithm.algorithm)(&data));

    for a in ALGORITHMS.iter().skip(1) {
        let result = adapt_empty_subrange((a.algorithm)(&data));
        if result.value != solution.value {
            return false;
        }
    }

    true
}
