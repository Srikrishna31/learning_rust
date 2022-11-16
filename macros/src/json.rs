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
