use libc::c_int;

pub const SQLITE_OK:         c_int =   0;
pub const SQLITE_ERROR:      c_int =   1;
pub const SQLITE_INTERNAL:   c_int =   2;
pub const SQLITE_PERM:       c_int =   3;
pub const SQLITE_ABORT:      c_int =   4;
pub const SQLITE_BUSY:       c_int =   5;
pub const SQLITE_LOCKED:     c_int =   6;
pub const SQLITE_NOMEM:      c_int =   7;
pub const SQLITE_READONLY:   c_int =   8;
pub const SQLITE_INTERRUPT:  c_int =   9;
pub const SQLITE_IOERR:      c_int =  10;
pub const SQLITE_CORRUPT:    c_int =  11;
pub const SQLITE_NOTFOUND:   c_int =  12;
pub const SQLITE_FULL:       c_int =  13;
pub const SQLITE_CANTOPEN:   c_int =  14;
pub const SQLITE_PROTOCOL:   c_int =  15;
pub const SQLITE_EMPTY:      c_int =  16;
pub const SQLITE_SCHEMA:     c_int =  17;
pub const SQLITE_TOOBIG:     c_int =  18;
pub const SQLITE_CONSTRAINT: c_int =  19;
pub const SQLITE_MISMATCH:   c_int =  20;
pub const SQLITE_MISUSE:     c_int =  21;
pub const SQLITE_NOLFS:      c_int =  22;
pub const SQLITE_AUTH:       c_int =  23;
pub const SQLITE_FORMAT:     c_int =  24;
pub const SQLITE_RANGE:      c_int =  25;
pub const SQLITE_NOTADB:     c_int =  26;
pub const SQLITE_NOTICE:     c_int =  27;
pub const SQLITE_WARNING:    c_int =  28;
pub const SQLITE_ROW:        c_int = 100;
pub const SQLITE_DONE:       c_int = 101;

pub const SQLITE_IOERR_READ:              c_int = SQLITE_IOERR      | ( 1 << 8);
pub const SQLITE_IOERR_SHORT_READ:        c_int = SQLITE_IOERR      | ( 2 << 8);
pub const SQLITE_IOERR_WRITE:             c_int = SQLITE_IOERR      | ( 3 << 8);
pub const SQLITE_IOERR_FSYNC:             c_int = SQLITE_IOERR      | ( 4 << 8);
pub const SQLITE_IOERR_DIR_FSYNC:         c_int = SQLITE_IOERR      | ( 5 << 8);
pub const SQLITE_IOERR_TRUNCATE:          c_int = SQLITE_IOERR      | ( 6 << 8);
pub const SQLITE_IOERR_FSTAT:             c_int = SQLITE_IOERR      | ( 7 << 8);
pub const SQLITE_IOERR_UNLOCK:            c_int = SQLITE_IOERR      | ( 8 << 8);
pub const SQLITE_IOERR_RDLOCK:            c_int = SQLITE_IOERR      | ( 9 << 8);
pub const SQLITE_IOERR_DELETE:            c_int = SQLITE_IOERR      | (10 << 8);
pub const SQLITE_IOERR_BLOCKED:           c_int = SQLITE_IOERR      | (11 << 8);
pub const SQLITE_IOERR_NOMEM:             c_int = SQLITE_IOERR      | (12 << 8);
pub const SQLITE_IOERR_ACCESS:            c_int = SQLITE_IOERR      | (13 << 8);
pub const SQLITE_IOERR_CHECKRESERVEDLOCK: c_int = SQLITE_IOERR      | (14 << 8);
pub const SQLITE_IOERR_LOCK:              c_int = SQLITE_IOERR      | (15 << 8);
pub const SQLITE_IOERR_CLOSE:             c_int = SQLITE_IOERR      | (16 << 8);
pub const SQLITE_IOERR_DIR_CLOSE:         c_int = SQLITE_IOERR      | (17 << 8);
pub const SQLITE_IOERR_SHMOPEN:           c_int = SQLITE_IOERR      | (18 << 8);
pub const SQLITE_IOERR_SHMSIZE:           c_int = SQLITE_IOERR      | (19 << 8);
pub const SQLITE_IOERR_SHMLOCK:           c_int = SQLITE_IOERR      | (20 << 8);
pub const SQLITE_IOERR_SHMMAP:            c_int = SQLITE_IOERR      | (21 << 8);
pub const SQLITE_IOERR_SEEK:              c_int = SQLITE_IOERR      | (22 << 8);
pub const SQLITE_IOERR_DELETE_NOENT:      c_int = SQLITE_IOERR      | (23 << 8);
pub const SQLITE_IOERR_MMAP:              c_int = SQLITE_IOERR      | (24 << 8);
pub const SQLITE_IOERR_GETTEMPPATH:       c_int = SQLITE_IOERR      | (25 << 8);
pub const SQLITE_IOERR_CONVPATH:          c_int = SQLITE_IOERR      | (26 << 8);
pub const SQLITE_IOERR_VNODE:             c_int = SQLITE_IOERR      | (27 << 8);
pub const SQLITE_IOERR_AUTH:              c_int = SQLITE_IOERR      | (28 << 8);
pub const SQLITE_IOERR_BEGIN_ATOMIC:      c_int = SQLITE_IOERR      | (29 << 8);
pub const SQLITE_IOERR_COMMIT_ATOMIC:     c_int = SQLITE_IOERR      | (30 << 8);
pub const SQLITE_IOERR_ROLLBACK_ATOMIC:   c_int = SQLITE_IOERR      | (31 << 8);
pub const SQLITE_IOERR_DATA:              c_int = SQLITE_IOERR      | (32 << 8);
pub const SQLITE_IOERR_CORRUPTFS:         c_int = SQLITE_IOERR      | (33 << 8);
pub const SQLITE_IOERR_IN_PAGE:           c_int = SQLITE_IOERR      | (34 << 8);

pub const SQLITE_LOCKED_SHAREDCACHE:      c_int = SQLITE_LOCKED     | ( 1 << 8);
pub const SQLITE_LOCKED_VTAB:             c_int = SQLITE_LOCKED     | ( 2 << 8);

pub const SQLITE_BUSY_RECOVERY:           c_int = SQLITE_BUSY       | ( 1 << 8);
pub const SQLITE_BUSY_SNAPSHOT:           c_int = SQLITE_BUSY       | ( 2 << 8);
pub const SQLITE_BUSY_TIMEOUT:            c_int = SQLITE_BUSY       | ( 3 << 8);

pub const SQLITE_CANTOPEN_NOTEMPDIR:      c_int = SQLITE_CANTOPEN   | ( 1 << 8);
pub const SQLITE_CANTOPEN_ISDIR:          c_int = SQLITE_CANTOPEN   | ( 2 << 8);
pub const SQLITE_CANTOPEN_FULLPATH:       c_int = SQLITE_CANTOPEN   | ( 3 << 8);
pub const SQLITE_CANTOPEN_CONVPATH:       c_int = SQLITE_CANTOPEN   | ( 4 << 8);
pub const SQLITE_CANTOPEN_DIRTYWAL:       c_int = SQLITE_CANTOPEN   | ( 5 << 8);
pub const SQLITE_CANTOPEN_SYMLINK:        c_int = SQLITE_CANTOPEN   | ( 6 << 8);

pub const SQLITE_CORRUPT_VTAB:            c_int = SQLITE_CORRUPT    | ( 1 << 8);
pub const SQLITE_CORRUPT_SEQUENCE:        c_int = SQLITE_CORRUPT    | ( 2 << 8);
pub const SQLITE_CORRUPT_INDEX:           c_int = SQLITE_CORRUPT    | ( 3 << 8);

pub const SQLITE_READONLY_RECOVERY:       c_int = SQLITE_READONLY   | ( 1 << 8);
pub const SQLITE_READONLY_CANTLOCK:       c_int = SQLITE_READONLY   | ( 2 << 8);
pub const SQLITE_READONLY_ROLLBACK:       c_int = SQLITE_READONLY   | ( 3 << 8);
pub const SQLITE_READONLY_DBMOVED:        c_int = SQLITE_READONLY   | ( 4 << 8);
pub const SQLITE_READONLY_CANTINIT:       c_int = SQLITE_READONLY   | ( 5 << 8);
pub const SQLITE_READONLY_DIRECTORY:      c_int = SQLITE_READONLY   | ( 6 << 8);

pub const SQLITE_ABORT_ROLLBACK:          c_int = SQLITE_ABORT      | ( 2 << 8);

pub const SQLITE_CONSTRAINT_CHECK:        c_int = SQLITE_CONSTRAINT | ( 1 << 8);
pub const SQLITE_CONSTRAINT_COMMITHOOK:   c_int = SQLITE_CONSTRAINT | ( 2 << 8);
pub const SQLITE_CONSTRAINT_FOREIGNKEY:   c_int = SQLITE_CONSTRAINT | ( 3 << 8);
pub const SQLITE_CONSTRAINT_FUNCTION:     c_int = SQLITE_CONSTRAINT | ( 4 << 8);
pub const SQLITE_CONSTRAINT_NOTNULL:      c_int = SQLITE_CONSTRAINT | ( 5 << 8);
pub const SQLITE_CONSTRAINT_PRIMARYKEY:   c_int = SQLITE_CONSTRAINT | ( 6 << 8);
pub const SQLITE_CONSTRAINT_TRIGGER:      c_int = SQLITE_CONSTRAINT | ( 7 << 8);
pub const SQLITE_CONSTRAINT_UNIQUE:       c_int = SQLITE_CONSTRAINT | ( 8 << 8);
pub const SQLITE_CONSTRAINT_VTAB:         c_int = SQLITE_CONSTRAINT | ( 9 << 8);
pub const SQLITE_CONSTRAINT_ROWID:        c_int = SQLITE_CONSTRAINT | (10 << 8);
pub const SQLITE_CONSTRAINT_PINNED:       c_int = SQLITE_CONSTRAINT | (11 << 8);
pub const SQLITE_CONSTRAINT_DATATYPE:     c_int = SQLITE_CONSTRAINT | (12 << 8);

pub const SQLITE_NOTICE_RECOVER_WAL:      c_int = SQLITE_NOTICE     | ( 1 << 8);
pub const SQLITE_NOTICE_RECOVER_ROLLBACK: c_int = SQLITE_NOTICE     | ( 2 << 8);
pub const SQLITE_NOTICE_RBU:              c_int = SQLITE_NOTICE     | ( 3 << 8);

pub const SQLITE_WARNING_AUTOINDEX:       c_int = SQLITE_WARNING    | ( 1 << 8);

pub const SQLITE_AUTH_USER:               c_int = SQLITE_AUTH       | ( 1 << 8);

pub const SQLITE_OK_LOAD_PERMANENTLY:     c_int = SQLITE_OK         | ( 1 << 8);
pub const SQLITE_OK_SYMLINK:              c_int = SQLITE_OK         | ( 2 << 8);

pub const SQLITE_OPEN_READONLY:       c_int = 0x00000001;
pub const SQLITE_OPEN_READWRITE:      c_int = 0x00000002;
pub const SQLITE_OPEN_CREATE:         c_int = 0x00000004;
pub const SQLITE_OPEN_DELETEONCLOSE:  c_int = 0x00000008;
pub const SQLITE_OPEN_EXCLUSIVE:      c_int = 0x00000010;
pub const SQLITE_OPEN_AUTOPROXY:      c_int = 0x00000020;
pub const SQLITE_OPEN_URI:            c_int = 0x00000040;
pub const SQLITE_OPEN_MEMORY:         c_int = 0x00000080;
pub const SQLITE_OPEN_MAIN_DB:        c_int = 0x00000100;
pub const SQLITE_OPEN_TEMP_DB:        c_int = 0x00000200;
pub const SQLITE_OPEN_TRANSIENT_DB:   c_int = 0x00000400;
pub const SQLITE_OPEN_MAIN_JOURNAL:   c_int = 0x00000800;
pub const SQLITE_OPEN_TEMP_JOURNAL:   c_int = 0x00001000;
pub const SQLITE_OPEN_SUBJOURNAL:     c_int = 0x00002000;
pub const SQLITE_OPEN_MASTER_JOURNAL: c_int = 0x00004000;
pub const SQLITE_OPEN_NOMUTEX:        c_int = 0x00008000;
pub const SQLITE_OPEN_FULLMUTEX:      c_int = 0x00010000;
pub const SQLITE_OPEN_SHAREDCACHE:    c_int = 0x00020000;
pub const SQLITE_OPEN_PRIVATECACHE:   c_int = 0x00040000;
pub const SQLITE_OPEN_WAL:            c_int = 0x00080000;
pub const SQLITE_OPEN_NOFOLLOW:       c_int = 0x01000000;
pub const SQLITE_OPEN_EXRESCODE:      c_int = 0x02000000;

pub const SQLITE_IOCAP_ATOMIC:                c_int = 0x00000001;
pub const SQLITE_IOCAP_ATOMIC512:             c_int = 0x00000002;
pub const SQLITE_IOCAP_ATOMIC1K:              c_int = 0x00000004;
pub const SQLITE_IOCAP_ATOMIC2K:              c_int = 0x00000008;
pub const SQLITE_IOCAP_ATOMIC4K:              c_int = 0x00000010;
pub const SQLITE_IOCAP_ATOMIC8K:              c_int = 0x00000020;
pub const SQLITE_IOCAP_ATOMIC16K:             c_int = 0x00000040;
pub const SQLITE_IOCAP_ATOMIC32K:             c_int = 0x00000080;
pub const SQLITE_IOCAP_ATOMIC64K:             c_int = 0x00000100;
pub const SQLITE_IOCAP_SAFE_APPEND:           c_int = 0x00000200;
pub const SQLITE_IOCAP_SEQUENTIAL:            c_int = 0x00000400;
pub const SQLITE_IOCAP_UNDELETABLE_WHEN_OPEN: c_int = 0x00000800;
pub const SQLITE_IOCAP_POWERSAFE_OVERWRITE:   c_int = 0x00001000;
pub const SQLITE_IOCAP_IMMUTABLE:             c_int = 0x00002000;
pub const SQLITE_IOCAP_BATCH_ATOMIC:          c_int = 0x00004000;

pub const SQLITE_LOCK_NONE:      c_int = 0;
pub const SQLITE_LOCK_SHARED:    c_int = 1;
pub const SQLITE_LOCK_RESERVED:  c_int = 2;
pub const SQLITE_LOCK_PENDING:   c_int = 3;
pub const SQLITE_LOCK_EXCLUSIVE: c_int = 4;

pub const SQLITE_SYNC_NORMAL:   c_int = 0x00002;
pub const SQLITE_SYNC_FULL:     c_int = 0x00003;
pub const SQLITE_SYNC_DATAONLY: c_int = 0x00010;

pub const SQLITE_FCNTL_LOCKSTATE:             c_int =  1;
pub const SQLITE_FCNTL_GET_LOCKPROXYFILE:     c_int =  2;
pub const SQLITE_FCNTL_SET_LOCKPROXYFILE:     c_int =  3;
pub const SQLITE_FCNTL_LAST_ERRNO:            c_int =  4;
pub const SQLITE_FCNTL_SIZE_HINT:             c_int =  5;
pub const SQLITE_FCNTL_CHUNK_SIZE:            c_int =  6;
pub const SQLITE_FCNTL_FILE_POINTER:          c_int =  7;
pub const SQLITE_FCNTL_SYNC_OMITTED:          c_int =  8;
pub const SQLITE_FCNTL_WIN32_AV_RETRY:        c_int =  9;
pub const SQLITE_FCNTL_PERSIST_WAL:           c_int = 10;
pub const SQLITE_FCNTL_OVERWRITE:             c_int = 11;
pub const SQLITE_FCNTL_VFSNAME:               c_int = 12;
pub const SQLITE_FCNTL_POWERSAFE_OVERWRITE:   c_int = 13;
pub const SQLITE_FCNTL_PRAGMA:                c_int = 14;
pub const SQLITE_FCNTL_BUSYHANDLER:           c_int = 15;
pub const SQLITE_FCNTL_TEMPFILENAME:          c_int = 16;
pub const SQLITE_FCNTL_MMAP_SIZE:             c_int = 18;
pub const SQLITE_FCNTL_TRACE:                 c_int = 19;
pub const SQLITE_FCNTL_HAS_MOVED:             c_int = 20;
pub const SQLITE_FCNTL_SYNC:                  c_int = 21;
pub const SQLITE_FCNTL_COMMIT_PHASETWO:       c_int = 22;
pub const SQLITE_FCNTL_WIN32_SET_HANDLE:      c_int = 23;
pub const SQLITE_FCNTL_WAL_BLOCK:             c_int = 24;
pub const SQLITE_FCNTL_ZIPVFS:                c_int = 25;
pub const SQLITE_FCNTL_RBU:                   c_int = 26;
pub const SQLITE_FCNTL_VFS_POINTER:           c_int = 27;
pub const SQLITE_FCNTL_JOURNAL_POINTER:       c_int = 28;
pub const SQLITE_FCNTL_WIN32_GET_HANDLE:      c_int = 29;
pub const SQLITE_FCNTL_PDB:                   c_int = 30;
pub const SQLITE_FCNTL_BEGIN_ATOMIC_WRITE:    c_int = 31;
pub const SQLITE_FCNTL_COMMIT_ATOMIC_WRITE:   c_int = 32;
pub const SQLITE_FCNTL_ROLLBACK_ATOMIC_WRITE: c_int = 33;
pub const SQLITE_FCNTL_LOCK_TIMEOUT:          c_int = 34;
pub const SQLITE_FCNTL_DATA_VERSION:          c_int = 35;
pub const SQLITE_FCNTL_SIZE_LIMIT:            c_int = 36;
pub const SQLITE_FCNTL_CKPT_DONE:             c_int = 37;
pub const SQLITE_FCNTL_RESERVE_BYTES:         c_int = 38;
pub const SQLITE_FCNTL_CKPT_START:            c_int = 39;
pub const SQLITE_FCNTL_EXTERNAL_READER:       c_int = 40;
pub const SQLITE_FCNTL_CKSM_FILE:             c_int = 41;
pub const SQLITE_FCNTL_RESET_CACHE:           c_int = 42;

pub const SQLITE_ACCESS_EXISTS:    c_int = 0;
pub const SQLITE_ACCESS_READWRITE: c_int = 1;
pub const SQLITE_ACCESS_READ:      c_int = 2;

pub const SQLITE_SHM_UNLOCK:    c_int = 1;
pub const SQLITE_SHM_LOCK:      c_int = 2;
pub const SQLITE_SHM_SHARED:    c_int = 4;
pub const SQLITE_SHM_EXCLUSIVE: c_int = 8;
pub const SQLITE_SHM_NLOCK:     c_int = 8;

pub const SQLITE_CONFIG_SINGLETHREAD:        c_int =  1;
pub const SQLITE_CONFIG_MULTITHREAD:         c_int =  2;
pub const SQLITE_CONFIG_SERIALIZED:          c_int =  3;
pub const SQLITE_CONFIG_MALLOC:              c_int =  4;
pub const SQLITE_CONFIG_GETMALLOC:           c_int =  5;
pub const SQLITE_CONFIG_SCRATCH:             c_int =  6;
pub const SQLITE_CONFIG_PAGECACHE:           c_int =  7;
pub const SQLITE_CONFIG_HEAP:                c_int =  8;
pub const SQLITE_CONFIG_MEMSTATUS:           c_int =  9;
pub const SQLITE_CONFIG_MUTEX:               c_int = 10;
pub const SQLITE_CONFIG_GETMUTEX:            c_int = 11;
pub const SQLITE_CONFIG_LOOKASIDE:           c_int = 13;
pub const SQLITE_CONFIG_PCACHE:              c_int = 14;
pub const SQLITE_CONFIG_GETPCACHE:           c_int = 15;
pub const SQLITE_CONFIG_LOG:                 c_int = 16;
pub const SQLITE_CONFIG_URI:                 c_int = 17;
pub const SQLITE_CONFIG_PCACHE2:             c_int = 18;
pub const SQLITE_CONFIG_GETPCACHE2:          c_int = 19;
pub const SQLITE_CONFIG_COVERING_INDEX_SCAN: c_int = 20;
pub const SQLITE_CONFIG_SQLLOG:              c_int = 21;
pub const SQLITE_CONFIG_MMAP_SIZE:           c_int = 22;
pub const SQLITE_CONFIG_WIN32_HEAPSIZE:      c_int = 23;
pub const SQLITE_CONFIG_PCACHE_HDRSZ:        c_int = 24;
pub const SQLITE_CONFIG_PMASZ:               c_int = 25;
pub const SQLITE_CONFIG_STMTJRNL_SPILL:      c_int = 26;
pub const SQLITE_CONFIG_SMALL_MALLOC:        c_int = 27;
pub const SQLITE_CONFIG_SORTERREF_SIZE:      c_int = 28;
pub const SQLITE_CONFIG_MEMDB_MAXSIZE:       c_int = 29;
pub const SQLITE_CONFIG_ROWID_IN_VIEW:       c_int = 30;

pub const SQLITE_DBCONFIG_MAINDBNAME:            c_int = 1000;
pub const SQLITE_DBCONFIG_LOOKASIDE:             c_int = 1001;
pub const SQLITE_DBCONFIG_ENABLE_FKEY:           c_int = 1002;
pub const SQLITE_DBCONFIG_ENABLE_TRIGGER:        c_int = 1003;
pub const SQLITE_DBCONFIG_ENABLE_FTS3_TOKENIZER: c_int = 1004;
pub const SQLITE_DBCONFIG_ENABLE_LOAD_EXTENSION: c_int = 1005;
pub const SQLITE_DBCONFIG_NO_CKPT_ON_CLOSE:      c_int = 1006;
pub const SQLITE_DBCONFIG_ENABLE_QPSG:           c_int = 1007;
pub const SQLITE_DBCONFIG_TRIGGER_EQP:           c_int = 1008;
pub const SQLITE_DBCONFIG_RESET_DATABASE:        c_int = 1009;
pub const SQLITE_DBCONFIG_DEFENSIVE:             c_int = 1010;
pub const SQLITE_DBCONFIG_WRITABLE_SCHEMA:       c_int = 1011;
pub const SQLITE_DBCONFIG_LEGACY_ALTER_TABLE:    c_int = 1012;
pub const SQLITE_DBCONFIG_DQS_DML:               c_int = 1013;
pub const SQLITE_DBCONFIG_DQS_DDL:               c_int = 1014;
pub const SQLITE_DBCONFIG_ENABLE_VIEW:           c_int = 1015;
pub const SQLITE_DBCONFIG_LEGACY_FILE_FORMAT:    c_int = 1016;
pub const SQLITE_DBCONFIG_TRUSTED_SCHEMA:        c_int = 1017;
pub const SQLITE_DBCONFIG_STMT_SCANSTATUS:       c_int = 1018;
pub const SQLITE_DBCONFIG_REVERSE_SCANORDER:     c_int = 1019;
pub const SQLITE_DBCONFIG_MAX:                   c_int = 1019;

pub const SQLITE_DENY:   c_int = 1;
pub const SQLITE_IGNORE: c_int = 2;

pub const SQLITE_CREATE_INDEX:        c_int =  1;
pub const SQLITE_CREATE_TABLE:        c_int =  2;
pub const SQLITE_CREATE_TEMP_INDEX:   c_int =  3;
pub const SQLITE_CREATE_TEMP_TABLE:   c_int =  4;
pub const SQLITE_CREATE_TEMP_TRIGGER: c_int =  5;
pub const SQLITE_CREATE_TEMP_VIEW:    c_int =  6;
pub const SQLITE_CREATE_TRIGGER:      c_int =  7;
pub const SQLITE_CREATE_VIEW:         c_int =  8;
pub const SQLITE_DELETE:              c_int =  9;
pub const SQLITE_DROP_INDEX:          c_int = 10;
pub const SQLITE_DROP_TABLE:          c_int = 11;
pub const SQLITE_DROP_TEMP_INDEX:     c_int = 12;
pub const SQLITE_DROP_TEMP_TABLE:     c_int = 13;
pub const SQLITE_DROP_TEMP_TRIGGER:   c_int = 14;
pub const SQLITE_DROP_TEMP_VIEW:      c_int = 15;
pub const SQLITE_DROP_TRIGGER:        c_int = 16;
pub const SQLITE_DROP_VIEW:           c_int = 17;
pub const SQLITE_INSERT:              c_int = 18;
pub const SQLITE_PRAGMA:              c_int = 19;
pub const SQLITE_READ:                c_int = 20;
pub const SQLITE_SELECT:              c_int = 21;
pub const SQLITE_TRANSACTION:         c_int = 22;
pub const SQLITE_UPDATE:              c_int = 23;
pub const SQLITE_ATTACH:              c_int = 24;
pub const SQLITE_DETACH:              c_int = 25;
pub const SQLITE_ALTER_TABLE:         c_int = 26;
pub const SQLITE_REINDEX:             c_int = 27;
pub const SQLITE_ANALYZE:             c_int = 28;
pub const SQLITE_CREATE_VTABLE:       c_int = 29;
pub const SQLITE_DROP_VTABLE:         c_int = 30;
pub const SQLITE_FUNCTION:            c_int = 31;
pub const SQLITE_SAVEPOINT:           c_int = 32;
pub const SQLITE_COPY:                c_int =  0;
pub const SQLITE_RECURSIVE:           c_int = 33;

pub const SQLITE_TRACE_STMT:    c_int = 0x01;
pub const SQLITE_TRACE_PROFILE: c_int = 0x02;
pub const SQLITE_TRACE_ROW:     c_int = 0x04;
pub const SQLITE_TRACE_CLOSE:   c_int = 0x08;

pub const SQLITE_LIMIT_LENGTH:              c_int =  0;
pub const SQLITE_LIMIT_SQL_LENGTH:          c_int =  1;
pub const SQLITE_LIMIT_COLUMN:              c_int =  2;
pub const SQLITE_LIMIT_EXPR_DEPTH:          c_int =  3;
pub const SQLITE_LIMIT_COMPOUND_SELECT:     c_int =  4;
pub const SQLITE_LIMIT_VDBE_OP:             c_int =  5;
pub const SQLITE_LIMIT_FUNCTION_ARG:        c_int =  6;
pub const SQLITE_LIMIT_ATTACHED:            c_int =  7;
pub const SQLITE_LIMIT_LIKE_PATTERN_LENGTH: c_int =  8;
pub const SQLITE_LIMIT_VARIABLE_NUMBER:     c_int =  9;
pub const SQLITE_LIMIT_TRIGGER_DEPTH:       c_int = 10;
pub const SQLITE_LIMIT_WORKER_THREADS:      c_int = 11;

pub const SQLITE_PREPARE_PERSISTENT: c_int = 0x01;
pub const SQLITE_PREPARE_NORMALIZE:  c_int = 0x02;
pub const SQLITE_PREPARE_NO_VTAB:    c_int = 0x04;

pub const SQLITE_INTEGER: c_int = 1;
pub const SQLITE_FLOAT:   c_int = 2;
pub const SQLITE_TEXT:    c_int = 3;
pub const SQLITE_BLOB:    c_int = 4;
pub const SQLITE_NULL:    c_int = 5;

pub const SQLITE_UTF8:          c_int = 1;
pub const SQLITE_UTF16LE:       c_int = 2;
pub const SQLITE_UTF16BE:       c_int = 3;
pub const SQLITE_UTF16:         c_int = 4;
pub const SQLITE_ANY:           c_int = 5;
pub const SQLITE_UTF16_ALIGNED: c_int = 8;

pub const SQLITE_DETERMINISTIC: c_int = 0x000000800;
pub const SQLITE_DIRECTONLY:    c_int = 0x000080000;
pub const SQLITE_SUBTYPE:       c_int = 0x000100000;
pub const SQLITE_INNOCUOUS:     c_int = 0x000200000;
pub const SQLITE_RESULT_SUBTYPE:c_int = 0x001000000;

pub const SQLITE_STATIC:    c_int =  0;
pub const SQLITE_TRANSIENT: c_int = -1;

pub const SQLITE_WIN32_DATA_DIRECTORY_TYPE: c_int = 1;
pub const SQLITE_WIN32_TEMP_DIRECTORY_TYPE: c_int = 2;

pub const SQLITE_TXN_NONE:  c_int = 0;
pub const SQLITE_TXN_READ:  c_int = 1;
pub const SQLITE_TXN_WRITE: c_int = 2;

pub const SQLITE_INDEX_SCAN_UNIQUE:          c_int =   1;
pub const SQLITE_INDEX_CONSTRAINT_EQ:        c_int =   2;
pub const SQLITE_INDEX_CONSTRAINT_GT:        c_int =   4;
pub const SQLITE_INDEX_CONSTRAINT_LE:        c_int =   8;
pub const SQLITE_INDEX_CONSTRAINT_LT:        c_int =  16;
pub const SQLITE_INDEX_CONSTRAINT_GE:        c_int =  32;
pub const SQLITE_INDEX_CONSTRAINT_MATCH:     c_int =  64;
pub const SQLITE_INDEX_CONSTRAINT_LIKE:      c_int =  65;
pub const SQLITE_INDEX_CONSTRAINT_GLOB:      c_int =  66;
pub const SQLITE_INDEX_CONSTRAINT_REGEXP:    c_int =  67;
pub const SQLITE_INDEX_CONSTRAINT_NE:        c_int =  68;
pub const SQLITE_INDEX_CONSTRAINT_ISNOT:     c_int =  69;
pub const SQLITE_INDEX_CONSTRAINT_ISNOTNULL: c_int =  70;
pub const SQLITE_INDEX_CONSTRAINT_ISNULL:    c_int =  71;
pub const SQLITE_INDEX_CONSTRAINT_IS:        c_int =  72;
pub const SQLITE_INDEX_CONSTRAINT_LIMIT:     c_int =  73;
pub const SQLITE_INDEX_CONSTRAINT_OFFSET:    c_int =  74;
pub const SQLITE_INDEX_CONSTRAINT_FUNCTION:  c_int = 150;

pub const SQLITE_MUTEX_FAST:          c_int =  0;
pub const SQLITE_MUTEX_RECURSIVE:     c_int =  1;
pub const SQLITE_MUTEX_STATIC_MAIN:   c_int =  2;
pub const SQLITE_MUTEX_STATIC_MASTER: c_int =  2;
pub const SQLITE_MUTEX_STATIC_MEM:    c_int =  3;
pub const SQLITE_MUTEX_STATIC_MEM2:   c_int =  4;
pub const SQLITE_MUTEX_STATIC_OPEN:   c_int =  4;
pub const SQLITE_MUTEX_STATIC_PRNG:   c_int =  5;
pub const SQLITE_MUTEX_STATIC_LRU:    c_int =  6;
pub const SQLITE_MUTEX_STATIC_LRU2:   c_int =  7;
pub const SQLITE_MUTEX_STATIC_PMEM:   c_int =  7;
pub const SQLITE_MUTEX_STATIC_APP1:   c_int =  8;
pub const SQLITE_MUTEX_STATIC_APP2:   c_int =  9;
pub const SQLITE_MUTEX_STATIC_APP3:   c_int = 10;
pub const SQLITE_MUTEX_STATIC_VFS1:   c_int = 11;
pub const SQLITE_MUTEX_STATIC_VFS2:   c_int = 12;
pub const SQLITE_MUTEX_STATIC_VFS3:   c_int = 13;

pub const SQLITE_TESTCTRL_FIRST:                c_int =  5;
pub const SQLITE_TESTCTRL_PRNG_SAVE:            c_int =  5;
pub const SQLITE_TESTCTRL_PRNG_RESTORE:         c_int =  6;
pub const SQLITE_TESTCTRL_PRNG_RESET:           c_int =  7;
pub const SQLITE_TESTCTRL_FK_NO_ACTION:         c_int =  7;
pub const SQLITE_TESTCTRL_BITVEC_TEST:          c_int =  8;
pub const SQLITE_TESTCTRL_FAULT_INSTALL:        c_int =  9;
pub const SQLITE_TESTCTRL_BENIGN_MALLOC_HOOKS:  c_int = 10;
pub const SQLITE_TESTCTRL_PENDING_BYTE:         c_int = 11;
pub const SQLITE_TESTCTRL_ASSERT:               c_int = 12;
pub const SQLITE_TESTCTRL_ALWAYS:               c_int = 13;
pub const SQLITE_TESTCTRL_RESERVE:              c_int = 14;
pub const SQLITE_TESTCTRL_JSON_SELFCHECK:       c_int = 14;
pub const SQLITE_TESTCTRL_OPTIMIZATIONS:        c_int = 15;
pub const SQLITE_TESTCTRL_ISKEYWORD:            c_int = 16;
pub const SQLITE_TESTCTRL_SCRATCHMALLOC:        c_int = 17;
pub const SQLITE_TESTCTRL_INTERNAL_FUNCTIONS:   c_int = 17;
pub const SQLITE_TESTCTRL_LOCALTIME_FAULT:      c_int = 18;
pub const SQLITE_TESTCTRL_EXPLAIN_STMT:         c_int = 19;
pub const SQLITE_TESTCTRL_ONCE_RESET_THRESHOLD: c_int = 19;
pub const SQLITE_TESTCTRL_NEVER_CORRUPT:        c_int = 20;
pub const SQLITE_TESTCTRL_VDBE_COVERAGE:        c_int = 21;
pub const SQLITE_TESTCTRL_BYTEORDER:            c_int = 22;
pub const SQLITE_TESTCTRL_ISINIT:               c_int = 23;
pub const SQLITE_TESTCTRL_SORTER_MMAP:          c_int = 24;
pub const SQLITE_TESTCTRL_IMPOSTER:             c_int = 25;
pub const SQLITE_TESTCTRL_PARSER_COVERAGE:      c_int = 26;
pub const SQLITE_TESTCTRL_RESULT_INTREAL:       c_int = 27;
pub const SQLITE_TESTCTRL_PRNG_SEED:            c_int = 28;
pub const SQLITE_TESTCTRL_EXTRA_SCHEMA_CHECKS:  c_int = 29;
pub const SQLITE_TESTCTRL_SEEK_COUNT:           c_int = 30;
pub const SQLITE_TESTCTRL_TRACEFLAGS:           c_int = 31;
pub const SQLITE_TESTCTRL_TUNE:                 c_int = 32;
pub const SQLITE_TESTCTRL_LOGEST:               c_int = 33;
pub const SQLITE_TESTCTRL_USELONGDOUBLE:        c_int = 34;
pub const SQLITE_TESTCTRL_LAST:                 c_int = 34;

pub const SQLITE_STATUS_MEMORY_USED:        c_int = 0;
pub const SQLITE_STATUS_PAGECACHE_USED:     c_int = 1;
pub const SQLITE_STATUS_PAGECACHE_OVERFLOW: c_int = 2;
pub const SQLITE_STATUS_SCRATCH_USED:       c_int = 3;
pub const SQLITE_STATUS_SCRATCH_OVERFLOW:   c_int = 4;
pub const SQLITE_STATUS_MALLOC_SIZE:        c_int = 5;
pub const SQLITE_STATUS_PARSER_STACK:       c_int = 6;
pub const SQLITE_STATUS_PAGECACHE_SIZE:     c_int = 7;
pub const SQLITE_STATUS_SCRATCH_SIZE:       c_int = 8;
pub const SQLITE_STATUS_MALLOC_COUNT:       c_int = 9;

pub const SQLITE_DBSTATUS_LOOKASIDE_USED:      c_int =  0;
pub const SQLITE_DBSTATUS_CACHE_USED:          c_int =  1;
pub const SQLITE_DBSTATUS_SCHEMA_USED:         c_int =  2;
pub const SQLITE_DBSTATUS_STMT_USED:           c_int =  3;
pub const SQLITE_DBSTATUS_LOOKASIDE_HIT:       c_int =  4;
pub const SQLITE_DBSTATUS_LOOKASIDE_MISS_SIZE: c_int =  5;
pub const SQLITE_DBSTATUS_LOOKASIDE_MISS_FULL: c_int =  6;
pub const SQLITE_DBSTATUS_CACHE_HIT:           c_int =  7;
pub const SQLITE_DBSTATUS_CACHE_MISS:          c_int =  8;
pub const SQLITE_DBSTATUS_CACHE_WRITE:         c_int =  9;
pub const SQLITE_DBSTATUS_DEFERRED_FKS:        c_int = 10;
pub const SQLITE_DBSTATUS_CACHE_USED_SHARED:   c_int = 11;
pub const SQLITE_DBSTATUS_CACHE_SPILL:         c_int = 12;
pub const SQLITE_DBSTATUS_MAX:                 c_int = 12;

pub const SQLITE_STMTSTATUS_FULLSCAN_STEP: c_int =  1;
pub const SQLITE_STMTSTATUS_SORT:          c_int =  2;
pub const SQLITE_STMTSTATUS_AUTOINDEX:     c_int =  3;
pub const SQLITE_STMTSTATUS_VM_STEP:       c_int =  4;
pub const SQLITE_STMTSTATUS_REPREPARE:     c_int =  5;
pub const SQLITE_STMTSTATUS_RUN:           c_int =  6;
pub const SQLITE_STMTSTATUS_FILTER_MISS:   c_int =  7;
pub const SQLITE_STMTSTATUS_FILTER_HIT:    c_int =  8;
pub const SQLITE_STMTSTATUS_MEMUSED:       c_int = 99;

pub const SQLITE_CHECKPOINT_PASSIVE:  c_int = 0;
pub const SQLITE_CHECKPOINT_FULL:     c_int = 1;
pub const SQLITE_CHECKPOINT_RESTART:  c_int = 2;
pub const SQLITE_CHECKPOINT_TRUNCATE: c_int = 3;

pub const SQLITE_VTAB_CONSTRAINT_SUPPORT: c_int = 1;
pub const SQLITE_VTAB_INNOCUOUS:          c_int = 2;
pub const SQLITE_VTAB_DIRECTONLY:         c_int = 3;
pub const SQLITE_VTAB_USES_ALL_SCHEMAS:   c_int = 4;

pub const SQLITE_ROLLBACK: c_int = 1;
pub const SQLITE_FAIL:     c_int = 3;
pub const SQLITE_REPLACE:  c_int = 5;

pub const SQLITE_SCANSTAT_NLOOP:    c_int = 0;
pub const SQLITE_SCANSTAT_NVISIT:   c_int = 1;
pub const SQLITE_SCANSTAT_EST:      c_int = 2;
pub const SQLITE_SCANSTAT_NAME:     c_int = 3;
pub const SQLITE_SCANSTAT_EXPLAIN:  c_int = 4;
pub const SQLITE_SCANSTAT_SELECTID: c_int = 5;
pub const SQLITE_SCANSTAT_PARENTID: c_int = 6;
pub const SQLITE_SCANSTAT_NCYCLE:   c_int = 7;

pub const SQLITE_SCANSTAT_COMPLEX: c_int = 0x0001;

pub const SQLITE_SERIALIZE_NOCOPY: c_int = 0x001;

pub const SQLITE_DESERIALIZE_FREEONCLOSE: c_int = 1;
pub const SQLITE_DESERIALIZE_RESIZEABLE:  c_int = 2;
pub const SQLITE_DESERIALIZE_READONLY:    c_int = 4;

pub const SQLITE_SESSION_OBJCONFIG_SIZE:  c_int = 1;
pub const SQLITE_SESSION_OBJCONFIG_ROWID: c_int = 2;

pub const SQLITE_CHANGESETSTART_INVERT: c_int = 0x0002;

pub const SQLITE_CHANGESETAPPLY_NOSAVEPOINT: c_int = 0x0001;
pub const SQLITE_CHANGESETAPPLY_INVERT:      c_int = 0x0002;
pub const SQLITE_CHANGESETAPPLY_IGNORENOOP:  c_int = 0x0004;
pub const SQLITE_CHANGESETAPPLY_FKNOACTION:  c_int = 0x0008;

pub const SQLITE_CHANGESET_DATA:        c_int = 1;
pub const SQLITE_CHANGESET_NOTFOUND:    c_int = 2;
pub const SQLITE_CHANGESET_CONFLICT:    c_int = 3;
pub const SQLITE_CHANGESET_CONSTRAINT:  c_int = 4;
pub const SQLITE_CHANGESET_FOREIGN_KEY: c_int = 5;

pub const SQLITE_CHANGESET_OMIT:    c_int = 0;
pub const SQLITE_CHANGESET_REPLACE: c_int = 1;
pub const SQLITE_CHANGESET_ABORT:   c_int = 2;

pub const SQLITE_SESSION_CONFIG_STRMSIZE: c_int = 1;
