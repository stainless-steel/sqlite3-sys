use libc::{c_char, c_double, c_int, c_uchar, c_uint, c_void};

use types::*;

extern "C" {
    pub fn sqlite3_activate_cerod(p: *const c_char);
    pub fn sqlite3_activate_see(p: *const c_char);
    pub fn sqlite3_aggregate_context(p: *mut sqlite3_context, n: c_int) -> *mut c_void;
    pub fn sqlite3_aggregate_count(p: *mut sqlite3_context) -> c_int;
    pub fn sqlite3_auto_extension(f: Option<sqlite3_auto_extension_callback>) -> c_int;
    pub fn sqlite3_backup_finish(p: *mut sqlite3_backup) -> c_int;
    pub fn sqlite3_backup_init(p: *mut sqlite3, p: *const c_char, p: *mut sqlite3,
                               p: *const c_char) -> *mut sqlite3_backup;
    pub fn sqlite3_backup_pagecount(p: *mut sqlite3_backup) -> c_int;
    pub fn sqlite3_backup_remaining(p: *mut sqlite3_backup) -> c_int;
    pub fn sqlite3_backup_step(p: *mut sqlite3_backup, n: c_int) -> c_int;
    pub fn sqlite3_bind_blob(p: *mut sqlite3_stmt, n: c_int, p: *const c_void, n: c_int,
                             f: Option<sqlite3_callback>) -> c_int;
    pub fn sqlite3_bind_blob64(p: *mut sqlite3_stmt, n: c_int, p: *const c_void, n: sqlite3_uint64,
                               f: Option<sqlite3_callback>) -> c_int;
    pub fn sqlite3_bind_double(p: *mut sqlite3_stmt, n: c_int, n: c_double) -> c_int;
    pub fn sqlite3_bind_int(p: *mut sqlite3_stmt, n: c_int, n: c_int) -> c_int;
    pub fn sqlite3_bind_int64(p: *mut sqlite3_stmt, n: c_int, n: sqlite3_int64) -> c_int;
    pub fn sqlite3_bind_null(p: *mut sqlite3_stmt, n: c_int) -> c_int;
    pub fn sqlite3_bind_parameter_count(p: *mut sqlite3_stmt) -> c_int;
    pub fn sqlite3_bind_parameter_index(p: *mut sqlite3_stmt, p: *const c_char) -> c_int;
    pub fn sqlite3_bind_parameter_name(p: *mut sqlite3_stmt, n: c_int) -> *const c_char;
    pub fn sqlite3_bind_text(p: *mut sqlite3_stmt, n: c_int, p: *const c_char, n: c_int,
                             f: Option<sqlite3_callback>) -> c_int;
    pub fn sqlite3_bind_text16(p: *mut sqlite3_stmt, n: c_int, p: *const c_void, n: c_int,
                               f: Option<sqlite3_callback>) -> c_int;
    pub fn sqlite3_bind_text64(p: *mut sqlite3_stmt, n: c_int, p: *const c_char, n: sqlite3_uint64,
                               f: Option<sqlite3_callback>, n: c_uchar) -> c_int;
    pub fn sqlite3_bind_value(p: *mut sqlite3_stmt, n: c_int, p: *const sqlite3_value) -> c_int;
    pub fn sqlite3_bind_zeroblob(p: *mut sqlite3_stmt, n: c_int, n: c_int) -> c_int;
    pub fn sqlite3_bind_zeroblob64(p: *mut sqlite3_stmt, n: c_int, n: sqlite3_uint64) -> c_int;
    pub fn sqlite3_blob_bytes(p: *mut sqlite3_blob) -> c_int;
    pub fn sqlite3_blob_close(p: *mut sqlite3_blob) -> c_int;
    pub fn sqlite3_blob_open(p: *mut sqlite3, p: *const c_char, p: *const c_char, p: *const c_char,
                             n: sqlite3_int64, n: c_int, pp: *mut *mut sqlite3_blob) -> c_int;
    pub fn sqlite3_blob_read(p: *mut sqlite3_blob, p: *mut c_void, n: c_int, n: c_int) -> c_int;
    pub fn sqlite3_blob_reopen(p: *mut sqlite3_blob, n: sqlite3_int64) -> c_int;
    pub fn sqlite3_blob_write(p: *mut sqlite3_blob, p: *const c_void, n: c_int, n: c_int) -> c_int;
    pub fn sqlite3_busy_handler(p: *mut sqlite3, f: Option<sqlite3_busy_callback>, p: *mut c_void)
                                -> c_int;
    pub fn sqlite3_busy_timeout(p: *mut sqlite3, n: c_int) -> c_int;
    pub fn sqlite3_cancel_auto_extension(f: Option<sqlite3_auto_extension_callback>) -> c_int;
    pub fn sqlite3_changes(p: *mut sqlite3) -> c_int;
    pub fn sqlite3_clear_bindings(p: *mut sqlite3_stmt) -> c_int;
    pub fn sqlite3_close(p: *mut sqlite3) -> c_int;
    pub fn sqlite3_close_v2(p: *mut sqlite3) -> c_int;
    pub fn sqlite3_collation_needed(p: *mut sqlite3, p: *mut c_void,
                                    f: Option<sqlite3_collation_need_callback>) -> c_int;
    pub fn sqlite3_collation_needed16(p: *mut sqlite3, p: *mut c_void,
                                      f: Option<sqlite3_collation_need16_callback>) -> c_int;
    pub fn sqlite3_column_blob(p: *mut sqlite3_stmt, n: c_int) -> *const c_void;
    pub fn sqlite3_column_bytes(p: *mut sqlite3_stmt, n: c_int) -> c_int;
    pub fn sqlite3_column_bytes16(p: *mut sqlite3_stmt, n: c_int) -> c_int;
    pub fn sqlite3_column_count(p: *mut sqlite3_stmt) -> c_int;
    pub fn sqlite3_column_database_name(p: *mut sqlite3_stmt, n: c_int) -> *const c_char;
    pub fn sqlite3_column_database_name16(p: *mut sqlite3_stmt, n: c_int) -> *const c_void;
    pub fn sqlite3_column_decltype(p: *mut sqlite3_stmt, n: c_int) -> *const c_char;
    pub fn sqlite3_column_decltype16(p: *mut sqlite3_stmt, n: c_int) -> *const c_void;
    pub fn sqlite3_column_double(p: *mut sqlite3_stmt, n: c_int) -> c_double;
    pub fn sqlite3_column_int(p: *mut sqlite3_stmt, n: c_int) -> c_int;
    pub fn sqlite3_column_int64(p: *mut sqlite3_stmt, n: c_int) -> sqlite3_int64;
    pub fn sqlite3_column_name(p: *mut sqlite3_stmt, n: c_int) -> *const c_char;
    pub fn sqlite3_column_name16(p: *mut sqlite3_stmt, n: c_int) -> *const c_void;
    pub fn sqlite3_column_origin_name(p: *mut sqlite3_stmt, n: c_int) -> *const c_char;
    pub fn sqlite3_column_origin_name16(p: *mut sqlite3_stmt, n: c_int) -> *const c_void;
    pub fn sqlite3_column_table_name(p: *mut sqlite3_stmt, n: c_int) -> *const c_char;
    pub fn sqlite3_column_table_name16(p: *mut sqlite3_stmt, n: c_int) -> *const c_void;
    pub fn sqlite3_column_text(p: *mut sqlite3_stmt, n: c_int) -> *const c_uchar;
    pub fn sqlite3_column_text16(p: *mut sqlite3_stmt, n: c_int) -> *const c_void;
    pub fn sqlite3_column_type(p: *mut sqlite3_stmt, n: c_int) -> c_int;
    pub fn sqlite3_column_value(p: *mut sqlite3_stmt, n: c_int) -> *mut sqlite3_value;
    pub fn sqlite3_commit_hook(p: *mut sqlite3, f: Option<sqlite3_commit_hook_callback>,
                               p: *mut c_void) -> *mut c_void;
    pub fn sqlite3_compileoption_get(n: c_int) -> *const c_char;
    pub fn sqlite3_compileoption_used(p: *const c_char) -> c_int;
    pub fn sqlite3_complete(p: *const c_char) -> c_int;
    pub fn sqlite3_complete16(p: *const c_void) -> c_int;
    pub fn sqlite3_config(n: c_int, ...) -> c_int;
    pub fn sqlite3_context_db_handle(p: *mut sqlite3_context) -> *mut sqlite3;
    pub fn sqlite3_create_collation(p: *mut sqlite3, p: *const c_char, n: c_int, p: *mut c_void,
                                    f: Option<sqlite3_create_collation_callback>) -> c_int;
    pub fn sqlite3_create_collation16(p: *mut sqlite3, p: *const c_void, n: c_int, p: *mut c_void,
                                      f: Option<sqlite3_create_collation16_callback>) -> c_int;
    pub fn sqlite3_create_collation_v2(p: *mut sqlite3, p: *const c_char, n: c_int, p: *mut c_void,
                                       f: Option<sqlite3_create_collation_callback>,
                                       f: Option<sqlite3_callback>) -> c_int;
    pub fn sqlite3_create_function(p: *mut sqlite3, p: *const c_char, n: c_int, n: c_int,
                                   p: *mut c_void, f: Option<sqlite3_create_function_callback1>,
                                   f: Option<sqlite3_create_function_callback1>,
                                   f: Option<sqlite3_create_function_callback2>) -> c_int;
    pub fn sqlite3_create_function16(p: *mut sqlite3, p: *const c_void, n: c_int, n: c_int,
                                     p: *mut c_void, f: Option<sqlite3_create_function_callback1>,
                                     f: Option<sqlite3_create_function_callback1>,
                                     f: Option<sqlite3_create_function_callback2>) -> c_int;
    pub fn sqlite3_create_function_v2(p: *mut sqlite3, p: *const c_char, n: c_int, n: c_int,
                                      p: *mut c_void, f: Option<sqlite3_create_function_callback1>,
                                      f: Option<sqlite3_create_function_callback1>,
                                      f: Option<sqlite3_create_function_callback2>,
                                      f: Option<sqlite3_callback>) -> c_int;
    pub fn sqlite3_create_module(p: *mut sqlite3, p: *const c_char, p: *const sqlite3_module,
                                 p: *mut c_void) -> c_int;
    pub fn sqlite3_create_module_v2(p: *mut sqlite3, p: *const c_char, p: *const sqlite3_module,
                                    p: *mut c_void, f: Option<sqlite3_callback>) -> c_int;
    pub fn sqlite3_data_count(p: *mut sqlite3_stmt) -> c_int;
    pub fn sqlite3_db_cacheflush(p: *mut sqlite3) -> c_int;
    pub fn sqlite3_db_config(p: *mut sqlite3, n: c_int, ...) -> c_int;
    pub fn sqlite3_db_filename(p: *mut sqlite3, p: *const c_char) -> *const c_char;
    pub fn sqlite3_db_handle(p: *mut sqlite3_stmt) -> *mut sqlite3;
    pub fn sqlite3_db_mutex(p: *mut sqlite3) -> *mut sqlite3_mutex;
    pub fn sqlite3_db_readonly(p: *mut sqlite3, p: *const c_char) -> c_int;
    pub fn sqlite3_db_release_memory(p: *mut sqlite3) -> c_int;
    pub fn sqlite3_db_status(p: *mut sqlite3, n: c_int, p: *mut c_int, p: *mut c_int, n: c_int)
                             -> c_int;
    pub fn sqlite3_declare_vtab(p: *mut sqlite3, p: *const c_char) -> c_int;
    pub fn sqlite3_enable_load_extension(p: *mut sqlite3, n: c_int) -> c_int;
    pub fn sqlite3_enable_shared_cache(n: c_int) -> c_int;
    pub fn sqlite3_errcode(p: *mut sqlite3) -> c_int;
    pub fn sqlite3_errmsg(p: *mut sqlite3) -> *const c_char;
    pub fn sqlite3_errmsg16(p: *mut sqlite3) -> *const c_void;
    pub fn sqlite3_errstr(n: c_int) -> *const c_char;
    pub fn sqlite3_exec(p: *mut sqlite3, p: *const c_char, f: Option<sqlite3_exec_callback>,
                        p: *mut c_void, pp: *mut *mut c_char) -> c_int;
    pub fn sqlite3_expired(p: *mut sqlite3_stmt) -> c_int;
    pub fn sqlite3_extended_errcode(p: *mut sqlite3) -> c_int;
    pub fn sqlite3_extended_result_codes(p: *mut sqlite3, n: c_int) -> c_int;
    pub fn sqlite3_expanded_sql(p: *mut sqlite3_stmt) -> *mut c_char;
    pub fn sqlite3_file_control(p: *mut sqlite3, p: *const c_char, n: c_int, p: *mut c_void)
                                -> c_int;
    pub fn sqlite3_finalize(p: *mut sqlite3_stmt) -> c_int;
    pub fn sqlite3_free(p: *mut c_void);
    pub fn sqlite3_free_table(pp: *mut *mut c_char);
    pub fn sqlite3_get_autocommit(p: *mut sqlite3) -> c_int;
    pub fn sqlite3_get_auxdata(p: *mut sqlite3_context, n: c_int) -> *mut c_void;
    pub fn sqlite3_get_table(p: *mut sqlite3, p: *const c_char, ppp: *mut *mut *mut c_char,
                             p: *mut c_int, p: *mut c_int, pp: *mut *mut c_char) -> c_int;
    pub fn sqlite3_global_recover() -> c_int;
    pub fn sqlite3_initialize() -> c_int;
    pub fn sqlite3_interrupt(p: *mut sqlite3);
    pub fn sqlite3_key(p: *mut sqlite3, p: *const c_void, n: c_int) -> c_int;
    pub fn sqlite3_key_v2(p: *mut sqlite3, p: *const c_char, p: *const c_void, n: c_int) -> c_int;
    pub fn sqlite3_last_insert_rowid(p: *mut sqlite3) -> sqlite3_int64;
    pub fn sqlite3_libversion() -> *const c_char;
    pub fn sqlite3_libversion_number() -> c_int;
    pub fn sqlite3_limit(p: *mut sqlite3, n: c_int, n: c_int) -> c_int;
    pub fn sqlite3_load_extension(p: *mut sqlite3, p: *const c_char, p: *const c_char,
                                  pp: *mut *mut c_char) -> c_int;
    pub fn sqlite3_log(n: c_int, p: *const c_char, ...);
    pub fn sqlite3_malloc(n: c_int) -> *mut c_void;
    pub fn sqlite3_malloc64(n: sqlite3_uint64) -> *mut c_void;
    pub fn sqlite3_memory_alarm(f: Option<sqlite3_memory_alarm_callback>, p: *mut c_void,
                                n: sqlite3_int64) -> c_int;
    pub fn sqlite3_memory_highwater(n: c_int) -> sqlite3_int64;
    pub fn sqlite3_memory_used() -> sqlite3_int64;
    pub fn sqlite3_mprintf(p: *const c_char, ...) -> *mut c_char;
    pub fn sqlite3_msize(p: *mut c_void) -> sqlite3_uint64;
    pub fn sqlite3_mutex_alloc(n: c_int) -> *mut sqlite3_mutex;
    pub fn sqlite3_mutex_enter(p: *mut sqlite3_mutex);
    pub fn sqlite3_mutex_free(p: *mut sqlite3_mutex);
    pub fn sqlite3_mutex_held(p: *mut sqlite3_mutex) -> c_int;
    pub fn sqlite3_mutex_leave(p: *mut sqlite3_mutex);
    pub fn sqlite3_mutex_notheld(p: *mut sqlite3_mutex) -> c_int;
    pub fn sqlite3_mutex_try(p: *mut sqlite3_mutex) -> c_int;
    pub fn sqlite3_next_stmt(p: *mut sqlite3, p: *mut sqlite3_stmt) -> *mut sqlite3_stmt;
    pub fn sqlite3_open(p: *const c_char, pp: *mut *mut sqlite3) -> c_int;
    pub fn sqlite3_open16(p: *const c_void, pp: *mut *mut sqlite3) -> c_int;
    pub fn sqlite3_open_v2(p: *const c_char, pp: *mut *mut sqlite3, n: c_int, p: *const c_char)
                           -> c_int;
    pub fn sqlite3_os_end() -> c_int;
    pub fn sqlite3_os_init() -> c_int;
    pub fn sqlite3_overload_function(p: *mut sqlite3, p: *const c_char, n: c_int) -> c_int;
    pub fn sqlite3_prepare(p: *mut sqlite3, p: *const c_char, n: c_int,
                           pp: *mut *mut sqlite3_stmt, pp: *mut *const c_char) -> c_int;
    pub fn sqlite3_prepare16(p: *mut sqlite3, p: *const c_void, n: c_int,
                             pp: *mut *mut sqlite3_stmt, pp: *mut *const c_void) -> c_int;
    pub fn sqlite3_prepare16_v2(p: *mut sqlite3, p: *const c_void, n: c_int,
                                pp: *mut *mut sqlite3_stmt, pp: *mut *const c_void) -> c_int;
    pub fn sqlite3_prepare_v2(p: *mut sqlite3, p: *const c_char, n: c_int,
                              pp: *mut *mut sqlite3_stmt, pp: *mut *const c_char) -> c_int;
    pub fn sqlite3_preupdate_count(p: *mut sqlite3) -> c_int;
    pub fn sqlite3_preupdate_depth(p: *mut sqlite3) -> c_int;
    pub fn sqlite3_preupdate_hook(p: *mut sqlite3, f: Option<sqlite3_preupdate_hook_callback>,
                                  p: *mut c_void) -> *mut c_void;
    pub fn sqlite3_preupdate_new(p: *mut sqlite3, n: c_int, pp: *mut *mut sqlite3_value) -> c_int;
    pub fn sqlite3_preupdate_old(p: *mut sqlite3, n: c_int, pp: *mut *mut sqlite3_value) -> c_int;
    pub fn sqlite3_profile(p: *mut sqlite3, f: Option<sqlite3_profile_callback>, p: *mut c_void)
                           -> *mut c_void;
    pub fn sqlite3_progress_handler(p: *mut sqlite3, n: c_int,
                                    f: Option<sqlite3_progress_handler_callback>, p: *mut c_void);
    pub fn sqlite3_randomness(n: c_int, p: *mut c_void);
    pub fn sqlite3_realloc(p: *mut c_void, n: c_int) -> *mut c_void;
    pub fn sqlite3_realloc64(p: *mut c_void, n: sqlite3_uint64) -> *mut c_void;
    pub fn sqlite3_rekey(p: *mut sqlite3, p: *const c_void, n: c_int) -> c_int;
    pub fn sqlite3_rekey_v2(p: *mut sqlite3, p: *const c_char, p: *const c_void, n: c_int)
                            -> c_int;
    pub fn sqlite3_release_memory(n: c_int) -> c_int;
    pub fn sqlite3_reset(p: *mut sqlite3_stmt) -> c_int;
    pub fn sqlite3_reset_auto_extension();
    pub fn sqlite3_result_blob(p: *mut sqlite3_context, p: *const c_void, n: c_int,
                               f: Option<sqlite3_callback>);
    pub fn sqlite3_result_blob64(p: *mut sqlite3_context, p: *const c_void, n: sqlite3_uint64,
                                 f: Option<sqlite3_callback>);
    pub fn sqlite3_result_double(p: *mut sqlite3_context, n: c_double);
    pub fn sqlite3_result_error(p: *mut sqlite3_context, p: *const c_char, n: c_int);
    pub fn sqlite3_result_error16(p: *mut sqlite3_context, p: *const c_void, n: c_int);
    pub fn sqlite3_result_error_code(p: *mut sqlite3_context, n: c_int);
    pub fn sqlite3_result_error_nomem(p: *mut sqlite3_context);
    pub fn sqlite3_result_error_toobig(p: *mut sqlite3_context);
    pub fn sqlite3_result_int(p: *mut sqlite3_context, n: c_int);
    pub fn sqlite3_result_int64(p: *mut sqlite3_context, n: sqlite3_int64);
    pub fn sqlite3_result_null(p: *mut sqlite3_context);
    pub fn sqlite3_result_subtype(p: *mut sqlite3_context, n: c_uint);
    pub fn sqlite3_result_text(p: *mut sqlite3_context, p: *const c_char, n: c_int,
                               f: Option<sqlite3_callback>);
    pub fn sqlite3_result_text16(p: *mut sqlite3_context, p: *const c_void, n: c_int,
                                 f: Option<sqlite3_callback>);
    pub fn sqlite3_result_text16be(p: *mut sqlite3_context, p: *const c_void, n: c_int,
                                   f: Option<sqlite3_callback>);
    pub fn sqlite3_result_text16le(p: *mut sqlite3_context, p: *const c_void, n: c_int,
                                   f: Option<sqlite3_callback>);
    pub fn sqlite3_result_text64(p: *mut sqlite3_context, p: *const c_char, n: sqlite3_uint64,
                                 f: Option<sqlite3_callback>, n: c_uchar);
    pub fn sqlite3_result_value(p: *mut sqlite3_context, p: *mut sqlite3_value);
    pub fn sqlite3_result_zeroblob(p: *mut sqlite3_context, n: c_int);
    pub fn sqlite3_result_zeroblob64(p: *mut sqlite3_context, n: sqlite3_uint64) -> c_int;
    pub fn sqlite3_rollback_hook(p: *mut sqlite3, f: Option<sqlite3_callback>, p: *mut c_void)
                                 -> *mut c_void;
    pub fn sqlite3_set_authorizer(p: *mut sqlite3, f: Option<sqlite3_set_authorizer_callback>,
                                  p: *mut c_void) -> c_int;
    pub fn sqlite3_set_auxdata(p: *mut sqlite3_context, n: c_int, p: *mut c_void,
                               f: Option<sqlite3_callback>);
    pub fn sqlite3_shutdown() -> c_int;
    pub fn sqlite3_sleep(n: c_int) -> c_int;
    pub fn sqlite3_snapshot_cmp(p: *mut sqlite3_snapshot, p: *mut sqlite3_snapshot) -> c_int;
    pub fn sqlite3_snapshot_free(p: *mut sqlite3_snapshot);
    pub fn sqlite3_snapshot_get(p: *mut sqlite3, p: *const c_char, pp: *mut *mut sqlite3_snapshot)
                                -> c_int;
    pub fn sqlite3_snapshot_open(p: *mut sqlite3, p: *const c_char, p: *mut sqlite3_snapshot)
                                 -> c_int;
    pub fn sqlite3_snapshot_recover(p: *mut sqlite3, p: *const c_char) -> c_int;
    pub fn sqlite3_snprintf(n: c_int, p: *mut c_char, p: *const c_char, ...) -> *mut c_char;
    pub fn sqlite3_soft_heap_limit(n: c_int);
    pub fn sqlite3_soft_heap_limit64(n: sqlite3_int64) -> sqlite3_int64;
    pub fn sqlite3_sourceid() -> *const c_char;
    pub fn sqlite3_sql(p: *mut sqlite3_stmt) -> *const c_char;
    pub fn sqlite3_status(n: c_int, p: *mut c_int, p: *mut c_int, n: c_int) -> c_int;
    pub fn sqlite3_status64(n: c_int, p: *mut sqlite3_int64, p: *mut sqlite3_int64, n: c_int)
                            -> c_int;
    pub fn sqlite3_step(p: *mut sqlite3_stmt) -> c_int;
    pub fn sqlite3_stmt_busy(p: *mut sqlite3_stmt) -> c_int;
    pub fn sqlite3_stmt_readonly(p: *mut sqlite3_stmt) -> c_int;
    pub fn sqlite3_stmt_scanstatus(p: *mut sqlite3_stmt, n: c_int, n: c_int, p: *mut c_void)
                                   -> c_int;
    pub fn sqlite3_stmt_scanstatus_reset(p: *mut sqlite3_stmt);
    pub fn sqlite3_stmt_status(p: *mut sqlite3_stmt, n: c_int, n: c_int) -> c_int;
    pub fn sqlite3_strglob(p: *const c_char, p: *const c_char) -> c_int;
    pub fn sqlite3_stricmp(p: *const c_char, p: *const c_char) -> c_int;
    pub fn sqlite3_strlike(p: *const c_char, p: *const c_char, n: c_uint) -> c_int;
    pub fn sqlite3_strnicmp(p: *const c_char, p: *const c_char, n: c_int) -> c_int;
    pub fn sqlite3_system_errno(p: *mut sqlite3) -> c_int;
    pub fn sqlite3_table_column_metadata(p: *mut sqlite3, p: *const c_char, p: *const c_char,
                                         p: *const c_char, pp: *mut *const c_char,
                                         pp: *mut *const c_char, p: *mut c_int, p: *mut c_int,
                                         p: *mut c_int) -> c_int;
    pub fn sqlite3_test_control(n: c_int, ...) -> c_int;
    pub fn sqlite3_thread_cleanup();
    pub fn sqlite3_threadsafe() -> c_int;
    pub fn sqlite3_total_changes(p: *mut sqlite3) -> c_int;
    pub fn sqlite3_trace(p: *mut sqlite3, f: Option<sqlite3_trace_callback>, p: *mut c_void)
                         -> *mut c_void;
    pub fn sqlite3_trace_v2(p: *mut sqlite3, n: c_uint, f: Option<sqlite3_trace_v2_callback>,
                            p: *mut c_void) -> c_int;
    pub fn sqlite3_transfer_bindings(p: *mut sqlite3_stmt, p: *mut sqlite3_stmt) -> c_int;
    pub fn sqlite3_unlock_notify(p: *mut sqlite3, f: Option<sqlite3_unlock_notify_callback>,
                                 p: *mut c_void) -> c_int;
    pub fn sqlite3_update_hook(p: *mut sqlite3, f: Option<sqlite3_update_hook_callback>,
                               p: *mut c_void) -> *mut c_void;
    pub fn sqlite3_uri_boolean(p: *const c_char, p: *const c_char, n: c_int) -> c_int;
    pub fn sqlite3_uri_int64(p: *const c_char, p: *const c_char, n: sqlite3_int64)
                             -> sqlite3_int64;
    pub fn sqlite3_uri_parameter(p: *const c_char, p: *const c_char) -> *const c_char;
    pub fn sqlite3_user_data(p: *mut sqlite3_context) -> *mut c_void;
    pub fn sqlite3_value_blob(p: *mut sqlite3_value) -> *const c_void;
    pub fn sqlite3_value_bytes(p: *mut sqlite3_value) -> c_int;
    pub fn sqlite3_value_bytes16(p: *mut sqlite3_value) -> c_int;
    pub fn sqlite3_value_double(p: *mut sqlite3_value) -> c_double;
    pub fn sqlite3_value_dup(p: *const sqlite3_value) -> *mut sqlite3_value;
    pub fn sqlite3_value_free(p: *mut sqlite3_value);
    pub fn sqlite3_value_int(p: *mut sqlite3_value) -> c_int;
    pub fn sqlite3_value_int64(p: *mut sqlite3_value) -> sqlite3_int64;
    pub fn sqlite3_value_numeric_type(p: *mut sqlite3_value) -> c_int;
    pub fn sqlite3_value_subtype(p: *mut sqlite3_value) -> c_uint;
    pub fn sqlite3_value_text(p: *mut sqlite3_value) -> *const c_uchar;
    pub fn sqlite3_value_text16(p: *mut sqlite3_value) -> *const c_void;
    pub fn sqlite3_value_text16be(p: *mut sqlite3_value) -> *const c_void;
    pub fn sqlite3_value_text16le(p: *mut sqlite3_value) -> *const c_void;
    pub fn sqlite3_value_type(p: *mut sqlite3_value) -> c_int;
    // sqlite3_version
    pub fn sqlite3_vfs_find(p: *const c_char) -> *mut sqlite3_vfs;
    pub fn sqlite3_vfs_register(p: *mut sqlite3_vfs, n: c_int) -> c_int;
    pub fn sqlite3_vfs_unregister(p: *mut sqlite3_vfs) -> c_int;
    // sqlite3_vmprintf
    // sqlite3_vsnprintf
    pub fn sqlite3_vtab_config(p: *mut sqlite3, n: c_int, ...) -> c_int;
    pub fn sqlite3_vtab_on_conflict(p: *mut sqlite3) -> c_int;
    pub fn sqlite3_wal_autocheckpoint(p: *mut sqlite3, n: c_int) -> c_int;
    pub fn sqlite3_wal_checkpoint(p: *mut sqlite3, p: *const c_char) -> c_int;
    pub fn sqlite3_wal_checkpoint_v2(p: *mut sqlite3, p: *const c_char, n: c_int, p: *mut c_int,
                                     p: *mut c_int) -> c_int;
    pub fn sqlite3_wal_hook(p: *mut sqlite3, f: Option<sqlite3_wal_hook_callback>,
                            p: *mut c_void) -> *mut c_void;
}
