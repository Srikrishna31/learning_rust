use enums;
use enums::*;
/// match performs pattern matching: patterns are the parts that appear before => symbol.
/// Expressions produce values; patterns consume values. The two use a lot of the same syntax.
/// When a pattern contains simple identifiers like units and count, those become local variables
/// in the code following the patterns. Whatever is present in the values is copied or moved into
/// the new variables.
pub fn rough_time_to_english(rt:RoughTime) -> String {
    match rt {
        RoughTime::InThePast(units, count) =>
            format!("{count} {} ago", units.plural()),
        RoughTime::JustNow => format!("just now"),
        RoughTime::InTheFuture(unit, 1) => format!("a {} from now", unit.singular()),
        RoughTime::InTheFuture(TimeUnit::Hours, 1) => format("an hour from now"),
        RoughTime::InTheFuture(count, units) =>
            format!("{count} {} from now", units.plural()),
    }
}

/// Tuple patterns match tuples. They're useful any time you want to get multiple pieces of data
/// involved in a single match:
pub fn describe_point(x: i32, y:i32) -> &'static str {
    use std::cmp::Ordering::*;
    match(x.cmp(&0), y.cmp(&0)) {
        (Equal, Equal) => "at the origin",
        (_, Equal) => "on the x axis",
        (Equal, _) => "on the y axis",
        (Greater, Greater) => "in the first quadrant",
        (Less, Greater) => "in the second quadrant",
        _ => "somewhere else",
    }
}


/// Array patterns match arrays. They're often used to filter out some special-case values and are
/// useful any time you're working with arrays whose values have a different meaning based on
/// position.
pub fn hsl_to_rgb(hsl: [u8; 3]) -> [u8; 3] {
    match hsl {
        [_, _, 0] => [0,0,0],
        [_,_,255] => [255,255,255],
        _ => [1,1,1],
    }
}

pub fn greet_people(names: &[&str]) -> () {
    match names {
        [] => println!("Hello, nobody"),
        [a] => println!("Hello, {a}."),
        [a,b] => println!("Hello, {a} and {b}."),
        [a, .., b] => println!("Hello, everyone from {a} to {b}."),
    }
}

/// Rust patterns support two features for working with references. ref patterns borrow parts of a
/// matched value. & patterns match references.
/// Matching a noncopyable value moves the value.
/// A pattern of the form pat1 | pat2 matches if either subpattern matches:
///     let at_end = match chars.peek() {
///         Some(&'\r' | &'\n') | None => true,
///         _ => false,
///     };
/// Binding with @ patterns
/// Finally, x @ pattern matches exactly like the given pattern, but on success, instead of creating
/// variables for parts of the matched value, it creates a single variable x and moves or copies the
/// whole value into it:
///     match chars.next() {
///         Some(digit @ '0'..='9') => read_number(digit, chars),
///     },
///
/// Although patterns are most prominent in match expressions, they are also allowed in several
/// other places:
/// * unpack a struct into local variables
/// * unpack a function argument that's a tuple
/// * iterate over keys and values of a HashMap
/// * automatically dereference an argument to a closure
/// The same concept exists in other languages: In JavaScript, it's called destructuring, while in
/// Python, it's unpacking.
///
/// Patterns that always match are called irrefutable match, and they are allowed in the above four
/// places. A refutable pattern is one that might not match.
impl<T: Ord> BinaryTree<T> {
    pub fn add(&mut self, value: T) {
        match *self {
            BinaryTree::Empty => {
                *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                    element: value,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                }))
            }
            BinaryTree::NonEmpty(ref mut node) => {
                if value <= node.element {
                    node.left.add(value);
                } else {
                    node.right.add(value);
                }
            }
        }
    }
}
