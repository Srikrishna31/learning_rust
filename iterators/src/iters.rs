use std::collections::BTreeMap;
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


/// Iterator Adapters
/// Once you have an iterator in hand, the Iterator trait provides a broad selection of adapter
/// methods, or simply adapters, that consume one iterator and build a new one with useful behaviors.
///
/// map and filter
/// The map adapter lets you transform an iterator by applying a closure to its items. The filter
/// adapter lets you filter out items from an iterator, using a closure to decide which to keep and
/// which to drop.
/// The adapters signatures are as follows:
///
///     fn map<B,F>(self, f:F) -> impl Iterator<Item=B>
///         where Self: Sized, F: FnMut(Self::Item) -> B;
///
///     fn filter<P>(self, predicate: P) -> impl Iterator<Item=Self::Item>
///         where Self: Sized, P: FnMut(&Self::Item) -> bool;
///
/// A map iterator passes each item to its closure by value and, in turn, passes along ownership of
/// the closure's result to its consumer.
/// A filter iterator passes each item to its closure by shared reference, retaining ownership in case
/// the item is selected to be passed on to its consumer.
///
/// Simply calling an adapter on an iterator doesn't consume any items; it just returns a new iterator,
/// ready to produce its own items by drawing from the first iterator as needed. In a chain of adapters,
/// the only way to make any work actually get done is to call next on the final iterator.
///
/// Another important point is that iterator adapters are a zero-overhead abstraction. Since map, filter
/// and their companions are generic, applying them to an iterator specializes their code for the
/// specific iterator type involved. This means Rust has enough information to inline each iterator's
/// next method to its consumer and then translate the entire arrangement into machine code as a unit.
pub(crate) fn map_and_filter() -> () {
    let text = "   ponies   \n      giraffees\niguanas      \nsquid".to_string();
    let v = text.lines()
        .map(str::trim)
        .collect::<Vec<&str>>();

    assert_eq!(v, ["ponies", "giraffees", "iguanas", "squid"]);
}


/// filter_map and flat_map
/// The filter_map adapter is similar to map excep that it lets its closure either transform the item
/// into a new item or drop the item from the iteration. Thus, it's a bit like a combination of filter
/// and map. It's signature is as follows:
///
///     fn filter_map<B, F>(self, F: F) -> impl Iterator<Item=B>
///         where Self: Sized, F: FnMut(Self::Item) -> Option<B>;
use std::str::FromStr;

pub(crate) fn filter_map() -> () {
    let text = "1\nfrond    .25     289\n3.1415 estuary\n";
    for number in text
        .split_whitespace()
        .filter_map(|w| f64::from_str(w).ok())
    {
        println!("{:4.2}", number.sqrt());
    }
}


/// You can think of the flat_map adapter as continuing in the same vein as map and filter_map, except
/// that now the closure can return not just one item (as with map) or zero or one items(as with
/// filter_map), but a sequence of any number of items. The flat_map iterator produces the concatenation
/// of the sequences the closure returns. Below is the signature of flat_map:
///
///     fn flat_map<U,F>(self, f:F) -> impl Iterator<Item=U::Item>
///         where F: FnMut(Self::Item) -> U,U:IntoIterator
/// The closure passed to flat_map must return an iterable, but any sort of iterable will do.
pub(crate) fn flat_map() -> () {
    use std::collections::HashMap;

    let mut major_cities = HashMap::new();
    major_cities.insert("Japan", vec!["Tokyo", "Kyoto"]);
    major_cities.insert("United States", vec!["Portland", "Nashville"]);
    major_cities.insert("Brazil", vec!["Sao Paulo", "Brasilia"]);
    major_cities.insert("Kenya", vec!["Nairobi", "Mombasa"]);
    major_cities.insert("Netherlands", vec!["Amsterdam", "Utrecht"]);

    let countries = ["Japan", "Brazil", "Kenya"];

    for &city in countries.iter().flat_map(|country| &major_cities[country]) {
        println!("{}", city);
    }
}


/// flatten
/// The flatten adapter concatenates an iterator's items, assuming each item is itself an iterable.
/// The signature of flatten is as follows:
///
///     fn flatten(self) -> impl Iterator<Item=Self::Item::Item>
///         where Self::Item: IntoIterator;
pub(crate) fn flatten() -> () {
    let mut parks = BTreeMap::new();
    parks.insert("Portland", vec!["Mt. Tabor Park", "Forest Park"]);
    parks.insert("Kyoto", vec!["Tadasu-no-Mori Forest", "Maruyama Koen"]);
    parks.insert("Nashville", vec!["Percy Warner Park", "Dragon Park"]);

    let all_parks: Vec<_> = parks.values().flatten().cloned().collect();

    assert_eq!(all_parks,
                vec!["Tadasu-no-Mori Forest", "Maruyama Koen", "Percy Warner Park", "Dragon Park",
                "Mt. Tabor Park", "Forest Park"]);


    assert_eq!(vec![None, Some("day"), None, Some("one")]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>(), vec!["day", "one"]);
}


/// take and take_while
/// The Iterator trait's take and take_while adapters let you end an iteration after a certain number
/// of items or when a closure decides to cut things off. Their signatures are as follows:
///
///     fn take(self, n: usize) -> impl Iterator<Item=Self::Item>
///         where Self: Sized;
///
///     fn take_while<P>(self, predicate: P) -> impl Iterator<Item=Self::Item>
///         where Self: Sized, P: FnMut(&Self::Item) -> bool;
///
/// Both take ownership of an iterator and return a new iterator that passes along items from the first
/// one, possibly ending the sequence earlier.
pub(crate) fn take_while() -> () {
    let message = "To: jimb\r\n\
                         From: superego <editor@oreilly.com>\r\n\
                         \r\n\
                         Did you get any writing done today?\r\n\
                         When will you stop wasting time plotting fractals?\r\n";
    for header in message.lines().take_while(|l| !l.is_empty()) {
        println!("{header}");
    }
}

/// skip and skip_while
/// These are the complement of take and take_while: they drop a certain number of items from the
/// beginning of an iteration, or drop items until a closure finds one acceptable, and then pass the
/// remaining items through unchanged.
///
///     fn skip(self, n : usize) -> impl Iterator<Item=Self::Item>
///         where Self: Sized;
///
///     fn skip_while(self, predicate: P) -> impl Iterator<Item=Self::Item>
///         where Self: Sized, P: FnMut(&Self::Item) -> bool;
pub(crate) fn skip_while() -> () {
    let message = "To: jimb\r\n\
                         From: superego <editor@oreilly.com>\r\n\
                         \r\n\
                         Did you get any writing done today?\r\n\
                         When will you stop wasting time plotting fractals?\r\n";

    for body in message.lines()
        .skip_while(|l|!l.is_empty())
        .skip(1) {
        println!("{}", body);
    }

}

use std::iter::Peekable;

/// peekable
/// A peekable iterator lets you peek at the next item that will be produced without actually consuming
/// it.
///
///     fn peekable(self) -> std::iter::Peekable<Self>
///         where Self: Sized;
///
/// Here, Peekable<Self> is a struct that implements Iterator<Item=Self::Item>, and Self is the type
/// of the underlying iterator.
/// A Peekable iterator has an additional method peek that returns an Option<&Item>: None if the
/// underlying iterator is done and otherwise Some(r), where r is a shared reference to the next item.
/// Calling peek tries to draw the next item from the underlying iterator, and if there is one, caches
/// it until the next call to next. All the other Iterator methods on Peekable know about this cache:
/// for example, iter.last() on a peekable iterator iter knows to check the cache after exhausing the
/// underlying iterator.
pub(crate) fn parse_number<I>(tokens: &mut Peekable<I>) -> u32
    where I : Iterator<Item=char>
{
    let mut n = 0;
    loop {
        match tokens.peek() {
            Some(r) if r.is_digit(10) => {
                n = n*10 + r.to_digit(10).unwrap();
            }
            _ => return n
        }
        tokens.next();
    }
}


pub(crate) struct Flaky(pub(crate) bool);

/// fuse
/// The fuse adapter takes any iterator and produces one that will definitely continue to return None
/// once it has done so the first time.
/// The fuse adapter is probably most useful in generic code that needs to work with iterators of
/// uncertain origin. Rather than hoping that every iterator you'll have to deal with will be
/// well-behaved, you can use fuse to make sure.
impl Iterator for Flaky {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 {
            self.0 = false;
            Some("totally the last item")
        } else {
            self.0 = true;
            None
        }
    }
}
