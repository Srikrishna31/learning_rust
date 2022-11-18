mod libgit2;
mod git;

use std::ffi::{CString, CStr};
use std::os::raw::c_char;

extern {
    /// You can also declare global variables in extern blocks. POSIX systems have a global variable
    /// named environ that holds the values of the process's environment variables. In Rust, you
    /// declare as the following:
    static environ: *mut *mut c_char;
}

fn main() {
    println!("Hello, world!");

    let rust_str = "I'll be back";
    let null_terminated = CString::new(rust_str).unwrap();

    unsafe {
        assert_eq!(libgit2::strlen(null_terminated.as_ptr()), 12);

        if !environ.is_null() && !(*environ).is_null()
        {
            let var = CStr::from_ptr(*environ);
            println!("first environment variable : {}", var.to_string_lossy())
        }

        libgit2::git_libgit2_init();
        libgit2::git_libgit2_shutdown();
    }

    let path = std::env::args().skip(1).next().expect("usage: git-toy PATH");

    let repo = git::Repository::open(&path).expect("opening repository");

    let commit_oid = repo.reference_name_to_id("HEAD").expect("looking up 'HEAD' reference");

    let commit = repo.find_commit(&commit_oid).expect("looking up commit");

    let author = commit.author();

    println!("{} <{}>\n", author.name().unwrap_or("(none)"), author.email().unwrap_or("none"));

    println!("{}", commit.message().unwrap_or("(none)"));
}
