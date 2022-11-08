
#[derive(Clone, Copy, Debug)]
struct Complex<T> {
    re: T,
    im: T,
}

use std::cmp::Ordering;
use std::ops::Neg;

impl <T> Neg for Complex<T> where T: Neg<Output=T> {
    type Output = Complex<T>;
    fn neg(self) -> Complex<T> {
        Complex {
            re: -self.re,
            im: -self.im,
        }
    }
}

use std::ops::Add;

impl<L, R> Add<Complex<R>> for Complex<L>
    where L: Add<R>,
{
    type Output = Complex<L::Output>;
    fn add(self, rhs: Complex<R>) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

use std::ops::Sub;

impl <L, R> Sub<Complex<R>> for Complex<L>
    where L : Sub<R>,
{
    type Output = Complex<L::Output>;
    fn sub(self, rhs: Complex<R>) -> Self::Output {
        Complex {
            re : self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

use std::ops::Mul;

impl <T> Mul for Complex<T>
    where  T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Copy
{
    type Output = Self;
    fn mul(self, rhs: Complex<T>) -> Self {
        Complex {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

trait Negate {
    type Output;
    fn neg(self) -> Self::Output;
}

trait Not {
    type Output;
    fn not(self) -> Self::Output;
}


/// A compound assignment expression is one like x += y or x &= y: it takes two operands, performs
/// some operation on them like addition or a bitwise AND, and stores the result back in the left
/// operand. In Rust, the value of a compound assignment expression is always (), never the value
/// stored.
trait AddAssignExample<Rhs=Self> {
    fn add_assign(&mut self, rhs: Rhs);
}

use std::ops::AddAssign;

/// The built-in trait for a compound assignment operator is completely independent of the built-in
/// trait for the corresponding binary operator. Implementing std::ops::Add doesnot automatically
/// implement std::ops::AddAssign; if you want Rust to permit your type as the lefthand operand of a
/// += operator, you must implement AddAssign yourself.
impl<T> AddAssign for Complex<T>
    where T: AddAssign<T>,
{
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

/// Unlike the arithmetic and bitwise traits, which take their operands by value, PartialEq takes its
/// operands by reference. This means that comparing non-Copy values like Strings, Vecs, or HashMaps
/// doesn't cause them to be moved, which would be troublesome. The syntax 'where Rhs: ?Sized', relaxes
/// Rust's usual requirement that type parameters must be sized types, letting us write traits like
/// PartialEq<str> or PartialEq<[T]>. The eq and ne methods take parameters of type &Rhs, and comparing
/// something with a &str or a &[T] is completely reasonable.
trait PartialEqExample<Rhs=Self> where Rhs: ?Sized, {
    fn eq(&self, other: &Rhs) -> bool;
    fn ne(&self, other: &Rhs) -> bool {
        !self.eq(other)
    }
}

/// PartialEq - Why it is named as such?
/// The traditional mathematical definition of an equivalence relation, of which equality is one
/// instance, imposes three requirements. For any values x and y:
/// * If x == y is true, then y == x must be true as well. In other words, swapping the two sides of
/// an equality comparison doesn't affect the result.
/// * If x == y and y == z, then it must be the case that x == z. Given any chain of values, each
/// equal to the next, each value in the chain is directly equal to every other. Equality is contagious.
/// * It must always be true that x == x.
/// So, while Rust's == operator meets the first two requirements for equivalence relations, it clearly
/// doesn't meet the third when used on IEEE floating-point values. This is called a partial
/// equivalence relation, so Rust uses the name PartialEq for the == operator's built-in trait.
impl<T: PartialEq> PartialEq for Complex<T> {
    fn eq(&self, other: &Complex<T>) -> bool {
        self.re == other.re && self.im == other.im
    }
}


/// Rust specifies the behavior of the ordered comparison operators <,>,<= and >= all in terms of a
/// single trait, std::cmp::PartialOrd:
/// Like the other binary operators, to compare values of two types Left and Right, Left must
/// implement PartialOrd<Right>.
trait PartialOrdExample<Rhs=Self> : PartialEq<Rhs>
    where Rhs: ?Sized,
{
    /// If partial_cmp returns None, that means self and other are unordered wrt each other: neither
    /// is greater than the other, nor are they equal. Among all of Rust's primitive types, only comparisons
    /// between floating-point values ever return None: specifically, comparing a NaN value with
    /// anything else returns None.
    fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;

    /// Below functions contain default implementations in the Rust standard library.
    fn lt(&self, other: &Rhs) -> bool;
    fn le(&self, other: &Rhs) -> bool;
    fn gt(&self, other: &Rhs) -> bool;
    fn ge(&self, other: &Rhs) -> bool;
}



#[derive(Debug, PartialEq)]
struct Interval<T> {
    lower: T, //inclusive
    upper: T, // exclusive
}

use std::cmp::{PartialOrd};

/// We would like to make values of Interval type partially ordered: one interval is less than another
/// if it falls entirely before the other, with no overlap. If two unequal intervals overlap, they're
/// unordered: some element of each side is less than some element of the other. And two equal
/// intervals are simply equal.
impl <T: PartialOrd> PartialOrd<Interval<T>> for Interval<T> {
    fn partial_cmp(&self, other: &Interval<T>) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.lower >= other.upper {
            Some(Ordering::Greater)
        } else if self.upper <= other.lower {
            Some(Ordering::Less)
        } else {
            None
        }
    }
}

/// Index and IndexMut
/// You can specify how an indexing expression like a[i] works on your type by implementing the
/// std::ops::Index and std::ops::IndexMut traits. Arrays support the [] operator directly, but on
/// any other type, the expression a[i] is normally shorthand for *a.index(i), where index is a
/// method of the std::ops::Index trait. However, if the expression is being assigned to or borrowed
/// mutably, it's instead shorthand for *a.index_mut(i), a call to the method of the std::ops::IndexMut
/// trait.
/// Note that these traits take tye type of the index expression as a parameter. You can index a slice
/// with a single usize, referring to a single element, because slices implement Index<usize>. But
/// you can refer to a subslice with an expression like a[i..j] because they also implement
/// Index<Range<usize>>. That expression is shorthand for:
///     *a.index(std::ops::Range{start:i, end: j})
/// Rust automatically selects index_mut when the indexing expression occurs in a context where it's
/// necessary.
/// One limitation of IndexMut is that, by design, it must return a mutable reference to some value.
/// This is why you can't use an expression like m["a"] = 10; to insert a value into the HashMap m:
/// the table would need to create an entry for "a" first, with some default value, and return a
/// mutable reference to that. But not all types have cheap default values, and some may be expensive
/// to drop; it would be waste to create such a value only to be immediately dropped by assignment.
/// (There are plans to improve this in later versions of the language.)
trait IndexExample<Idx> {
    type Output: ?Sized;
    fn index(&self, index: Idx) -> &Self::Output;
}

trait IndexMutExample<Idx> {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output;
}


struct Image<P> {
    width: usize,
    pixels: Vec<P>,
}

impl <P: Default + Copy> Image<P> {
    fn new(width: usize, height: usize) -> Image<P> {
        Image {
            width, pixels: vec![P::default(); width*height],
        }
    }
}

impl<P> std::ops::Index<usize> for Image<P> {
    type Output = [P];
    fn index(&self, index: usize) -> &Self::Output {
        let start = row*self.width;
        &self.pixels[start..start + self.width]
    }
}

impl<P> std::ops::IndexMut<usize> for Image<P> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let start = row * self.width;
        &mut self.pixels[start..start + self.width]
    }
}

fn main() {
    println!("Hello, world!");

    let x = Complex{re:5, im:2};
    let y = Complex{re:2, im:5};

    assert_eq!(x*y, Complex{re: 0, im: 29});

    assert!(Interval{lower:10, upper:20} < Interval{lower: 20, upper: 40});
    assert!(Interval{lower: 7, upper: 8} >= Interval{lower: 0, upper: 1});
    assert!(Interval{lower: 7, upper: 8} <= Interval{lower: 7, upper: 8});

    let left = Interval{lower: 10, upper: 30};
    let right = Interval{lower: 20, upper: 40};

    assert!(!(left < right));
    assert!(!(left >= right));
}
