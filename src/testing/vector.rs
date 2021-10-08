use quickcheck::Arbitrary;
use std::{
    fmt::{Debug, Display},
    ops::{Add, Mul},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ElementOf<T> {
    pub data: Vec<T>,
    pub element: T,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NoElementOf<T> {
    pub data: Vec<T>,
    pub element: T,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AllNegative<T> {
    pub data: Vec<T>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AllPositive<T> {
    pub data: Vec<T>,
}

fn fmt_vec<T: Display>(vec: &Vec<T>) -> String {
    let mut result = String::from('<');

    for v in vec {
        result = format!("{} {},", result, v);
    }

    if vec.len() > 0 {
        result.pop();
    }

    result.push('>');
    result
}

impl<T: Display + Clone> Display for ElementOf<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} element of <{}>", &self.element, fmt_vec(&self.data))
    }
}

impl<T: Arbitrary> Arbitrary for ElementOf<T> {
    fn arbitrary(mut g: &mut quickcheck::Gen) -> Self {
        let mut data = Vec::new();
        while data.is_empty() {
            data = Vec::<T>::arbitrary(&mut g);
        }

        let element = g.choose(&data).unwrap().clone();
        ElementOf { data, element }
    }
}

impl<T: Display + Clone> Display for NoElementOf<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} not an element of <{}>",
            &self.element,
            fmt_vec(&self.data)
        )
    }
}

impl<T: Arbitrary + PartialEq> Arbitrary for NoElementOf<T> {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let mut data = Vec::new();
        while data.is_empty() {
            data = Vec::<T>::arbitrary(g);
        }

        let mut element = T::arbitrary(g);
        while data.contains(&element) {
            element = T::arbitrary(g);
        }

        NoElementOf { data, element }
    }
}

impl<T: Display + Clone> Display for AllNegative<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", fmt_vec(&self.data))
    }
}

impl<T> Arbitrary for AllNegative<T>
where
    T: Arbitrary + Clone + From<i8> + Mul<T, Output = T> + Add<T, Output = T> + PartialOrd,
{
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let mut data = Vec::<T>::arbitrary(g);
        for i in 0..data.len() {
            while data[i] >= T::from(0) {
                data[i] = T::arbitrary(g);
            }
        }

        AllNegative { data }
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        quickcheck::empty_shrinker()
    }
}

impl<T: Display + Clone> Display for AllPositive<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", fmt_vec(&self.data))
    }
}

impl<T> Arbitrary for AllPositive<T>
where
    T: Arbitrary + Clone + From<i8> + Mul<T, Output = T> + Add<T, Output = T> + PartialOrd,
{
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let mut data = Vec::<T>::arbitrary(g);
        for i in 0..data.len() {
            while data[i] <= T::from(0) {
                data[i] = T::arbitrary(g);
            }
        }

        AllPositive { data }
    }
}
