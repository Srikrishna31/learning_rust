mod iters;

/// An iterator is any value that implements the std::iter::Iterator trait.
trait IteratorExample {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // many more default methods.
}

/// If there's natural way to iterate over some type, that type can implement std::iter::IntoIterator,
/// whose into_iter method takes a value and returns an iterator over it:
trait IntoIteratorExample where Self::IntoIter : Iterator<Item=Self::Item> {
    type Item;
    type IntoIter: Iterator;
    fn into_iter(self) -> Self::IntoIter;
}

fn main() {
    let v = vec![4, 20, 12, 8, 6];
    let mut iterator = v.iter();
    assert_eq!(iterator.next(), Some(&4));
    assert_eq!(iterator.next(), Some(&20));
    assert_eq!(iterator.next(), Some(&12));
    assert_eq!(iterator.next(), Some(&8));
    assert_eq!(iterator.next(), Some(&6));
    assert_eq!(iterator.next(), None);

    use std::ffi::OsStr;
    use std::path::Path;

    let path = Path::new("C:/Users/krishna/Downloads/Fedora.iso");
    let mut iterator1 = path.iter();

    assert_eq!(iterator1.next(), Some(OsStr::new("C:")));
    assert_eq!(iterator1.next(), Some(OsStr::new("Users")));
    assert_eq!(iterator1.next(), Some(OsStr::new("krishna")));
    assert_eq!(iterator1.next(), Some(OsStr::new("Downloads")));
    assert_eq!(iterator1.next(), Some(OsStr::new("Fedora.iso")));

    // When a type implements IntoIterator, you can call its into_iter method yourself, just as a for
    // loop would:
    use std::collections::BTreeSet;
    let mut favorites = BTreeSet::new();
    favorites.insert("Lucy in the Sky with Diamonds".to_string());
    favorites.insert("Liebestraume No. 3".to_string());

    let mut it = favorites.into_iter();
    assert_eq!(it.next(), Some("Liebestraume No. 3".to_string()));
    assert_eq!(it.next(), Some("Lucy in the Sky with Diamonds".to_string()));
    assert_eq!(it.next(), None);

    println!("Hello, world!");


    use rand::random;
    use std::iter::from_fn;

    // Generate the lengths of 1000 random line segments whose endpoints are uniformly distributed
    // across the interval [0,1].
    let lengths: Vec<f64> = from_fn(|| Some((random::<f64>() - random::<f64>()).abs()))
                    .take(1000)
                    .collect();

    iters::dump(lengths);

    assert_eq!(iters::fibonacci().take(8).collect::<Vec<_>>(),
                    vec![1,1,2,3,5,8,13,21]);
}
