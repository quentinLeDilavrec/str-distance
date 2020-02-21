//! Compute distances between strings

#![forbid(unsafe_code)]
#![allow(unused)]

use std::ops::Deref;

pub use levenshtein::{DamerauLevenshtein, Levenshtein};
pub use ratcliff::RatcliffObershelp;

pub mod jaro;
pub mod levenshtein;
pub mod qgram;
pub mod ratcliff;
mod utils;

pub fn strdistance<S, T, D>(a: S, b: T, dist: D)
where
    S: AsRef<str>,
    T: AsRef<str>,
    D: Distance,
{
    unimplemented!()
}

pub fn strcompare<S, T, D>(a: S, b: T, dist: D)
where
    S: AsRef<str>,
    T: AsRef<str>,
    D: Distance,
{
    unimplemented!()
}

// drawbacks from trait: needs instance

pub trait Distance {
    type Dist;
    fn distance<S, T>(&self, a: S, b: T) -> Self::Dist
    where
        S: AsRef<str>,
        T: AsRef<str>;
}

pub struct Score(pub f64);

/// Convenience
pub trait DistanceElement {
    fn distance<S, D>(&self, other: S, dist: D)
    where
        S: AsRef<str>,
        D: Distance;
}

impl<T: AsRef<str>> DistanceElement for T {
    fn distance<S, D>(&self, other: S, dist: D)
    where
        S: AsRef<str>,
        D: Distance,
    {
        let a = self.as_ref();

        unimplemented!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DistanceValue {
    Exact(usize),
    Exceeded(usize),
}

impl Into<usize> for DistanceValue {
    fn into(self) -> usize {
        *self
    }
}

impl Deref for DistanceValue {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        match self {
            DistanceValue::Exact(val) | DistanceValue::Exceeded(val) => val,
        }
    }
}
