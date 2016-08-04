// crypto_auth.h

pub const crypto_auth_BYTES: usize = crypto_auth_hmacsha512256_BYTES;
pub const crypto_auth_KEYBYTES: usize = crypto_auth_hmacsha512256_KEYBYTES;
pub const crypto_auth_PRIMITIVE: *const c_char = (b"hmacsha512256\0" as *const u8) as *const c_char;


extern {
    pub fn crypto_auth_bytes() -> size_t;
    pub fn crypto_auth_keybytes() -> size_t;
    pub fn crypto_auth_primitive() -> *const c_char;
    pub fn crypto_auth(a: *mut u8,
                       m: *const u8,
                       mlen: c_ulonglong,
                       k: *const u8) -> c_int;
    pub fn crypto_auth_verify(a: *const u8,
                              m: *const u8,
                              mlen: c_ulonglong,
                              k: *const u8) -> c_int;
}


#[test]
fn test_crypto_auth_bytes() {
    assert!(unsafe { crypto_auth_bytes() as usize } == crypto_auth_BYTES)
}
#[test]
fn test_crypto_auth_keybytes() {
    assert!(unsafe { crypto_auth_keybytes() as usize } ==
            crypto_auth_KEYBYTES)
}
#[test]
fn test_crypto_auth_primitive() {
    use std::ffi::CStr;
    unsafe {
        assert_eq!(CStr::from_ptr(crypto_auth_PRIMITIVE), CStr::from_ptr(crypto_auth_primitive()));
    }
}
