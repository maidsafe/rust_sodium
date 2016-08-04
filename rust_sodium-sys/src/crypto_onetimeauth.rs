// crypto_onetimeauth.h

pub const crypto_onetimeauth_BYTES: usize =
    crypto_onetimeauth_poly1305_BYTES;
pub const crypto_onetimeauth_KEYBYTES: usize =
    crypto_onetimeauth_poly1305_KEYBYTES;
pub const crypto_onetimeauth_PRIMITIVE: *const c_char = (b"poly1305\0" as *const u8) as *const c_char;


extern {
    pub fn crypto_onetimeauth_bytes() -> size_t;
    pub fn crypto_onetimeauth_keybytes() -> size_t;
    pub fn crypto_onetimeauth_primitive() -> *const c_char;
}


#[test]
fn test_crypto_onetimeauth_bytes() {
    assert!(unsafe { crypto_onetimeauth_bytes() as usize } ==
            crypto_onetimeauth_BYTES)
}
#[test]
fn test_crypto_onetimeauth_keybytes() {
    assert!(unsafe { crypto_onetimeauth_keybytes() as usize } ==
            crypto_onetimeauth_KEYBYTES)
}
#[test]
fn test_crypto_onetimeauth_primitive() {
    use std::ffi::CStr;
    unsafe {
        assert_eq!(CStr::from_ptr(crypto_onetimeauth_PRIMITIVE),
                   CStr::from_ptr(crypto_onetimeauth_primitive()));
    }
}
