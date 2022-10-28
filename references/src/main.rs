
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

    //assert!(rx == rrx); //error: type mismatch: `&i32` vs `&&i32`
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
    (1..n+1).product()
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

/*
Sharing vs Mutation
 */
fn extend (vec: &mut Vec<f64>, slice: &[f64]) {
    for elt in slice {
        vec.push(*elt);
    }
}

fn test_extend() {
    let mut wave = Vec::<f64>::new();
    let head = vec![0.0, 1.0];
    let tail = [0.0, -1.0];

    //extend(&mut wave, &wave); //error: cannot borrow 'wave' as immutable because it is also
    //borrowed as mutable.
    assert_eq!(wave, vec![0.0, 1.0, 0.0, -1.0,
                          0.0, 1.0, 0.0, -1.0]);
}

/*
* Shared access is read-only access.
Values borrowed by shared references are read-only. Across the lifetime of a shared reference,
neither its referent, nor anything reachable from that referent can be changed by anything. There
exist no live mutable references to anything in that structure, its owner is held read-only, and
so on.

* Mutable access is exclusive access.
A value borrowed by a mutable reference is reachable exclusively via that reference. Across the
lifetime of a mutable reference, there is no other usable path to its referent or to any value
reachable from there. The only references whose lifetimes may overlap with a mutable reference are
those you borrow from the mutable reference itself.
 */

fn sharing_vs_mutation() -> () {
    {
        let mut x = 10;
        let r1 = &x;
        let r2 = &x; //ok: multiple shared borrows permitted
        x+=10; //error: cannot assign to 'x', because it is borrowed.
        let m = &mut x; //error: cannot borrow 'x' as mutable because it is also borrowed as immutable

        println!("{r1}, {r2}, {m}"); //the references are used here, so their lifetimes must last at least this long.
    }

    {
        let mut y = 20;
        let m1 = &mut y;
        let m2 = &mut y; //error: cannot borrow as mutable more than once.
        let z = y; //error: cannot use 'y' because it was mutably borrowed.

        println!("{m1}, {m2}, {z}"); //references are used here.
    }

    //It is okay to reborrow a shared reference from a shared reference.
    {
        let mut w = (107, 109);
        let r = &w;
        let r0 = &r.0; //ok: reborrowing shared as shared
        let m1 = &mut r.1;  //error: can't reborrow shared as mutable.
        println!("{r0}"); //r0 gets used here.
    }

    //You can reborrow from a mutable reference:
    {
        let mut v = (136, 139);
        let m = &mut v;
        let m0 = &mut m.0;  //ok: reborrowing mutable from mutable
        *m0 = 137;
        let r1 = &m.1;      //ok: reborrowing shared from mutable, and doesn't overlap with m0
        v.1;    //error: access through other paths still forbidden
        println!("{r1}");
    }

    {
        let mut x = 42;
        let p = &x;
        assert_eq!(*p, 42);
        x += 1; //error: cannot assign to x because it is borrowed
        assert_eq!(*p, 42);     //if you take out the assignment, this is true.
    }
}

/*
The immiscibility of shared and mutable references really demonstrates its value when writing
concurrent code. A data race is possible only when some value is both mutable and shared between
threads - which is exactly what Rust's reference rules eliminate. A concurrent Rust program that
avoids unsafe code is free of data races by construction.
 */
