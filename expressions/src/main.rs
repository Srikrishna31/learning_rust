/*
Rust : An expression language
In C/C++, there is a sharp distinction between 'expressions' and 'statements': Expressions have values,
Statements don't.

Rust is what is called an expression language. This means it follows an older tradition, dating back
to Lisp, where expressions do all the work. In Rust, if and match can produce values.
 */
/*
An if expression can be used to initialize a variable:
A match expression can be passed as an argument to a function or macro

Blocks are the most general kind of expression. A block produces a value and can be used anywhere a
value is needed. The value of the block is the value of its last expression.
Most lines of Rust code do end with either a semicolon or curly braces, just like C or Java. And if
a block looks like C code, with semicolons in all the familiar places, then it will run just like a
C block, and its value will be ().
 */

use std::cmp::Ordering;
use std::io;

fn main() {

    let msg = {
        //let-declaration: semicolon is always required
        let dandelion_control = "abc";

        //expression + semicolon: method is called, return value dropped
        dandelion_control.to_string();

        //expression with no semicolon: method is called, return value stored in 'msg'
        dandelion_control.as_bytes()
    };

    //General structure of let expressions:
    // let name: type = expr;

    let name;
    if "def".contains("d") {
        name = "abc";
    } else {
        name = "xyz";
    }

    use std::fs::File;
    use std::io::{BufReader};
    use std::io::prelude::*;

    let path = "abc.txt";
    let mut file = File::open(path)?;
    for line in BufReader::new(file).lines() {
        let line = line?;
        /*
        The inner let declaration creates a new, second variable of a different type. The type of the
        first variable is Result<String, io::Error>. The second line is a String. Its definition
        supersedes the first's for the rest of the block. This is called shadowing and is very
        common in rust programs.
         */
    }
    println!("Hello, world!");
}

/*
Items within a block:
A block can also contain item declarations. An item is simply any declaration that could appear
globally in a program or module, such as a fn, struct, or use.
 */

struct FileInfo {
    timestamp: i32,
    path:String
}

fn show_files() -> io::Result<()> {

    let mut v = vec![];

    /*
    When fn is declared inside a block, its scope is the entire block - that is, it can be used
    throughout the enclosing block. But a nested function cannot access local variables or arguments
    that happen to be in the enclosing block.
     */
    fn cmp_by_timestamp_then_name<'r, 's>(a: &'r FileInfo, b: &'s FileInfo) -> Ordering {
        a.timestamp.cmp(&b.timestamp)
            .reverse()
            .then(a.path.cmp(&b.path))
    }

    v.sort_by(cmp_by_timestamp_then_name);

    Ok(())
}

/*
The general form of a match expression is:
match value {
    pattern => expr,
}

The comma after an arm may be dropped if the expr is a block.

Rust checks the given value against each pattern in turn, starting with the first. When a pattern
matches, the corresponding expr is evaluated, and the match expression is complete; no further
patterns are checked. At least one of the patterns must match. Rust prohibits match expressions
that do not cover all possible values. All arms of a match expression must have the same type.
 */

/*
There are four looping exprssions:
while condition {
    block
}

while let pattern = expr {
    block
}

loop {
    block
}

for pattern in iterable {
    blocks
}

Loops are expressions in Rust, but the value of a while or for loop is always (), so their value
isn't very useful. A loop expression can produce a value if you specify one.

* A while loop behaves exactly like the C equivalent.
* The while let loop is analogous to if let. At the beginning of each loop iteration, the value of
expr either matches the given pattern, in which case the block runs, or doesn't, in which case the
loop exits.
* Use loop to write infinite loops. It executes the block repeatedly forever (or until a break or
return is reached or the thread panics).
* A for loop evaluates the iterable expression and then evaluates the block once for each value in
the resulting iterator.

 */
fn loop_fun() -> () {
    //Each call to 'next_line()' returns either 'Some(line)', where line is a line of input, or
    //'None', if we've reached the end of the input. Return the first line that starts with "answer: ",
    // Otherwise, return "answer: nothing"
    let answer = loop {
        if let Some(line) = next_line() {
            if line.starts_with("answer: ") {
                break line;
            } else {
                break "answer: nothing"
            }
        }
    };
}

fn break_fun() -> () {
    //A break can have both a label and a value expression:
    //Find the root of the first perfect square in the series
    let sqrt = 'outer: loop {
        let n = 100;
        for i in 1.. {
            let square = i*i;
            if square == n {
                break 'outer i;
            }
            if square > n {
                break;
            }
        }
    };

    println!("{sqrt}");
}

/*
Expressions that don't finish normally are assigned the special type !, and they're exempt from the
rules about types having to match. You can see ! in the function signature of std::process::exit();
fn exit(code: i32) -> !

The ! means that exit() never returns. It's a divergent function.
Of course, Rust then considers it an error if the function can return normally.
 */

/*
Automatic type conversions:
A few conversions involving reference types are so straightforward that the language performs them
even without a cast. One trivial example is a mut reference to a non-mut reference.
Several more significant automatic conversions can happen, though:
* Values of type &String auto-convert to type &str without a cast.\
* Values of type &Vec<i32> auto-convert to &[i32].
* Values of type &Box<Chessboard> auto-convert to &Chessboard.

These are called 'deref coercions', because they apply to types that implement the Deref built-in trait.
The purpose of Deref coercion is to make smart pointer types, like Box, behave as much like the
underlying value as possible. Using a Box<Chessboard> is mostly just like using a plain Chessboard,
thanks to Deref.

User-defined types can implement the Deref trait too.
 */
