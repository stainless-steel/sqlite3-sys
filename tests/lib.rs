extern crate libc;
extern crate sqlite3_sys;
extern crate temporary;

use std::ffi::{CStr, CString};
use std::ptr;
use temporary::Directory;

use libc::{c_char, c_int, c_void};
use sqlite3_sys::*;

macro_rules! c_str(($pointer:expr) => (CStr::from_ptr($pointer as *const _)));
macro_rules! c_string(($string:expr) => (ok!(CString::new($string))));
macro_rules! ok(($result:expr) => ($result.unwrap()));
macro_rules! success(($result:expr) => (assert_eq!($result, SQLITE_OK)));

#[test]
fn failure() {
    open(|database| unsafe { test_failure(database) });
}

#[test]
fn workflow() {
    open(|database| unsafe { test_workflow(database) });
}

unsafe fn test_failure(database: *mut sqlite3) {
    match sqlite3_exec(database, c_string!(":)").as_ptr(), None, ptr::null_mut(),
                       ptr::null_mut()) {
        SQLITE_OK => assert!(false),
        _ => assert_eq!(c_str!(sqlite3_errmsg(database)),
                        &c_string!(r#"unrecognized token: ":""#)[..]),
    }
}

unsafe fn test_workflow(database: *mut sqlite3) {
    {
        let query = c_string!("CREATE TABLE `users` (id INTEGER, name VARCHAR(255), age REAL);");
        success!(sqlite3_exec(database, query.as_ptr(), None, ptr::null_mut(), ptr::null_mut()));
    }
    {
        let query = c_string!("INSERT INTO `users` (id, name, age) VALUES (?, ?, ?);");
        let mut statement = ptr::null_mut();
        success!(sqlite3_prepare(database, query.as_ptr(), -1, &mut statement, ptr::null_mut()));
        let name = c_string!("Alice");
        success!(sqlite3_bind_int(statement, 1, 1));
        success!(sqlite3_bind_text(statement, 2, name.as_ptr(), -1, None));
        success!(sqlite3_bind_double(statement, 3, 20.99));
        assert_eq!(sqlite3_step(statement), SQLITE_DONE);
        assert_eq!(sqlite3_column_count(statement), 0);
        success!(sqlite3_finalize(statement));
    }
    {
        let query = c_string!("SELECT * FROM `users`;");
        let mut done = false;
        success!(sqlite3_exec(database, query.as_ptr(), Some(list), &mut done as *mut _ as *mut _,
                              ptr::null_mut()));
        assert!(done);
    }
    {
        let query = c_string!("SELECT * FROM `users`;");
        let mut statement = ptr::null_mut();
        success!(sqlite3_prepare(database, query.as_ptr(), -1, &mut statement, ptr::null_mut()));
        assert_eq!(sqlite3_step(statement), SQLITE_ROW);
        assert_eq!(sqlite3_column_count(statement), 3);
        assert_eq!(sqlite3_column_int(statement, 0), 1);
        assert_eq!(c_str!(sqlite3_column_text(statement, 1)), &c_string!("Alice")[..]);
        assert_eq!(sqlite3_column_double(statement, 2), 20.99);
        assert_eq!(sqlite3_step(statement), SQLITE_DONE);
        assert_eq!(sqlite3_column_count(statement), 3);
        success!(sqlite3_finalize(statement));
    }

    extern fn list(done: *mut c_void, count: c_int, values: *mut *mut c_char, _: *mut *mut c_char)
                   -> c_int {

        unsafe {
            assert_eq!(count, 3);
            assert_eq!(c_str!(*values), &c_string!("1")[..]);
            assert_eq!(c_str!(*values.offset(1)), &c_string!("Alice")[..]);
            assert_eq!(c_str!(*values.offset(2)), &c_string!("20.99")[..]);
            *(done as *mut bool) = true;
        }
        0
    }
}

fn open<F>(mut code: F) where F: FnMut(*mut sqlite3) {
    let (path, _directory) = setup();
    let mut database = ptr::null_mut();
    unsafe {
        success!(sqlite3_open_v2(path.as_ptr(), &mut database,
                                 SQLITE_OPEN_CREATE | SQLITE_OPEN_READWRITE, ptr::null()));
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
