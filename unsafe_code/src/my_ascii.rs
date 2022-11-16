/// Unsafe blocks
/// An unsafe block looks just like an ordinary Rust block preceded by the unsafe key word, with the
/// difference that you can use unsafe features in the block:
///
///     unsafe {
///         String::from_utf8_unchecked(ascii)
///     }
///
/// Like an ordinary Rust block, the value of an unsafe block is that of its final expression, or ()
/// if it doesn't have one.
/// An unsafe block unlocks five additional options for you:
/// * You can call unsafe functions. Each unsafe function must specify its own contract, depending on
/// its purpose.
/// * You can dereference raw pointers. Safe code can pass raw pointers around, compare them, and
/// create them by conversion from references(or even from integers), but only unsafe code can
/// actually use them to access memory.
/// * You can access the fields of unions, which the compiler can't be sure contain valid bit patterns
/// for their respective types.
/// * You can access mutable static variables. Rust can't be sure when threads are using mutable
/// static variables, so their contract requires you to ensure all access is properly synchronized.
/// You can access functions and variables declared through Rust's foreign function interface. These
/// are considered unsafe even when immutable, since they are visible to code written in other
/// languages that may not respect Rust's safety rules.
#[derive(Debug, Eq, PartialEq)]
pub struct Ascii (
    // This must hold only well-formed ASCII text: bytes from `0` to `0x7f`
    Vec<u8>,
);

impl Ascii {
    /// Create an `Ascii` from the ASCII text in `bytes`. Return a `NotAsciiError` error if `bytes`
    /// contains any non-ASCII characters.
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Ascii, NotAsciiError> {
        if bytes.iter().any(|&byte| !byte.is_ascii()) {
            return Err(NotAsciiError(bytes));
        }
        Ok(Ascii(bytes))
    }

    /// An unsafe function definition looks like an ordinary function definition preceded by the
    /// unsafe keyword. The body of an unsafe function is automatically considered an unsafe block.
    /// You may call unsafe functions only within unsafe blocks. This means that marking a function
    /// unsafe warns its callers that the function has a contract they must satisfy to avoid
    /// undefined behavior.
    /// Construct an `Ascii` value from `bytes`, without checking whether `bytes` actually contains
    /// well-formed ASCII.
    /// This constructor is infallible, and returns an `Ascii` directly, rather than a
    /// `Result<Ascii, NotAsciiError>` as the `from_bytes` constructor does.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `bytes` contains only ASCII characters: bytes no greater than
    /// 0x7f. Otherwise, the effect is undefined.
    ///
    /// # Behavior
    ///  There are two critical facts about bugs and unsafe code:
    /// * Bugs that occur before the unsafe block can break contracts.
    /// * The consequences of breaking a contract may appear after you leave the unsafe block.
    pub unsafe fn from_bytes_unchecked(bytes: Vec<u8>) -> Ascii {
        Ascii(bytes)
    }
}

/// When conversion fails, we give back the vector we couldn't convert. This should implement
/// `std::error::Error`; omitted for brevity
#[derive(Debug, Eq, PartialEq)]
pub struct NotAsciiError(pub Vec<u8>);

// Safe, efficient conversion, implemented using unsafe code.
impl From<Ascii> for String {
    /// # Unsafe Block or Unsafe function?
    /// * If it's possible to misuse the function in a way that compiles fine but still causes
    /// undefined behavior, you must mark it unsafe. The rules for using the function correctly are
    /// its contract; the existence of a contract is what makes the function unsafe.
    /// * Otherwise, the function is safe: no well-typed call to it can cause undefined behavior. It
    /// should **not** be marked unsafe.
    fn from(ascii: Ascii) -> String {
        // If this module has no bugs, this is safe, because well-formed ASCII text is also well-formed UTF-8
        unsafe { String::from_utf8_unchecked(ascii.0) }
    }
}


/// # Undefined Behavior
/// Below are Rust's rules for well-behaved programs:
/// * The program must not read uninitialized memory.
/// * The program must not create invalid primitive values:
///     - References, boxes, or fn pointers that are null
///     - bool values that are not either 0 or 1
///     - enum values with invalid discriminant values
///     - char values that are not valid, non-surrogate Unicode code points.
///     - str values that are not well-formed UTF-8
///     - Fat pointers with invalid vtables/slice lengths
///     - Any value of the "never" type, written !, for functions that don't return
/// * The rules for references must be followed. No reference may outlive its referent; shared
/// access is read-only access; and mutable access is exclusive access.
/// * The program must not deference null, incorrectly aligned or dangling pointers.
/// * The program must not use a pointer to access memory outside the allocation with which the pointer
/// is associated.
/// * The program must be free of data races. A data race occurs when two threads access the same
/// memory location without synchronization, and at least one of the accesses is a write.
/// * The program must not unwind across a call made from another language, via the foreign function
/// interface.
/// * The program must comply with the contracts of standard library functions.
///
/// Any violation of these rules constitutes undefined behavior and renders Rust's efforts to optimize
/// your program and translate it into machine language untrustworthy.
///
/// # Unsafe Traits
/// An *unsafe trait* is a trait that has a contract Rust cannot check or enforce that implementers
/// must satisfy to avoid undefined behavior. To implement an unsafe trait, you must mark the implementation
/// as unsafe.
pub unsafe trait Zeroable {}

unsafe impl Zeroable for u8 {}
unsafe impl Zeroable for i32 {}
unsafe impl Zeroable for usize {}
unsafe impl Zeroable for u32 {}
unsafe impl Zeroable for u16 {}
unsafe impl Zeroable for i16 {}
unsafe impl Zeroable for isize {}
unsafe impl Zeroable for i8 {}


pub fn zeroed_vector<T>(len: usize) -> Vec<T>
    where T: Zeroable
{
    let mut vec = Vec::with_capacity(len);
    unsafe {
        std::ptr::write_bytes(vec.as_mut_ptr(), 0, len);
        vec.set_len(len);
    }
    vec
}
