/// A generic struct can also take parameters that are constant values.
/// A polynomial of degree N - 1.
pub(crate) struct Polynomial<const N: usize> {
    /// The coefficients of the polynomial.
    ///
    /// For a polynomial a + bx + cx2 + ... + zxn-1,
    /// the `i`th element is the coefficient of xi.
    pub (crate)coefficients: [f64; N]
}

/// A const generic parameter may be any integer type, char, or bool. Floating-point numbers, enums,
/// and other types are not permitted.
impl<const N: usize> Polynomial<N> {
    pub fn new(coefficients: [f64; N]) -> Polynomial<N> {
        Polynomial { coefficients}
    }

    /// Evaluate the polynomial at `x`
    pub fn eval(&self, x: f64) -> f64 {
        //Horner's method is numerically stable, efficient and simple:
        // c0 + x(c1 + x(c2 + x(c3 + ... x(cn-1 + xcn))))
        let mut sum = 0.0;
        for i in (0..N).rev() {
            sum = self.coefficients[i] + x*sum;
        }
        sum
    }
}


/// If the struct takes other kinds of generic parameters, lifetime parameters must come first,
/// followed by types, followed by any const values.
struct LumpOfReferences<'a, T, const N: usize> {
    the_lump: [&'a T; N]
}

/// When we a little bit of mutable data inside an otherwise immutable value, it is called
/// interior mutability.
/// A Cell<T> is a struct that contains a single private value of type T. The only special thing
/// about a Cell is that you can get and set the field even if you don't have mut access to the
/// Cell itself:
/// Cell::new(value) -> Creates a new Cell, moving the given value into it.
/// Cell.get() -> Returns a copy of the value in the cell.
/// Cell.set(value) -> Stores the given value in the cell, dropping the previously stored value.
/// This method takes self as a non-mut reference:
///     fn set(&self, value: T) //note: not `&mut self`
use std::cell::Cell;

pub struct SpiderRobot {
    hardware_error_count : Cell<u32>,
}

impl SpiderRobot {
    pub fn add_hardware_error(&self) {
        let n = self.hardware_error_count.get();
        self.hardware_error_count.set(n+1);
    }

    pub fn has_hardware_errors(&self) -> bool {
        self.hardware_error_count.get() > 0
    }
}

/// Cell doesnot let you call mut methods on a shared value. The .get() method returns a copy of the
/// value in the cell, so it works only if T implements the Copy trait.
/// Like Cell<T>, RefCell<T> is a generic type that contains a single value of type T. Unlike Cell,
/// RefCell supports borrowing references to its T value:
/// RefCell::new(value) -> Creates a new RefCell, moving value into it.
/// ref_cell.borrow() --> Returns a Ref<T>, which is essentially just a shared reference to the value
/// stored in ref_cell. This methods panics, if the value is already mutably borrowed.
/// ref_cell.borrow_mut() -> Returns a RefMut<T>, essentially a mutable reference to the value in
/// ref_cell. This method panics if the value is already borrowed.
/// ref_cell.try_borrow(), ref_cell.try_borrow_mut() -> Work just like borrow() and borrow_mut(), but
/// return a Result. Instead of panicking if the value is already mutably borrowed, they return an
/// Err value.
/// Cells qre easy to use. They are not thread-safe - Rust therefore will not allow multiple threads
/// to access them at once.
pub fn refcell() -> () {
    use std::cell::RefCell;

    let ref_cell: RefCell<String>  = RefCell::new("hello".to_string());

    let r = ref_cell.borrow();
    let count = r.len();
    assert_eq!(count, 5);

    let mut w = ref_cell.borrow_mut();  //panic: already borrowed
    w.push_str("world");
}
