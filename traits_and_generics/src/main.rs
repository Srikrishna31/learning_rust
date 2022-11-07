mod traits;

use std::io::Write;
use crate::traits::*;

/// The phrase <W:Write> means that throughout the body of this function, W stands for some type that
/// implements the Write trait. Type parameters are usually single uppercase letters, by convention.
/// Rust generates machine code for each different type parameter of the function call. This process
/// is known as monomorphization, and the compiler handles it all automatically.
fn say_hello<W:Write>(out: &mut W) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}

fn main() -> std::io::Result<()>{

    use std::fs::File;
    let mut local_file = File::create("hello.txt")?;
    say_hello(&mut local_file)?;

    let mut bytes = vec![];
    say_hello(&mut bytes)?;
    assert_eq!(bytes, b"hello world\n");

    let mut buf: Vec<u8> = vec![];
    /// A reference to a trait type, like Writer, is called a trait object. Like any other reference,
    /// a trait object points to some value, it has a lifetime, and it can be either mut or shared.
    /// What makes a trait object different is that Rust usually doesn't know the type of the referent
    /// at compile time. So a trait object includes a little extra information about the referent's
    /// type. This is strictly for Rust's own use behind the scenes: when you call writer.write(data),
    /// Rust needs the type information to dynamically call the right write method depending on the
    /// type of *writer. You can't query the type information directly, and Rust doesnot support
    /// downcasting from the trait object &mut dyn Write back to a concrete type like Vec<u8>.
    /// In memory, a trait object is a fat pointer consisting of a pointer to the value, plus a pointer
    /// to a table representing that value's type. Each trait object therefore takes up two machine
    /// words.
    /// In Rust, as in C++, the vtable is generated once, at compile time, and shared by all objects
    /// of the same type.
    /// In C++, the vtable pointer, or vptr, is stored as part of the struct. Rust uses fat pointers
    /// instead. The struct itself contains nothing but its fields. This way, a struct can implement
    /// dozens of traits without containing dozens of vptrs. Even types like i32, which aren't big
    /// enough to accommodate a vptr, can implement traits.
    /// Rust automatically converts ordinary references into trait objects when needed. At the point
    /// where the conversion happens, Rust knows the referent's true type, so it just adds the address
    /// of the appropriate vtable, turning the regular pointer into a fat pointer.
    let writer: &mut dyn Write = &mut buf;

    dot_product::<3>([0.2, 0.4, 0.6], [0., 0., 1.]);
    dot_product([3.,4.], [-6., 1.]);

    assert_eq!('$'.is_emoji(), false);
    Ok(())
}

fn dot_product<const N: usize>(a: [f64; N], b: [f64; N]) -> f64 {
    let mut sum = 0.;
    for i in 0..N {
        sum += a[i] * b[i];
    }
    sum
}
