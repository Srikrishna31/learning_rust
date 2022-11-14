use std::net::TcpListener;
use std::io;
use std::thread::spawn;
use std::error::Error;
use reqwest;

/// A simple echo server
/// Accept connections forever, spawning a thread for each one.
pub(crate) fn echo_main(addr: &str) -> io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    println!("listening on {addr}");

    loop {
        let (mut stream, addr) = listener.accept()?;
        println!("Connection received from {addr}");

        let mut write_stream = stream.try_clone()?;
        spawn(move || {
            io::copy(&mut stream, &mut write_stream).expect("error in client thread: ");
            println!("connection closed");
        });
    }
}

pub(crate) fn http_get_main(url: &str) -> Result<(), Box<dyn Error>> {
    let mut response = reqwest::blocking::get(url)?;
    if !response.status().is_success() {
        Err(format!("{}", response.status()))?;
    }

    let stdout = io::stdout();
    io::copy(&mut response, &mut stdout.lock())?;

    Ok(())
}
