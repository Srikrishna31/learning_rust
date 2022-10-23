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

/*
Rust has three types for representing a sequence of values in memory:
* The type [T;N] represents an array of N values, each of type T. An array's size is a constant
determined at compile time and is part of the type; you can't append new elements or shrink an array.

* The type Vec<T>, called a vector of Ts, is a dynamically allocated, growable sequence of values of
type T. A vector's elements live on the heap, so you can resize vectors at will: push new elements
onto them, append other vectors to them, delete elements and so on.

* The types &[T] and &mut [T], called a shared slice of Ts and mutable slice of Ts are references to
a series of elements that are a part of some other value, like an array or vector.


Given a value v of any of these three types, the expression v.len() gives the number of elements in
v, and v[i] refers to the ith element of v. The first element is v[0], and the last element is
v[v.len() - 1]. Rust checks that i always falls within this range; if it doesn't, the expression
panics. The length of v may be zero, in which case any attempt to index it will panic. i must be a
usize value; you can't use any other integer type as an index.
 */

fn arrays() -> () {
    let lazy : [u32;6] = [1,2,4,7,11,16];
    let tax = ["Animalia", "Arthropoda", "Insecta"];

    assert_eq!(lazy[3], 7);
    assert_eq!(tax.len(),3);
}

fn vectors() -> () {
    //The vec! macro is equivalent to calling Vec::new to create a new, empty vector and then pushing
    //elements onto it.
    let mut primes = vec![2,3,5,7];
    assert_eq!(primes.iter().product(), 210);

    primes.push(11);
    primes.push(13);
    assert_eq!(primes.iter().product(), 30030);

    //Another possibility is to build a vector from the values produced by an iterator:
    let v: Vec<i32> = (0..5).collect();
    assert_eq!(v, [0,1,2,3,4]);

    //A palindrome!
    let mut palindrome = vec!["a man", "a plan", "a canal", "panama"];
    palindrome.reverse();
    assert_eq!(palindrome, vec!["panama", "a canal", "a plan", "a man"]);


    /*
    A Vec<T> consists of three values: a pointer to the heap-allocated buffer for the elements, which
    is created and owned by Vec<T>; the number of elements that buffer has the capacity to store; and
    the number it actually contains now. When the buffer has reached its capacity, adding another
    element to the vector entails allocating a larger buffer, copying the present contents into it.,
    updading the vector's pionter and capacity to describe the new buffer, and finally freeing the
    old one.
     */
}

fn new_pixel_buffer(rows: usize, cols:usize) -> Vec<u8> {

    vec![0; rows*cols]
}


fn slices() -> () {
    /*
    A slice, written [T] without specifying the length, is a region of an array or vector.
    Since a slice can be any length, slices can't be stored directly in variables or passed as
    function arguments. Slices are always passed by reference.

    A reference to a slice is a fat pointer: a two-word value comprising a pointer to the slice's
    first element, and the number of elements in the slice.
     */
    let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let a: [f64; 4] = [0.0, -0.707, -1.0, -0.707];

    let sv = &v;
    let sa = &a;

    let print = |x:&[f64]| {
        for elt in n {
            println!("{elt}");
        }
    };

    print(&a);
    print(&v);

    print(&v[0..2]); //print the first two elements of v
    print(&a[2..]); //print the elements of a starting with a[2]
    print(&sv[1..3]); // print v[1] and v[2]
}


fn strings() ->() {
    // String literals are enclosed in double quotes. They use the same backslash escape sequences
    // as char literals:
    let speech = "\"Ouch!\" said the well.\n";

    //If one line of a string ends with a backslash, then the newline character and the leading
    //white space on the next line are dropped:
    println!("It was a bright, cold day in April, and \
            there were four of us-\
            more or less.");

    //A raw string is tagged with the lowercase letter r. All backslashes and whitespace characters
    //inside a raw string are included verbatim in the string. No escape sequences are recognized.
    let default_win_install_path = r"C:\Program Files\Gorillas";

    let pattern = Regex::new(r"\d+(\.\d+)*");

    //Byte strings are a slice of u8 values - that is, bytes-rather than Unicode text:
    let method = b"GET";
    assert_eq!(method, &[b'G', b'E', b'T']);

    //Rust strings are sequences of Unicode characters, but they are not stored in memory as arrays
    //of chars. Instead, they are stored using UTF-8, a variable-width encoding. Each ASCII
    //characters in a string is stored in one byte. Other characters take up multiple bytes.
    //A &str (pronounced "stir" or "string slice") is a reference to a run of UTF-8 text owned by
    //someone else: it "borrows" the text.
    //A string literal is &str that refers to preallocated text, typically stored in read-only memory
    //along with the program's machine code.
    //The type &mut str does exist, but it is not very useful, since almost any operation on UTF-8
    //can change its overall byte length, and a slice cannot reallocate its referent.
    //&str is very much like &[T]: a fat pointer to some data. String is analogous to Vec<T>.

    /*
    The format!() macro works just like println!(), except that it returns a new String instead of
    writing text to stdout, and it doesn't automatically add a new line at the end:
     */
    let a = format!("{}o{:02}'{:02}``N", 24, 5, 23);
}

fn type_aliases() -> () {
    //The type keyword can be used like typedef in C++ to declare a new name for an existing type:
    type Bytes = Vec<u8>;

    let decode = |data: &Bytes| {};
}
