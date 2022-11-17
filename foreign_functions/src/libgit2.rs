
/// Rust's std::os::raw module defines a set of Rust types that are guaranteed to have the same
/// representation as certain C types. These cover the primitive integer and character types.
/// For defining Rust struct types compatible with C structs, you can use the #[repr(C)] attribute.
/// Placing this above a struct definition asks Rust to lay out the analogous C struct type.
use std::os::raw::{c_char, c_int};

/// The #[repr(C)] attribute affects only the layout of the struct itself, not the representations of
/// its individual fields, so to match the C struct, each field must use the C-like type as well:
/// *const c_char for char *, c_int for int, and so on.
///
/// Whereas C and C++ guarantee that a structure's members appear in memory in the order they're declared
/// each at a distinct address, Rust reorders fields to minimize the overall size of the struct, and
/// zero-sized types take up no space. The #[repr(C)] attribute tells Rust to follow C's rules for the
/// given type.
#[repr(C)]
pub struct git_error {
    pub message: *const c_char,
    pub klass: c_int,
}

#[repr(C)]
#[allow(non_camel_case_types)]
enum git_error_code {
    GIT_OK = 0,
    GIT_ERROR = -1,
    GIT_ENOTFOUND = -3,
    GIT_EEXISTS = -4,
}

#[repr(C)]
enum Tag {
    Float = 0,
    Int = 1,
}

#[repr(C)]
union FloatOrInt {
    f: f32,
    i: i32,
}

#[repr(C)]
struct Value {
    tag: Tag,
    union: FloatOrInt
}

fn is_zero(v: Value) -> bool {
    use self::Tag::*;
    unsafe {
        match v {
            Value{ tag: Int, union: FloatOrInt{i : 0}} => true,
            Value{tag: Int, union: FloatOrInt{f:num} } => num == 0.0,
            _ => false,
        }
    }
}

extern {
    /// # Declaring Foreign Functions and Variables
    /// An extern block declares functions or variables defined in some other library that the final Rust
    /// executable will be linked with. This gives Rust the function's name and type, while leaving the
    /// definition to be linked in later.
    ///
    /// Rust assumes that functions declared inside extern blocks use C conventions for passing arguments
    /// and accepting return values. They are defined as unsafe functions.
    pub(crate) fn strlen(s: *const c_char) -> usize;
}

#[link(name = "git2")]
extern {
    /// # Using Functions from Libraries
    /// To use functions provided by a particular library, you can place a #[link] attribute atop the
    /// extern block that names the library Rust should link the executable with.
    pub fn git_libgit2_init() -> c_int;
    pub fn git_libgit2_shutdown() -> c_int;
}

