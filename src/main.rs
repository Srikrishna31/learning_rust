fn main() {
    println!("Hello, world!");

    println!("Gcd of 5 and 7 is {}", gcd(5, 7));
}

//By default, once a variable is initialized, it's value can't be changed, but placing the mut(short
//for mutable) before the parameters n and m allows our function body to assign to them.
fn gcd(mut n:u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            // Rust only infers types within function bodies: you must write out the types of function
            // parameters and return values as written above.
            let t = m;
            m=n;
            n=t;
        }
        m=m%n;
    }
    // If a function body ends with an expression that is NOT followed by a semicolon, that's the
    // functions return value. In fact, any block surrounded by curly braces can function as an
    // expression.
    // It's typical in Rust to use this form to establish the function's value when control "falls
    // of the end" of the function, and use return statements only for explicit early returns from
    // the midst of a function.
    n
}


// #[test] is an example of an attribute. Attributes are an open-ended system for marking functions
// and other declarations with extra information, like attributes in C++ and C#, or annotations in
// Java. They're used to control compiler warnings and code style checks, include code conditionally,
// tell Rust how to interact with code written in other languages, and so on.
#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2*3*5*11*17, 3*7*11*13*19), 3*11);
}
