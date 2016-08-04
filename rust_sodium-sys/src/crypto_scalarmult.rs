// crypto_scalarmult.h

pub const crypto_scalarmult_BYTES: usize = crypto_scalarmult_curve25519_BYTES;
pub const crypto_scalarmult_SCALARBYTES: usize = crypto_scalarmult_curve25519_SCALARBYTES;
pub const crypto_scalarmult_PRIMITIVE: *const c_char = (b"curve25519\0" as *const u8) as *const c_char;

extern {
    pub fn crypto_scalarmult_bytes() -> size_t;
    pub fn crypto_scalarmult_scalarbytes() -> size_t;
    pub fn crypto_scalarmult_primitive() -> *const c_char;
    pub fn crypto_scalarmult_base(
        q: *mut u8,
        n: *const u8) -> c_int;
    pub fn crypto_scalarmult(
        q: *mut u8,
        n: *const u8,
        p: *const u8) -> c_int;
}

#[test]
fn test_crypto_scalarmult_bytes() {
    assert_eq!(unsafe { crypto_scalarmult_bytes() as usize },
               crypto_scalarmult_BYTES);
}

#[test]
fn test_crypto_scalarmult_scalarbytes() {
    assert_eq!(unsafe { crypto_scalarmult_scalarbytes() as usize },
               crypto_scalarmult_SCALARBYTES);
}

#[test]
fn test_crypto_scalarmult_primitive() {
    use std::ffi::CStr;
    unsafe {
        assert_eq!(CStr::from_ptr(crypto_scalarmult_PRIMITIVE),
                   CStr::from_ptr(crypto_scalarmult_primitive()));
    }
}
