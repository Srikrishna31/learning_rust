mod readers;
mod writers;
mod network;

/// Rust's standard library features for input and output are organized around three traits: Read,
/// BufRead and Write:
/// * Values that implement Read have methods for byte-oriented input. They're called readers.
/// * Values that implement BufRead are buffered readers. They support all the methods of Read, plus
/// methods for reading lines of text and so forth.
/// * Values that implement Write support both byte-oriented and UTF-8 text output. They're called writers.
fn main() {
    println!("Hello, world!");

    writers::write_json();

    writers::paths();

    // network::echo_main("127.0.0.1:17007").expect("error: ");

    network::http_get_main("https://doc.rust-lang.org/book/")?;

    let result = readers::grep_main();
    if let Err(err) = result {
        eprintln!("{err}");
        std::process::exit(1);
    }
}


/// The self keyword here declares io as an alias to the std::io module. That way, std::io::Result
/// and std::io::Error can be written more concisely as io::Result and io::Error, and so on.
use std::io::{self, Read, Write, ErrorKind};

const DEFAULT_BUF_SIZE : usize = 8*1024;

/// Since there are standard traits for readers and writers (std::io::Read and std::io::Write), it's
/// quite common to write generic code that works across a variety of input or output channels.
/// This is the implementation of std::io::copy() from Rust's standard library. Since it's generic,
/// you can use it to copy data from a File to a TcpStream, from Stdin to an in-memory Vec<u8> etc.
fn copy_example<R: ?Sized, W: ?Sized>(reader: &mut R, writer: &mut W) -> io::Result<u64>
    where R: Read, W: Write
{
    let mut buf = [0; DEFAULT_BUF_SIZE];
    let mut written = 0;
    loop {
        let len = match reader.read(&mut buf) {
            Ok(0) => return Ok(written),
            Ok(len) => len,
            Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };
        writer.write_all(&buf[..len])?;
        written += len as u64;
    }
}
