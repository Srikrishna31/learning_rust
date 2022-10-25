
use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

/*
A reference lets you access a value without affecting its ownership. References come in two kinds:

* A 'shared reference' lets you read but not modify its referent. However, you can have as many
shared references to a particular value at a time as you like. The expression &e yields a shared
reference to e's value; if e has the type T, then &e has the type &T, pronounced "ref T". Shared
references are Copy.

* If you have a 'mutable reference' to a value, you may both read and modify the value. However, you
may not have any other references of any sort to that value active at the same time. The expression
&mut e yields a mutable reference to e's value; you write its type as &mut T, which is pronounced
"ref mute T". Mutable references are not Copy.

You can think of the distinction between shared and mutable references as a way to enforce a multiple
readers or single writer rule at compile time. In fact, this rule doesn't apply only to references;
it covers the borrowed value's owner as well. As long as there are shared references to a value, not
even its owner can modify it; the value is locked down. Keeping sharing and mutable fully separate
turns out to be essential to memory safety.

When we pass a value to a function in a way that moves ownership of the value to the function, we say
that we have passed it by value. If we instead pass the function a reference to the value, we say that
we have passed the value by reference.
 */
fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {artist}");
        for work in works{
            println!(" {work}");
        }
    }
}

fn references() -> () {
    /*
    In C++, references are created implicitly by conversion, and dereferenced implicitly too.
    In Rust, references are created explicitly with the & operator, and dereferenced explicitly with
    the * operator:
     */
    let x = 10;
    let r = &x;
    assert!(*r == 10);

    let mut y = 32;
    let m = &mut y;
    *m += 32;
    assert!(*m ==64);

    let z = 50;
    //let n = &mut z; //Rust doesn't allow to create a mutable reference to an unmutable value.

    /*
    The . operator can also implicitly borrow a reference to its left operand, if needed for a method
    call.
     */
    let mut v = vec![1973, 1968];
    v.sort(); //implicitly borrows a mutable reference to v
    (&mut v).sort(); //equivalent, but more verbose.

    /*
    In a nutshell, whereas C++ converts implicitly between references and lvalues(that is, expressions
    referring to locations in memory), with these conversions appearing anywhere they're needed, in
    Rust you use the & and * operators to create and follow references, with the exception of the
    . operator, which borrows and dereferences implicitly.
     */
}

fn references_to_references() -> () {
    //Rust permits references to references:
    struct Point {x: i32, y:i32}
    let point = Point{x:1000, y:729};

    let r = &point;
    let rr = &r;
    let rrr = &rr;

    // The . operator follows as many references as it takes to find its target:
    assert_eq!(rrr.y, 729);

    //Like the . operator, Rust's comparision operators "see through" any number of references:
    let x = 10;
    let y = 10;

    let rx = &x;
    let ry = &y;

    let rrx = &rx;
    let rry = &ry;

    assert!(rrx <= rry);
    assert!(rrx == rry);

    assert!(!std::ptr::eq(rx, ry)); //references to rx and ry are equal, but occupy different addresses.

    assert!(rx == rrx); //error: type mismatch: `&i32` vs `&&i32`
    assert!(rx == *rrx); //this is okay


    /*
    References are never null
    In Rust, if you need a value that is either a reference to something or not, use the type Option<&T>.
    At the machine level, Rust represents None as a null pointer, and Some(r), where r is a &T value,
    as the nonzero address, so Option<&T> is just as efficient as a nullable pointer in C or C++,
    even though it's safer: its type requires you to check whether it's None before you can use it.
     */
}
fn main() {
    let mut table = Table::new();
    table.insert("Gesualdo".to_string(),
                    vec!["many madrigals".to_string(),
                    "Tenerbrae Responsoria".to_string()]);
    table.insert("Caravaggio".to_string(),
                 vec!["The Musicians".to_string(),
                         "The Calling of St. Matthew".to_string()]);
    table.insert("Cellini".to_string(),
                    vec!["Perseus with the head of Medusa".to_string(),
                            "a salt cellar".to_string()]);

    show(&table);
}


/*
Borrowing References to Arbitrary Expressions
Rust lets you borrow a reference to the value of any sort of expression at all:
 */
fn factorial(n:usize) -> usize {
    (1..n+1).product();
}

fn expression_references() -> () {
    let r = &factorial(6);
    assert_eq!(r + &1009 , 1729);
}
/*
In situations like this Rust simply creates an anonymous variable to hold the expression's value
and make the reference point to that. The lifetime of this anonmous variable depends on following:

* If you immediately assign the reference to a variable in a let statement(or make it part of some
struct or array that is being immediately assigned), then Rust makes the anonymous variable live as
long as the variable the let initializes.

* Otherwise, the anonymous variable lives to the end of the enclosing statement.
 */


/*
Reference Safety
Rust tries to assign each reference type in your program a 'lifetime' that meets the constraints
imposed by how it is used. A lifetime is some stretch of your program for which a reference could be
safe to use: a statement, an expression, the scope of some variable, or the like. Lifetimes are
entirely figments of Rust's compile-time imagination. At run time, a reference is nothing but an
address; its lifetime is part of its type and has no run-time representation.
First rule: The variable's lifetime must contain or enclose that of the reference borrowed from it.
Second rule: if you store a reference in a variable r, the reference's type must be good for the
entire lifetime of that variable, from its initialization until its last use.
 */

/*
static: Rust's equivalent of a global variable is called a static: it's a value that's created when
the program starts and lasts until it terminates.
 */

/*
Lifetime parameters for functions
For functions accepting references as parameters, an additional lifetime parameter may be specified,
which lets Rust to reason about reference safety and point out errors:
fn f<'a>(p: &'a i32) { ... }
When we write this, we're defining a function that takes a reference to an i32 with any given lifetime
'a.
Rust also has the notion of 'static lifetime, which encapuslates the lifetime of a static variable
reference, which can only be used for storing references to static variables.

In addition, you only need to worry about lifetime parameters when defining functions and types;
when using them, Rust infers them for you.

Lifetimes in function signatures let Rust assess the relationships between the references you pass to
function and those the function returns, and they ensure they're being used safely.
 */

/*
Structs containing references

Whenever a reference type appears inside another type's definition, you must write out its lifetime.
Eg:
struct S {
    r: &'a i32
}

Now the S type has a lifetime, just as reference types do. Each value you create of type S gets a
fresh lifetime 'a, which becomes constrained by how you use the value. The lifetime of any reference
you store in r had better enclose 'a, and 'a must outlast the lifetime of wherever you store the S.

A type's lifetime parameters always reveal whether it contains references with interesting (that is,
non-'static) lifetimes and what those lifetimes are.
For examplef, suppose we have a parsing function that takes a slice of bytes and returns a structure
holding the results of the parse:
fn parse_record<'i>(input: &'i [u8]) -> Record<'i> { ... }
Without looking into the definition of Record type at all, we can tell that, if we receive a Record
from parse_record, whatever references it contains must point into the input buffer we passed in, and
nowhere else(except perhaps 'static values).

It's not just references and types like S that have lifetimes. Every type in Rust has a lifetime,
including i32 and String. Most are simply 'static, meaning that values of those types can live for
as long as you like; for example a Vec<i32> is self-contained and needn't be dropped before any
particular variable goes out of scope. But a type like Vec<&'a i32> has a lifetime that must be
enclosed by 'a: it must be dropped while its referents are still alive.
 */
