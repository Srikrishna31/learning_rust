/*
Panic
A program panics when it encounters something so messed up that there must be a bug in the program
itself. Something like:
* Out-of-bounds array access
* Integer division by zero
* Calling .expect() on a Result that happens to be Err
* Assertion Failure
 */

/*
Unwinding
This is the default behavior when a panic happens:
* Any error message is printed to the terminal.
* The stack is unwound. This is a lot like C++ exception handling.
Any temporary values, local variables, or arguments that the current function was using are dropped,
in the reverse of the order they were created. Once the current function call is cleaned up, we move
on to its caller, dropping its variables and arguments the same way. Then we move to that function's
caller and so on up the stack.
* Finally, the thread exits. If the panicking thread was the main thread, then the whole process
exits(with a nonzero exit code).


Panic is per thread. One thread can be panicking while other threads are going on about their
normal business.
There is also a way to catch stack unwinding, allowing the thread to survive and continue running.
The standard library function std::panic::catch_unwind() does this.
 */

/*
Aborting
If a .drop() method triggers a second panic while Rust is still trying to clean up after the first,
this is considered fatal. Rust stops unwinding and aborts the whole process.

Also, Rust's panic behavior is customizable. If you compile with -C panic=abort, the first panic in
your program immediately aborts the process.(With this option, Rust doesn't need to know how to
unwind the stack, so this can reduce the size of your compiled code.)
 */
fn main() {
    println!("Hello, world!");
}

/*
Result

Rust doesn't have exceptions. Instead, functions that can fail have a return type that says so:

fn get_weather(location: LatLng) -> Result<WeatherReport, io::Error>
 */

use std::error::Error;
use std::io::{Write, stderr};

///Dump an error message to 'stderr'.
/// If another error happens while building the error message or writing to 'stderr', it is ignored.
fn print_error(mut err: &dyn Error) {
    let _ = writeln!(stderr(), "error: {err}");
    while let Some(source) = err.source() {
        let _ = writeln!(stderr(), "caused by: {source}");
        err=source;
    }
}


/*
Propagating Errors
Rust has a ? operator that does error propagation:
let weather = get_weather(hometown)?;
The behavior of ? depends on whether this function returns a success result or an error result:
* On success, it unwraps the Result to get the success value inside. The type of weather here is
not Result<WeatherReport, io::Error> but simply WeatherReport.
* On error, it immediately returns from the enclosing function, passing the error result up the call
chain. To ensure that this works, ? can only be used on a Result in functions that have a Result
return type.

The above operator is a short form for:
let weather = match get_weather(hometown) {
    Ok(success_value) => success_value,
    Err(err) => return Err(err)
}

? also works similarly with the Option type. In a function that returns Option, you can use ? to
unwrap a value and return early in case of None.
 */

#[derive(Debug, Clone)]
pub struct JsonError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

use std::fmt;

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {

        write!(f, "{} ({}:{}", self.message, self.line, self.column)
    }
}

//Errors should implement the std::error::Error trait, but the default definitions for the Error
//methods are fine.
impl std::error::Error for JsonError {}

fn json_error() -> Result<(), JsonError> {
    let current_line = 0;
    let current_column = 0;

    Err(JsonError{
        message: "expected ']' at end of array".to_string(),
        line: current_line,
        column: current_column
    })
}
