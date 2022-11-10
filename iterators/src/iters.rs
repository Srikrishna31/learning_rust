use std::fmt::Debug;
/// IntoIterator
/// Most collections actually provide several implementations of IntoIterator for shared references
/// (&T), mutable references (&mut T), and moves (T):
/// * Given a shared reference to the collection, into_iter returns an iterator that produces shared
/// references to its items.
/// * Given a mutable reference to the collection, into_iter returns an iterator that produces mutable
/// references to the items.
/// * When passed the collection by value, into_iter returns an iterator that takes ownership of the
/// collection and returns- items by value; the items' ownership moves from the collection to the
/// consumer, and the original collection is consumed in the process.
pub(crate) fn dump<T,U>(t: T)
    where T: IntoIterator<Item=U>,
          U: Debug
{
    for u in t {
        println!("{:?}", u);
    }
}


/// If each item depends on the one before, the std::iter::successors function works nicely. You can
/// provide an initial item and a function that takes one item and returns an Option of the next. If
/// it returns None, the iteration ends.
use num::Complex;
use std::iter::successors;

pub(crate) fn escape_time(c:Complex<f64>, limit: usize) -> Option<usize> {
    let zero = Complex{re: 0.0, im: 0.0};

    successors(Some(zero), |&z| {Some(z*z + c)})
        .take(limit)
        .enumerate()
        .find(|(_i, z)| z.norm_sqr() > 4.0)
        .map(|(i, _z)| i)
}


/// Both from_fn and successors accept FnMut closures, so your closures can capture and modify variables
/// from surrounding scopes.
pub(crate) fn fibonacci() -> impl Iterator<Item=usize> {
    let mut state = (0, 1);

    std::iter::from_fn(move || {
        state = (state.1, state.0 + state.1);
        Some(state.0)
    })
}
