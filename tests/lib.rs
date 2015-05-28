extern crate libc;
extern crate sqlite3_sys;
extern crate temporary;

use std::ffi::{CStr, CString};
use std::ops::Deref;
use temporary::Directory;

use libc::{c_char, c_int, c_void};
use sqlite3_sys::*;

macro_rules! ok(
    ($result:expr) => ($result.unwrap());
);

macro_rules! success(
    ($result:expr) => (assert!($result == SQLITE_OK));
);

macro_rules! c_str(
    ($pointer:expr) => (CStr::from_ptr($pointer as *const _));
);

macro_rules! c_string(
    ($string:expr) => (ok!(CString::new($string)));
);

#[test]
fn workflow() {
    open(|database| unsafe {
        success!(sqlite3_exec(database, c_string!(
            "CREATE TABLE `users` (id INTEGER, name VARCHAR(255), age REAL);"
        ).as_ptr(), None, 0 as *mut _, 0 as *mut _));

        {
            let mut statement = 0 as *mut _;
            let mut tail = 0 as *const _;
            success!(sqlite3_prepare(database, c_string!(
                "INSERT INTO `users` (id, name, age) VALUES (?, ?, ?);"
            ).as_ptr(), -1, &mut statement, &mut tail));

            let name = c_string!("Alice");
            success!(sqlite3_bind_int(statement, 1, 1));
            success!(sqlite3_bind_text(statement, 2, name.as_ptr(), -1, None));
            success!(sqlite3_bind_double(statement, 3, 20.99));
            assert!(sqlite3_step(statement) == SQLITE_DONE);

            success!(sqlite3_finalize(statement));
        }

        {
            let mut done = false;
            success!(sqlite3_exec(database, c_string!(
                "SELECT * FROM `users`;"
            ).as_ptr(), Some(list), &mut done as *mut _ as *mut _, 0 as *mut _));
            assert!(done);
        }

        {
            let mut statement = 0 as *mut _;
            let mut tail = 0 as *const _;
            success!(sqlite3_prepare(database, c_string!(
                "SELECT * FROM `users`;"
            ).as_ptr(), -1, &mut statement, &mut tail));

            assert!(sqlite3_step(statement) == SQLITE_ROW);
            assert!(sqlite3_column_int(statement, 0) == 1);
            assert!(c_str!(sqlite3_column_text(statement, 1)) == c_string!("Alice").deref());
            assert!(sqlite3_column_double(statement, 2) == 20.99);

            assert!(sqlite3_step(statement) == SQLITE_DONE);

            success!(sqlite3_finalize(statement));
        }
    });

    extern fn list(done: *mut c_void, count: c_int, values: *mut *mut c_char,
                   _: *mut *mut c_char) -> c_int {

        unsafe {
            assert!(count == 3);

            assert!(c_str!(*values) == c_string!("1").deref());
            assert!(c_str!(*values.offset(1)) == c_string!("Alice").deref());
            assert!(c_str!(*values.offset(2)) == c_string!("20.99").deref());

            *(done as *mut bool) = true;
        }
        0
    }
}

fn open<F>(mut code: F) where F: FnMut(*mut sqlite3) {
    let (path, _directory) = setup();
    let mut database = 0 as *mut _;
    unsafe {
        success!(sqlite3_open(path.as_ptr(), &mut database));
        code(database);
        success!(sqlite3_close(database));
    }
}

fn setup() -> (CString, Directory) {
    let directory = ok!(Directory::new("sqlite3-sys"));
    let path = directory.path().join("database.sqlite3");
    let path = c_string!(ok!(path.to_str()));
    (path, directory)
}
