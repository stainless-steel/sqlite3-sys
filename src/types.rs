use libc::{c_char, c_int, c_longlong, c_uint, c_ulonglong, c_void};

pub enum sqlite3 {}
pub enum sqlite3_backup {}
pub enum sqlite3_blob {}
pub enum sqlite3_context {}
pub enum sqlite3_module {}
pub enum sqlite3_mutex {}
pub enum sqlite3_snapshot {}
pub enum sqlite3_stmt {}
pub enum sqlite3_value {}
pub enum sqlite3_vfs {}

pub type sqlite3_int64 = c_longlong;
pub type sqlite3_uint64 = c_ulonglong;

pub type sqlite3_callback = extern "C" fn(*mut c_void);

pub type sqlite3_auto_extension_callback = extern "C" fn();
pub type sqlite3_busy_callback = extern "C" fn(*mut c_void, c_int) -> c_int;
pub type sqlite3_collation_need_callback = extern "C" fn(*mut c_void, *mut sqlite3, c_int,
                                                         *const c_char);
pub type sqlite3_collation_need16_callback = extern "C" fn(*mut c_void, *mut sqlite3, c_int,
                                                           *const c_void);
pub type sqlite3_commit_hook_callback = extern "C" fn(*mut c_void) -> c_int;
pub type sqlite3_create_collation_callback = extern "C" fn(*mut c_void, c_int, *const c_void,
                                                           c_int, *const c_void) -> c_int;
pub type sqlite3_create_collation16_callback = extern "C" fn(*mut c_void, c_int, *const c_void,
                                                             c_int, *const c_void) -> c_int;
pub type sqlite3_create_function_callback1 = extern "C" fn(*mut sqlite3_context, c_int,
                                                           *mut *mut sqlite3_value);
pub type sqlite3_create_function_callback2 = extern "C" fn(*mut sqlite3_context);
pub type sqlite3_exec_callback = extern "C" fn(*mut c_void, c_int, *mut *mut c_char,
                                               *mut *mut c_char) -> c_int;
pub type sqlite3_memory_alarm_callback = extern "C" fn(*mut c_void, sqlite3_int64, c_int);
pub type sqlite3_preupdate_hook_callback = extern "C" fn(*mut c_void, *mut sqlite3, c_int,
                                                         *const c_char, *const c_char,
                                                         sqlite3_int64, sqlite3_int64);
pub type sqlite3_profile_callback = extern "C" fn(*mut c_void, *const c_char, sqlite3_uint64);
pub type sqlite3_progress_handler_callback = extern "C" fn(*mut c_void) -> c_int;
pub type sqlite3_set_authorizer_callback = extern "C" fn(*mut c_void, c_int, *const c_char,
                                                         *const c_char, *const c_char,
                                                         *const c_char) -> c_int;
pub type sqlite3_trace_callback = extern "C" fn(*mut c_void, *const c_char);
pub type sqlite3_trace_v2_callback = extern "C" fn(c_uint, *mut c_void, *mut c_void, *mut c_void)
                                                   -> c_int;
pub type sqlite3_unlock_notify_callback = extern "C" fn(*mut *mut c_void, c_int);
pub type sqlite3_update_hook_callback = extern "C" fn(*mut c_void, c_int, *const c_char,
                                                      *const c_char, sqlite3_int64);
pub type sqlite3_wal_hook_callback = extern "C" fn(*mut c_void, *mut sqlite3,
                                                   *const c_char, c_int) -> c_int;
