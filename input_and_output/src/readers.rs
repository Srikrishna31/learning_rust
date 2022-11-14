use std::error::Error;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::path::PathBuf;

/// Buffered Readers
/// For efficiency, readers and writers can be buffered, which simply means they have a chunk of
/// memory (a buffer) that holds some input or output data in memory. This saves on system calls.
/// The actual default size of a BufReader's buffer is several kilobytes, so a single system read
/// can serve hundreds of .read_line() calls. This matters because system calls are slow.
/// The operating system has a buffer too: system calls are slow, but reading data from a disk is
/// slower.
/// Stdin.lock()
/// Rust standard library protects stdin with a mutex. We call .lock() to lock stdin for the current
/// thread's exclusive use; it returns an StdinLock value that implements BufRead. At the end of the
/// loop, the StdinLock is dropped, releasing the mutex. Without a mutex, two threads trying to read
/// from stdin at the same time would cause undefined behavior. C has the same issue and solves it the
/// same way: all of the C standard input and output functions obtain a lock behind the scenes. The
/// only difference is that in Rust, the lock is part of the API.
fn grep<R>(target: &str, reader: R) -> io::Result<()>
    where R: BufRead
{
    for line_result in reader.lines() {
        let line = line_result?;
        if line.contains(target) {
            println!("{line}");
        }
    }
    Ok(())
}


/// Note that a File is not automatically buffered. File implements Read but not BufRead. However, its
/// easy to create a buffered reader for a File, or any other unbuffered reader. BufReader::new(reader)
/// does this.
/// In most languages, files are buffered by default. If you want unbuffered input or output, you have
/// to figure out how to turn buffering off. In Rust, File and BufReader are two separate library
/// features, because sometimes you want files without buffering, and sometimes you want buffering
/// without files.
pub(crate) fn grep_main() -> Result<(), Box<dyn Error>> {
    //Get the command-line arguments. The first argument is the string to search for; the rest are
    // filenames
    let mut args = std::env::args().skip(1);
    let target = match args.next() {
        Some(s) => s,
        None => Err("usage: grep PATTERN FILE...")?
    };
    let files: Vec<PathBuf> = args.map(PathBuf::from).collect();

    if files.is_empty() {
        let stdin = io::stdin();
        grep(&target, stdin.lock())?;
    } else {
        for file in files {
            let f = File::open(file)?;
            grep(&target, BufReader::new(f))?;
        }
    }

    Ok(())
}
