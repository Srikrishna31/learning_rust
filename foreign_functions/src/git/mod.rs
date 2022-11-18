mod raw;

use std::{error, fmt, result, ptr, mem};

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

pub type Result<T> = result::Result<T, Error>;

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
        Self::ensure_initialized();

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

                assert_eq!(libc::atexit(Self::shutdown), 0);
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

use std::ffi::CString;

#[cfg(unix)]
fn path_to_cstring(path: &Path) -> Result<CString> {
    // The `as_bytes` method exists only on Unix-like systems.
    use std::os::unix::ffi::OsStrExt;

    Ok(CString::new(path.as_os_str().as_bytes())?)
}

#[cfg(windows)]
fn path_to_cstring(path: &Path) -> Result<CString> {
    // Try to convert to UTF-8. If this fails, libgit2 can't handle the path anyway
    match path.to_str() {
        Some(s) => Ok(CString::new(s)?),
        None => {
            let message = format!("Couldn't convert path '{}' to UTF-8", path.display());

            Err(message.into())
        }
    }
}


impl From<String> for Error {
    fn from(message: String) -> Error {
        Error {code: -1, message, class:0}
    }
}

// NulError is what `CString::new` returns if a string has embedded zero bytes.
impl From<std::ffi::NulError> for Error {
    fn from(e:std::ffi::NulError) -> Error {
        Error {code: -1, message: e.to_string(), class: 0}
    }
}


/// The identifier of some sort of object stored in the Git object database: a commit, tree, blob,
/// tag, etc. This is a wide hash of the object's contents.
pub struct Oid {
    pub raw: raw::git_oid
}

use std::os::raw::c_char;

impl Repository {
    pub fn reference_name_to_id(&self, name: &str) -> Result<Oid> {
        let name = CString::new(name)?;
        unsafe {
            let oid = {
                let mut oid = mem::MaybeUninit::uninit();
                check(raw::git_reference_name_to_id(oid.as_mut_ptr(), self.raw,
                                name.as_ptr() as *const c_char))?;
                oid.assume_init()
            };
            Ok(Oid {raw: oid})
        }
    }
}

use std::marker::PhantomData;

/// A git_commit object must never outlive the git_repository object it was retrieved from. Rust's
/// lifetimes let the code capture this rule precisely.
pub struct Commit<'repo> {
    // This must always be a pointer to a usable `git_commit` structure.
    raw: *mut raw::git_commit,
    /// The type PhantomData<&'repo Repository> indicates that Rust should treat Commit<'repo> as if
    /// it held a reference with lifetime 'repo to some Repository.
    _marker: PhantomData<&'repo Repository>
}

impl Repository {
    pub fn find_commit(&self, oid: &Oid) -> Result<Commit> {
        let mut commit = ptr::null_mut();
        unsafe {
            check(raw::git_commit_lookup(&mut commit, self.raw, &oid.raw))?;
        }
        Ok(Commit {raw: commit, _marker: PhantomData})
    }
}

/// When a commit is dropped it must free its raw::git_commit:
impl <'repo> Drop for Commit<'repo> {
    fn drop(&mut self) {
        unsafe {
            raw::git_commit_free(self.raw);
        }
    }
}

/// From a Commit, you can borrow a Signature (a name and email address) and the text of the commit
/// message:
impl<'repo> Commit<'repo> {
    pub fn author(&self) -> Signature {
        unsafe {
            Signature {
                raw: raw::git_commit_author(self.raw),
                _marker: PhantomData
            }
        }
    }

    pub fn message(&self) -> Option<&str> {
        unsafe {
            let message = raw::git_commit_message(self.raw);
            char_ptr_to_str(self, message)
        }
    }
}

/// A git_signature object always borrows its text from elsewhere; in particular, signatures returned
/// by git_commit_author borrow their text from the git_commit. So our safe Signature type includes
/// a PhantomData<&'text str> to tell Rust to behave as if it contained a &str with a lifetime of 'text.
/// Just as before, Commit::author properly connects this 'text lifetime of the Signature it returns
/// to that of the Commit without us needing to write a thing. The Commit::message method does the
/// same with the Option<&str> holding the commit message.
pub struct Signature<'text> {
    raw: *const raw::git_signature,
    _marker: PhantomData<&'text str>
}

impl <'text> Signature<'text> {
    /// Return the author's name as a `&str`, or `None` if it is not well-formed UTF-8.
    pub fn name(&self) -> Option<&str> {
        unsafe {
            char_ptr_to_str(self, (*self.raw).name)
        }
    }

    /// Return the author's email as a `&str`, or `None` if it is not well-formed UTF-8.
    pub fn email(&self) -> Option<&str> {
        unsafe {
            char_ptr_to_str(self, (*self.raw).email)
        }
    }
}

/// Try to borrow a `&str` from `ptr`, given that `ptr` may be null or refer to ill-formed UTF-8.
/// Give the result a lifetime as if it were borrowed from `_owner`.
///
/// Safety: if `ptr` is non-null, it must point to a null-terminated C string that is safe to access
/// for atleast as long as the lifetime of `_owner`.
///
/// The CStr::from_ptr function returns a &CStr whose lifetime is completely unbounded, since it was
/// borrowed from a dereferenced raw pointer. Unbounded lifetimes are almost always inaccurate, so
/// it's good to constrain them as soon as possible. Including the _owner parameter causes Rust to
/// attribute its lifetime to the return value's type, so callers can receive a more accurately
/// bounded reference.
unsafe fn char_ptr_to_str<'o, T: 'o>(_owner: &'o T, ptr: *const c_char) -> Option<&'o str> {
    if ptr.is_null() {
        return None;
    } else {
        CStr::from_ptr(ptr).to_str().ok()
    }
}
