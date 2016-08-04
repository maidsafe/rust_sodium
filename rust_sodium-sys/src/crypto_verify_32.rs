// crypto_verify_32.h

pub const crypto_verify_32_BYTES: usize = 32;

extern {
    pub fn crypto_verify_32_bytes() -> size_t;
    pub fn crypto_verify_32(
        x: *const u8,
        y: *const u8) -> c_int;
}


#[test]
fn test_crypto_verify_32_bytes() {
   assert_eq!(unsafe { crypto_verify_32_bytes() as usize },
                       crypto_verify_32_BYTES);
}
