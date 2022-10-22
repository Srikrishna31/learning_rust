fn main() {
    // When an integer operation overflows, Rust panics in a debug build, but in a release build,
    // the operation wraps around: it produces the value equivalent to the mathematically correct
    // result modulo the range of the value. (In neither case is overflow undefined behavior, as it
    // is in C and C++).
    let mut i = 1;
    loop {
        i *= 10; // panic: attempt to multiply with overflow (but only in debug builds!)
    }

    //When this default behavior isn't what you need, the integer types provide methods that let
    //you spell out exactly what you want.
    let mut j = 1_i8;
    loop {
        j = j.checked_mul(10).expect("multiplication overflowed");
    }

    //Checked operations return an Option of the result: Some(v) if the mathematically correct
    //result can be represented as a value of that type, or None if it cannot.
    assert_eq!(10_u8.checked_add(20), Some(30));
    assert_eq!(100_u8.checked_add(200), None);

    //Wrapping operations return the value equivalent to the mathematically correct result modulo
    //the range of the value.
    assert_eq!(100_u16.wrapping_mul(200), 20000);
    assert_eq!(500_u16.wrapping_mul(500), 53392);

    //Saturating operations return the representable value that is closest to the mathematically
    //correct result. In other words, the result is "clamped" to the maximum and minimum values the
    //type can represent
    assert_eq!(32760_i16.saturating_add(10), 32767);
    assert_eq!((-32760_i16).saturating_sub(10), -32768);

    //Overflowing operations return a tuple (result, overflowed), where result is what the wrapping
    //version of the function would return, and overflowed is a bool indicating whether overflow
    //occurred:
    assert_eq!(255_u8.overflowing_sub(2), (253, false));
    assert_eq!(255_u8.overflowing_add(2), (1, true));


    //Method calls have higher precedence than prefix operators, so be sure to correctly parenthesize
    //method calls on negated values
    assert_eq!((-1.01f64).floor(), -2.0);

    //A tuple is a pair, or triple, quadruple, quintuple, etc. (hence, n-tuple or tuple), of values
    //of asserted types. Tuples allow only constants as indices, like t.4. You can't write t.i or
    //t[i] to get the ith element.
    //The other commonly used tuple type is the zero-tuple (). This is traditionally called the
    //unit type because it has only one value, also written (). This is equivalent to void in C/C++.
    println!("Hello, world!");
}

fn reference() -> () {
    /*
    The expression &x produces a reference to x; in Rust terminology, we say it borrows a reference
    to x. Given a reference r, the expression *r refers to the value r points to. These are very much
    like the & and * operators in C and C++. And like a C pointer, a reference does not automatically
    free any resources when it goes out of scope.
    And unlike C, Rust tracks the ownership and lifetimes of values, so mistakes like dangling
    pointers, double frees, and pointer invalidation are ruled out at compile time.

    Rust references come in two flavors:
    &T
    An immutable, shared reference. You can have many shared references to a given value at a time,
    but they are read-only: modifying the value they point to is forbidden, as with const T* in C.

    &mut T\
    A mutable, exclusive reference. You can read and modify the value it points, as with a T* in C.
    But for as long as the reference exists, you may not have any other references of any kind to
    that value. In fact, the only way you may access the value at all is through the mutable reference.


    Rust uses this dichotomy between shared and mutable references to enforce a "single writer or
    multiple readers" rule: either you can read and writ the value, or it can be shared by any
    number of readers, but never both at the same time. This separation enforced by compile-time
    checks, is central to Rust's safety guarantees.
     */
    let i = 30;

    let j = &i;

    println!("{}", *j);

    let k = &mut i;
    println!("{}", *j);

    println!("{}", *k);
}


fn references_continued() -> ()
{
    let mut i = 40;

    let j = &i;

    //*j = 51; //Doesn't allow to mutate the value, since the reference is immutable, although the
    //value is declared mutable.

    println!("{}", *j);

    let k =&mut i;

    println!("{}", *j); //Doesn't allow to use j, since a mutable reference has been declared above.

    *k = 70;
}

fn boxes() -> ()
{
    //The simplest way to allocate a value in the heap is to use Box::new;
    let t = (12, "eggs");
    let b = Box::new(t); //allocate a tuple in the heap
    /*
    The type of t is (i32, &str), so the type of b is Box<(i32, &str)>. The call to Box::new
    allocates enough memory to contain the tuple on the heap. When b goes out of scope, the memory
    is freed immediately, unless b has been moved - by returning it for eg.
     */
}

fn raw_pointers() -> ()
{
    /*
    Rust also has the raw pointer types *mut T and *const T. Raw pointers really are like pointers
    in C++. Using a raw pointer is unsafe, because Rust makes no effort to track what it points to.
    However, you may only dereference- raw pointers within an unsafe block. An unsafe block is Rust's
    opt-in mechanism for advanced language features whose safety is up to you.
     */
}

