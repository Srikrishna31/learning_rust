mod raw;

use std::{error, fmt, result, ptr};

#[derive(Debug)]
pub struct Error {
    code: i32,
    message: String,
    class: i32,
}

impl fmt::Display for Error {
    fn fmt(&self, f:&mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        // Displaying an `Error` simply displays the message from libgit2.
        self.message.fmt(f)
    }
}

impl error::Error for Error {}

pub type Result<T> = reuslt::Result<T, Error>;

use std::os::raw::c_int;
use std::ffi::CStr;

fn check(code: c_int) -> Result<c_int> {
    if code >= 0 {
        return Ok(code);
    }

    unsafe {
        let error = raw::giterr_last();

        // libgit2 ensures that (*error).message is always non-null and null terminated, so this call is safe.
        let message = CStr::from_ptr((*error).message).to_string_lossy().into_owned();

        Err(Error {
            code: code as i32,
            message,
            class: (*error).klass as i32,
        })
    }
}

// A Git repository
pub struct Repository {
    // This must always be a pointer to a live `git_repository` structure. No other `Repository` may
    // point to it.
    raw: *mut raw::git_repository
}

use std::path::Path;

impl Repository {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Repository> {
        ensure_initialized();

        let path = path_to_cstring(path.as_ref())?;
        let mut repo = ptr::null_mut();
        unsafe {
            check(raw::git_repository_open(&mut repo, path.as_ptr()))?;
        }
        Ok(Repository {raw: repo})
    }

    /// The std::sync::Once type helps run initialization code in a thread-safe way. Only the first
    /// thread to call ONCE.call_once runs the given closure. Any subsequent calls, by this thread or
    /// any other, block until the first has completed and then return immediately, without running
    /// the closure again. Once the closure has finished, calling ONCE.call_once is cheap, requiring
    /// nothing more than an atomic load of a flag stored in ONCE.
    fn ensure_initialized() {
        static ONCE: std::sync::Once = std::sync::Once::new();
        ONCE.call_once(|| {
            unsafe {
                check(raw::git_libgit2_init())
                    .expect("initializing libgit2 failed");

                assert_eq!(libc::atexit(shutdown), 0);
            }
        });
    }

    extern fn shutdown() {
        unsafe {
            if let Err(e) = check(raw::git_libgit2_shutdown()) {
                eprintln!("shutting down libgit2 failed: {e}");
                std::process::abort();
            }
        }
    }
}


impl Drop for Repository {
    fn drop (&mut self) {
        unsafe {
            raw::git_repository_free(self.raw);
        }
    }
}
