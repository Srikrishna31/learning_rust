/*
One of Rust's great strengths is its support for concurrent programming. The same rules that ensure
Rust programs are free of memory errors also ensure threads can share memory only in ways that avoid
data races.
* If you use a mutex to coordinate threads making changes to a shared data structure, Rust ensures
that you can't access the data except when you're holding the lock, and releases the lock automatically
when you're done. In C and C++, the relationship between a mutex and the data it protects is left
to the comments.

* If you want to share read-only data among several threads, Rust ensures that you cannot modify the
data accidentally. In C and C++, the type system can help with this, but it's easy to get it wrong.

* If you transfer ownership of a data structure from one thread to another, Rust makes sure you have
indeed relinquished all access to it. In C and C++, it's up to you to check that nothing on the
sending thread will ever touch the data again. If you don't get it right, the effects can depend on
what happens to be in the processor's cache and how many writes to memory you've done recently.

All Rust functions are thread-safe.
 */

use num::Complex;

fn main() {
    println!("Hello, world!");
}

//Below is a documentation comment, which the rustdoc can parse and produce online documentation.
/// Try to determine if `c` is in the Mandelbrot set, using at most `limit` iterations to decide.
/// If `c` is not a member, return `Some(i)`, where `i` is the number of iterations it took for `c`
/// to leave the circle of radius 2 centered on the origin. If `c` seems to be a member (more
/// precisely, if we reached the iteration limit without being able to prove that `c` is not a member),
/// return `None`.
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex{re: 0.0, im: 0.0};
    //This for loop simply iterates over the range of integers starting with 0 and up to (but
    //not including) limit.
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z*z + c;
    }

    None
}


use std::str::FromStr;

/// Parse the string `s` as a coordinate pair, like `"400x600"` or `"`1.0, 0.5"`.
///
/// Specifically `s` should have the form <left><sep><right>, where <sep> is the character given by
/// the `separator` argument, and <left> and <right> are both strings that can be parsed by
/// `T::from_str`, `separator` must be an ASCII character.
///
/// If `s` has the proper form, return `Some<(x,y)>`. If it doesn't parse correctly, return `None`.
// <T:FromStr> => For any type T that implements the FromStr trait
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match(T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l,r)),
                //The wildcard pattern _ matches anything and ignores its value.
                _ => None
            }
        }
    }
}


#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','),None);
    assert_eq!(parse_pair::<i32>(",10", ','),None);
    assert_eq!(parse_pair::<i32>("10,20", ','),Some((10,20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','),None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'),None);
    assert_eq!(parse_pair::<f64>("0.5x1.5,", 'x'),Some((0.5, 1.5)));
}
