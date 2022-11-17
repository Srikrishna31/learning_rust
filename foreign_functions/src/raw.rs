#![allow(non_camel_case_types)]

use std::ffi::CStr;
use std::os::raw::{c_int, c_char, c_uchar};

#[link(name="git2")]
extern {
    pub fn git_libgit2_init() -> c_int;
    pub fn git_libgit2_shutdown() -> c_int;
    pub fn giterr_last() -> *const git_error;

    pub fn git_repository_open(out: *mut *mut git_repository,
                                path: *const c_char) -> c_int;
    pub fn git_repository_free(repo: *mut git_repository);

    pub fn git_reference_name_to_id(out: *mut git_oid,
                                    repo: *mut git_repository,
                                    refrence: *const c_char) -> c_int;

    pub fn git_commit_lookup(out: *mut *mut git_commit,
                            repo: *mut git_repository,
                            id: *const git_oid) -> c_int;

    pub fn git_commit_author(commit: *const git_commit) -> *const git_signature;
    pub fn git_commit_message(commit: *const git_commit) -> *const c_char;
    pub fn git_commit_free(commit: *mut git_commit);
}

/// This is a struct type containing an array with no elements. Since the _private field isn't pub,
/// values of this type cannot be constructed outside this module, which is perfect as the reflection
/// of a C type that only libgit2 should ever construct, and which is manipulated solely through
/// raw pointers.
#[repr(C)] pub struct git_repository { _private: [u8; 0]}
#[repr(C)] pub struct git_commit { _private: [u8; 0]}
#[repr(C)]
pub struct git_error {
    pub message: *const c_char,
    pub klass: c_int
}

pub const GIT_OID_RAWSZ: usize = 20;

#[repr(C)]
pub struct git_oid {
    pub id: [c_uchar; GIT_OID_RAWSZ]
}

pub type git_time_t = i64;

#[repr(C)]
pub struct git_time {
    pub time: git_time_t,
    pub offset: c_int
}

#[repr(C)]
pub struct git_signature {
    pub name: *const c_char,
    pub email: *const c_char,
    pub when: git_time
}


pub fn check(activity: &'static str, status: c_int) -> c_int {
    if status < 0 {
        unsafe {
            let error = &*giterr_last();
            println!("error while {activity}: {} ({})",
                CStr::from_ptr(error.message).to_string_lossy(),
                error.klass);

            std::process::exit(1);
        }
    }
    status
}

pub unsafe fn show_commit(commit: *const git_commit) {
    let author = git_commit_author(commit);

    let name = CStr::from_ptr((*author).name).to_string_lossy();
    let email = CStr::from_ptr((*author).email).to_string_lossy();

    println!("{name} <{email}>\n");

    let message = git_commit_message(commit);

    println!("{}", CStr::from_ptr(message).to_string_lossy());
}
