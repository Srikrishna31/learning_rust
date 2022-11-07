struct Canvas;

impl Canvas {
    fn write_at(&self, x:i32, y:i32, c:char) ->() {

    }
}
trait Visible {
    fn draw(&self, canvas: &mut Canvas);

    fn hit_test(&self, x:i32, y:i32) -> bool;
}

struct Broom {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

/// To implement a trait, use the syntax impl TraitName for Type:
impl Visible for Broom {
    fn draw(&self, canvas: &mut Canvas) {
        for y in self.broomstick_range() {
            canvas.write_at(self.x, y, '|');
        }
    }

    fn hit_test(&self, x: i32, y: i32) -> bool {
        self.x == x && self.y - self.height - 1 <= y && y <= self.y
    }
}

use std::ops::Range;
/// If we wanted to add a helper method in support of Broom::draw(), we would have to define it in a
/// separate block
impl Broom {
    fn broomstick_range(&self) -> Range<i32> {
        self.y - self.height -1 ..self.y
    }
}


pub struct Sink;

use std::io::{Write, Result};

impl Write for Sink {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }

    //Apart from the above methods, the Write trait contains a lot of default methods, which are
    //not required to be implemented here.
}


/// A trait can use the keyword Self as a type. A trait that uses the Self type is incompatible with
/// trait objects.
pub trait Cln {
    /// Using Self as the return type here means that the type of x.clone() is the same as the type
    /// of x, whatever that might be. If x is a String, then the type of x.clone() is String - not dyn
    /// Cln or any other cloneable type.
    fn clone(&self) -> Self;
}

/// Rust lets you implement any trait on any type, as long as either the trait or the type is
/// introduced in the current crate.
pub(crate) trait IsEmoji {
    fn is_emoji(&self) -> bool;
}

/// Implement IsEmoji for the built-in character type.
impl IsEmoji for char {
    fn is_emoji(&self) -> bool {
        false
    }
}

enum Direction {
    Up, Down, Left, Right
}
/// We can declare that a trait is an extension of another trait.
/// The phrase trait Creature: Visible means that all creatures are visible. Every type that implements
/// Creature must also implement the Visible trait. We can implement the two traits in either order,
/// but it's an error to implement Creature for a type without also implementing Visible. Here, we say
/// that Creature is a subtrait of Visible, and that Visible is Creature's supertrait.
/// Subtraits resemble subinterfaces in Java or C#, in that users can assume that any value that
/// implements a subtrait implements its supertrait as well. But in Rust, a subtrait doesnot
/// inherit the associated items of its supertrait; each trait still needs to be in scope if you want
/// to call its methods.
trait Creature: Visible {
    fn position(&self) -> (i32, i32);
    fn facing(&self) -> Direction;
}


/// Associated Types (or How Iterators Work)
/// The first feature of this trait, type Item;, is an associated type. Each type that implements
/// Iterator must specify what type of item it produces.
/// The second feature, the next() method, uses the associated type in its return value. next()
/// returns an Option<Self::Item>: either Some(item), the next value in the sequence, or None when
/// there are no more values to visit. The type is written as Self::Item, because Item is a feature
/// of each type of iterator, not a standalone type.
pub trait Iterator {
    type Item;

    fn next (&mut self) -> Option<Self::Item>;
}

/// Orphan Rule: When you implement a trait, either the trait or the type must be new in the current
/// crate.
/// It helps Rust ensure that trait implementations are unique. Your code can't impl Write for u8,
/// because both Write and u8 are defined in the standard library. If Rust let crates do that, there
/// could be multiple implementations of Write for u8, in different crates, and Rust would have no
/// reasonable way to decide which implementation to use for a given method call.
struct Args;

// (code from the std::env standard library module)
impl Iterator for Args {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        None
    }
}


/// Generic Traits (or How Operator Overloading Works)
/// Multiplication in Rust uses this trait:
/// std::ops::Mul, the trait for types that support `*`.
/// Generic traits get a special dispensation when it comes to the orphan rule: you can implement a
/// foreign trait for a foreign type, so long as one of the trait's type parameters is a type
/// defined in the current crate.
pub trait Mult<RHS> {
    /// The resulting type after applying the `*` operator
    type Output;

    /// The method for the `*` operator
    fn mul(self, rhs: RHS) -> Self::Output;
}


/// Associated Constants
/// Like structs and enums, traits can have associated constants. Associated constants in traits have
/// special power, though. Like associated types and functions, you can declare them but not give
/// them a value:
trait Float {
    const ZERO: Self;
    const ONE: Self;
}

/// Then implementations of the trait can define these values:
impl Float for f32 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
}
/// Note that associated constants can't be used with trait objects, since the compiler relies on type
/// information about the implementation in order to pick the right value at compile time.
impl Float for f64 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
}

use std::ops::{Add, Mul};

fn add_one<T: Float + Add<Output=T>>(value: T) -> T {
    value + T::ONE
}

fn dot<N>(v1: &[N], v2: &[N]) -> N
    where N:Add<Output=N> + Mul<Output=N> + Default + Copy
{
    let mut total: N = N::default();
    for i in 0..v1.len() {
        total = total + v1[i] * v2[i];
    }
    total
}

#[test]
fn test_dot() {
    assert_eq!(dot(&[1,2,3,4], &[1,1,1,1]), 10);
    assert_eq!(dot(&[53.0, 7.0], &[1.0, 5.0]), 88.0);
}
