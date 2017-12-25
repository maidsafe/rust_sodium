// crypto_hash_sha256.h

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_hash_sha256_state {
    state: [u32; 8],
    count: u64,
    buf: [u8; 64],
}
pub const crypto_hash_sha256_BYTES: usize = 32;


extern "C" {
    pub fn crypto_hash_sha256_bytes() -> size_t;
    pub fn crypto_hash_sha256(h: *mut u8, m: *const u8, mlen: c_ulonglong) -> c_int;
    pub fn crypto_hash_sha256_init(state: *mut crypto_hash_sha256_state) -> c_int;
    pub fn crypto_hash_sha256_update(state: *mut crypto_hash_sha256_state,
                                     m: *const u8,
                                     mlen: c_ulonglong)
                                     -> c_int;
    pub fn crypto_hash_sha256_final(state: *mut crypto_hash_sha256_state, h: *mut u8) -> c_int;
}


#[test]
fn test_crypto_hash_sha256_bytes() {
    assert!(unsafe { crypto_hash_sha256_bytes() as usize } == crypto_hash_sha256_BYTES)
}
