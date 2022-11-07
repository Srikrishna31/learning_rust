
#[derive(Clone, Copy, Debug)]
struct Complex<T> {
    re: T,
    im: T,
}

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
/// doesn't cause them to be moved, which would be troublesome. The syntax 'where Rhs: ?Sized'
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

fn main() {
    println!("Hello, world!");

    let x = Complex{re:5, im:2};
    let y = Complex{re:2, im:5};

    assert_eq!(x*y, Complex{re: 0, im: 29});
}
