// crypto_stream.h

pub const crypto_stream_KEYBYTES: usize = crypto_stream_xsalsa20_KEYBYTES;
pub const crypto_stream_NONCEBYTES: usize =
    crypto_stream_xsalsa20_NONCEBYTES;
pub const crypto_stream_PRIMITIVE: *const c_char = (b"xsalsa20\0" as *const u8) as *const c_char;


extern {
    pub fn crypto_stream_keybytes() -> size_t;
    pub fn crypto_stream_noncebytes() -> size_t;
    pub fn crypto_stream_primitive() -> *const c_char;
}


#[test]
fn test_crypto_stream_keybytes() {
    assert!(unsafe { crypto_stream_keybytes() as usize } ==
            crypto_stream_KEYBYTES)
}
#[test]
fn test_crypto_stream_noncebytes() {
    assert!(unsafe { crypto_stream_noncebytes() as usize } ==
            crypto_stream_NONCEBYTES)
}
#[test]
fn test_crypto_stream_primitive() {
    use std::ffi::CStr;
    unsafe {
        assert_eq!(CStr::from_ptr(crypto_stream_PRIMITIVE),
                   CStr::from_ptr(crypto_stream_primitive()));
    }
}
