///
/// # Raw Pointers
/// A **raw pointer** in Rust is an unconstrained pointer. You can use raw pointers to form all sorts
/// of structures that Rust's checked pointer types cannot, like doubly linked lists or arbitrary
/// graphs of objects. But because raw pointers are so flexible, Rust cannot tell whether you are
/// using them safely or not, so you can dereference them only in an unsafe block.
/// Raw pointers are essentially equivalent to C or C++ pointers, so they're also useful for
/// interacting with code written in those languages.
///
/// There are two kind of raw pointers:
/// * A *mut T is a raw pointer to a T that permits modifying its referent.
/// * A *const T is a raw pointer to a T that only permits reading its referent.
///
/// (There is no plain *T type; you must always specify either const or mut.)
///
/// Although Rust implicitly dereferences safe pointer types in various situations, raw pointer
/// dereferences must be explicit:
/// * The . operator will not implicitly dereference a raw pointer; you must write `(*raw).field` or
/// `(*raw).method(...)`
/// * Raw pointers do not implement Deref, so deref coercions donot apply to them.
/// * Operators like == and < compare raw pointers as addresses: two raw pointers are equal if they
/// point to the same location in memory. Similarly, hashing a new pointer hashes the address it points
/// to, not the value of its referent.
/// * Formatting traits like std::fmt::Display follow references automatically, but don't handle raw
/// pointers at all. The exceptions are std::fmt::Debug and std::fmt::Pointer, which show raw pointers
/// as hexadecimal addresses, without dereferencing them.
pub fn raw_pointers() {
    let mut x = 10;
    let ptr_x = &mut x as *mut i32;

    let y = Box::new(20);
    let ptr_y = &*y as *const i32;

    unsafe {
        *ptr_x += *ptr_y;
    }

    assert_eq!(x, 30);

    let trucks = vec!["garbage truck", "dump truck", "moonstruck"];
    let first : *const &str = &trucks[0];
    let last: *const &str = &trucks[2];

    assert_eq!(unsafe { last.offset_from(first)}, 2);
    assert_eq!(unsafe {first.offset_from(last)}, -2);

    assert!(!option_to_raw(Some(&("pea", "pod"))).is_null());
    assert_eq!(option_to_raw::<i32>(None), std::ptr::null());
}

/// Unlike boxes and references, raw pointers can be null, like NULL in C or nullptr in C++. This
/// example has no unsafe blocks: creating raw pointers, passing them around, and comparing them
/// are all safe. Only dereferencing a raw pointer is unsafe.
fn option_to_raw<T>(opt: Option<&T>) -> *const T {
    match opt {
        None => std::ptr::null(),
        Some(r) => r as *const T
    }
}

use std::marker::PhantomData;
use std::mem::align_of;

/// A `&T` and a `bool`, wrapped up in a single word. The type `T` must require at least two-byte al
/// alignment.
///
/// This sort of technique is used regularly in garbage collectors and virtual machines, where certain
/// types - say, the type representing the object - are so numerous that adding even a single word to
/// each value would drastically increase memory use.
pub struct RefWithFlag<'a, T> {
    ptr_and_bit: usize,
    /// The PhantomData is necessary for Rust to know how to treat lifetimes in code that use RefWithFlag.
    behaves_like: PhantomData<&'a T> // occupies no space
}

impl<'a, T: 'a> RefWithFlag<'a, T> {
    pub fn new(ptr: &'a T, flag: bool) -> RefWithFlag<T> {
        assert!(align_of::<T>() %2 == 0);
        RefWithFlag {
            ptr_and_bit: ptr as *const T as usize | flag as usize,
            behaves_like: PhantomData
        }
    }

    /// Borrowing a raw pointer's referent gives you a reference with an unbounded lifetime: Rust will
    /// accord the reference whatever lifetime would make the code around it check, if there is one.
    /// In this case, since get_ref's return type is &'a T, Rust sees that the reference's lifetime is
    /// the same as RefWithFlag's lifetime parameter 'a, which is just what we want: that's the lifetime
    /// of the reference we started with.
    pub fn get_ref(&self) -> &'a T {
        unsafe {
            let ptr = (self.ptr_and_bit & !1) as *const T;
            &*ptr
        }
    }

    pub fn get_flag(&self) -> bool {
        self.ptr_and_bit & 1 != 0
    }
}
