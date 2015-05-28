#![allow(non_camel_case_types)]

extern crate libc;

use libc::{c_char, c_double, c_int, c_longlong, c_uchar, c_ulonglong, c_void};

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
    pub fn sqlite3_column_double(stmt: *mut sqlite3_stmt, i: c_int) -> c_double;
    pub fn sqlite3_column_int(stmt: *mut sqlite3_stmt, i: c_int) -> c_int;
    pub fn sqlite3_column_int64(stmt: *mut sqlite3_stmt, i: c_int) -> sqlite3_int64;
    pub fn sqlite3_column_text(stmt: *mut sqlite3_stmt, i: c_int) -> *const c_uchar;
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
