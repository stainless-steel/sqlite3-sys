use core::ffi::{c_char, c_int, c_void};

use crate::base::sqlite3;

extern "C" {
    pub fn sqlite3_key(db: *mut sqlite3, pKey: *const c_void, nKey: c_int) -> c_int;
    pub fn sqlite3_key_v2(
        db: *mut sqlite3,
        zDbName: *const c_char,
        pKey: *const c_void,
        nKey: c_int,
    ) -> c_int;
    pub fn sqlite3_rekey(db: *mut sqlite3, pKey: *const c_void, nKey: c_int) -> c_int;
    pub fn sqlite3_rekey_v2(
        db: *mut sqlite3,
        zDbName: *const c_char,
        pKey: *const c_void,
        nKey: c_int,
    ) -> c_int;
}
