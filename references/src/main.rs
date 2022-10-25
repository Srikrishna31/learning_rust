
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
