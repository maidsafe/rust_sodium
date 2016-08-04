// utils.h

extern {
    pub fn sodium_memzero(pnt: *mut c_void, len: size_t);
    pub fn sodium_memcmp(b1_: *const c_void, b2_: *const c_void, len: size_t) -> c_int;
    pub fn sodium_increment(n: *mut u8, len: size_t);

    pub fn sodium_mlock(addr: *mut c_void, len: size_t) -> c_int;
    pub fn sodium_munlock(addr: *mut c_void, len: size_t) -> c_int;

    pub fn sodium_malloc(len: size_t) -> *mut c_void;
    pub fn sodium_allocarray(count: size_t, size: size_t) -> *mut c_void;
    pub fn sodium_free(ptr: *mut c_void);

    pub fn sodium_mprotect_noaccess(ptr: *mut c_void) -> c_int;
    pub fn sodium_mprotect_readonly(ptr: *mut c_void) -> c_int;
    pub fn sodium_mprotect_readwrite(ptr: *mut c_void) -> c_int;
}
