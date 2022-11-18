/// You can tell Rust where to search for libraries by writing a build script, Rust code that
/// Cargo compiles and runs at build time. Build scripts can do all sorts of things: generate code
/// dynamically, compile C code to be included in the crate and so on.
fn main() {
    if cfg!(unix) {
        println!(r"cargo:rustc-link-search=native=/home/gl-571/libgit2-1.5.0/build");
    } else if cfg!(windows) {
        println!(r"cargo:rustc-link-search=native=d:\libgit2-1.5.0\build\Debug");
    }
}
