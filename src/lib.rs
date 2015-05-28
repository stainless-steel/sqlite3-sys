#[cfg(test)]
extern crate temporary;

extern crate libc;

use libc::{c_char, c_int};

#[repr(C)]
pub struct sqlite3;

pub const SQLITE_OK: c_int = 0;

extern "C" {
    pub fn sqlite3_open(filename: *const c_char, ppDb: *mut *mut sqlite3) -> c_int;
    pub fn sqlite3_close(pDb: *mut sqlite3) -> c_int;
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

    #[test]
    fn open_close() {
        let (path, _directory) = setup();
        let mut sqlite = 0 as *mut ::sqlite3;
        unsafe {
            success!(::sqlite3_open(path.as_ptr(), &mut sqlite));
            success!(::sqlite3_close(sqlite));
        }
    }

    fn setup() -> (CString, Directory) {
        let directory = ok!(Directory::new("sqlite-sys"));
        let path = directory.path().join("database.sqlite3");
        let path = ok!(CString::new(ok!(path.to_str())));
        (path, directory)
    }
}
