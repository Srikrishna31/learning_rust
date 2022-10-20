// A trait is a collection of methods that types can implement. Any type that implements the
// FromStr trait has a from_str method that tries to parse a value of they type from a string.
use std::str::FromStr;
//The second use declaration brings in the std::env module, which provides several useful functions
// and types for interacting with the execution environment, including the args function, which gives
// access to the program's command-line arguments.
use std::env;

fn main() {
    //Rust infers the type of Vec to be Vec<u64> since we push a u64, and also pass the vector's
    //elements to gcd function.
    let mut numbers = Vec::new(); //Vec is equivalent to C++'s Vector, Python's list

    // The args function returns an `iterator`, a value that produces each argument on demand, and
    // indicates when we're done.
    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg)
            //from_str returns a Result type which can be Ok(v) or Err(e) type.
                         .expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        //This macro writes out to standard error output stream.
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    //When we iterate we want to tell Rust that `ownership` of the vector should remain with numbers
    //We are merely `borrowing` its elements for the loop. The & operator in &numbers[1..] borrows a
    //reference to the vector's elements from the second onward. The for loop iterates over the
    //referenced elements, letting m borrow each element in succession. The * operator in *m
    //dereferences m, yielding the value it refers to; this is the next u64 we want to pass to gcd.
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {d}", numbers);
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

/* A Rust package, whether a library or an executable, is called a `crate`; Cargo and crates.io both
derive their names from this term.
 */
