use std::collections::HashSet;

fn main() {

    retain();
}

/// Vec::dedup()
/// Retain trick for vectors:
fn retain() {
    let mut byte_vec = b"Missssssssissippi".to_vec();

    let mut seen = HashSet::new();
    byte_vec.retain(|r| seen.insert(*r));

    assert_eq!(&byte_vec, b"Misp");
}

/// Rust has several methods that can borrow mut references to two or more parts of an array, slice,
/// or vector at once. These methods are safe, because by design, they always split the data into
/// nonoverlapping regions. Many of these methods are also handy for working with non-mut slices, so
/// there are mut and non-mut versions of each.
/// A sliding window of size 2 is handy for exploring how a data series changes from one data point
/// to the next:
