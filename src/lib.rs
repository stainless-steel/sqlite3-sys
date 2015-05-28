#![allow(non_camel_case_types)]

#[cfg(test)]
extern crate temporary;

extern crate libc;

use libc::{c_char, c_double, c_int, c_longlong, c_ulonglong, c_void};

#[repr(C)]
pub struct sqlite3;

#[repr(C)]
pub struct sqlite3_stmt;

pub type sqlite_int64 = c_longlong;
pub type sqlite_uint64 = c_ulonglong;

pub type sqlite3_int64 = sqlite_int64;
pub type sqlite3_uint64 = sqlite_uint64;

pub const SQLITE_OK: c_int = 0;
pub const SQLITE_ERROR: c_int = 1;
pub const SQLITE_INTERNAL: c_int = 2;
pub const SQLITE_PERM: c_int = 3;
pub const SQLITE_ABORT: c_int = 4;
pub const SQLITE_BUSY: c_int = 5;
pub const SQLITE_LOCKED: c_int = 6;
pub const SQLITE_NOMEM: c_int = 7;
pub const SQLITE_READONLY: c_int = 8;
pub const SQLITE_INTERRUPT: c_int = 9;
pub const SQLITE_IOERR: c_int = 10;
pub const SQLITE_CORRUPT: c_int = 11;
pub const SQLITE_NOTFOUND: c_int = 12;
pub const SQLITE_FULL: c_int = 13;
pub const SQLITE_CANTOPEN: c_int = 14;
pub const SQLITE_PROTOCOL: c_int = 15;
pub const SQLITE_EMPTY: c_int = 16;
pub const SQLITE_SCHEMA: c_int = 17;
pub const SQLITE_TOOBIG: c_int = 18;
pub const SQLITE_CONSTRAINT: c_int = 19;
pub const SQLITE_MISMATCH: c_int = 20;
pub const SQLITE_MISUSE: c_int = 21;
pub const SQLITE_NOLFS: c_int = 22;
pub const SQLITE_AUTH: c_int = 23;
pub const SQLITE_FORMAT: c_int = 24;
pub const SQLITE_RANGE: c_int = 25;
pub const SQLITE_NOTADB: c_int = 26;
pub const SQLITE_NOTICE: c_int = 27;
pub const SQLITE_WARNING: c_int = 28;
pub const SQLITE_ROW: c_int = 100;
pub const SQLITE_DONE: c_int = 101;

pub type sqlite3_exec_callback = extern fn(*mut c_void, c_int, *mut *mut c_char,
                                           *mut *mut c_char) -> c_int;

pub type sqlite3_bind_callback = extern fn(*mut c_void);

extern "C" {
    pub fn sqlite3_bind_double(stmt: *mut sqlite3_stmt, i: c_int, value: c_double) -> c_int;
    pub fn sqlite3_bind_int(stmt: *mut sqlite3_stmt, i: c_int, value: c_int) -> c_int;
    pub fn sqlite3_bind_int64(stmt: *mut sqlite3_stmt, i: c_int, value: sqlite3_int64) -> c_int;

    pub fn sqlite3_bind_text(stmt: *mut sqlite3_stmt, i: c_int, data: *const c_char, n: c_int,
                             del: Option<sqlite3_bind_callback>) -> c_int;

    pub fn sqlite3_close(db: *mut sqlite3) -> c_int;
    pub fn sqlite3_errmsg(db: *mut sqlite3) -> *const c_char;

    pub fn sqlite3_exec(db: *mut sqlite3, sql: *const c_char,
                        callback: Option<sqlite3_exec_callback>, arg: *mut c_void,
                        errmsg: *mut *mut c_char) -> c_int;

    pub fn sqlite3_finalize(stmt: *mut sqlite3_stmt) -> c_int;
    pub fn sqlite3_free(p: *mut c_void);
    pub fn sqlite3_malloc(n: c_int) -> *mut c_void;
    pub fn sqlite3_open(filename: *const c_char, db: *mut *mut sqlite3) -> c_int;

    pub fn sqlite3_prepare(db: *mut sqlite3, sql: *const c_char, n: c_int,
                           stmt: *mut *mut sqlite3_stmt, tail: *mut *const c_char) -> c_int;

    pub fn sqlite3_reset(stmt: *mut sqlite3_stmt) -> c_int;
    pub fn sqlite3_step(stmt: *mut sqlite3_stmt) -> c_int;
}

#[cfg(test)]
mod tests {
    use std::ffi::{CStr, CString};
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
    fn workflow() {
        use libc::{c_char, c_int, c_void};

        open(|database| unsafe {
            success!(::sqlite3_exec(database, c_str!(
                "CREATE TABLE `users` (id INTEGER, name VARCHAR(255), age REAL);"
            ).as_ptr(), None, 0 as *mut _, 0 as *mut _));

            let mut statement = 0 as *mut _;
            let mut tail = 0 as *const _;
            success!(::sqlite3_prepare(database, c_str!(
                "INSERT INTO `users` (id, name, age) VALUES (?, ?, ?);"
            ).as_ptr(), -1, &mut statement, &mut tail));

            let name = c_str!("Alice");
            success!(::sqlite3_bind_int(statement, 1, 1));
            success!(::sqlite3_bind_text(statement, 2, name.as_ptr(), -1, None));
            success!(::sqlite3_bind_double(statement, 3, 20.99));
            assert!(::sqlite3_step(statement) == ::SQLITE_DONE);
            success!(::sqlite3_finalize(statement));

            extern fn list(done: *mut c_void, count: c_int, values: *mut *mut c_char,
                           _: *mut *mut c_char) -> c_int {

                unsafe {
                    use std::ops::Deref;
                    assert!(count == 3);

                    assert!(CStr::from_ptr(*values) ==
                            ok!(CString::new("1")).deref());

                    assert!(CStr::from_ptr(*values.offset(1)) ==
                            ok!(CString::new("Alice")).deref());

                    assert!(CStr::from_ptr(*values.offset(2)) ==
                            ok!(CString::new("20.99")).deref());

                    *(done as *mut bool) = true;
                }

                0
            }

            let mut done = false;

            success!(::sqlite3_exec(database, c_str!(
                "SELECT * FROM `users`;"
            ).as_ptr(), Some(list), &mut done as *mut _ as *mut _, 0 as *mut _));

            assert!(done);
        });
    }

    fn open<F>(mut code: F) where F: FnMut(*mut ::sqlite3) {
        let (path, _directory) = setup();
        let mut database = 0 as *mut _;
        unsafe {
            success!(::sqlite3_open(path.as_ptr(), &mut database));
            code(database);
            success!(::sqlite3_close(database));
        }
    }

    fn setup() -> (CString, Directory) {
        let directory = ok!(Directory::new("sqlite-sys"));
        let path = directory.path().join("database.sqlite3");
        let path = c_str!(ok!(path.to_str()));
        (path, directory)
    }
}
