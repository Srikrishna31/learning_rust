/// A sized type is one whose values all have the same size in memory. Almost all types in Rust are
/// sized: every u64 takes eight bytes, every (f32, f32, f32) tuple twelve. Even enums are sized: no
/// matter which variant is actually present, an enum always occupies enough space to hold its larges
/// variant. And although a Vec<T> owns a heap-allocated buffer whose size can vary, the Vec value
/// itself is a pointer to the buffer, its capacity, and its length, so Vec<T> is a sized type.
///
/// All sized types implement the std::marker::Sized trait, which has no methods or associated types.
/// Rust implements it automatically for all types to which it applies; you can't implement it yourself.
/// The only use for Sized is as a bound for type variables: a bound like T:Sized requires T to be a
/// type whose size is known at compile time. Traits of this sort are called marker traits, because
/// the Rust language itself uses them to mark certain types as having characteristics of interest.
///
/// However, Rust also has a few unsized types whose values are not all the same size. For example,
/// the string slice type str (note, without an &) is unsized. The string literals "diminutive" and
/// "big" are references to str slices that occupy ten and three bytes. Array slice types like [T]
/// (again, without an &) are unsized, too: a shared reference like &[u8] can point to a [u8] slice
/// of any size. Because the str and [T] types denote sets of values of varying sizes, they are unsized
/// types. The other common kind of unsized type in Rust is a dyn type, the referent of a trait object.
///
/// Rust can't store unsized values in variables ore pass them as arguments. You can only deal with
/// them through pointers like &str or Box<dyn Write>, which themselves are sized. A pointer to an
/// unsized value is always a fat pointer, two words wide: a pointer to a slice also carries the
/// slice's length, and a trait object also carries a pointer to a vtable of method implementations.
/// When a type variable has the ?Sized bound, people often say it is questionably sized: it might be
/// Sized, or it might not.
///
/// Aside from slices and trait objects, there is one more kind of unsized type. A struct type's last
/// field (but only its last) may be unsized, and such a struct itself is unsized.
pub(crate) struct RcBox<T: ?Sized> {
    pub(crate) ref_count: usize,
    pub(crate) value: T,
}

use std::fmt::Display;

/// You can't build an RcBox<dyn Display> value directly. Instead, you first need to create an ordinary
/// sized RcBox whose value type implements Display, like RcBox<String>. Rust then lets you convert
/// a reference &RcBox<String> to a fat reference &RcBox<dyn Display>
pub(crate) fn display(boxed: &RcBox<dyn Display>) {
    println!("For your enjoyment: {}", &boxed.value);
}

