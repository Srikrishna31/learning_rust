/// You can specify how dereferencing operators like * and . behave on your types by implementing
/// the std::ops::Deref and std::ops::DerefMut traits. Pointer types like Box<T> and Rc<T> implement
/// these traits so that they can behave as Rust's built-in pointer types do. For example, if you have
/// a Box<Complex> value b, then *b refers to the Complex value that b points to, and b.re refers to
/// its real component. If the context assigns or borrows a mutable reference to the referent, Rust
/// uses the DerefMut(dereference mutably) trait; otherwise, read-only access is enough, and it uses
/// Deref.
trait DerefExample {
    type Target: ?Sized;
    fn deref(&self) -> &Self::Target;
}

trait DerefMutExample : DerefExample {
    /// The deref and deref_mut methods take a &Self reference and return a &Self::Target reference.
    /// Target should be something that Self contains, owns or refers to: for Box<Complex> the Target
    /// type is Complex. Note that DerefMut extends Deref: if you can dereference something and modify
    /// it, certainly you should be able to borrow a shared reference to it as well. Since the methods
    /// return a reference with the same lifetime as &self, self remains borrowed for as long as the
    /// returned reference lives.
    fn deref_mut(&mut self) -> &mut Self::Target;
}

/// The Deref and DerefMut traits play another role as well. Since deref takes a &Self reference and
/// returns a &Self::Target reference, Rust uses this to automatically convert references of the
/// former type into the latter. In other words, if inserting a deref call would prevent a type
/// mismatch, Rust inserts one for you. Implementing DerefMut enables the corresponding conversion
/// for mutable references. These are called `deref coercions`: one type is being "coerced" into
/// behaving as another.
/// Rust will apply several deref coercions in succession if necessary. For example, you can apply
/// split_at directly to an Rc<String>, since &Rc<String> dereferences to &String, which dereferences
/// to &str, which has the split_at method.
pub(crate) struct Selector<T> {
    /// Elements available in this `Selector`.
    pub(crate) elements: Vec<T>,

    /// The index of the "current" element in `elements`. A `Selector` behaves like a pointer to the
    /// current element.
    pub(crate) current: usize,
}

/// To make the Selector behave as the doc comment claims, you must implement Deref and DerefMut for
/// the type:
use std::ops::{Deref, DerefMut};

/// The Deref and DerefMut traits are designed for implementing smart pointer types like Box, Rc and
/// Arc, and types that serve as owning versions of something you would also frequently use by reference,
/// the way Vec<T> and String serve as owning versions of [T] and str. You should not implement Deref
/// and DerefMut for a type just to make the Target type's methods appear on it automatically, the
/// way a C++ base class's methods are visible on a subclass. This will not always work as you expect
/// and can be confusing when it goes awry.
impl <T> Deref for Selector<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.elements[self.current]
    }
}

impl <T> DerefMut for Selector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.elements[self.current]
    }
}

pub(crate) fn show_it(thing: &str) {
    println!("{}", thing);
}

use std::fmt::Display;

pub(crate) fn show_it_generic<T:Display>(thing: T) {
    println!("{}", thing);
}
