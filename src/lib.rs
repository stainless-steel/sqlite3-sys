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
    fn open() {
        let directory = setup();
        let path = directory.path().join("dummy.sqlite3");
        let path = ok!(CString::new(ok!(path.to_str())));

        let mut sqlite = 0 as *mut super::sqlite3;
        unsafe {
            success!(super::sqlite3_open(path.as_ptr(), &mut sqlite));
            success!(super::sqlite3_close(sqlite));
        }
    }

    fn setup() -> Directory {
        ok!(Directory::new("sqlite-sys"))
    }
}
