use std::collections::{HashSet, BinaryHeap, HashMap};
use std::collections::binary_heap::PeekMut;

fn main() {
    retain();

    heap();

    entries();
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
fn heap() {
    let mut heap = BinaryHeap::from(vec![2,3,8,6,9,5,4]);
    assert_eq!(heap.peek(), Some(&9));
    assert_eq!(heap.pop(), Some(9));

    if let Some(mut top) = heap.peek_mut() {
        if *top > 10 {
            PeekMut::pop(top);
        } else {
            *top = 5;
        }
    };

}


/// Entries
/// Both HashMap and BTreeMp have a corresponding Entry type. The point of entries is to eliminate
/// redundant map lookups.
/// The Entry value returned by map.entry(key) acts like a reference to a place within the map that's
/// either occupied by a key-value pair, or vacant, meaning there's no entry there yet. If vacant, the
/// entry's .or_insert_with() method inserts a new value.
///
///     pub fn entry<'a>(&'a mut self, key: K) -> Entry<'a, K,V>
///
/// The Entry type has a lifetime parameter 'a because it's effectively a fancy kind of borrowed mut
/// reference to the map. As long as the Entry exists, it has exclusive access to the map.
/// The Entry type is an enum, defined like this for HashMap (and similarly for BTreeMap):
///
///     pub enum Entry<'a, K, V> {
///         Occupied(OccupiedEntry<'a, K, V>
///         Vacant(VacantEntry<'a, K, V>
///     }
///
/// The OccupiedEntry and VacantEntry types have methods for inserting, removing, and accessing entries
/// without repeating the initial lookup.
fn entries() {
    let text = "This is a random text".to_string();

    let mut word_frequency : HashMap<&str, u32> = HashMap::new();
    for c in text.split_whitespace() {
        word_frequency.entry(c)
            .and_modify(|count| *count +=1)
            .or_insert(1);
    }

    for (key, value) in &word_frequency {
        println!("{key}: {value}");
    }
}

use enums_and_patterns::enums::RoughTime;
/// Hashing
/// std::hash::Hash is the standard library trait for hashable types. HashMap keys and HashSet
/// elements must implement both Hash and Eq. Most built in types that implement Eq also implement Hash.
/// One principle of the standard library is that a value should have the same hash code regardless of
/// where you store it or how you point to it. Therefore, a reference has the same hash code as the
/// value it refers to, and a Box has the same hash code as the boxed value. A vector has the same
/// hash code as the slice containing all its data, &vec[..]. A string has the same hash code as a &str
/// with the same characters.
/// If you implement PartialEq by hand for a type, you should also implement Hash by hand.
struct Artifact {
    id: u32,
    name: String,
    cultures: Vec<Culture>,
    date: RoughTime
}

enum Culture {
    Mayan, Inca, Aztec, Hindu
}

impl PartialEq for Artifact {
    fn eq(&self, other: &Artifact) -> bool {
        self.id ==other.id
    }
}

impl Eq for Artifact {}

use std::hash::{Hash, Hasher};

impl Hash for Artifact {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.id.hash(hasher);
    }
}
