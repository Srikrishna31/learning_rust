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
