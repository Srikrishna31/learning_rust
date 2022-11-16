/// Macros are a kind of shorthand. During compilation, before types are checked and long before any
/// machine code is generated, each macro call is expanded-that is, it's replaced with some Rust code.
/// macro_rules! is the main way to define macros in Rust. The `!` is only included when calling a
/// macro, not when defining it.
/// A macro defined with macro_rules! works entirely by pattern matching. The body of a macro is just
/// a series of rules:
///
///     ( pattern1 ) => ( template1 );
///     ( pattern2 ) => ( template2 );
///     ...
///
/// Rust expands macros very early during compilation. You can't call a macro before it is defined,
/// because Rust expands each macro call before it even looks at the rest of the program. (By contrast,
/// functions and other items don't have to be in any particular order. It's OK to call a function that
/// won't be defined until later in the crate.)
/// Macro patterns are a mini language within Rust. They're essentially regular expressions for matching
/// code. But where regular expressions operate on characters, patterns operate on tokens-the numbers,
/// names, punctuation marks, and so forth that are the building blocks of Rust programs. This means
/// you can use comments and whitespace freely in macro patterns to make them as readable as possible.
macro_rules! my_vec {
    // This rule handles uses like vec![0u8; 1000].
    ($elem:expr; $n:expr) => {
        ::std::vec::from_elem($elem, $n)
    };
    // This rule handles vec!["udon", "ramen", "soba"]. The pattern, $($x:expr),* uses repetition feature
    // of macros. It matches 0 or more expressions, separated by commas. More generally, the syntax
    // $(PATTERN),* is used to match any comma-separated list, where each item in the list matches PATTERN.
    // The * here has the same meaning as in regular expressions (0 or more). You can also use + to
    // require atleast one match, or ? for zero or one match.
    // The code fragment $x is not just a single expression, but a list of expressions. The template
    // for this rule uses repetition syntax too.
    ($ ($x:expr),*) => {
        <[_]>::into_vec(Box::new([$($x),*]))
    };
    // Unlike the rest of Rust, patterns using $( ... ),* do not automatically support an optional
    // trailing comma. However, there's a standard trick for supporting trailing commas by adding
    // an extra rule. This is what this third rule does.
    // We use `$(...),+ ,` to match a list with an extra comma. Then, in the template, we call vec!
    // recursively, leaving the extra comma out. This time the second rule will match.
    ($ ($x:expr),+ ,) => {
        vec![$($x),*]
    };
}


#[derive(Clone, PartialEq, Debug)]
pub enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>)
}

impl From<bool> for Json {
    fn from(b: bool) -> Json {
        Json::Boolean(b)
    }
}

impl From<String> for Json {
    fn from(s: String) -> Json {
        Json::String(s)
    }
}

impl<'a> From<&'a str> for Json {
    fn from(s: &'a str) -> Json {
        Json::String(s.to_string())
    }
}

/// For all numeric types, its better to write a macro and invoke it.
macro_rules! impl_from_num_for_json {
    ( $($t:ident)* ) => {
        $(
            impl From<$t> for Json {
                fn from(n: $t) -> Json {
                    Json::Number(n as f64)
                }
            }
        )*
    };
}

impl_from_num_for_json!(u8 i8 u16 i16 u32 i32 u64 i64 u128 i128 usize isize f32 f64);

pub use std::collections::HashMap;
pub use std::boxed::Box;
pub use std::string::ToString;

/// Token trees: it is either a properly matched pair of brackets, (...), [...], or {...}, and
/// everything in between, including nested token trees, or a single token that isn't a bracket, like
/// 1926 or "Knots".
/// Token trees are exactly what we need for our json! macro. Every JSON value is a single token tree:
/// numbers, strings, Boolean values, and null are all single tokens; objects and arrays are bracketed.
#[macro_export]
macro_rules! json {
    (null) => {
        // $crate acts like an absolute path to the root module of the crate where the macro was
        // defined. Instead of saying Json, we can write $crate::Json, which works even if Json was
        // not imported.
        $crate::Json::Null
    };
    ([$($element:tt),*]) => {
        $crate::Json::Array(vec![$(json!($element)),*])
    };
    ({ $($key:tt : $value:tt),*}) => {
        $crate::Json::Object($crate::json::Box::new(vec![
            $( ($crate::json::ToString::to_string($key), json!($value)) ),*
        ].into_iter().collect()))
    };
    ($other:tt) => {
        $crate::Json::from($other)  //Handle Boolean/Number/String
    }
}

/// Whenever macros use temporary variables in the code expansion, Rust renames those variables, after
/// pasting it in the target place. This feature, first implemented in Scheme macros, is called
/// hygiene, and so Rust is said to have hygienic macros.
/// This prevents name collisions with the local variables of the code calling the macros.
#[test]
fn json_null() {
    assert_eq!(json!(null), Json::Null);
}


#[test]
fn json_array_with_json_element() {
    let macro_generate_value = json!(
        [
            //valid json that doesn't match `$element:expr`
            {
                "pitch":440.0
            }
        ]
    );

    let hand_coded_value = Json::Array(vec![
        Json::Object(Box::new(vec![("pitch".to_string(), Json::Number(440.0))].into_iter().collect()))
    ]);

    assert_eq!(macro_generate_value, hand_coded_value);
}
