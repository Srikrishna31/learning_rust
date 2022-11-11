/// Reversible Iterators - rev
/// Some iterators are able to draw items from both ends of the sequence. You can reverse such
/// iterators by using the rev adapter.
///
///     trait DoubleEndedIterator : Iterator {
///         fn next_back(&mut self) -> Option<Self::Item>
///     }
///
/// You can think of a double-ended iterator as having two fingers marking the current front and back
/// of the sequence. Drawing items from either end advances that finger toward the other; when the
/// two meet, the iteration is done.
/// Most iterator adapters, if applied to a reversible iterator, return another reversible iterator.
/// For example, map and filter preserve reversibility.
pub(crate) fn rev_iterator() -> () {
    let bee_parts = vec!["head", "thorax", "abdomen"];

    let mut iter = bee_parts.iter();

    assert_eq!(iter.next(), Some(&"head"));
    assert_eq!(iter.next_back(), Some(&"abdomen"));
    assert_eq!(iter.next(), Some(&"thorax"));

    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next(), None);

    let meals = ["breakfast", "lunch", "dinner"];
    let mut iter = meals.iter().rev();

    assert_eq!(iter.next(), Some(&"dinner"));
    assert_eq!(iter.next(), Some(&"lunch"));
    assert_eq!(iter.next(), Some(&"breakfast"));
    assert_eq!(iter.next(), None);
}


/// inspect
/// The inspect adapter is handy for debugging pipelines of iterator adapters, but it isn't used much
/// in production code. It simply applies a closure to a shared reference to each item and then passes
/// the item through. The closure can't affect the items, but it can do things like print them or make
/// assertions about them.
pub(crate) fn inspect() -> () {
    let upper_case: String = "gross".chars()
        .inspect(|c| println!("before: {:?}", c))
        .flat_map(|c| c.to_uppercase())
        .inspect(|c| println!("after: {:?}", c))
        .collect();

    assert_eq!(upper_case, "GROSS");

}


/// chain
/// The chain adapter appends one iterator to another. More precisely, i1.chain(i2) returns an iterator
/// that draws items from i1 until it's exhausted and then draws items from i2.
///
///     fn chain<U>(self, other: U) -> impl Iterator<Item=Self::Item>
///         where Self:Sized, U: IntoIterator<Item=Self::Item>;
///
/// A chain iterator is reversible, if both (or all) of its underlying iterators are.
/// A chain iterator keeps track of whether each of the two underlying iterators has returned None and
/// directs next and next_back calls to one or the other as appropriate.
pub(crate) fn chain() -> () {
    let v: Vec<i32> = (1..4).chain([20,30,40]).collect();

    assert_eq!(v, [1,2,3,20,30,40]);

    let u : Vec<i32> = (1..4).chain([20,30,40]).rev().collect();
    assert_eq!(u, [40,30,20,3,2,1]);
}


/// enumerate
/// The enumerate adapter attaches a running index to the sequence, taking an iterator that produces
/// items A, B, C, ... and returning an iterator that produces pairs (0, A), (1, B), (2, C), ...
/// Consumers can use that index to distinguish one item from another and establish the context in
/// which to process each one.
///
/// zip
/// The zip adapter combines two iterators into a single iterator that produces pairs holding one value
/// from each iterator, like a zipper joining its two sides into a single seam. The zipped iterator ends
/// when either of the two underlying iterators ends.
pub(crate) fn zip() -> () {
    let v: Vec<_> = (0..).zip("ABCD".chars()).collect();
    assert_eq!(v, vec![(0, 'A'), (1, 'B'), (2, 'C'), (3, 'D')]);

    use std::iter::repeat;

    let endings = ["once", "twice", "chicken soup with rice"];
    let rhyme: Vec<_> = repeat("going")
        .zip(endings)
        .collect();

    assert_eq!(rhyme, vec![("going", "once"),
                           ("going", "twice"),
                           ("going", "chicken soup with rice")]);
}


/// by_ref
/// An iterator's by_ref method borrows a mutable reference to the iterator so that you can apply
/// adapters to the reference. When you're done consuming items from these adapters, you drop them,
/// the borrow ends, and you regain access to your original iterator.
///
///     impl<'a, I: Iterator + ?Sized> Iterator for &'a mut I {
///         type Item = I::Item
///         fn next(&mut self) -> Option<I::Item> {
///             (**self).next()
///         }
///         fn size(&self) -> (usize, Option<usize>) {
///             (**self).size_hint()
///         }
///     }
pub(crate) fn by_ref() -> () {
    let message = "To: jimb\r\n\
                         From: id\r\n\
                         \r\n\
                         Ooooooh, donuts!\r\n";
    let mut lines = message.lines();

    println!("Headers:");
    for header in lines.by_ref().take_while(|l| !l.is_empty()) {
        println!("{header}");
    }

    /// The call lines.by_ref() borrows a mutable reference to the iterator, and it is this reference
    /// that the take_while iterator takes ownership of. That iterator goes out of scope at the end
    /// of the first for loop, meaning that the borrow has ended, so you can use lines again in the
    /// second for loop.
    println!("\nBody:");
    for body in lines {
        println!("{body}");
    }
}


/// cloned, copied
/// The cloned adapter takes an iterator that produces references and returns an iterator that produces
/// values cloned from those references, much like iter.map(|item| item.clone()). Naturally, the referent
/// type must implement Clone.
/// The copied adapter is the same idea, but more restrictive: the referent type must implement Copy.
pub(crate) fn clone() -> () {
    let a = ['1', '2', '3', '='];

    assert_eq!(a.iter().next(), Some(&'1'));
    assert_eq!(a.iter().cloned().next(), Some('1'));
}



/// cycle
/// The cycle adapter returns an iterator that endlessly repeats the sequence produced by the underlying
/// iterator. The underlying iterator must implement std::clone::Clone so that cycle can save its initial
/// state and reuse it each time the cycle starts again.
pub(crate) fn cycle() -> () {
    let dirs = ["North", "East", "South", "West"];
    let mut spin = dirs.iter().cycle();

    assert_eq!(spin.next(), Some(&"North"));
    assert_eq!(spin.next(), Some(&"East"));
    assert_eq!(spin.next(), Some(&"South"));
    assert_eq!(spin.next(), Some(&"West"));
    assert_eq!(spin.next(), Some(&"North"));
    assert_eq!(spin.next(), Some(&"East"));

    use std::iter::{once, repeat};

    let fizzes = repeat("").take(2).chain(once("fizz")).cycle();
    let buzzes = repeat("").take(4).chain(once("buzz")).cycle();
    let fizzes_buzzes = fizzes.zip(buzzes);

    let fizz_buzz = (1..100).zip(fizzes_buzzes)
        .map(|tuple|
                match tuple {
                    (i, ("", "")) => i.to_string(),
                    (_, (fizz,buzz)) => format!("{fizz}{buzz}"),
                });

    for line in fizz_buzz {
        println!("{line}");
    }
}
