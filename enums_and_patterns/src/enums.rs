use std::cmp::Ordering::{self, *}; //`*` to import all children

fn compare(n: i32, m:i32) -> Ordering {
    if n < m {
        Less
    } else if n > m {
        Greater
    } else {
        Equal
    }
}

/// In memory, values of C-style enums are stored as integers.
/// Occasionally it's useful to tell Rust which integers to use.
/// Otherwise Rust will assign the numbers for you,starting at 0. By default, Rust stores C-style
/// enums using the smallest built-in integer type that can accommodate them.
pub(crate) enum HttpStatus {
    Ok = 200,
    NotModified = 304,
    NotFound = 404,
}


/// As with structs, the compiler will implement features like the == operator for you, but you
/// have to ask.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TimeUnit {
    Seconds, Minutes, Hours, Days, Months, Years,
}

/// Enums can have methods, just like structs
 impl TimeUnit {
    pub fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }

    pub fn singular(self) -> &'static str {
        self.plural().trim_end_matches('s')
    }
}


/// Two of the variants in this enum, InThePast and InTheFuter, take arguments. These are called
/// tuple variants. Like tuple structs, these constructors are functions that create new
/// RoughTime values
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}


pub (crate) struct Point3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3d {
    pub(crate) const ORIGIN:Point3d = Point3d {x: 0.0, y:0.0, z:0.0};
}

/// Enums can also have struct variants, which contain names fields, just like ordinary structs:
/// All constructors and fields of an enum share the same visibility as the enum itself.
pub(crate) enum Shape {
    Sphere { center: Point3d, radius: f32},
    Cuboid {corner1: Point3d, corner2: Point3d},
}

use std:: collections::HashMap;

/// In memory, enums with data are stored as a small integer tag, plus enough memory to hold all the
/// fields of the largest variant. The tag field is for Rust's internal use. It tells which
/// constructor created the value and therefore which fields it has.
/// Enums are also useful for quickly implementing tree-like data structures.
/// The JSON standard specifies the various data types that can appear in a JSON document: null,
/// Boolean values, numbers, strings, arrays of JSON values, and objects with string keys and JSON
/// values. The Json enum below simply spells out these types:
pub enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>),
}


/// Enums can be generic. Option and Result from standard library are great examples.
/// One unobvious detail is that Rust can eliminate the tag field of Option<T> when the type T is a
/// reference, Box, or other smart pointer type. Since none of those pointer types is allowed to be
/// zero, Rust can represent Option<Box<i32>>, say, as a single machine word: 0 for None and nonzero
/// for Some pointer. This makes such Option types close analogues to C/C++ pointer values that
/// could be null. The difference is that Rust's type system requires you to check that an Option is
/// Some before you can use its contents. This effectively eliminates null pointer dereferences.
pub enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>)
}

/// Each BinaryTree value is either Empty or NonEmpty. If it's Empty, then it contains no data at
/// all. If NonEmpty, then it has a Box, a pointer to a heap-allocated TreeNode.
/// Each TreeNode value contains one actual element, as well as two more BinaryTree values. This
/// means a tree can contain subtrees, and thus a NonEmpty tree can have any number of descendants.
pub struct TreeNode<T> {
    pub element: T,
    pub left: BinaryTree<T>,
    pub right: BinaryTree<T>,
}
