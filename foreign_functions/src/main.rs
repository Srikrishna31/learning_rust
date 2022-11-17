mod libgit2;
mod raw;

use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use std::{mem, ptr};

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
    let path = CString::new(path).expect("path contains null characters");

    unsafe {
        raw::check("Initializing library", raw::git_libgit2_init());

        let mut repo = ptr::null_mut();
        raw::check("opening repository", raw::git_repository_open(&mut repo, path.as_ptr()));

        let c_name = b"HEAD\0".as_ptr() as *const c_char;
        let oid = {
            let mut oid = mem::MaybeUninit::uninit();
            raw::check("looking up HEAD", raw::git_reference_name_to_id(oid.as_mut_ptr(), repo, c_name));
            oid.assume_init()
        };

        let mut commit = ptr::null_mut();
        raw::check("looking up commit", raw::git_commit_lookup(&mut commit, repo, &oid));

        raw::show_commit(commit);

        raw::git_commit_free(commit);
        raw::git_repository_free(repo);

        raw::check("shutting down library", raw::git_libgit2_shutdown());
    }
}
