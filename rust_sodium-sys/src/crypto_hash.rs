// crypto_hash.h

pub const crypto_hash_BYTES: usize = crypto_hash_sha512_BYTES;
pub const crypto_hash_PRIMITIVE: *const c_char = (b"sha512\0" as *const u8) as *const c_char;


extern {
    pub fn crypto_hash_bytes() -> size_t;
    pub fn crypto_hash(h: *mut u8,
                       m: *const u8,
                       mlen: c_ulonglong) -> c_int;
    pub fn crypto_hash_primitive() -> *const c_char;
}


#[test]
fn test_crypto_hash_bytes() {
    assert!(unsafe { crypto_hash_bytes() as usize } == crypto_hash_BYTES)
}
#[test]
fn test_crypto_hash_primitive() {
    use std::ffi::CStr;
    unsafe {
        assert_eq!(CStr::from_ptr(crypto_hash_PRIMITIVE), CStr::from_ptr(crypto_hash_primitive()));
    }
}
