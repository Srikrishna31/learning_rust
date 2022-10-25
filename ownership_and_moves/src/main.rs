/*
In Rust, every value has a single owner that determines its lifetime. When the owner is freed-dropped,
in Rust terminology - the owned value is dropped too.
 */

fn print_padovan() -> () {
    //A variable ownes its value. When control leaves the block in which the variable is declared,
    //the variable is dropped, so its value is dropped along with it.
    let mut padovan = vec![1,1,1];  //allocated here
    for i in 3..10 {
        padovan.push(padovan[i - 3] + padovan[i - 2]);
    }

    println!("P(1..10)={:?}", padovan);
} //dropped here.

// Just as variables own their values, structs own their fields, and tuples, arrays, and vectors own
//their elements.
/*
It follows that the owners and their owned values form trees: your owner is your parent, and the values
you own are your children. And at the ultimate root of each tree is a variable; when that variable
goes out of scope, the entire tree goes with it.
Every value in a Rust program is a member of some tree, rooted in some variable.
 */

/*
Rust programs don't usually explicitly drop values at all. The way to drop a value in Rust is to
remove it from the ownership tree somehow: by leaving the scope of a variable, or deleting an element
from a vector, or something of that sort. At that point, Rust ensures the value is properly dropped,
along with everything it owns.

The ownership can be extended in the following ways in Rust:
* You can move values from one owner to another. This allows you to build, rearrang, and tear down
the tree.
* Very simple types like integers, floating-point numbers, and characters are excused from the
ownership rules. These are called Copy types.
* The standard library provides the reference-counted pointer types Rc and Arc, which allow values
to have multiple owners, under some restrictions.
* You can "borrow a reference" to a value; references are non-owning pointers, with limited lifetimes.
 */


/**
Moves
In Rust, for most types, operations like assigning a value to a variable, passing it to a function,
or returning it from a function don't copy the value: they move it. The source relinquishes ownership
 of the value to the destination and becomes uninitialized; the destination now controls the value's
 lifetime. Rust programs build up and tear down complex structures one value at a time, one move at
 a time.
*/

fn moves() -> () {
    //Like C and C++, Rust puts plain string literals like "udon" in read-only memory, so for a
    //heap allocation, the .to_string() method is called.
    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    //In Rust, assignments of most types move the value from the source to the destination, leaving
    //the source uninitialized.
    let t = s;
    let u = s; //This would assign the uninitialized value s to u. Rust prudently prohibits
    //using uninitialized values, so the compiler rejects this code.
}

fn more_moves() -> () {
    let mut s = "Govinda".to_string();
    s = "Siddhartha".to_string(); //value "Govinda" dropped here.

    let mut j = "Govinda".to_string();
    let k = s;
    s = "Siddhartha".to_string(); // nothing is dropped here.

    /*
    Rust applies move semantics to almost any use of a value.
    * Passing arguments to functions moves ownership to the function's parameters,
    * Returning a value from a function moves ownership to the caller.
    * Building a tuple moves the values into the tuple and so on.

    Moves always apply to the value proper, not the heap storage they own. For vectors and strings,
    the value proper is the three-word header alone; the potentially large element arrays and text
    buffers sit where they are in the heap.
     */
}

fn move_indexed_content() -> () {
    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }

    //Pull out random elements from the vector:
    let third = v[2]; //error: cannot move out of index of Vec
    let fifth = v[4]; //here too.

    //1. Pop a value off the end of the vector:
    let fifth = v.pop().expect("vector empty!");
    assert_eq!(fifth, "105");

    //2. Move a value out of a given index in the vector, and move the last element into its spot
    let second = v.swap_remove(1);
    assert_eq!(second, "102");

    //3. Swap in another value for the one we're taking out:
    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    assert_eq!(third, "103");

}

/*
Copy types:
Assigning a value of a Copy type copies the value, rather than moving it. The source of the assignment
remains initialized and usable, with the same value it had before. Passing Copy types to functions
and constructors behaves similarly.
The standard Copy types include all the machine integer and floating-point numeric types, the char
and bool types, and a few others. A tuple or fixed-size array of Copy types is itself a Copy type.
As a rule of thumb, any type that needs to do something special when a value is dropped cannot be
Copy. By default, struct and enum types are not Copy.
 */

/*
Rc and Arc: Shared Ownership
Rc - Reference Counted type.
The Rc and Arc types are very similar; the only difference between them is that an Arc is safe to
share between threads directly - the name Arc is short for atomic reference count-whereas a plain
Rc uses faster non-thread-safe code to update its reference count. If you don't need to share the
pointers between threads, there's no reason to pay the performance penalty of an Arc, so you should
use Rc; Rust will prevent you from accidentally passing one across a thread boundary. The two types
are otherwise equivalent.
 */

fn rc() -> () {
    use std::rc::Rc;

    let s = Rc::new("shirataki".to_string());
    //Cloning an Rc<T> value does not copy the T; instead, it simply creates another pointer to it
    //and increments the reference count.
    let t = s.clone();
    let u = s.clone();

    /*
    Rust's memory and thread-safety guarantees depend on ensuring that no value is ever simultaneously
    shared and mutable. Rust assumes the referent of an Rc pointer might in general be shared, so it
    must not be mutable.
     */
}
fn main() {

    print_padovan();
    println!("Hello, world!");
}
