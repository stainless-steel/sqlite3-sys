#![allow(improper_ctypes, non_camel_case_types)]

extern crate libc;

use libc::{c_char, c_double, c_int, c_longlong, c_uchar, c_ulonglong, c_void};

mod constants;
pub use constants::*;

#[repr(C)]
pub struct sqlite3;

#[repr(C)]
pub struct sqlite3_stmt;

pub type sqlite_int64 = c_longlong;
pub type sqlite_uint64 = c_ulonglong;

pub type sqlite3_int64 = sqlite_int64;
pub type sqlite3_uint64 = sqlite_uint64;

pub type sqlite3_bind_callback = extern fn(*mut c_void);
pub type sqlite3_busy_callback = extern fn(*mut c_void, c_int) -> c_int;

pub type sqlite3_exec_callback = extern fn(*mut c_void, c_int, *mut *mut c_char,
                                           *mut *mut c_char) -> c_int;

extern "C" {
    pub fn sqlite3_bind_double(stmt: *mut sqlite3_stmt, i: c_int, value: c_double) -> c_int;
    pub fn sqlite3_bind_int(stmt: *mut sqlite3_stmt, i: c_int, value: c_int) -> c_int;
    pub fn sqlite3_bind_int64(stmt: *mut sqlite3_stmt, i: c_int, value: sqlite3_int64) -> c_int;

    pub fn sqlite3_bind_text(stmt: *mut sqlite3_stmt, i: c_int, data: *const c_char, n: c_int,
                             del: Option<sqlite3_bind_callback>) -> c_int;

    pub fn sqlite3_busy_handler(db: *mut sqlite3, busy: Option<sqlite3_busy_callback>,
                                arg: *mut c_void) -> c_int;

    pub fn sqlite3_busy_timeout(db: *mut sqlite3, ms: c_int) -> c_int;
    pub fn sqlite3_close(db: *mut sqlite3) -> c_int;

    #[cfg(feature = "sqlite3-close-v2")]
    pub fn sqlite3_close_v2(db: *mut sqlite3) -> c_int;

    pub fn sqlite3_column_blob(stmt: *mut sqlite3_stmt, i: c_int) -> *const c_void;
    pub fn sqlite3_column_bytes(stmt: *mut sqlite3_stmt, i: c_int) -> c_int;
    pub fn sqlite3_column_count(stmt: *mut sqlite3_stmt) -> c_int;
    pub fn sqlite3_column_double(stmt: *mut sqlite3_stmt, i: c_int) -> c_double;
    pub fn sqlite3_column_int(stmt: *mut sqlite3_stmt, i: c_int) -> c_int;
    pub fn sqlite3_column_int64(stmt: *mut sqlite3_stmt, i: c_int) -> sqlite3_int64;
    pub fn sqlite3_column_text(stmt: *mut sqlite3_stmt, i: c_int) -> *const c_uchar;
    pub fn sqlite3_column_type(stmt: *mut sqlite3_stmt, i: c_int) -> c_int;
    pub fn sqlite3_errcode(db: *mut sqlite3) -> c_int;
    pub fn sqlite3_errmsg(db: *mut sqlite3) -> *const c_char;

    #[cfg(feature = "sqlite3-errstr")]
    pub fn sqlite3_errstr(code: c_int) -> *const c_char;

    pub fn sqlite3_exec(db: *mut sqlite3, sql: *const c_char,
                        callback: Option<sqlite3_exec_callback>, arg: *mut c_void,
                        errmsg: *mut *mut c_char) -> c_int;

    pub fn sqlite3_finalize(stmt: *mut sqlite3_stmt) -> c_int;
    pub fn sqlite3_free(p: *mut c_void);
    pub fn sqlite3_malloc(n: c_int) -> *mut c_void;
    pub fn sqlite3_open(filename: *const c_char, db: *mut *mut sqlite3) -> c_int;

    pub fn sqlite3_open_v2(filename: *const c_char, db: *mut *mut sqlite3, flags: c_int,
                           vfs: *const c_char) -> c_int;

    pub fn sqlite3_prepare(db: *mut sqlite3, sql: *const c_char, n: c_int,
                           stmt: *mut *mut sqlite3_stmt, tail: *mut *const c_char) -> c_int;

    pub fn sqlite3_prepare_v2(db: *mut sqlite3, sql: *const c_char, n: c_int,
                              stmt: *mut *mut sqlite3_stmt, tail: *mut *const c_char) -> c_int;

    pub fn sqlite3_reset(stmt: *mut sqlite3_stmt) -> c_int;
    pub fn sqlite3_step(stmt: *mut sqlite3_stmt) -> c_int;
}
