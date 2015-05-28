#![allow(non_camel_case_types)]

#[cfg(test)]
extern crate temporary;

extern crate libc;

use libc::{c_char, c_int, c_void};

#[repr(C)]
pub struct sqlite3;

pub const SQLITE_OK: c_int = 0;

pub type sqlite3_exec_callback = extern fn(*mut c_void, c_int, *mut *mut c_char,
                                           *mut *mut c_char) -> c_int;

extern "C" {
    pub fn sqlite3_malloc(n: c_int) -> *mut c_void;
    pub fn sqlite3_free(p: *mut c_void);

    pub fn sqlite3_errmsg(db: *mut sqlite3) -> *const c_char;

    pub fn sqlite3_open(filename: *const c_char, db: *mut *mut sqlite3) -> c_int;
    pub fn sqlite3_close(db: *mut sqlite3) -> c_int;

    pub fn sqlite3_exec(db: *mut sqlite3, sql: *const c_char,
                        callback: Option<sqlite3_exec_callback>,
                        arg: *mut c_void, errmsg: *mut *mut c_char) -> c_int;
}

#[cfg(test)]
mod tests {
    use std::ffi::CString;
    use temporary::Directory;

    macro_rules! ok(
        ($result:expr) => ($result.unwrap());
    );

    macro_rules! success(
        ($result:expr) => (assert!($result == ::SQLITE_OK));
    );

    macro_rules! c_str(
        ($string:expr) => (ok!(CString::new($string)));
    );

    #[test]
    fn exec() {
        open(|sqlite| unsafe {
            success!(::sqlite3_exec(sqlite, c_str!(
                "CREATE TABLE `foo` (bar INTEGER, baz INTEGER);"
            ).as_ptr(), None, 0 as *mut _, 0 as *mut _));
        });
    }

    fn open<F>(mut code: F) where F: FnMut(*mut ::sqlite3) {
        let (path, _directory) = setup();
        let mut sqlite = 0 as *mut ::sqlite3;
        unsafe {
            success!(::sqlite3_open(path.as_ptr(), &mut sqlite));
            code(sqlite);
            success!(::sqlite3_close(sqlite));
        }
    }

    fn setup() -> (CString, Directory) {
        let directory = ok!(Directory::new("sqlite-sys"));
        let path = directory.path().join("database.sqlite3");
        let path = c_str!(ok!(path.to_str()));
        (path, directory)
    }
}
