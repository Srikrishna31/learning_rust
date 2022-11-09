mod drop;
mod sized;
mod deref;

use std::hash::Hash;
use crate::deref::Selector;
use crate::drop::*;

fn main() {
    let mut a = Appellation{
        name: "Zeus".to_string(),
        nicknames: vec!["cloud collector".to_string(),
                        "king of the gods".to_string()]
    };
    println!("before assignment");
    a = Appellation { name: "Hera".to_string(), nicknames: vec![]};
    println!("at end of block");

    let p;
    {
        let q = Appellation {
            name: "Cardamine hirsuta".to_string(),
            nicknames: vec!["shotweed".to_string(),
                            "bittercress".to_string()]
        };

        if true {
            p = q;
        }
    }
    println!("Sproing! What was that?");

    let boxed_lunch = sized::RcBox {
        ref_count: 1,
        value: "lunch".to_string()
    };

    drop(a);

    use std::fmt::Display;

    let boxed_displayable: &sized::RcBox<dyn Display> = &boxed_lunch;

    sized::display(&boxed_lunch); //Rust automatically creates RcBox<dyn Display> type
    sized::display(boxed_displayable);

    let mut s = Selector {
        elements : vec!['x', 'y', 'z'], current: 2
    };

    // Because `Selector` implements `Deref`, we can use the `*` operator to refer to its current element
    assert_eq!(*s, 'z');

    // Assert that 'z' is alphabetic, using a method of `char` directly on a `Selector`, via
    // deref coercion.
    assert!(s.is_alphabetic());

    //Change the 'z' to a 'w', by assigning to the `Selector`'s referent.
    *s = 'w';

    assert_eq!(s.elements, ['x', 'y', 'w']);

    let s = Selector { elements: vec!["good", "bad", "ugly"], current: 2};

    /// The deref coercions come with a caveat that can cause some confusion: Rust applies them to
    /// resolve type conflicts, but not to satisfy bounds on type variables. For example, the
    /// following code works fine:
    deref::show_it(&s);

    /// However, if we change the function to generic function, rust complains.
    /// Since you are passing an argument of type &Selector<&str> and the function's parameter type is
    /// &T, the type variable T must be Selector<&str>. Then, Rust checks whether the bound
    /// T: Display is satisfied: since it doesnot apply deref coercions to satisfy bounds on type
    /// variables, this check fails.
    deref::show_it_generic(&*s );

    use std::collections::HashSet;
    let squares = [4, 9, 16, 25, 36, 49, 64];
    let (powers_of_two, impure) : (HashSet<i32>, HashSet<i32>)
        = squares.iter().partition(|&n| n & (n - 1) == 0);

    assert_eq!(powers_of_two.len(), 3);
    assert_eq!(impure.len(), 4);

    let (upper, lower): (String, String) =
        "Great Teacher Onizuka".chars().partition(|&c| c.is_uppercase());
    assert_eq!(upper, "GTO");
    assert_eq!(lower, "reat eacher nizuka");

    let err = Error::FileNotFound("c:/af/bdf".as_ref());
    println!("Disaster has struck: {}", describe(&err));
}


/// Clone
/// The std::clone::Clone trait is for types that can make copies of themselves. Clone is defined
/// as follows:
trait CloneExample: Sized {
    /// The clone method should construct an independent copy of self and return it. Since this
    /// method's return type is Self and functions may not return unsized values, the Clone trait
    /// itself extends the Sized trait: this has the effect of bounding implementations' Self types
    /// to be Sized.
    fn clone (&self) -> Self;

    /// The clone_from method modifies self into a copy of source. The default definition of clone_from
    /// simply clones source and then moves that into *self. This always works, but for some types,
    /// there is a faster way to get the same effect. For example, suppose s and t are Strings. The
    /// statement s = t.clone(); must clone t, drop the old value of s, and then move the cloned value
    /// into s; that's one heap allocation and one heap deallocation. But if the heap buffer belonging
    /// to the original s has enough capacity to hold t's contents, no allocation or deallocation is
    /// necessary; you can simply copy t's text into s's buffer and adjust the length. In generic code
    /// you should use clone_from whenever possible to take advantage of optimized implementations
    /// when present.
    fn clone_from(&mut self, source: &Self) {
        *self = source.clone();
    }
}


/// Copy
/// A type is Copy if it implements the std::marker::Copy marker trait, which is defined as follows.
/// But because Copy is a marker trait with special meaning to the language, Rust permits a type to
/// implement Copy only if a shallow byte-for-byte copy is all it needs. Types that own any other
/// resources, like heap buffers or operating system handles, cannot implement Copy.
/// Any type that implements the Drop trait cannot be Copy. Rust presumes that if a type needs special
/// cleanup code, it must also require special copying code and thus can't be Copy.
/// Think carefully before making a type Copy. Although doing so makes the type easier to use, it
/// places heavy restrictions on its implementation.
trait CopyExample: Clone {}


/// Default
/// Sometypes have a reasonably obvious default value: the default vector or string is empty, the
/// default number is zero, the default Option is None, and so on.
/// Another common use of Default is to produce default values for structs that represent a large
/// collection of parameters, most of which you won't usually need to change.
///
/// If a type T implements Default, then the standard library implements Default automatically for
/// Rc<T>, Arc<T>, Box<T>, Cell<T>, RefCell<T>, Cow<T>, Mutex<T> and RwLock<T>.
/// If all the element types of a tuple type implement Default, then the tuple type does too,
/// defaulting to a tuple holding each element's default value.
/// Rust doesn't implicitly implement Default for struct types, but if all of a struct's fields
/// implement Default, you can implement Default for the struct automatically using #[derive(Default)].
trait DefaultExample {
    /// The default method simply returns a fresh value of Self.
    fn default() -> Self;
}

/// AsRef and AsMut
/// When a type implements AsRef<T>, that means that you can borrow a &T from it efficiently. AsMut
/// is the analogue for mutable references. Their definitions are as follows:
/// AsRef is typically used to make functions more flexible in the argument types they accept.
trait AsRefExample<T: ?Sized> {
    fn as_ref(&self) -> &T;
}

trait AsMut<T: ?Sized> {
    fn as_mut(&mut self) -> &mut T;
}


/// Borrow and BorrowMut
/// The std::borrow::Borrow trait is similar to AsRef: if a type implements Borrow<T>, then its
/// borrow method efficiently borrows a &T from it. But Borrow imposes more restrictions: a type
/// should implement Borrow<T> only when a  &T hashes and compares the same way as the value it's
/// borrowed from. This makes Borrow valuable in dealing with keys in hash tables and trees or when
/// dealing with values that will be hashed or compared for some other reason.
trait Borrow <Borrowed: ?Sized> {
    fn borrow(&self) -> &Borrowed;
}

/// Borrow is designed to address a specific situation with generic hash tables and other associative
/// collection types.
struct HashMap<T, U>
{
    key: T,
    value: U,
}

impl <K, V> HashMap<K, V> where K: Eq + Hash
{
    /// If you can borrow an entry's key as an &Q and the resulting reference hashes and compares
    /// just the way the key itself would, then clearly &Q ought to be an acceptable key type. Since
    /// String implements Borrow<str> and Borrow<String>, this version of get allows you to pass
    /// either &String or &str as a key, as needed.
    fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
        where K: Borrow<Q>,
              Q: Eq + Hash
    {
        None
    }
}

/// The BorrowMut trait is analogue of Borrow for mutable references:
trait BorrowMut<Borrowed: ?Sized> : Borrow<Borrowed> {
    fn borrow_mut(&mut self) -> &mut Borrowed;
}

/// ToOwned
/// Given a reference, the usual way to produce an owned copy of its referent is to call clone,
/// assuming the type implements std::clone::Clone. But what if you want to clone a &str or a &[i32]?
/// What you probably want is a String or a Vec<i32>, but Clone's definition doesn't permit that: by
/// definition, cloning a &T must always return a value of type T, and str and [u8] are unsized; they
/// aren't even types that a function could return.
/// The std::borrow::ToOwned trait provides a slightly looser way to convert a reference to an owned
/// value:
trait ToOwned {
    type Owned: Borrow<Self>;

    /// Unlike clone, which must return exactly Self, to_owned can return anything you could borrow
    /// a &Self from: the Owned type must implement Borrow<Self>. You can borrow a &[T] from a Vec<T>,
    /// so [T] can implement ToOwned<Owned=Vec<T>>, as long as T implements Clone, so that we can
    /// copy the slice's elements into the vector. Similarly, str implements ToOwned<Owned=String>,
    /// Path implements ToOwned<Owned=PathBuf> and so on.
    fn to_owned(&self) -> Self::Owned;
}


/// Borrow and ToOwned at Work: The Humble Cow
/// In some cases when you cannot decide whether to borrow or own until the program is running, the
/// std::borrow::Cow (for "clone on write") provides one way.
/// A Cow<B> either borrows a shared reference to a B or owns a value from which we could borrow such
/// a reference.
/// You can also get a mutable reference to a Cow's value by calling its to_mut method, which returns
/// a &mut B. If the Cow happens to be Cow::Borrowed, to_mut simply calls the reference's to_owned
/// method to get its own copy of the referent, changes the Cow into a Cow::Owned, and borrows a
/// mutable reference to the newly owned value. This is the "clone on write" behavior the type's name
/// refers to.
/// Similarly, Cow has an into_owned method that promotes the reference to an owned value, if necessary
/// and then returns it, moving ownership to the caller and consuming the Cow in the process.
enum CowExample<'a, B: ?Sized> where B: ToOwned
{
    Borrowed(&'a B),
    Owned(<B as ToOwned>::Owned),
}



use std::path::{PathBuf, Path};
use std::borrow::Cow;
use std::convert::AsRef;

enum Error<'a> {
    OutOfMemory,
    StackOverflow,
    MachineOnFire,
    Unfathomable,
    FileNotFound(&'a Path),
}

fn describe(error: &Error) -> Cow<'static, str> {
    match *error {
        Error::OutOfMemory => "out of memory".into(),
        Error::StackOverflow => "stack overflow".into(),
        Error::MachineOnFire => "machine on fire".into(),
        Error::Unfathomable => "machine bewildered".into(),
        Error::FileNotFound(ref path) => {
            format!("file not found: {}", path.display()).into()
        }
    }
}
