use std::io::prelude::*;

/// count, sum, product
/// The count method draws items from an iterator until it returns None, and tells you how many it got.
/// The sum and product methods compute the sum or product of the iterator's items, which must be
/// integers or floating-point numbers.
pub(crate) fn count_sum_product() -> () {
    let stdin = std::io::stdin();
    //println!("{}", stdin.lock().lines().count());

    assert_eq!((1..=20).sum::<i32>(), 210);
    assert_eq!((1..=20).product::<i64>(), 2432902008176640000);
}

/// max, min
/// The min and max methods on Iterator return the least or greatest item the iterator produces. The
/// iterator's item type must implement std::cmp::Ord so that items can be compared with one another.
/// Rust's floating-point types f32 and f64 implement only std::cmp::PartialOrd, not std::cmp::Ord,
/// so you can't use the min and max methods to compute the least or greatest of a sequence of
/// floating-point numbers.
/// If you know how you would like to handle NaN values, you can use the max_by and min_by iterator
/// methods instead, which let you supply your own comparison function.
pub(crate) fn min_max() -> () {
    assert_eq!([-2,0,1,0,-2,-5].iter().max(), Some(&1));
    assert_eq!([-2,0,1,0,-2,-5].iter().min(), Some(&-5));
}

/// max_by, min_by
/// The max_by and min_by methods return the maximum or minimum item the iterator produces, as
/// determined by a comparison function you provide.
pub(crate) fn min_max_by() -> () {
    use std::cmp::Ordering;

    // Compare two f64 values. Panic if given a NaN
    let cmp = |lhs: &f64, rhs:&f64| lhs.partial_cmp(rhs).unwrap();

    let numbers = [1.0, 4.0, 2.0];

    assert_eq!(numbers.iter().copied().max_by(cmp), Some(4.0));
    assert_eq!(numbers.iter().copied().min_by(cmp), Some(1.0));

    let nums = [1.0, 4.0, std::f64::NAN, 2.0];
    //assert_eq!(nums.iter().copied().max_by(cmp), Some(4.0)); //Panics
}

/// max_by_key, min_by_key
/// The max_by_key and min_by_key methods on Iterator let you select the maximum or minimum item as
/// determined by a closure applied to each item. The closure can select some field of the item or
/// perform a computation on the items.
///
///     fn min_by_key<B:Ord, F>(self, f:F) -> Option<Self::Item>
///         where Self: Sized, F: FnMut(&Self::Item) -> B;
///
///     fn max_by_key<B:Ord, F>(self, f:F) -> Option<Self::Item>
///         where Self: Sized, F: FnMut(&Self::Item) -> B;
///
/// That is, given a closure that takes an item and returns any ordered type B, return the item for
/// which the closure returned the maximum or minimum B, or None if no items were produced.
pub(crate) fn min_max_by_key() -> () {
    use std::collections::HashMap;

    let mut populations = HashMap::new();
    populations.insert("Portland", 583_776);
    populations.insert("Fossil", 449);
    populations.insert("Greenhorn", 2);
    populations.insert("Boring", 7_762);
    populations.insert("The Dalles", 15_340);

    assert_eq!(populations.iter().max_by_key(|&(_name, pop)| pop),
                Some((&"Portland", &583_776)));
    assert_eq!(populations.iter().min_by_key(|&(_name, pop)| pop),
                Some((&"Greenhorn", &2)));
}

/// Comparing item sequences
/// Although iterators do not support Rust's comparison operators, they do provide methods like eq
/// and lt that do the same job, drawing pairs of items from the iterators and comparing them until
/// a decision can be reached.
/// Iterators provide the eq and ne methods for equality comparisons, and lt, le, gt and ge methods
/// for ordered comparisons. The cmp and partial_cmp methods behave like the corresponding methods of
/// the Ord and PartialOrd traits.
pub(crate) fn eq_lt() -> () {
    let packed = "Helen of Troy";
    let spaced = "Helen     of          Troy";
    let obscure = "Helen of Sandusky";

    assert_ne!(packed, spaced);
    assert!(packed.split_whitespace().eq(spaced.split_whitespace()));

    // This is true because ' ' < 'o'.
    assert!(spaced < obscure);

    assert!(spaced.split_whitespace().gt(obscure.split_whitespace()));
}


/// any and all
/// The any and all methods apply a closure to each item the iterator produces and return true if the
/// closure returns true for any item, or for all the items:
/// These methods consume only as many items as they need to determine the answer.
pub(crate) fn any_all() -> () {
    let id = "Iterator";

    assert!(id.chars().any(char::is_uppercase));
    assert!(id.chars().all(char::is_lowercase));
}


/// position, rposition and ExactSizeIterator
/// The position method applies a closure to each item from the iterator and returns the index of the
/// first item for which the closure returns true. More precisely, it returns an Option of the index.
/// The rposition method is the same, except that it searches from the right. The rposition iterator
/// requires a reversible iterator so that it can draw items from the right end of the sequence. It
/// also requires an exact-size iterator so that it can assign indices the same way position would,
/// starting with 0 at the left. An exact-size iterator is one that implements the std::iter::ExactSizeIterator
/// trait:
///
///     trait ExactSizeItearator: Iterator {
///         fn len(&self) -> usize { ... }
///         fn is_empty(&self) -> bool { ... }
///     }
pub(crate) fn position_rposition() -> () {
    let text = "Xerxes";
    assert_eq!(text.chars().position(|c| c == 'e'), Some(1));
    assert_eq!(text.chars().position(|c| c == 'z'), None);

    let bytes = b"Xerxes";
    assert_eq!(bytes.iter().rposition(|&c| c == b'e'), Some(4));
    assert_eq!(bytes.iter().rposition(|&c| c == b'X'), Some(0));
}

/// fold and rfold
/// The fold method is a very general tool for accumulating some sort of result over the entire
/// sequence of items an iterator produces. Given an initial value, which we'll call accumulator, and
/// a closure, fold repeatedly applies the closure to the current accumulator and the next item from
/// the iterator. The value the closure returns is taken as the new accumulator, to be passed to the
/// closure with the next item. The final accumulator value is what fold itself returns. If the
/// sequence is empty, fold simply returns the initial accumulator.
/// The fold method's signature is as follows:
///
///     fn fold<A, F>(self, init: A, f: F) -> A
///         where Self: Sized, F: FnMut(A, Self::Item) -> A;
///
/// Note that the accumulator values are moved into and out of the closure, so you can use fold with
/// non-Copy accumulator types.
/// The rfold method is the same as fold, except that it requires a double-ended iterator, and processes
/// its items from last to first.
pub(crate) fn fold_rfold() -> () {
    let a = [5,6,7,8,9,10];

    assert_eq!(a.iter().fold(0, |n, _| n + 1), 6);  //count
    assert_eq!(a.iter().fold(0, |n, i| n + i), 45); //sum
    assert_eq!(a.iter().fold(1, |n, i| n*i), 151200);   //product

    let a = ["Pack", "my", "box", "with", "five", "dozen", "liquor", "jugs"];
    let pangram = a.iter()
        .fold(String::new(), |s,w| s+ w + " ");

    assert_eq!(pangram, "Pack my box with five dozen liquor jugs ");

    let weird_pangram = a.iter().rfold(String::new(), |s,w| s+w+" ");
    assert_eq!(weird_pangram, "jugs liquor dozen five with box my Pack ");
}


use std::error::Error;
use std::io::prelude::*;
use std::str::FromStr;
/// try_fold and try_rfold
/// The try_fold method is the same as fold, except that iteration can exit early, without consuming
/// all the values from the iterator. The value returned by the closure you pass to try_fold indicates
/// whether it should return immediately, or continue folding the iterator's items.
///
/// Your closure can return any one of several types, indicating how folding should proceed:
/// * If your closure returns Result<T,E>, then returning Ok(v) tells try_fold to continue folding,
/// with v as the new accumulator value. Returning Err(e) causes folding to stop immediately. The
/// fold's final value is a Result carrying the final accumulator value, or the error returned by the
/// closure.
/// * If your closure returns Option<T>, then Some(v) indicates that folding should continue with v
/// as the new accumulator value, and None indicates that iteration should stop immediately. The fold's
/// final value is also an Option.
/// * Finally, the closure can return a std::ops::ControlFlow value. This type is an enum with two
/// variants, Continue(c) and Break(b), meaning to continue with new accumulator value c, or stop early.
/// The result of the fold is a ControlFlow value: Continue(v) if the fold consumed the entire iterator,
/// yielding the final accumulator value v; or Break(b), if the closure returned that value.
///
/// Continue(c) and Break(b) behave exactly like Ok(c) and Err(b). The advantage of using ControlFlow
/// instead of Result is that it makes your code a little more legible when an early exit doesn't
/// indicate an error, but merely that the answer is ready early.
///
/// Because try_fold is so flexible, it is used to implement many of Iterator's other consumer methods.
/// If you are implementing your own iterator type, it's worth investigating whether your iterator
/// could implement try_fold more efficiently than the default definition from the Iterator trait.
/// If you can speed up try_fold, all the other methods built on it will benefit as well.
/// The try_rfold method, as its name suggests, is the same as try_fold, except that it draws values
/// from the back, instead of the front, and requires a double-ended iterator.
pub(crate) fn try_fold_try_rfold() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();

    let sum = stdin.lock().lines()
        .try_fold(0, |sum, line| -> Result<u64, Box<dyn Error>> {
            Ok(sum + u64::from_str(&line?.trim())?)
        })?;

    println!("{sum}");
    Ok(())
}


/// nth, nth_back
/// The nth method takes an index n, skips that many items from the iterator, and returns the next
/// item, or None if the sequence ends before that point. Calling .nth(0) is equivalent to .next().
/// It doesn't take ownership of the iterator the way an adapter would, so you can call it many times.
///
///     fn nth(&mut self, n:usize) -> Option<Self::Item>
///         where Self: Sized;
/// The nth_back method is much the same, except that it draws from the back of a double-ended
/// iterator. Calling .nth_back(0) is equivalent to .next_back(): it returns the last item, or None
/// if the iterator is empty.
///
/// last
/// The last method returns the last item the iterator produces, or None if it's empty.
///
///     fn last(self) -> Option<Self::Item>;
///
/// This consumes all the iterator's items starting from the front, even if the iterator is reversible.
/// If you have a reversible iterator and don't need to consume all its items, you should instead
/// just write iter.next_back().
pub(crate) fn nth_nthback_last() -> () {
    let mut squares = (0..10).map(|i| i*i);

    assert_eq!(squares.nth(4), Some(16));
    assert_eq!(squares.nth(0), Some(25));
    assert_eq!(squares.nth(6), None);

    let copy = (0..10).map(|i| i*i);
    assert_eq!(copy.last(), Some(81));
}

/// find, rfind and find_map
/// The find method draws items from an iterator, returning the first item for which the given closure
/// returns true, or None if the sequence ends before a suitable item is found.
///     fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
///         where Self: Sized,
///                 P: FnMut(&Self::Item) -> bool;
///
/// The rfind method is similar, but it requires a double-ended iterator and searches values from
/// back to front, returning the last item for which the closure returns true.
///
/// Sometimes your closure isn't just a simple predicate casting a Boolean judgment on each item and
/// moving on: it might be something more complex that produces an interesting value in its own right.
/// In this case, find_map is just what you want.
///
///     fn find_map<B, F>(&mut self, f:F) -> Option<B> where
///         F: FnMut(Self::Item) -> Option<B>;
///
/// This is just like find, except that instead of returning bool, the closure should return an Option
/// of some value. find_map returns the first Option that is Some.
///
/// Building Collections and FromIterator
/// collect method can be used to build any kind of collection from Rust's standard library, as long
/// as the iterator produces a suitable item type. Naturally, collect itself doesn't know how to
/// construct all these types. Rather, when some collection type like Vec or HashMap knows how to
/// construct itself from an iterator, it implements the std::iter::FromIterator trait, for which
/// collect is just a convenient veneer:
///
///     trait FromIterator<A>:Sized {
///         fn from_iter<T: IntoIterator<Item=A>>(iter: T) -> Self;
///     }
///
/// If a collection type implements FromIterator<A>, then its type-associated function from_iter
/// builds a value of that type from an iterable producing items of type A.
///
/// The Extend Trait
/// If a type implements the std::iter::Extend trait, then its extend method adds an iterable's items
/// to the collection:
///
///     trait Extend<A> {
///         fn  extend<T>(&mut self, iter: T)
///             where T: IntoIterator<Item=A>;
///     }
pub(crate) fn extend() -> () {
    let mut v: Vec<_> = (0..5).map(|i| 1 << i).collect();
    v.extend([31,57,99,163]);

    assert_eq!(v, [1,2,4,8,16,31,57,99,163]);
}


/// partition
/// The partition method divides an iterator's items among two collections, using a closure to decide
/// where each item belongs.
/// Like collect, partition can make any sort of collections you like, although both must be of the
/// same type. And like collect, you'll need to specify the return type.
///
///     fn partition<B, F>(self, f:F) -> (B,B)
///         where Self: Sized,
///               B: Default + Extend<Self::Item>
///               F: FnMut(&Self::Item) -> bool;
///
/// Whereas collect requires its result type to implement FromIterator, partition instead requires
/// std::default::Default, which all Rust collections implement by returning an empty collection,
/// and std::default::Extend.
pub(crate) fn partition() {
    let things = ["doorknob", "mushroom", "noodle", "giraffe", "grapefruit"];

    let (living, nonliving) : (Vec<&str>, Vec<&str>) = things.iter().partition(|name| name.as_bytes()[0] & 1 != 0);

    assert_eq!(living, vec!["mushroom", "giraffe", "grapefruit"]);
    assert_eq!(nonliving, vec!["doorknob", "noodle"]);
}


/// for_each and try_for_each
/// The for_each method simply applies a closure to each item.
/// If your closure needs to be fallible or exit early, you can use try_for_each.
pub(crate) fn for_each_try_for_each() -> () {
    ["doves", "hens", "birds"].iter()
        .zip(["turtle", "french", "calling"])
        .zip(2..5)
        .rev()
        .map(|((item, kind), quantity)| format!("{quantity} {kind} {item}"))
        .for_each(|gift| println!("You have received: {gift}"));
}
