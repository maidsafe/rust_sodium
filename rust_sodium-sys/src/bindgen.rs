// Copyright 2018 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under the MIT license <LICENSE-MIT
// http://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.

pub const SODIUM_VERSION_STRING: *const libc::c_char =
    (b"1.0.17\0" as *const libc::c_uchar) as *const libc::c_char;
pub const SODIUM_LIBRARY_VERSION_MAJOR: u32 = 10;
pub const SODIUM_LIBRARY_VERSION_MINOR: u32 = 2;
pub const crypto_aead_aes256gcm_KEYBYTES: u32 = 32;
pub const crypto_aead_aes256gcm_NSECBYTES: u32 = 0;
pub const crypto_aead_aes256gcm_NPUBBYTES: u32 = 12;
pub const crypto_aead_aes256gcm_ABYTES: u32 = 16;
pub const crypto_aead_chacha20poly1305_ietf_KEYBYTES: u32 = 32;
pub const crypto_aead_chacha20poly1305_ietf_NSECBYTES: u32 = 0;
pub const crypto_aead_chacha20poly1305_ietf_NPUBBYTES: u32 = 12;
pub const crypto_aead_chacha20poly1305_ietf_ABYTES: u32 = 16;
pub const crypto_aead_chacha20poly1305_KEYBYTES: u32 = 32;
pub const crypto_aead_chacha20poly1305_NSECBYTES: u32 = 0;
pub const crypto_aead_chacha20poly1305_NPUBBYTES: u32 = 8;
pub const crypto_aead_chacha20poly1305_ABYTES: u32 = 16;
pub const crypto_aead_chacha20poly1305_IETF_KEYBYTES: u32 = 32;
pub const crypto_aead_chacha20poly1305_IETF_NSECBYTES: u32 = 0;
pub const crypto_aead_chacha20poly1305_IETF_NPUBBYTES: u32 = 12;
pub const crypto_aead_chacha20poly1305_IETF_ABYTES: u32 = 16;
pub const crypto_aead_xchacha20poly1305_ietf_KEYBYTES: u32 = 32;
pub const crypto_aead_xchacha20poly1305_ietf_NSECBYTES: u32 = 0;
pub const crypto_aead_xchacha20poly1305_ietf_NPUBBYTES: u32 = 24;
pub const crypto_aead_xchacha20poly1305_ietf_ABYTES: u32 = 16;
pub const crypto_aead_xchacha20poly1305_IETF_KEYBYTES: u32 = 32;
pub const crypto_aead_xchacha20poly1305_IETF_NSECBYTES: u32 = 0;
pub const crypto_aead_xchacha20poly1305_IETF_NPUBBYTES: u32 = 24;
pub const crypto_aead_xchacha20poly1305_IETF_ABYTES: u32 = 16;
pub const crypto_hash_sha512_BYTES: u32 = 64;
pub const crypto_auth_hmacsha512_BYTES: u32 = 64;
pub const crypto_auth_hmacsha512_KEYBYTES: u32 = 32;
pub const crypto_auth_hmacsha512256_BYTES: u32 = 32;
pub const crypto_auth_hmacsha512256_KEYBYTES: u32 = 32;
pub const crypto_auth_BYTES: u32 = 32;
pub const crypto_auth_KEYBYTES: u32 = 32;
pub const crypto_auth_PRIMITIVE: *const libc::c_char =
    (b"hmacsha512256\0" as *const libc::c_uchar) as *const libc::c_char;
pub const crypto_hash_sha256_BYTES: u32 = 32;
pub const crypto_auth_hmacsha256_BYTES: u32 = 32;
pub const crypto_auth_hmacsha256_KEYBYTES: u32 = 32;
pub const crypto_stream_xsalsa20_KEYBYTES: u32 = 32;
pub const crypto_stream_xsalsa20_NONCEBYTES: u32 = 24;
pub const crypto_box_curve25519xsalsa20poly1305_SEEDBYTES: u32 = 32;
pub const crypto_box_curve25519xsalsa20poly1305_PUBLICKEYBYTES: u32 = 32;
pub const crypto_box_curve25519xsalsa20poly1305_SECRETKEYBYTES: u32 = 32;
pub const crypto_box_curve25519xsalsa20poly1305_BEFORENMBYTES: u32 = 32;
pub const crypto_box_curve25519xsalsa20poly1305_NONCEBYTES: u32 = 24;
pub const crypto_box_curve25519xsalsa20poly1305_MACBYTES: u32 = 16;
pub const crypto_box_curve25519xsalsa20poly1305_BOXZEROBYTES: u32 = 16;
pub const crypto_box_curve25519xsalsa20poly1305_ZEROBYTES: u32 = 32;
pub const crypto_box_SEEDBYTES: u32 = 32;
pub const crypto_box_PUBLICKEYBYTES: u32 = 32;
pub const crypto_box_SECRETKEYBYTES: u32 = 32;
pub const crypto_box_NONCEBYTES: u32 = 24;
pub const crypto_box_MACBYTES: u32 = 16;
pub const crypto_box_PRIMITIVE: *const libc::c_char =
    (b"curve25519xsalsa20poly1305\0" as *const libc::c_uchar) as *const libc::c_char;
pub const crypto_box_BEFORENMBYTES: u32 = 32;
pub const crypto_box_SEALBYTES: u32 = 48;
pub const crypto_box_ZEROBYTES: u32 = 32;
pub const crypto_box_BOXZEROBYTES: u32 = 16;
pub const crypto_core_hsalsa20_OUTPUTBYTES: u32 = 32;
pub const crypto_core_hsalsa20_INPUTBYTES: u32 = 16;
pub const crypto_core_hsalsa20_KEYBYTES: u32 = 32;
pub const crypto_core_hsalsa20_CONSTBYTES: u32 = 16;
pub const crypto_core_hchacha20_OUTPUTBYTES: u32 = 32;
pub const crypto_core_hchacha20_INPUTBYTES: u32 = 16;
pub const crypto_core_hchacha20_KEYBYTES: u32 = 32;
pub const crypto_core_hchacha20_CONSTBYTES: u32 = 16;
pub const crypto_core_salsa20_OUTPUTBYTES: u32 = 64;
pub const crypto_core_salsa20_INPUTBYTES: u32 = 16;
pub const crypto_core_salsa20_KEYBYTES: u32 = 32;
pub const crypto_core_salsa20_CONSTBYTES: u32 = 16;
pub const crypto_core_salsa2012_OUTPUTBYTES: u32 = 64;
pub const crypto_core_salsa2012_INPUTBYTES: u32 = 16;
pub const crypto_core_salsa2012_KEYBYTES: u32 = 32;
pub const crypto_core_salsa2012_CONSTBYTES: u32 = 16;
pub const crypto_core_salsa208_OUTPUTBYTES: u32 = 64;
pub const crypto_core_salsa208_INPUTBYTES: u32 = 16;
pub const crypto_core_salsa208_KEYBYTES: u32 = 32;
pub const crypto_core_salsa208_CONSTBYTES: u32 = 16;
pub const crypto_generichash_blake2b_BYTES_MIN: u32 = 16;
pub const crypto_generichash_blake2b_BYTES_MAX: u32 = 64;
pub const crypto_generichash_blake2b_BYTES: u32 = 32;
pub const crypto_generichash_blake2b_KEYBYTES_MIN: u32 = 16;
pub const crypto_generichash_blake2b_KEYBYTES_MAX: u32 = 64;
pub const crypto_generichash_blake2b_KEYBYTES: u32 = 32;
pub const crypto_generichash_blake2b_SALTBYTES: u32 = 16;
pub const crypto_generichash_blake2b_PERSONALBYTES: u32 = 16;
pub const crypto_generichash_BYTES_MIN: u32 = 16;
pub const crypto_generichash_BYTES_MAX: u32 = 64;
pub const crypto_generichash_BYTES: u32 = 32;
pub const crypto_generichash_KEYBYTES_MIN: u32 = 16;
pub const crypto_generichash_KEYBYTES_MAX: u32 = 64;
pub const crypto_generichash_KEYBYTES: u32 = 32;
pub const crypto_generichash_PRIMITIVE: *const libc::c_char =
    (b"blake2b\0" as *const libc::c_uchar) as *const libc::c_char;
pub const crypto_hash_BYTES: u32 = 64;
pub const crypto_hash_PRIMITIVE: *const libc::c_char =
    (b"sha512\0" as *const libc::c_uchar) as *const libc::c_char;
pub const crypto_kdf_blake2b_BYTES_MIN: u32 = 16;
pub const crypto_kdf_blake2b_BYTES_MAX: u32 = 64;
pub const crypto_kdf_blake2b_CONTEXTBYTES: u32 = 8;
pub const crypto_kdf_blake2b_KEYBYTES: u32 = 32;
pub const crypto_kdf_BYTES_MIN: u32 = 16;
pub const crypto_kdf_BYTES_MAX: u32 = 64;
pub const crypto_kdf_CONTEXTBYTES: u32 = 8;
pub const crypto_kdf_KEYBYTES: u32 = 32;
pub const crypto_kdf_PRIMITIVE: *const libc::c_char =
    (b"blake2b\0" as *const libc::c_uchar) as *const libc::c_char;
pub const crypto_kx_PUBLICKEYBYTES: u32 = 32;
pub const crypto_kx_SECRETKEYBYTES: u32 = 32;
pub const crypto_kx_SEEDBYTES: u32 = 32;
pub const crypto_kx_SESSIONKEYBYTES: u32 = 32;
pub const crypto_kx_PRIMITIVE: *const libc::c_char =
    (b"x25519blake2b\0" as *const libc::c_uchar) as *const libc::c_char;
pub const crypto_onetimeauth_poly1305_BYTES: u32 = 16;
pub const crypto_onetimeauth_poly1305_KEYBYTES: u32 = 32;
pub const crypto_onetimeauth_BYTES: u32 = 16;
pub const crypto_onetimeauth_KEYBYTES: u32 = 32;
pub const crypto_onetimeauth_PRIMITIVE: *const libc::c_char =
    (b"poly1305\0" as *const libc::c_uchar) as *const libc::c_char;
pub const crypto_pwhash_argon2i_ALG_ARGON2I13: u32 = 1;
pub const crypto_pwhash_argon2i_BYTES_MIN: u32 = 16;
pub const crypto_pwhash_argon2i_PASSWD_MIN: u32 = 0;
pub const crypto_pwhash_argon2i_PASSWD_MAX: u32 = 4294967295;
pub const crypto_pwhash_argon2i_SALTBYTES: u32 = 16;
pub const crypto_pwhash_argon2i_STRBYTES: u32 = 128;
pub const crypto_pwhash_argon2i_STRPREFIX: *const libc::c_char =
    (b"$argon2i$\0" as *const libc::c_uchar) as *const libc::c_char;
pub const crypto_pwhash_argon2i_OPSLIMIT_MIN: u32 = 3;
pub const crypto_pwhash_argon2i_OPSLIMIT_MAX: u32 = 4294967295;
pub const crypto_pwhash_argon2i_MEMLIMIT_MIN: u32 = 8192;
pub const crypto_pwhash_argon2i_OPSLIMIT_INTERACTIVE: u32 = 4;
pub const crypto_pwhash_argon2i_MEMLIMIT_INTERACTIVE: u32 = 33554432;
pub const crypto_pwhash_argon2i_OPSLIMIT_MODERATE: u32 = 6;
pub const crypto_pwhash_argon2i_MEMLIMIT_MODERATE: u32 = 134217728;
pub const crypto_pwhash_argon2i_OPSLIMIT_SENSITIVE: u32 = 8;
pub const crypto_pwhash_argon2i_MEMLIMIT_SENSITIVE: u32 = 536870912;
pub const crypto_pwhash_argon2id_ALG_ARGON2ID13: u32 = 2;
pub const crypto_pwhash_argon2id_BYTES_MIN: u32 = 16;
pub const crypto_pwhash_argon2id_PASSWD_MIN: u32 = 0;
pub const crypto_pwhash_argon2id_PASSWD_MAX: u32 = 4294967295;
pub const crypto_pwhash_argon2id_SALTBYTES: u32 = 16;
pub const crypto_pwhash_argon2id_STRBYTES: u32 = 128;
pub const crypto_pwhash_argon2id_STRPREFIX: *const libc::c_char =
    (b"$argon2id$\0" as *const libc::c_uchar) as *const libc::c_char;
pub const crypto_pwhash_argon2id_OPSLIMIT_MIN: u32 = 1;
pub const crypto_pwhash_argon2id_OPSLIMIT_MAX: u32 = 4294967295;
pub const crypto_pwhash_argon2id_MEMLIMIT_MIN: u32 = 8192;
pub const crypto_pwhash_argon2id_OPSLIMIT_INTERACTIVE: u32 = 2;
pub const crypto_pwhash_argon2id_MEMLIMIT_INTERACTIVE: u32 = 67108864;
pub const crypto_pwhash_argon2id_OPSLIMIT_MODERATE: u32 = 3;
pub const crypto_pwhash_argon2id_MEMLIMIT_MODERATE: u32 = 268435456;
pub const crypto_pwhash_argon2id_OPSLIMIT_SENSITIVE: u32 = 4;
pub const crypto_pwhash_argon2id_MEMLIMIT_SENSITIVE: u32 = 1073741824;
pub const crypto_pwhash_ALG_ARGON2I13: u32 = 1;
pub const crypto_pwhash_ALG_ARGON2ID13: u32 = 2;
pub const crypto_pwhash_ALG_DEFAULT: u32 = 2;
pub const crypto_pwhash_BYTES_MIN: u32 = 16;
pub const crypto_pwhash_PASSWD_MIN: u32 = 0;
pub const crypto_pwhash_PASSWD_MAX: u32 = 4294967295;
pub const crypto_pwhash_SALTBYTES: u32 = 16;
pub const crypto_pwhash_STRBYTES: u32 = 128;
pub const crypto_pwhash_STRPREFIX: *const libc::c_char =
    (b"$argon2id$\0" as *const libc::c_uchar) as *const libc::c_char;
pub const crypto_pwhash_OPSLIMIT_MIN: u32 = 1;
pub const crypto_pwhash_OPSLIMIT_MAX: u32 = 4294967295;
pub const crypto_pwhash_MEMLIMIT_MIN: u32 = 8192;
pub const crypto_pwhash_OPSLIMIT_INTERACTIVE: u32 = 2;
pub const crypto_pwhash_MEMLIMIT_INTERACTIVE: u32 = 67108864;
pub const crypto_pwhash_OPSLIMIT_MODERATE: u32 = 3;
pub const crypto_pwhash_MEMLIMIT_MODERATE: u32 = 268435456;
pub const crypto_pwhash_OPSLIMIT_SENSITIVE: u32 = 4;
pub const crypto_pwhash_MEMLIMIT_SENSITIVE: u32 = 1073741824;
pub const crypto_pwhash_PRIMITIVE: *const libc::c_char =
    (b"argon2i\0" as *const libc::c_uchar) as *const libc::c_char;
pub const crypto_scalarmult_curve25519_BYTES: u32 = 32;
pub const crypto_scalarmult_curve25519_SCALARBYTES: u32 = 32;
pub const crypto_scalarmult_BYTES: u32 = 32;
pub const crypto_scalarmult_SCALARBYTES: u32 = 32;
pub const crypto_scalarmult_PRIMITIVE: *const libc::c_char =
    (b"curve25519\0" as *const libc::c_uchar) as *const libc::c_char;
pub const crypto_secretbox_xsalsa20poly1305_KEYBYTES: u32 = 32;
pub const crypto_secretbox_xsalsa20poly1305_NONCEBYTES: u32 = 24;
pub const crypto_secretbox_xsalsa20poly1305_MACBYTES: u32 = 16;
pub const crypto_secretbox_xsalsa20poly1305_BOXZEROBYTES: u32 = 16;
pub const crypto_secretbox_xsalsa20poly1305_ZEROBYTES: u32 = 32;
pub const crypto_secretbox_KEYBYTES: u32 = 32;
pub const crypto_secretbox_NONCEBYTES: u32 = 24;
pub const crypto_secretbox_MACBYTES: u32 = 16;
pub const crypto_secretbox_PRIMITIVE: *const libc::c_char =
    (b"xsalsa20poly1305\0" as *const libc::c_uchar) as *const libc::c_char;
pub const crypto_secretbox_ZEROBYTES: u32 = 32;
pub const crypto_secretbox_BOXZEROBYTES: u32 = 16;
pub const crypto_stream_chacha20_KEYBYTES: u32 = 32;
pub const crypto_stream_chacha20_NONCEBYTES: u32 = 8;
pub const crypto_stream_chacha20_ietf_KEYBYTES: u32 = 32;
pub const crypto_stream_chacha20_ietf_NONCEBYTES: u32 = 12;
pub const crypto_stream_chacha20_IETF_KEYBYTES: u32 = 32;
pub const crypto_stream_chacha20_IETF_NONCEBYTES: u32 = 12;
pub const crypto_secretstream_xchacha20poly1305_ABYTES: u32 = 17;
pub const crypto_secretstream_xchacha20poly1305_HEADERBYTES: u32 = 24;
pub const crypto_secretstream_xchacha20poly1305_KEYBYTES: u32 = 32;
pub const crypto_secretstream_xchacha20poly1305_TAG_MESSAGE: u32 = 0;
pub const crypto_secretstream_xchacha20poly1305_TAG_PUSH: u32 = 1;
pub const crypto_secretstream_xchacha20poly1305_TAG_REKEY: u32 = 2;
pub const crypto_secretstream_xchacha20poly1305_TAG_FINAL: u32 = 3;
pub const crypto_shorthash_siphash24_BYTES: u32 = 8;
pub const crypto_shorthash_siphash24_KEYBYTES: u32 = 16;
pub const crypto_shorthash_siphashx24_BYTES: u32 = 16;
pub const crypto_shorthash_siphashx24_KEYBYTES: u32 = 16;
pub const crypto_shorthash_BYTES: u32 = 8;
pub const crypto_shorthash_KEYBYTES: u32 = 16;
pub const crypto_shorthash_PRIMITIVE: *const libc::c_char =
    (b"siphash24\0" as *const libc::c_uchar) as *const libc::c_char;
pub const crypto_sign_ed25519_BYTES: u32 = 64;
pub const crypto_sign_ed25519_SEEDBYTES: u32 = 32;
pub const crypto_sign_ed25519_PUBLICKEYBYTES: u32 = 32;
pub const crypto_sign_ed25519_SECRETKEYBYTES: u32 = 64;
pub const crypto_sign_BYTES: u32 = 64;
pub const crypto_sign_SEEDBYTES: u32 = 32;
pub const crypto_sign_PUBLICKEYBYTES: u32 = 32;
pub const crypto_sign_SECRETKEYBYTES: u32 = 64;
pub const crypto_sign_PRIMITIVE: *const libc::c_char =
    (b"ed25519\0" as *const libc::c_uchar) as *const libc::c_char;
pub const crypto_stream_KEYBYTES: u32 = 32;
pub const crypto_stream_NONCEBYTES: u32 = 24;
pub const crypto_stream_PRIMITIVE: *const libc::c_char =
    (b"xsalsa20\0" as *const libc::c_uchar) as *const libc::c_char;
pub const crypto_stream_salsa20_KEYBYTES: u32 = 32;
pub const crypto_stream_salsa20_NONCEBYTES: u32 = 8;
pub const crypto_verify_16_BYTES: u32 = 16;
pub const crypto_verify_32_BYTES: u32 = 32;
pub const crypto_verify_64_BYTES: u32 = 64;
pub const randombytes_SEEDBYTES: u32 = 32;
pub const sodium_base64_VARIANT_ORIGINAL: u32 = 1;
pub const sodium_base64_VARIANT_ORIGINAL_NO_PADDING: u32 = 3;
pub const sodium_base64_VARIANT_URLSAFE: u32 = 5;
pub const sodium_base64_VARIANT_URLSAFE_NO_PADDING: u32 = 7;
pub const crypto_stream_xchacha20_KEYBYTES: u32 = 32;
pub const crypto_stream_xchacha20_NONCEBYTES: u32 = 24;
pub const crypto_box_curve25519xchacha20poly1305_SEEDBYTES: u32 = 32;
pub const crypto_box_curve25519xchacha20poly1305_PUBLICKEYBYTES: u32 = 32;
pub const crypto_box_curve25519xchacha20poly1305_SECRETKEYBYTES: u32 = 32;
pub const crypto_box_curve25519xchacha20poly1305_BEFORENMBYTES: u32 = 32;
pub const crypto_box_curve25519xchacha20poly1305_NONCEBYTES: u32 = 24;
pub const crypto_box_curve25519xchacha20poly1305_MACBYTES: u32 = 16;
pub const crypto_box_curve25519xchacha20poly1305_SEALBYTES: u32 = 48;
pub const crypto_core_ed25519_BYTES: u32 = 32;
pub const crypto_core_ed25519_UNIFORMBYTES: u32 = 32;
pub const crypto_core_ed25519_SCALARBYTES: u32 = 32;
pub const crypto_core_ed25519_NONREDUCEDSCALARBYTES: u32 = 64;
pub const crypto_scalarmult_ed25519_BYTES: u32 = 32;
pub const crypto_scalarmult_ed25519_SCALARBYTES: u32 = 32;
pub const crypto_secretbox_xchacha20poly1305_KEYBYTES: u32 = 32;
pub const crypto_secretbox_xchacha20poly1305_NONCEBYTES: u32 = 24;
pub const crypto_secretbox_xchacha20poly1305_MACBYTES: u32 = 16;
pub const crypto_pwhash_scryptsalsa208sha256_BYTES_MIN: u32 = 16;
pub const crypto_pwhash_scryptsalsa208sha256_PASSWD_MIN: u32 = 0;
pub const crypto_pwhash_scryptsalsa208sha256_SALTBYTES: u32 = 32;
pub const crypto_pwhash_scryptsalsa208sha256_STRBYTES: u32 = 102;
pub const crypto_pwhash_scryptsalsa208sha256_STRPREFIX: *const libc::c_char =
    (b"$7$\0" as *const libc::c_uchar) as *const libc::c_char;
pub const crypto_pwhash_scryptsalsa208sha256_OPSLIMIT_MIN: u32 = 32768;
pub const crypto_pwhash_scryptsalsa208sha256_OPSLIMIT_MAX: u32 = 4294967295;
pub const crypto_pwhash_scryptsalsa208sha256_MEMLIMIT_MIN: u32 = 16777216;
pub const crypto_pwhash_scryptsalsa208sha256_OPSLIMIT_INTERACTIVE: u32 = 524288;
pub const crypto_pwhash_scryptsalsa208sha256_MEMLIMIT_INTERACTIVE: u32 = 16777216;
pub const crypto_pwhash_scryptsalsa208sha256_OPSLIMIT_SENSITIVE: u32 = 33554432;
pub const crypto_pwhash_scryptsalsa208sha256_MEMLIMIT_SENSITIVE: u32 = 1073741824;
pub const crypto_stream_salsa2012_KEYBYTES: u32 = 32;
pub const crypto_stream_salsa2012_NONCEBYTES: u32 = 8;
pub const crypto_stream_salsa208_KEYBYTES: u32 = 32;
pub const crypto_stream_salsa208_NONCEBYTES: u32 = 8;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
extern "C" {
    pub fn sodium_version_string() -> *const libc::c_char;
}
extern "C" {
    pub fn sodium_library_version_major() -> libc::c_int;
}
extern "C" {
    pub fn sodium_library_version_minor() -> libc::c_int;
}
extern "C" {
    pub fn sodium_library_minimal() -> libc::c_int;
}
extern "C" {
    pub fn sodium_init() -> libc::c_int;
}
extern "C" {
    pub fn sodium_set_misuse_handler(
        handler: ::std::option::Option<unsafe extern "C" fn()>,
    ) -> libc::c_int;
}
extern "C" {
    pub fn sodium_misuse();
}
extern "C" {
    pub fn crypto_aead_aes256gcm_is_available() -> libc::c_int;
}
extern "C" {
    pub fn crypto_aead_aes256gcm_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_aead_aes256gcm_nsecbytes() -> usize;
}
extern "C" {
    pub fn crypto_aead_aes256gcm_npubbytes() -> usize;
}
extern "C" {
    pub fn crypto_aead_aes256gcm_abytes() -> usize;
}
extern "C" {
    pub fn crypto_aead_aes256gcm_messagebytes_max() -> usize;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_aead_aes256gcm_state_ {
    pub opaque: [libc::c_uchar; 512usize],
}
#[test]
fn bindgen_test_layout_crypto_aead_aes256gcm_state_() {
    assert_eq!(
        ::std::mem::size_of::<crypto_aead_aes256gcm_state_>(),
        512usize,
        concat!("Size of: ", stringify!(crypto_aead_aes256gcm_state_))
    );
    assert_eq!(
        ::std::mem::align_of::<crypto_aead_aes256gcm_state_>(),
        1usize,
        concat!("Alignment of ", stringify!(crypto_aead_aes256gcm_state_))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<crypto_aead_aes256gcm_state_>())).opaque as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_aead_aes256gcm_state_),
            "::",
            stringify!(opaque)
        )
    );
}
pub type crypto_aead_aes256gcm_state = crypto_aead_aes256gcm_state_;
extern "C" {
    pub fn crypto_aead_aes256gcm_statebytes() -> usize;
}
extern "C" {
    pub fn crypto_aead_aes256gcm_encrypt(
        c: *mut libc::c_uchar,
        clen_p: *mut libc::c_ulonglong,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        ad: *const libc::c_uchar,
        adlen: libc::c_ulonglong,
        nsec: *const libc::c_uchar,
        npub: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_aead_aes256gcm_decrypt(
        m: *mut libc::c_uchar,
        mlen_p: *mut libc::c_ulonglong,
        nsec: *mut libc::c_uchar,
        c: *const libc::c_uchar,
        clen: libc::c_ulonglong,
        ad: *const libc::c_uchar,
        adlen: libc::c_ulonglong,
        npub: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_aead_aes256gcm_encrypt_detached(
        c: *mut libc::c_uchar,
        mac: *mut libc::c_uchar,
        maclen_p: *mut libc::c_ulonglong,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        ad: *const libc::c_uchar,
        adlen: libc::c_ulonglong,
        nsec: *const libc::c_uchar,
        npub: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_aead_aes256gcm_decrypt_detached(
        m: *mut libc::c_uchar,
        nsec: *mut libc::c_uchar,
        c: *const libc::c_uchar,
        clen: libc::c_ulonglong,
        mac: *const libc::c_uchar,
        ad: *const libc::c_uchar,
        adlen: libc::c_ulonglong,
        npub: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_aead_aes256gcm_beforenm(
        ctx_: *mut crypto_aead_aes256gcm_state,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_aead_aes256gcm_encrypt_afternm(
        c: *mut libc::c_uchar,
        clen_p: *mut libc::c_ulonglong,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        ad: *const libc::c_uchar,
        adlen: libc::c_ulonglong,
        nsec: *const libc::c_uchar,
        npub: *const libc::c_uchar,
        ctx_: *const crypto_aead_aes256gcm_state,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_aead_aes256gcm_decrypt_afternm(
        m: *mut libc::c_uchar,
        mlen_p: *mut libc::c_ulonglong,
        nsec: *mut libc::c_uchar,
        c: *const libc::c_uchar,
        clen: libc::c_ulonglong,
        ad: *const libc::c_uchar,
        adlen: libc::c_ulonglong,
        npub: *const libc::c_uchar,
        ctx_: *const crypto_aead_aes256gcm_state,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_aead_aes256gcm_encrypt_detached_afternm(
        c: *mut libc::c_uchar,
        mac: *mut libc::c_uchar,
        maclen_p: *mut libc::c_ulonglong,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        ad: *const libc::c_uchar,
        adlen: libc::c_ulonglong,
        nsec: *const libc::c_uchar,
        npub: *const libc::c_uchar,
        ctx_: *const crypto_aead_aes256gcm_state,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_aead_aes256gcm_decrypt_detached_afternm(
        m: *mut libc::c_uchar,
        nsec: *mut libc::c_uchar,
        c: *const libc::c_uchar,
        clen: libc::c_ulonglong,
        mac: *const libc::c_uchar,
        ad: *const libc::c_uchar,
        adlen: libc::c_ulonglong,
        npub: *const libc::c_uchar,
        ctx_: *const crypto_aead_aes256gcm_state,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_aead_aes256gcm_keygen(k: *mut libc::c_uchar);
}
extern "C" {
    pub fn crypto_aead_chacha20poly1305_ietf_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_aead_chacha20poly1305_ietf_nsecbytes() -> usize;
}
extern "C" {
    pub fn crypto_aead_chacha20poly1305_ietf_npubbytes() -> usize;
}
extern "C" {
    pub fn crypto_aead_chacha20poly1305_ietf_abytes() -> usize;
}
extern "C" {
    pub fn crypto_aead_chacha20poly1305_ietf_messagebytes_max() -> usize;
}
extern "C" {
    pub fn crypto_aead_chacha20poly1305_ietf_encrypt(
        c: *mut libc::c_uchar,
        clen_p: *mut libc::c_ulonglong,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        ad: *const libc::c_uchar,
        adlen: libc::c_ulonglong,
        nsec: *const libc::c_uchar,
        npub: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_aead_chacha20poly1305_ietf_decrypt(
        m: *mut libc::c_uchar,
        mlen_p: *mut libc::c_ulonglong,
        nsec: *mut libc::c_uchar,
        c: *const libc::c_uchar,
        clen: libc::c_ulonglong,
        ad: *const libc::c_uchar,
        adlen: libc::c_ulonglong,
        npub: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_aead_chacha20poly1305_ietf_encrypt_detached(
        c: *mut libc::c_uchar,
        mac: *mut libc::c_uchar,
        maclen_p: *mut libc::c_ulonglong,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        ad: *const libc::c_uchar,
        adlen: libc::c_ulonglong,
        nsec: *const libc::c_uchar,
        npub: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_aead_chacha20poly1305_ietf_decrypt_detached(
        m: *mut libc::c_uchar,
        nsec: *mut libc::c_uchar,
        c: *const libc::c_uchar,
        clen: libc::c_ulonglong,
        mac: *const libc::c_uchar,
        ad: *const libc::c_uchar,
        adlen: libc::c_ulonglong,
        npub: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_aead_chacha20poly1305_ietf_keygen(k: *mut libc::c_uchar);
}
extern "C" {
    pub fn crypto_aead_chacha20poly1305_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_aead_chacha20poly1305_nsecbytes() -> usize;
}
extern "C" {
    pub fn crypto_aead_chacha20poly1305_npubbytes() -> usize;
}
extern "C" {
    pub fn crypto_aead_chacha20poly1305_abytes() -> usize;
}
extern "C" {
    pub fn crypto_aead_chacha20poly1305_messagebytes_max() -> usize;
}
extern "C" {
    pub fn crypto_aead_chacha20poly1305_encrypt(
        c: *mut libc::c_uchar,
        clen_p: *mut libc::c_ulonglong,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        ad: *const libc::c_uchar,
        adlen: libc::c_ulonglong,
        nsec: *const libc::c_uchar,
        npub: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_aead_chacha20poly1305_decrypt(
        m: *mut libc::c_uchar,
        mlen_p: *mut libc::c_ulonglong,
        nsec: *mut libc::c_uchar,
        c: *const libc::c_uchar,
        clen: libc::c_ulonglong,
        ad: *const libc::c_uchar,
        adlen: libc::c_ulonglong,
        npub: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_aead_chacha20poly1305_encrypt_detached(
        c: *mut libc::c_uchar,
        mac: *mut libc::c_uchar,
        maclen_p: *mut libc::c_ulonglong,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        ad: *const libc::c_uchar,
        adlen: libc::c_ulonglong,
        nsec: *const libc::c_uchar,
        npub: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_aead_chacha20poly1305_decrypt_detached(
        m: *mut libc::c_uchar,
        nsec: *mut libc::c_uchar,
        c: *const libc::c_uchar,
        clen: libc::c_ulonglong,
        mac: *const libc::c_uchar,
        ad: *const libc::c_uchar,
        adlen: libc::c_ulonglong,
        npub: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_aead_chacha20poly1305_keygen(k: *mut libc::c_uchar);
}
extern "C" {
    pub fn crypto_aead_xchacha20poly1305_ietf_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_aead_xchacha20poly1305_ietf_nsecbytes() -> usize;
}
extern "C" {
    pub fn crypto_aead_xchacha20poly1305_ietf_npubbytes() -> usize;
}
extern "C" {
    pub fn crypto_aead_xchacha20poly1305_ietf_abytes() -> usize;
}
extern "C" {
    pub fn crypto_aead_xchacha20poly1305_ietf_messagebytes_max() -> usize;
}
extern "C" {
    pub fn crypto_aead_xchacha20poly1305_ietf_encrypt(
        c: *mut libc::c_uchar,
        clen_p: *mut libc::c_ulonglong,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        ad: *const libc::c_uchar,
        adlen: libc::c_ulonglong,
        nsec: *const libc::c_uchar,
        npub: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_aead_xchacha20poly1305_ietf_decrypt(
        m: *mut libc::c_uchar,
        mlen_p: *mut libc::c_ulonglong,
        nsec: *mut libc::c_uchar,
        c: *const libc::c_uchar,
        clen: libc::c_ulonglong,
        ad: *const libc::c_uchar,
        adlen: libc::c_ulonglong,
        npub: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_aead_xchacha20poly1305_ietf_encrypt_detached(
        c: *mut libc::c_uchar,
        mac: *mut libc::c_uchar,
        maclen_p: *mut libc::c_ulonglong,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        ad: *const libc::c_uchar,
        adlen: libc::c_ulonglong,
        nsec: *const libc::c_uchar,
        npub: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_aead_xchacha20poly1305_ietf_decrypt_detached(
        m: *mut libc::c_uchar,
        nsec: *mut libc::c_uchar,
        c: *const libc::c_uchar,
        clen: libc::c_ulonglong,
        mac: *const libc::c_uchar,
        ad: *const libc::c_uchar,
        adlen: libc::c_ulonglong,
        npub: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_aead_xchacha20poly1305_ietf_keygen(k: *mut libc::c_uchar);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_hash_sha512_state {
    pub state: [u64; 8usize],
    pub count: [u64; 2usize],
    pub buf: [u8; 128usize],
}
#[test]
fn bindgen_test_layout_crypto_hash_sha512_state() {
    assert_eq!(
        ::std::mem::size_of::<crypto_hash_sha512_state>(),
        208usize,
        concat!("Size of: ", stringify!(crypto_hash_sha512_state))
    );
    assert_eq!(
        ::std::mem::align_of::<crypto_hash_sha512_state>(),
        8usize,
        concat!("Alignment of ", stringify!(crypto_hash_sha512_state))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_hash_sha512_state>())).state as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_hash_sha512_state),
            "::",
            stringify!(state)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_hash_sha512_state>())).count as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_hash_sha512_state),
            "::",
            stringify!(count)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_hash_sha512_state>())).buf as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_hash_sha512_state),
            "::",
            stringify!(buf)
        )
    );
}
extern "C" {
    pub fn crypto_hash_sha512_statebytes() -> usize;
}
extern "C" {
    pub fn crypto_hash_sha512_bytes() -> usize;
}
extern "C" {
    pub fn crypto_hash_sha512(
        out: *mut libc::c_uchar,
        in_: *const libc::c_uchar,
        inlen: libc::c_ulonglong,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_hash_sha512_init(state: *mut crypto_hash_sha512_state) -> libc::c_int;
}
extern "C" {
    pub fn crypto_hash_sha512_update(
        state: *mut crypto_hash_sha512_state,
        in_: *const libc::c_uchar,
        inlen: libc::c_ulonglong,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_hash_sha512_final(
        state: *mut crypto_hash_sha512_state,
        out: *mut libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_auth_hmacsha512_bytes() -> usize;
}
extern "C" {
    pub fn crypto_auth_hmacsha512_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_auth_hmacsha512(
        out: *mut libc::c_uchar,
        in_: *const libc::c_uchar,
        inlen: libc::c_ulonglong,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_auth_hmacsha512_verify(
        h: *const libc::c_uchar,
        in_: *const libc::c_uchar,
        inlen: libc::c_ulonglong,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_auth_hmacsha512_state {
    pub ictx: crypto_hash_sha512_state,
    pub octx: crypto_hash_sha512_state,
}
#[test]
fn bindgen_test_layout_crypto_auth_hmacsha512_state() {
    assert_eq!(
        ::std::mem::size_of::<crypto_auth_hmacsha512_state>(),
        416usize,
        concat!("Size of: ", stringify!(crypto_auth_hmacsha512_state))
    );
    assert_eq!(
        ::std::mem::align_of::<crypto_auth_hmacsha512_state>(),
        8usize,
        concat!("Alignment of ", stringify!(crypto_auth_hmacsha512_state))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<crypto_auth_hmacsha512_state>())).ictx as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_auth_hmacsha512_state),
            "::",
            stringify!(ictx)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<crypto_auth_hmacsha512_state>())).octx as *const _ as usize
        },
        208usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_auth_hmacsha512_state),
            "::",
            stringify!(octx)
        )
    );
}
extern "C" {
    pub fn crypto_auth_hmacsha512_statebytes() -> usize;
}
extern "C" {
    pub fn crypto_auth_hmacsha512_init(
        state: *mut crypto_auth_hmacsha512_state,
        key: *const libc::c_uchar,
        keylen: usize,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_auth_hmacsha512_update(
        state: *mut crypto_auth_hmacsha512_state,
        in_: *const libc::c_uchar,
        inlen: libc::c_ulonglong,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_auth_hmacsha512_final(
        state: *mut crypto_auth_hmacsha512_state,
        out: *mut libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_auth_hmacsha512_keygen(k: *mut libc::c_uchar);
}
extern "C" {
    pub fn crypto_auth_hmacsha512256_bytes() -> usize;
}
extern "C" {
    pub fn crypto_auth_hmacsha512256_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_auth_hmacsha512256(
        out: *mut libc::c_uchar,
        in_: *const libc::c_uchar,
        inlen: libc::c_ulonglong,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_auth_hmacsha512256_verify(
        h: *const libc::c_uchar,
        in_: *const libc::c_uchar,
        inlen: libc::c_ulonglong,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
pub type crypto_auth_hmacsha512256_state = crypto_auth_hmacsha512_state;
extern "C" {
    pub fn crypto_auth_hmacsha512256_statebytes() -> usize;
}
extern "C" {
    pub fn crypto_auth_hmacsha512256_init(
        state: *mut crypto_auth_hmacsha512256_state,
        key: *const libc::c_uchar,
        keylen: usize,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_auth_hmacsha512256_update(
        state: *mut crypto_auth_hmacsha512256_state,
        in_: *const libc::c_uchar,
        inlen: libc::c_ulonglong,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_auth_hmacsha512256_final(
        state: *mut crypto_auth_hmacsha512256_state,
        out: *mut libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_auth_hmacsha512256_keygen(k: *mut libc::c_uchar);
}
extern "C" {
    pub fn crypto_auth_bytes() -> usize;
}
extern "C" {
    pub fn crypto_auth_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_auth_primitive() -> *const libc::c_char;
}
extern "C" {
    pub fn crypto_auth(
        out: *mut libc::c_uchar,
        in_: *const libc::c_uchar,
        inlen: libc::c_ulonglong,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_auth_verify(
        h: *const libc::c_uchar,
        in_: *const libc::c_uchar,
        inlen: libc::c_ulonglong,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_auth_keygen(k: *mut libc::c_uchar);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_hash_sha256_state {
    pub state: [u32; 8usize],
    pub count: u64,
    pub buf: [u8; 64usize],
}
#[test]
fn bindgen_test_layout_crypto_hash_sha256_state() {
    assert_eq!(
        ::std::mem::size_of::<crypto_hash_sha256_state>(),
        104usize,
        concat!("Size of: ", stringify!(crypto_hash_sha256_state))
    );
    assert_eq!(
        ::std::mem::align_of::<crypto_hash_sha256_state>(),
        8usize,
        concat!("Alignment of ", stringify!(crypto_hash_sha256_state))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_hash_sha256_state>())).state as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_hash_sha256_state),
            "::",
            stringify!(state)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_hash_sha256_state>())).count as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_hash_sha256_state),
            "::",
            stringify!(count)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_hash_sha256_state>())).buf as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_hash_sha256_state),
            "::",
            stringify!(buf)
        )
    );
}
extern "C" {
    pub fn crypto_hash_sha256_statebytes() -> usize;
}
extern "C" {
    pub fn crypto_hash_sha256_bytes() -> usize;
}
extern "C" {
    pub fn crypto_hash_sha256(
        out: *mut libc::c_uchar,
        in_: *const libc::c_uchar,
        inlen: libc::c_ulonglong,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_hash_sha256_init(state: *mut crypto_hash_sha256_state) -> libc::c_int;
}
extern "C" {
    pub fn crypto_hash_sha256_update(
        state: *mut crypto_hash_sha256_state,
        in_: *const libc::c_uchar,
        inlen: libc::c_ulonglong,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_hash_sha256_final(
        state: *mut crypto_hash_sha256_state,
        out: *mut libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_auth_hmacsha256_bytes() -> usize;
}
extern "C" {
    pub fn crypto_auth_hmacsha256_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_auth_hmacsha256(
        out: *mut libc::c_uchar,
        in_: *const libc::c_uchar,
        inlen: libc::c_ulonglong,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_auth_hmacsha256_verify(
        h: *const libc::c_uchar,
        in_: *const libc::c_uchar,
        inlen: libc::c_ulonglong,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_auth_hmacsha256_state {
    pub ictx: crypto_hash_sha256_state,
    pub octx: crypto_hash_sha256_state,
}
#[test]
fn bindgen_test_layout_crypto_auth_hmacsha256_state() {
    assert_eq!(
        ::std::mem::size_of::<crypto_auth_hmacsha256_state>(),
        208usize,
        concat!("Size of: ", stringify!(crypto_auth_hmacsha256_state))
    );
    assert_eq!(
        ::std::mem::align_of::<crypto_auth_hmacsha256_state>(),
        8usize,
        concat!("Alignment of ", stringify!(crypto_auth_hmacsha256_state))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<crypto_auth_hmacsha256_state>())).ictx as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_auth_hmacsha256_state),
            "::",
            stringify!(ictx)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<crypto_auth_hmacsha256_state>())).octx as *const _ as usize
        },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_auth_hmacsha256_state),
            "::",
            stringify!(octx)
        )
    );
}
extern "C" {
    pub fn crypto_auth_hmacsha256_statebytes() -> usize;
}
extern "C" {
    pub fn crypto_auth_hmacsha256_init(
        state: *mut crypto_auth_hmacsha256_state,
        key: *const libc::c_uchar,
        keylen: usize,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_auth_hmacsha256_update(
        state: *mut crypto_auth_hmacsha256_state,
        in_: *const libc::c_uchar,
        inlen: libc::c_ulonglong,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_auth_hmacsha256_final(
        state: *mut crypto_auth_hmacsha256_state,
        out: *mut libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_auth_hmacsha256_keygen(k: *mut libc::c_uchar);
}
extern "C" {
    pub fn crypto_stream_xsalsa20_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_stream_xsalsa20_noncebytes() -> usize;
}
extern "C" {
    pub fn crypto_stream_xsalsa20_messagebytes_max() -> usize;
}
extern "C" {
    pub fn crypto_stream_xsalsa20(
        c: *mut libc::c_uchar,
        clen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_stream_xsalsa20_xor(
        c: *mut libc::c_uchar,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_stream_xsalsa20_xor_ic(
        c: *mut libc::c_uchar,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        ic: u64,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_stream_xsalsa20_keygen(k: *mut libc::c_uchar);
}
extern "C" {
    pub fn crypto_box_curve25519xsalsa20poly1305_seedbytes() -> usize;
}
extern "C" {
    pub fn crypto_box_curve25519xsalsa20poly1305_publickeybytes() -> usize;
}
extern "C" {
    pub fn crypto_box_curve25519xsalsa20poly1305_secretkeybytes() -> usize;
}
extern "C" {
    pub fn crypto_box_curve25519xsalsa20poly1305_beforenmbytes() -> usize;
}
extern "C" {
    pub fn crypto_box_curve25519xsalsa20poly1305_noncebytes() -> usize;
}
extern "C" {
    pub fn crypto_box_curve25519xsalsa20poly1305_macbytes() -> usize;
}
extern "C" {
    pub fn crypto_box_curve25519xsalsa20poly1305_messagebytes_max() -> usize;
}
extern "C" {
    pub fn crypto_box_curve25519xsalsa20poly1305_seed_keypair(
        pk: *mut libc::c_uchar,
        sk: *mut libc::c_uchar,
        seed: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_box_curve25519xsalsa20poly1305_keypair(
        pk: *mut libc::c_uchar,
        sk: *mut libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_box_curve25519xsalsa20poly1305_beforenm(
        k: *mut libc::c_uchar,
        pk: *const libc::c_uchar,
        sk: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_box_curve25519xsalsa20poly1305_boxzerobytes() -> usize;
}
extern "C" {
    pub fn crypto_box_curve25519xsalsa20poly1305_zerobytes() -> usize;
}
extern "C" {
    pub fn crypto_box_curve25519xsalsa20poly1305(
        c: *mut libc::c_uchar,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        pk: *const libc::c_uchar,
        sk: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_box_curve25519xsalsa20poly1305_open(
        m: *mut libc::c_uchar,
        c: *const libc::c_uchar,
        clen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        pk: *const libc::c_uchar,
        sk: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_box_curve25519xsalsa20poly1305_afternm(
        c: *mut libc::c_uchar,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_box_curve25519xsalsa20poly1305_open_afternm(
        m: *mut libc::c_uchar,
        c: *const libc::c_uchar,
        clen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_box_seedbytes() -> usize;
}
extern "C" {
    pub fn crypto_box_publickeybytes() -> usize;
}
extern "C" {
    pub fn crypto_box_secretkeybytes() -> usize;
}
extern "C" {
    pub fn crypto_box_noncebytes() -> usize;
}
extern "C" {
    pub fn crypto_box_macbytes() -> usize;
}
extern "C" {
    pub fn crypto_box_messagebytes_max() -> usize;
}
extern "C" {
    pub fn crypto_box_primitive() -> *const libc::c_char;
}
extern "C" {
    pub fn crypto_box_seed_keypair(
        pk: *mut libc::c_uchar,
        sk: *mut libc::c_uchar,
        seed: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_box_keypair(pk: *mut libc::c_uchar, sk: *mut libc::c_uchar) -> libc::c_int;
}
extern "C" {
    pub fn crypto_box_easy(
        c: *mut libc::c_uchar,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        pk: *const libc::c_uchar,
        sk: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_box_open_easy(
        m: *mut libc::c_uchar,
        c: *const libc::c_uchar,
        clen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        pk: *const libc::c_uchar,
        sk: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_box_detached(
        c: *mut libc::c_uchar,
        mac: *mut libc::c_uchar,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        pk: *const libc::c_uchar,
        sk: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_box_open_detached(
        m: *mut libc::c_uchar,
        c: *const libc::c_uchar,
        mac: *const libc::c_uchar,
        clen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        pk: *const libc::c_uchar,
        sk: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_box_beforenmbytes() -> usize;
}
extern "C" {
    pub fn crypto_box_beforenm(
        k: *mut libc::c_uchar,
        pk: *const libc::c_uchar,
        sk: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_box_easy_afternm(
        c: *mut libc::c_uchar,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_box_open_easy_afternm(
        m: *mut libc::c_uchar,
        c: *const libc::c_uchar,
        clen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_box_detached_afternm(
        c: *mut libc::c_uchar,
        mac: *mut libc::c_uchar,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_box_open_detached_afternm(
        m: *mut libc::c_uchar,
        c: *const libc::c_uchar,
        mac: *const libc::c_uchar,
        clen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_box_sealbytes() -> usize;
}
extern "C" {
    pub fn crypto_box_seal(
        c: *mut libc::c_uchar,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        pk: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_box_seal_open(
        m: *mut libc::c_uchar,
        c: *const libc::c_uchar,
        clen: libc::c_ulonglong,
        pk: *const libc::c_uchar,
        sk: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_box_zerobytes() -> usize;
}
extern "C" {
    pub fn crypto_box_boxzerobytes() -> usize;
}
extern "C" {
    pub fn crypto_box(
        c: *mut libc::c_uchar,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        pk: *const libc::c_uchar,
        sk: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_box_open(
        m: *mut libc::c_uchar,
        c: *const libc::c_uchar,
        clen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        pk: *const libc::c_uchar,
        sk: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_box_afternm(
        c: *mut libc::c_uchar,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_box_open_afternm(
        m: *mut libc::c_uchar,
        c: *const libc::c_uchar,
        clen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_core_hsalsa20_outputbytes() -> usize;
}
extern "C" {
    pub fn crypto_core_hsalsa20_inputbytes() -> usize;
}
extern "C" {
    pub fn crypto_core_hsalsa20_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_core_hsalsa20_constbytes() -> usize;
}
extern "C" {
    pub fn crypto_core_hsalsa20(
        out: *mut libc::c_uchar,
        in_: *const libc::c_uchar,
        k: *const libc::c_uchar,
        c: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_core_hchacha20_outputbytes() -> usize;
}
extern "C" {
    pub fn crypto_core_hchacha20_inputbytes() -> usize;
}
extern "C" {
    pub fn crypto_core_hchacha20_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_core_hchacha20_constbytes() -> usize;
}
extern "C" {
    pub fn crypto_core_hchacha20(
        out: *mut libc::c_uchar,
        in_: *const libc::c_uchar,
        k: *const libc::c_uchar,
        c: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_core_salsa20_outputbytes() -> usize;
}
extern "C" {
    pub fn crypto_core_salsa20_inputbytes() -> usize;
}
extern "C" {
    pub fn crypto_core_salsa20_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_core_salsa20_constbytes() -> usize;
}
extern "C" {
    pub fn crypto_core_salsa20(
        out: *mut libc::c_uchar,
        in_: *const libc::c_uchar,
        k: *const libc::c_uchar,
        c: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_core_salsa2012_outputbytes() -> usize;
}
extern "C" {
    pub fn crypto_core_salsa2012_inputbytes() -> usize;
}
extern "C" {
    pub fn crypto_core_salsa2012_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_core_salsa2012_constbytes() -> usize;
}
extern "C" {
    pub fn crypto_core_salsa2012(
        out: *mut libc::c_uchar,
        in_: *const libc::c_uchar,
        k: *const libc::c_uchar,
        c: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    #[deprecated]
    pub fn crypto_core_salsa208_outputbytes() -> usize;
}
extern "C" {
    #[deprecated]
    pub fn crypto_core_salsa208_inputbytes() -> usize;
}
extern "C" {
    #[deprecated]
    pub fn crypto_core_salsa208_keybytes() -> usize;
}
extern "C" {
    #[deprecated]
    pub fn crypto_core_salsa208_constbytes() -> usize;
}
extern "C" {
    pub fn crypto_core_salsa208(
        out: *mut libc::c_uchar,
        in_: *const libc::c_uchar,
        k: *const libc::c_uchar,
        c: *const libc::c_uchar,
    ) -> libc::c_int;
}
#[repr(C)]
#[repr(align(64))]
#[derive(Copy, Clone)]
pub struct crypto_generichash_blake2b_state {
    pub opaque: [libc::c_uchar; 384usize],
}
impl Default for crypto_generichash_blake2b_state {
    fn default() -> crypto_generichash_blake2b_state {
        crypto_generichash_blake2b_state { opaque: [0; 384] }
    }
}
#[test]
fn bindgen_test_layout_crypto_generichash_blake2b_state() {
    assert_eq!(
        ::std::mem::size_of::<crypto_generichash_blake2b_state>(),
        384usize,
        concat!("Size of: ", stringify!(crypto_generichash_blake2b_state))
    );
    assert_eq!(
        ::std::mem::align_of::<crypto_generichash_blake2b_state>(),
        64usize,
        concat!(
            "Alignment of ",
            stringify!(crypto_generichash_blake2b_state)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<crypto_generichash_blake2b_state>())).opaque as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_generichash_blake2b_state),
            "::",
            stringify!(opaque)
        )
    );
}
extern "C" {
    pub fn crypto_generichash_blake2b_bytes_min() -> usize;
}
extern "C" {
    pub fn crypto_generichash_blake2b_bytes_max() -> usize;
}
extern "C" {
    pub fn crypto_generichash_blake2b_bytes() -> usize;
}
extern "C" {
    pub fn crypto_generichash_blake2b_keybytes_min() -> usize;
}
extern "C" {
    pub fn crypto_generichash_blake2b_keybytes_max() -> usize;
}
extern "C" {
    pub fn crypto_generichash_blake2b_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_generichash_blake2b_saltbytes() -> usize;
}
extern "C" {
    pub fn crypto_generichash_blake2b_personalbytes() -> usize;
}
extern "C" {
    pub fn crypto_generichash_blake2b_statebytes() -> usize;
}
extern "C" {
    pub fn crypto_generichash_blake2b(
        out: *mut libc::c_uchar,
        outlen: usize,
        in_: *const libc::c_uchar,
        inlen: libc::c_ulonglong,
        key: *const libc::c_uchar,
        keylen: usize,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_generichash_blake2b_salt_personal(
        out: *mut libc::c_uchar,
        outlen: usize,
        in_: *const libc::c_uchar,
        inlen: libc::c_ulonglong,
        key: *const libc::c_uchar,
        keylen: usize,
        salt: *const libc::c_uchar,
        personal: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_generichash_blake2b_init(
        state: *mut crypto_generichash_blake2b_state,
        key: *const libc::c_uchar,
        keylen: usize,
        outlen: usize,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_generichash_blake2b_init_salt_personal(
        state: *mut crypto_generichash_blake2b_state,
        key: *const libc::c_uchar,
        keylen: usize,
        outlen: usize,
        salt: *const libc::c_uchar,
        personal: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_generichash_blake2b_update(
        state: *mut crypto_generichash_blake2b_state,
        in_: *const libc::c_uchar,
        inlen: libc::c_ulonglong,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_generichash_blake2b_final(
        state: *mut crypto_generichash_blake2b_state,
        out: *mut libc::c_uchar,
        outlen: usize,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_generichash_blake2b_keygen(k: *mut libc::c_uchar);
}
extern "C" {
    pub fn crypto_generichash_bytes_min() -> usize;
}
extern "C" {
    pub fn crypto_generichash_bytes_max() -> usize;
}
extern "C" {
    pub fn crypto_generichash_bytes() -> usize;
}
extern "C" {
    pub fn crypto_generichash_keybytes_min() -> usize;
}
extern "C" {
    pub fn crypto_generichash_keybytes_max() -> usize;
}
extern "C" {
    pub fn crypto_generichash_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_generichash_primitive() -> *const libc::c_char;
}
pub type crypto_generichash_state = crypto_generichash_blake2b_state;
extern "C" {
    pub fn crypto_generichash_statebytes() -> usize;
}
extern "C" {
    pub fn crypto_generichash(
        out: *mut libc::c_uchar,
        outlen: usize,
        in_: *const libc::c_uchar,
        inlen: libc::c_ulonglong,
        key: *const libc::c_uchar,
        keylen: usize,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_generichash_init(
        state: *mut crypto_generichash_state,
        key: *const libc::c_uchar,
        keylen: usize,
        outlen: usize,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_generichash_update(
        state: *mut crypto_generichash_state,
        in_: *const libc::c_uchar,
        inlen: libc::c_ulonglong,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_generichash_final(
        state: *mut crypto_generichash_state,
        out: *mut libc::c_uchar,
        outlen: usize,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_generichash_keygen(k: *mut libc::c_uchar);
}
extern "C" {
    pub fn crypto_hash_bytes() -> usize;
}
extern "C" {
    pub fn crypto_hash(
        out: *mut libc::c_uchar,
        in_: *const libc::c_uchar,
        inlen: libc::c_ulonglong,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_hash_primitive() -> *const libc::c_char;
}
extern "C" {
    pub fn crypto_kdf_blake2b_bytes_min() -> usize;
}
extern "C" {
    pub fn crypto_kdf_blake2b_bytes_max() -> usize;
}
extern "C" {
    pub fn crypto_kdf_blake2b_contextbytes() -> usize;
}
extern "C" {
    pub fn crypto_kdf_blake2b_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_kdf_blake2b_derive_from_key(
        subkey: *mut libc::c_uchar,
        subkey_len: usize,
        subkey_id: u64,
        ctx: *const libc::c_char,
        key: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_kdf_bytes_min() -> usize;
}
extern "C" {
    pub fn crypto_kdf_bytes_max() -> usize;
}
extern "C" {
    pub fn crypto_kdf_contextbytes() -> usize;
}
extern "C" {
    pub fn crypto_kdf_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_kdf_primitive() -> *const libc::c_char;
}
extern "C" {
    pub fn crypto_kdf_derive_from_key(
        subkey: *mut libc::c_uchar,
        subkey_len: usize,
        subkey_id: u64,
        ctx: *const libc::c_char,
        key: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_kdf_keygen(k: *mut libc::c_uchar);
}
extern "C" {
    pub fn crypto_kx_publickeybytes() -> usize;
}
extern "C" {
    pub fn crypto_kx_secretkeybytes() -> usize;
}
extern "C" {
    pub fn crypto_kx_seedbytes() -> usize;
}
extern "C" {
    pub fn crypto_kx_sessionkeybytes() -> usize;
}
extern "C" {
    pub fn crypto_kx_primitive() -> *const libc::c_char;
}
extern "C" {
    pub fn crypto_kx_seed_keypair(
        pk: *mut libc::c_uchar,
        sk: *mut libc::c_uchar,
        seed: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_kx_keypair(pk: *mut libc::c_uchar, sk: *mut libc::c_uchar) -> libc::c_int;
}
extern "C" {
    pub fn crypto_kx_client_session_keys(
        rx: *mut libc::c_uchar,
        tx: *mut libc::c_uchar,
        client_pk: *const libc::c_uchar,
        client_sk: *const libc::c_uchar,
        server_pk: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_kx_server_session_keys(
        rx: *mut libc::c_uchar,
        tx: *mut libc::c_uchar,
        server_pk: *const libc::c_uchar,
        server_sk: *const libc::c_uchar,
        client_pk: *const libc::c_uchar,
    ) -> libc::c_int;
}
#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone)]
pub struct crypto_onetimeauth_poly1305_state {
    pub opaque: [libc::c_uchar; 256usize],
}
#[test]
fn bindgen_test_layout_crypto_onetimeauth_poly1305_state() {
    assert_eq!(
        ::std::mem::size_of::<crypto_onetimeauth_poly1305_state>(),
        256usize,
        concat!("Size of: ", stringify!(crypto_onetimeauth_poly1305_state))
    );
    assert_eq!(
        ::std::mem::align_of::<crypto_onetimeauth_poly1305_state>(),
        16usize,
        concat!(
            "Alignment of ",
            stringify!(crypto_onetimeauth_poly1305_state)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<crypto_onetimeauth_poly1305_state>())).opaque as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_onetimeauth_poly1305_state),
            "::",
            stringify!(opaque)
        )
    );
}
extern "C" {
    pub fn crypto_onetimeauth_poly1305_statebytes() -> usize;
}
extern "C" {
    pub fn crypto_onetimeauth_poly1305_bytes() -> usize;
}
extern "C" {
    pub fn crypto_onetimeauth_poly1305_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_onetimeauth_poly1305(
        out: *mut libc::c_uchar,
        in_: *const libc::c_uchar,
        inlen: libc::c_ulonglong,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_onetimeauth_poly1305_verify(
        h: *const libc::c_uchar,
        in_: *const libc::c_uchar,
        inlen: libc::c_ulonglong,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_onetimeauth_poly1305_init(
        state: *mut crypto_onetimeauth_poly1305_state,
        key: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_onetimeauth_poly1305_update(
        state: *mut crypto_onetimeauth_poly1305_state,
        in_: *const libc::c_uchar,
        inlen: libc::c_ulonglong,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_onetimeauth_poly1305_final(
        state: *mut crypto_onetimeauth_poly1305_state,
        out: *mut libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_onetimeauth_poly1305_keygen(k: *mut libc::c_uchar);
}
pub type crypto_onetimeauth_state = crypto_onetimeauth_poly1305_state;
extern "C" {
    pub fn crypto_onetimeauth_statebytes() -> usize;
}
extern "C" {
    pub fn crypto_onetimeauth_bytes() -> usize;
}
extern "C" {
    pub fn crypto_onetimeauth_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_onetimeauth_primitive() -> *const libc::c_char;
}
extern "C" {
    pub fn crypto_onetimeauth(
        out: *mut libc::c_uchar,
        in_: *const libc::c_uchar,
        inlen: libc::c_ulonglong,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_onetimeauth_verify(
        h: *const libc::c_uchar,
        in_: *const libc::c_uchar,
        inlen: libc::c_ulonglong,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_onetimeauth_init(
        state: *mut crypto_onetimeauth_state,
        key: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_onetimeauth_update(
        state: *mut crypto_onetimeauth_state,
        in_: *const libc::c_uchar,
        inlen: libc::c_ulonglong,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_onetimeauth_final(
        state: *mut crypto_onetimeauth_state,
        out: *mut libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_onetimeauth_keygen(k: *mut libc::c_uchar);
}
extern "C" {
    pub fn crypto_pwhash_argon2i_alg_argon2i13() -> libc::c_int;
}
extern "C" {
    pub fn crypto_pwhash_argon2i_bytes_min() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2i_bytes_max() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2i_passwd_min() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2i_passwd_max() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2i_saltbytes() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2i_strbytes() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2i_strprefix() -> *const libc::c_char;
}
extern "C" {
    pub fn crypto_pwhash_argon2i_opslimit_min() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2i_opslimit_max() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2i_memlimit_min() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2i_memlimit_max() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2i_opslimit_interactive() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2i_memlimit_interactive() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2i_opslimit_moderate() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2i_memlimit_moderate() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2i_opslimit_sensitive() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2i_memlimit_sensitive() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2i(
        out: *mut libc::c_uchar,
        outlen: libc::c_ulonglong,
        passwd: *const libc::c_char,
        passwdlen: libc::c_ulonglong,
        salt: *const libc::c_uchar,
        opslimit: libc::c_ulonglong,
        memlimit: usize,
        alg: libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_pwhash_argon2i_str(
        out: *mut libc::c_char,
        passwd: *const libc::c_char,
        passwdlen: libc::c_ulonglong,
        opslimit: libc::c_ulonglong,
        memlimit: usize,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_pwhash_argon2i_str_verify(
        str: *const libc::c_char,
        passwd: *const libc::c_char,
        passwdlen: libc::c_ulonglong,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_pwhash_argon2i_str_needs_rehash(
        str: *const libc::c_char,
        opslimit: libc::c_ulonglong,
        memlimit: usize,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_pwhash_argon2id_alg_argon2id13() -> libc::c_int;
}
extern "C" {
    pub fn crypto_pwhash_argon2id_bytes_min() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2id_bytes_max() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2id_passwd_min() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2id_passwd_max() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2id_saltbytes() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2id_strbytes() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2id_strprefix() -> *const libc::c_char;
}
extern "C" {
    pub fn crypto_pwhash_argon2id_opslimit_min() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2id_opslimit_max() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2id_memlimit_min() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2id_memlimit_max() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2id_opslimit_interactive() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2id_memlimit_interactive() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2id_opslimit_moderate() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2id_memlimit_moderate() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2id_opslimit_sensitive() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2id_memlimit_sensitive() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_argon2id(
        out: *mut libc::c_uchar,
        outlen: libc::c_ulonglong,
        passwd: *const libc::c_char,
        passwdlen: libc::c_ulonglong,
        salt: *const libc::c_uchar,
        opslimit: libc::c_ulonglong,
        memlimit: usize,
        alg: libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_pwhash_argon2id_str(
        out: *mut libc::c_char,
        passwd: *const libc::c_char,
        passwdlen: libc::c_ulonglong,
        opslimit: libc::c_ulonglong,
        memlimit: usize,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_pwhash_argon2id_str_verify(
        str: *const libc::c_char,
        passwd: *const libc::c_char,
        passwdlen: libc::c_ulonglong,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_pwhash_argon2id_str_needs_rehash(
        str: *const libc::c_char,
        opslimit: libc::c_ulonglong,
        memlimit: usize,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_pwhash_alg_argon2i13() -> libc::c_int;
}
extern "C" {
    pub fn crypto_pwhash_alg_argon2id13() -> libc::c_int;
}
extern "C" {
    pub fn crypto_pwhash_alg_default() -> libc::c_int;
}
extern "C" {
    pub fn crypto_pwhash_bytes_min() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_bytes_max() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_passwd_min() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_passwd_max() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_saltbytes() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_strbytes() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_strprefix() -> *const libc::c_char;
}
extern "C" {
    pub fn crypto_pwhash_opslimit_min() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_opslimit_max() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_memlimit_min() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_memlimit_max() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_opslimit_interactive() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_memlimit_interactive() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_opslimit_moderate() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_memlimit_moderate() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_opslimit_sensitive() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_memlimit_sensitive() -> usize;
}
extern "C" {
    pub fn crypto_pwhash(
        out: *mut libc::c_uchar,
        outlen: libc::c_ulonglong,
        passwd: *const libc::c_char,
        passwdlen: libc::c_ulonglong,
        salt: *const libc::c_uchar,
        opslimit: libc::c_ulonglong,
        memlimit: usize,
        alg: libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_pwhash_str(
        out: *mut libc::c_char,
        passwd: *const libc::c_char,
        passwdlen: libc::c_ulonglong,
        opslimit: libc::c_ulonglong,
        memlimit: usize,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_pwhash_str_alg(
        out: *mut libc::c_char,
        passwd: *const libc::c_char,
        passwdlen: libc::c_ulonglong,
        opslimit: libc::c_ulonglong,
        memlimit: usize,
        alg: libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_pwhash_str_verify(
        str: *const libc::c_char,
        passwd: *const libc::c_char,
        passwdlen: libc::c_ulonglong,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_pwhash_str_needs_rehash(
        str: *const libc::c_char,
        opslimit: libc::c_ulonglong,
        memlimit: usize,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_pwhash_primitive() -> *const libc::c_char;
}
extern "C" {
    pub fn crypto_scalarmult_curve25519_bytes() -> usize;
}
extern "C" {
    pub fn crypto_scalarmult_curve25519_scalarbytes() -> usize;
}
extern "C" {
    pub fn crypto_scalarmult_curve25519(
        q: *mut libc::c_uchar,
        n: *const libc::c_uchar,
        p: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_scalarmult_curve25519_base(
        q: *mut libc::c_uchar,
        n: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_scalarmult_bytes() -> usize;
}
extern "C" {
    pub fn crypto_scalarmult_scalarbytes() -> usize;
}
extern "C" {
    pub fn crypto_scalarmult_primitive() -> *const libc::c_char;
}
extern "C" {
    pub fn crypto_scalarmult_base(q: *mut libc::c_uchar, n: *const libc::c_uchar) -> libc::c_int;
}
extern "C" {
    pub fn crypto_scalarmult(
        q: *mut libc::c_uchar,
        n: *const libc::c_uchar,
        p: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_secretbox_xsalsa20poly1305_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_secretbox_xsalsa20poly1305_noncebytes() -> usize;
}
extern "C" {
    pub fn crypto_secretbox_xsalsa20poly1305_macbytes() -> usize;
}
extern "C" {
    pub fn crypto_secretbox_xsalsa20poly1305_messagebytes_max() -> usize;
}
extern "C" {
    pub fn crypto_secretbox_xsalsa20poly1305(
        c: *mut libc::c_uchar,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_secretbox_xsalsa20poly1305_open(
        m: *mut libc::c_uchar,
        c: *const libc::c_uchar,
        clen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_secretbox_xsalsa20poly1305_keygen(k: *mut libc::c_uchar);
}
extern "C" {
    pub fn crypto_secretbox_xsalsa20poly1305_boxzerobytes() -> usize;
}
extern "C" {
    pub fn crypto_secretbox_xsalsa20poly1305_zerobytes() -> usize;
}
extern "C" {
    pub fn crypto_secretbox_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_secretbox_noncebytes() -> usize;
}
extern "C" {
    pub fn crypto_secretbox_macbytes() -> usize;
}
extern "C" {
    pub fn crypto_secretbox_primitive() -> *const libc::c_char;
}
extern "C" {
    pub fn crypto_secretbox_messagebytes_max() -> usize;
}
extern "C" {
    pub fn crypto_secretbox_easy(
        c: *mut libc::c_uchar,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_secretbox_open_easy(
        m: *mut libc::c_uchar,
        c: *const libc::c_uchar,
        clen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_secretbox_detached(
        c: *mut libc::c_uchar,
        mac: *mut libc::c_uchar,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_secretbox_open_detached(
        m: *mut libc::c_uchar,
        c: *const libc::c_uchar,
        mac: *const libc::c_uchar,
        clen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_secretbox_keygen(k: *mut libc::c_uchar);
}
extern "C" {
    pub fn crypto_secretbox_zerobytes() -> usize;
}
extern "C" {
    pub fn crypto_secretbox_boxzerobytes() -> usize;
}
extern "C" {
    pub fn crypto_secretbox(
        c: *mut libc::c_uchar,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_secretbox_open(
        m: *mut libc::c_uchar,
        c: *const libc::c_uchar,
        clen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_stream_chacha20_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_stream_chacha20_noncebytes() -> usize;
}
extern "C" {
    pub fn crypto_stream_chacha20_messagebytes_max() -> usize;
}
extern "C" {
    pub fn crypto_stream_chacha20(
        c: *mut libc::c_uchar,
        clen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_stream_chacha20_xor(
        c: *mut libc::c_uchar,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_stream_chacha20_xor_ic(
        c: *mut libc::c_uchar,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        ic: u64,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_stream_chacha20_keygen(k: *mut libc::c_uchar);
}
extern "C" {
    pub fn crypto_stream_chacha20_ietf_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_stream_chacha20_ietf_noncebytes() -> usize;
}
extern "C" {
    pub fn crypto_stream_chacha20_ietf_messagebytes_max() -> usize;
}
extern "C" {
    pub fn crypto_stream_chacha20_ietf(
        c: *mut libc::c_uchar,
        clen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_stream_chacha20_ietf_xor(
        c: *mut libc::c_uchar,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_stream_chacha20_ietf_xor_ic(
        c: *mut libc::c_uchar,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        ic: u32,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_stream_chacha20_ietf_keygen(k: *mut libc::c_uchar);
}
extern "C" {
    pub fn crypto_secretstream_xchacha20poly1305_abytes() -> usize;
}
extern "C" {
    pub fn crypto_secretstream_xchacha20poly1305_headerbytes() -> usize;
}
extern "C" {
    pub fn crypto_secretstream_xchacha20poly1305_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_secretstream_xchacha20poly1305_messagebytes_max() -> usize;
}
extern "C" {
    pub fn crypto_secretstream_xchacha20poly1305_tag_message() -> libc::c_uchar;
}
extern "C" {
    pub fn crypto_secretstream_xchacha20poly1305_tag_push() -> libc::c_uchar;
}
extern "C" {
    pub fn crypto_secretstream_xchacha20poly1305_tag_rekey() -> libc::c_uchar;
}
extern "C" {
    pub fn crypto_secretstream_xchacha20poly1305_tag_final() -> libc::c_uchar;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct crypto_secretstream_xchacha20poly1305_state {
    pub k: [libc::c_uchar; 32usize],
    pub nonce: [libc::c_uchar; 12usize],
    pub _pad: [libc::c_uchar; 8usize],
}
#[test]
fn bindgen_test_layout_crypto_secretstream_xchacha20poly1305_state() {
    assert_eq!(
        ::std::mem::size_of::<crypto_secretstream_xchacha20poly1305_state>(),
        52usize,
        concat!(
            "Size of: ",
            stringify!(crypto_secretstream_xchacha20poly1305_state)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<crypto_secretstream_xchacha20poly1305_state>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(crypto_secretstream_xchacha20poly1305_state)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<crypto_secretstream_xchacha20poly1305_state>())).k as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_secretstream_xchacha20poly1305_state),
            "::",
            stringify!(k)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<crypto_secretstream_xchacha20poly1305_state>())).nonce
                as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_secretstream_xchacha20poly1305_state),
            "::",
            stringify!(nonce)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<crypto_secretstream_xchacha20poly1305_state>()))._pad as *const _
                as usize
        },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_secretstream_xchacha20poly1305_state),
            "::",
            stringify!(_pad)
        )
    );
}
extern "C" {
    pub fn crypto_secretstream_xchacha20poly1305_statebytes() -> usize;
}
extern "C" {
    pub fn crypto_secretstream_xchacha20poly1305_keygen(k: *mut libc::c_uchar);
}
extern "C" {
    pub fn crypto_secretstream_xchacha20poly1305_init_push(
        state: *mut crypto_secretstream_xchacha20poly1305_state,
        header: *mut libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_secretstream_xchacha20poly1305_push(
        state: *mut crypto_secretstream_xchacha20poly1305_state,
        c: *mut libc::c_uchar,
        clen_p: *mut libc::c_ulonglong,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        ad: *const libc::c_uchar,
        adlen: libc::c_ulonglong,
        tag: libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_secretstream_xchacha20poly1305_init_pull(
        state: *mut crypto_secretstream_xchacha20poly1305_state,
        header: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_secretstream_xchacha20poly1305_pull(
        state: *mut crypto_secretstream_xchacha20poly1305_state,
        m: *mut libc::c_uchar,
        mlen_p: *mut libc::c_ulonglong,
        tag_p: *mut libc::c_uchar,
        c: *const libc::c_uchar,
        clen: libc::c_ulonglong,
        ad: *const libc::c_uchar,
        adlen: libc::c_ulonglong,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_secretstream_xchacha20poly1305_rekey(
        state: *mut crypto_secretstream_xchacha20poly1305_state,
    );
}
extern "C" {
    pub fn crypto_shorthash_siphash24_bytes() -> usize;
}
extern "C" {
    pub fn crypto_shorthash_siphash24_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_shorthash_siphash24(
        out: *mut libc::c_uchar,
        in_: *const libc::c_uchar,
        inlen: libc::c_ulonglong,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_shorthash_siphashx24_bytes() -> usize;
}
extern "C" {
    pub fn crypto_shorthash_siphashx24_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_shorthash_siphashx24(
        out: *mut libc::c_uchar,
        in_: *const libc::c_uchar,
        inlen: libc::c_ulonglong,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_shorthash_bytes() -> usize;
}
extern "C" {
    pub fn crypto_shorthash_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_shorthash_primitive() -> *const libc::c_char;
}
extern "C" {
    pub fn crypto_shorthash(
        out: *mut libc::c_uchar,
        in_: *const libc::c_uchar,
        inlen: libc::c_ulonglong,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_shorthash_keygen(k: *mut libc::c_uchar);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_sign_ed25519ph_state {
    pub hs: crypto_hash_sha512_state,
}
#[test]
fn bindgen_test_layout_crypto_sign_ed25519ph_state() {
    assert_eq!(
        ::std::mem::size_of::<crypto_sign_ed25519ph_state>(),
        208usize,
        concat!("Size of: ", stringify!(crypto_sign_ed25519ph_state))
    );
    assert_eq!(
        ::std::mem::align_of::<crypto_sign_ed25519ph_state>(),
        8usize,
        concat!("Alignment of ", stringify!(crypto_sign_ed25519ph_state))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_sign_ed25519ph_state>())).hs as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_sign_ed25519ph_state),
            "::",
            stringify!(hs)
        )
    );
}
extern "C" {
    pub fn crypto_sign_ed25519ph_statebytes() -> usize;
}
extern "C" {
    pub fn crypto_sign_ed25519_bytes() -> usize;
}
extern "C" {
    pub fn crypto_sign_ed25519_seedbytes() -> usize;
}
extern "C" {
    pub fn crypto_sign_ed25519_publickeybytes() -> usize;
}
extern "C" {
    pub fn crypto_sign_ed25519_secretkeybytes() -> usize;
}
extern "C" {
    pub fn crypto_sign_ed25519_messagebytes_max() -> usize;
}
extern "C" {
    pub fn crypto_sign_ed25519(
        sm: *mut libc::c_uchar,
        smlen_p: *mut libc::c_ulonglong,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        sk: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_sign_ed25519_open(
        m: *mut libc::c_uchar,
        mlen_p: *mut libc::c_ulonglong,
        sm: *const libc::c_uchar,
        smlen: libc::c_ulonglong,
        pk: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_sign_ed25519_detached(
        sig: *mut libc::c_uchar,
        siglen_p: *mut libc::c_ulonglong,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        sk: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_sign_ed25519_verify_detached(
        sig: *const libc::c_uchar,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        pk: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_sign_ed25519_keypair(
        pk: *mut libc::c_uchar,
        sk: *mut libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_sign_ed25519_seed_keypair(
        pk: *mut libc::c_uchar,
        sk: *mut libc::c_uchar,
        seed: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_sign_ed25519_pk_to_curve25519(
        curve25519_pk: *mut libc::c_uchar,
        ed25519_pk: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_sign_ed25519_sk_to_curve25519(
        curve25519_sk: *mut libc::c_uchar,
        ed25519_sk: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_sign_ed25519_sk_to_seed(
        seed: *mut libc::c_uchar,
        sk: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_sign_ed25519_sk_to_pk(
        pk: *mut libc::c_uchar,
        sk: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_sign_ed25519ph_init(state: *mut crypto_sign_ed25519ph_state) -> libc::c_int;
}
extern "C" {
    pub fn crypto_sign_ed25519ph_update(
        state: *mut crypto_sign_ed25519ph_state,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_sign_ed25519ph_final_create(
        state: *mut crypto_sign_ed25519ph_state,
        sig: *mut libc::c_uchar,
        siglen_p: *mut libc::c_ulonglong,
        sk: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_sign_ed25519ph_final_verify(
        state: *mut crypto_sign_ed25519ph_state,
        sig: *const libc::c_uchar,
        pk: *const libc::c_uchar,
    ) -> libc::c_int;
}
pub type crypto_sign_state = crypto_sign_ed25519ph_state;
extern "C" {
    pub fn crypto_sign_statebytes() -> usize;
}
extern "C" {
    pub fn crypto_sign_bytes() -> usize;
}
extern "C" {
    pub fn crypto_sign_seedbytes() -> usize;
}
extern "C" {
    pub fn crypto_sign_publickeybytes() -> usize;
}
extern "C" {
    pub fn crypto_sign_secretkeybytes() -> usize;
}
extern "C" {
    pub fn crypto_sign_messagebytes_max() -> usize;
}
extern "C" {
    pub fn crypto_sign_primitive() -> *const libc::c_char;
}
extern "C" {
    pub fn crypto_sign_seed_keypair(
        pk: *mut libc::c_uchar,
        sk: *mut libc::c_uchar,
        seed: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_sign_keypair(pk: *mut libc::c_uchar, sk: *mut libc::c_uchar) -> libc::c_int;
}
extern "C" {
    pub fn crypto_sign(
        sm: *mut libc::c_uchar,
        smlen_p: *mut libc::c_ulonglong,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        sk: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_sign_open(
        m: *mut libc::c_uchar,
        mlen_p: *mut libc::c_ulonglong,
        sm: *const libc::c_uchar,
        smlen: libc::c_ulonglong,
        pk: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_sign_detached(
        sig: *mut libc::c_uchar,
        siglen_p: *mut libc::c_ulonglong,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        sk: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_sign_verify_detached(
        sig: *const libc::c_uchar,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        pk: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_sign_init(state: *mut crypto_sign_state) -> libc::c_int;
}
extern "C" {
    pub fn crypto_sign_update(
        state: *mut crypto_sign_state,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_sign_final_create(
        state: *mut crypto_sign_state,
        sig: *mut libc::c_uchar,
        siglen_p: *mut libc::c_ulonglong,
        sk: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_sign_final_verify(
        state: *mut crypto_sign_state,
        sig: *const libc::c_uchar,
        pk: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_stream_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_stream_noncebytes() -> usize;
}
extern "C" {
    pub fn crypto_stream_messagebytes_max() -> usize;
}
extern "C" {
    pub fn crypto_stream_primitive() -> *const libc::c_char;
}
extern "C" {
    pub fn crypto_stream(
        c: *mut libc::c_uchar,
        clen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_stream_xor(
        c: *mut libc::c_uchar,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_stream_keygen(k: *mut libc::c_uchar);
}
extern "C" {
    pub fn crypto_stream_salsa20_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_stream_salsa20_noncebytes() -> usize;
}
extern "C" {
    pub fn crypto_stream_salsa20_messagebytes_max() -> usize;
}
extern "C" {
    pub fn crypto_stream_salsa20(
        c: *mut libc::c_uchar,
        clen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_stream_salsa20_xor(
        c: *mut libc::c_uchar,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_stream_salsa20_xor_ic(
        c: *mut libc::c_uchar,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        ic: u64,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_stream_salsa20_keygen(k: *mut libc::c_uchar);
}
extern "C" {
    pub fn crypto_verify_16_bytes() -> usize;
}
extern "C" {
    pub fn crypto_verify_16(x: *const libc::c_uchar, y: *const libc::c_uchar) -> libc::c_int;
}
extern "C" {
    pub fn crypto_verify_32_bytes() -> usize;
}
extern "C" {
    pub fn crypto_verify_32(x: *const libc::c_uchar, y: *const libc::c_uchar) -> libc::c_int;
}
extern "C" {
    pub fn crypto_verify_64_bytes() -> usize;
}
extern "C" {
    pub fn crypto_verify_64(x: *const libc::c_uchar, y: *const libc::c_uchar) -> libc::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct randombytes_implementation {
    pub implementation_name: ::std::option::Option<unsafe extern "C" fn() -> *const libc::c_char>,
    pub random: ::std::option::Option<unsafe extern "C" fn() -> u32>,
    pub stir: ::std::option::Option<unsafe extern "C" fn()>,
    pub uniform: ::std::option::Option<unsafe extern "C" fn(upper_bound: u32) -> u32>,
    pub buf: ::std::option::Option<unsafe extern "C" fn(buf: *mut libc::c_void, size: usize)>,
    pub close: ::std::option::Option<unsafe extern "C" fn() -> libc::c_int>,
}
#[test]
fn bindgen_test_layout_randombytes_implementation() {
    assert_eq!(
        ::std::mem::size_of::<randombytes_implementation>(),
        if cfg!(target_pointer_width = "64") {
            48usize
        } else {
            24usize
        },
        concat!("Size of: ", stringify!(randombytes_implementation))
    );
    assert_eq!(
        ::std::mem::align_of::<randombytes_implementation>(),
        if cfg!(target_pointer_width = "64") {
            8usize
        } else {
            4usize
        },
        concat!("Alignment of ", stringify!(randombytes_implementation))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<randombytes_implementation>())).implementation_name as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(randombytes_implementation),
            "::",
            stringify!(implementation_name)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<randombytes_implementation>())).random as *const _ as usize
        },
        if cfg!(target_pointer_width = "64") {
            8usize
        } else {
            4usize
        },
        concat!(
            "Offset of field: ",
            stringify!(randombytes_implementation),
            "::",
            stringify!(random)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<randombytes_implementation>())).stir as *const _ as usize },
        if cfg!(target_pointer_width = "64") {
            16usize
        } else {
            8usize
        },
        concat!(
            "Offset of field: ",
            stringify!(randombytes_implementation),
            "::",
            stringify!(stir)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<randombytes_implementation>())).uniform as *const _ as usize
        },
        if cfg!(target_pointer_width = "64") {
            24usize
        } else {
            12usize
        },
        concat!(
            "Offset of field: ",
            stringify!(randombytes_implementation),
            "::",
            stringify!(uniform)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<randombytes_implementation>())).buf as *const _ as usize },
        if cfg!(target_pointer_width = "64") {
            32usize
        } else {
            16usize
        },
        concat!(
            "Offset of field: ",
            stringify!(randombytes_implementation),
            "::",
            stringify!(buf)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<randombytes_implementation>())).close as *const _ as usize
        },
        if cfg!(target_pointer_width = "64") {
            40usize
        } else {
            20usize
        },
        concat!(
            "Offset of field: ",
            stringify!(randombytes_implementation),
            "::",
            stringify!(close)
        )
    );
}
extern "C" {
    pub fn randombytes_seedbytes() -> usize;
}
extern "C" {
    pub fn randombytes_buf(buf: *mut libc::c_void, size: usize);
}
extern "C" {
    pub fn randombytes_buf_deterministic(
        buf: *mut libc::c_void,
        size: usize,
        seed: *const libc::c_uchar,
    );
}
extern "C" {
    pub fn randombytes_random() -> u32;
}
extern "C" {
    pub fn randombytes_uniform(upper_bound: u32) -> u32;
}
extern "C" {
    pub fn randombytes_stir();
}
extern "C" {
    pub fn randombytes_close() -> libc::c_int;
}
extern "C" {
    pub fn randombytes_set_implementation(impl_: *mut randombytes_implementation) -> libc::c_int;
}
extern "C" {
    pub fn randombytes_implementation_name() -> *const libc::c_char;
}
extern "C" {
    pub fn randombytes(buf: *mut libc::c_uchar, buf_len: libc::c_ulonglong);
}
extern "C" {
    pub static mut randombytes_salsa20_implementation: randombytes_implementation;
}
extern "C" {
    pub static mut randombytes_sysrandom_implementation: randombytes_implementation;
}
extern "C" {
    pub fn sodium_runtime_has_neon() -> libc::c_int;
}
extern "C" {
    pub fn sodium_runtime_has_sse2() -> libc::c_int;
}
extern "C" {
    pub fn sodium_runtime_has_sse3() -> libc::c_int;
}
extern "C" {
    pub fn sodium_runtime_has_ssse3() -> libc::c_int;
}
extern "C" {
    pub fn sodium_runtime_has_sse41() -> libc::c_int;
}
extern "C" {
    pub fn sodium_runtime_has_avx() -> libc::c_int;
}
extern "C" {
    pub fn sodium_runtime_has_avx2() -> libc::c_int;
}
extern "C" {
    pub fn sodium_runtime_has_avx512f() -> libc::c_int;
}
extern "C" {
    pub fn sodium_runtime_has_pclmul() -> libc::c_int;
}
extern "C" {
    pub fn sodium_runtime_has_aesni() -> libc::c_int;
}
extern "C" {
    pub fn sodium_runtime_has_rdrand() -> libc::c_int;
}
extern "C" {
    pub fn _sodium_runtime_get_cpu_features() -> libc::c_int;
}
extern "C" {
    pub fn sodium_memzero(pnt: *mut libc::c_void, len: usize);
}
extern "C" {
    pub fn sodium_stackzero(len: usize);
}
extern "C" {
    pub fn sodium_memcmp(
        b1_: *const libc::c_void,
        b2_: *const libc::c_void,
        len: usize,
    ) -> libc::c_int;
}
extern "C" {
    pub fn sodium_compare(
        b1_: *const libc::c_uchar,
        b2_: *const libc::c_uchar,
        len: usize,
    ) -> libc::c_int;
}
extern "C" {
    pub fn sodium_is_zero(n: *const libc::c_uchar, nlen: usize) -> libc::c_int;
}
extern "C" {
    pub fn sodium_increment(n: *mut libc::c_uchar, nlen: usize);
}
extern "C" {
    pub fn sodium_add(a: *mut libc::c_uchar, b: *const libc::c_uchar, len: usize);
}
extern "C" {
    pub fn sodium_sub(a: *mut libc::c_uchar, b: *const libc::c_uchar, len: usize);
}
extern "C" {
    pub fn sodium_bin2hex(
        hex: *mut libc::c_char,
        hex_maxlen: usize,
        bin: *const libc::c_uchar,
        bin_len: usize,
    ) -> *mut libc::c_char;
}
extern "C" {
    pub fn sodium_hex2bin(
        bin: *mut libc::c_uchar,
        bin_maxlen: usize,
        hex: *const libc::c_char,
        hex_len: usize,
        ignore: *const libc::c_char,
        bin_len: *mut usize,
        hex_end: *mut *const libc::c_char,
    ) -> libc::c_int;
}
extern "C" {
    pub fn sodium_base64_encoded_len(bin_len: usize, variant: libc::c_int) -> usize;
}
extern "C" {
    pub fn sodium_bin2base64(
        b64: *mut libc::c_char,
        b64_maxlen: usize,
        bin: *const libc::c_uchar,
        bin_len: usize,
        variant: libc::c_int,
    ) -> *mut libc::c_char;
}
extern "C" {
    pub fn sodium_base642bin(
        bin: *mut libc::c_uchar,
        bin_maxlen: usize,
        b64: *const libc::c_char,
        b64_len: usize,
        ignore: *const libc::c_char,
        bin_len: *mut usize,
        b64_end: *mut *const libc::c_char,
        variant: libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    pub fn sodium_mlock(addr: *mut libc::c_void, len: usize) -> libc::c_int;
}
extern "C" {
    pub fn sodium_munlock(addr: *mut libc::c_void, len: usize) -> libc::c_int;
}
extern "C" {
    pub fn sodium_malloc(size: usize) -> *mut libc::c_void;
}
extern "C" {
    pub fn sodium_allocarray(count: usize, size: usize) -> *mut libc::c_void;
}
extern "C" {
    pub fn sodium_free(ptr: *mut libc::c_void);
}
extern "C" {
    pub fn sodium_mprotect_noaccess(ptr: *mut libc::c_void) -> libc::c_int;
}
extern "C" {
    pub fn sodium_mprotect_readonly(ptr: *mut libc::c_void) -> libc::c_int;
}
extern "C" {
    pub fn sodium_mprotect_readwrite(ptr: *mut libc::c_void) -> libc::c_int;
}
extern "C" {
    pub fn sodium_pad(
        padded_buflen_p: *mut usize,
        buf: *mut libc::c_uchar,
        unpadded_buflen: usize,
        blocksize: usize,
        max_buflen: usize,
    ) -> libc::c_int;
}
extern "C" {
    pub fn sodium_unpad(
        unpadded_buflen_p: *mut usize,
        buf: *const libc::c_uchar,
        padded_buflen: usize,
        blocksize: usize,
    ) -> libc::c_int;
}
extern "C" {
    pub fn _sodium_alloc_init() -> libc::c_int;
}
extern "C" {
    pub fn crypto_stream_xchacha20_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_stream_xchacha20_noncebytes() -> usize;
}
extern "C" {
    pub fn crypto_stream_xchacha20_messagebytes_max() -> usize;
}
extern "C" {
    pub fn crypto_stream_xchacha20(
        c: *mut libc::c_uchar,
        clen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_stream_xchacha20_xor(
        c: *mut libc::c_uchar,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_stream_xchacha20_xor_ic(
        c: *mut libc::c_uchar,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        ic: u64,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_stream_xchacha20_keygen(k: *mut libc::c_uchar);
}
extern "C" {
    pub fn crypto_box_curve25519xchacha20poly1305_seedbytes() -> usize;
}
extern "C" {
    pub fn crypto_box_curve25519xchacha20poly1305_publickeybytes() -> usize;
}
extern "C" {
    pub fn crypto_box_curve25519xchacha20poly1305_secretkeybytes() -> usize;
}
extern "C" {
    pub fn crypto_box_curve25519xchacha20poly1305_beforenmbytes() -> usize;
}
extern "C" {
    pub fn crypto_box_curve25519xchacha20poly1305_noncebytes() -> usize;
}
extern "C" {
    pub fn crypto_box_curve25519xchacha20poly1305_macbytes() -> usize;
}
extern "C" {
    pub fn crypto_box_curve25519xchacha20poly1305_messagebytes_max() -> usize;
}
extern "C" {
    pub fn crypto_box_curve25519xchacha20poly1305_seed_keypair(
        pk: *mut libc::c_uchar,
        sk: *mut libc::c_uchar,
        seed: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_box_curve25519xchacha20poly1305_keypair(
        pk: *mut libc::c_uchar,
        sk: *mut libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_box_curve25519xchacha20poly1305_easy(
        c: *mut libc::c_uchar,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        pk: *const libc::c_uchar,
        sk: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_box_curve25519xchacha20poly1305_open_easy(
        m: *mut libc::c_uchar,
        c: *const libc::c_uchar,
        clen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        pk: *const libc::c_uchar,
        sk: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_box_curve25519xchacha20poly1305_detached(
        c: *mut libc::c_uchar,
        mac: *mut libc::c_uchar,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        pk: *const libc::c_uchar,
        sk: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_box_curve25519xchacha20poly1305_open_detached(
        m: *mut libc::c_uchar,
        c: *const libc::c_uchar,
        mac: *const libc::c_uchar,
        clen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        pk: *const libc::c_uchar,
        sk: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_box_curve25519xchacha20poly1305_beforenm(
        k: *mut libc::c_uchar,
        pk: *const libc::c_uchar,
        sk: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_box_curve25519xchacha20poly1305_easy_afternm(
        c: *mut libc::c_uchar,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_box_curve25519xchacha20poly1305_open_easy_afternm(
        m: *mut libc::c_uchar,
        c: *const libc::c_uchar,
        clen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_box_curve25519xchacha20poly1305_detached_afternm(
        c: *mut libc::c_uchar,
        mac: *mut libc::c_uchar,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_box_curve25519xchacha20poly1305_open_detached_afternm(
        m: *mut libc::c_uchar,
        c: *const libc::c_uchar,
        mac: *const libc::c_uchar,
        clen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_box_curve25519xchacha20poly1305_sealbytes() -> usize;
}
extern "C" {
    pub fn crypto_box_curve25519xchacha20poly1305_seal(
        c: *mut libc::c_uchar,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        pk: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_box_curve25519xchacha20poly1305_seal_open(
        m: *mut libc::c_uchar,
        c: *const libc::c_uchar,
        clen: libc::c_ulonglong,
        pk: *const libc::c_uchar,
        sk: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_core_ed25519_bytes() -> usize;
}
extern "C" {
    pub fn crypto_core_ed25519_uniformbytes() -> usize;
}
extern "C" {
    pub fn crypto_core_ed25519_scalarbytes() -> usize;
}
extern "C" {
    pub fn crypto_core_ed25519_nonreducedscalarbytes() -> usize;
}
extern "C" {
    pub fn crypto_core_ed25519_is_valid_point(p: *const libc::c_uchar) -> libc::c_int;
}
extern "C" {
    pub fn crypto_core_ed25519_add(
        r: *mut libc::c_uchar,
        p: *const libc::c_uchar,
        q: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_core_ed25519_sub(
        r: *mut libc::c_uchar,
        p: *const libc::c_uchar,
        q: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_core_ed25519_from_uniform(
        p: *mut libc::c_uchar,
        r: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_core_ed25519_scalar_random(r: *mut libc::c_uchar);
}
extern "C" {
    pub fn crypto_core_ed25519_scalar_invert(
        recip: *mut libc::c_uchar,
        s: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_core_ed25519_scalar_negate(neg: *mut libc::c_uchar, s: *const libc::c_uchar);
}
extern "C" {
    pub fn crypto_core_ed25519_scalar_complement(comp: *mut libc::c_uchar, s: *const libc::c_uchar);
}
extern "C" {
    pub fn crypto_core_ed25519_scalar_add(
        z: *mut libc::c_uchar,
        x: *const libc::c_uchar,
        y: *const libc::c_uchar,
    );
}
extern "C" {
    pub fn crypto_core_ed25519_scalar_sub(
        z: *mut libc::c_uchar,
        x: *const libc::c_uchar,
        y: *const libc::c_uchar,
    );
}
extern "C" {
    pub fn crypto_core_ed25519_scalar_reduce(r: *mut libc::c_uchar, s: *const libc::c_uchar);
}
extern "C" {
    pub fn crypto_scalarmult_ed25519_bytes() -> usize;
}
extern "C" {
    pub fn crypto_scalarmult_ed25519_scalarbytes() -> usize;
}
extern "C" {
    pub fn crypto_scalarmult_ed25519(
        q: *mut libc::c_uchar,
        n: *const libc::c_uchar,
        p: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_scalarmult_ed25519_noclamp(
        q: *mut libc::c_uchar,
        n: *const libc::c_uchar,
        p: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_scalarmult_ed25519_base(
        q: *mut libc::c_uchar,
        n: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_scalarmult_ed25519_base_noclamp(
        q: *mut libc::c_uchar,
        n: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_secretbox_xchacha20poly1305_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_secretbox_xchacha20poly1305_noncebytes() -> usize;
}
extern "C" {
    pub fn crypto_secretbox_xchacha20poly1305_macbytes() -> usize;
}
extern "C" {
    pub fn crypto_secretbox_xchacha20poly1305_messagebytes_max() -> usize;
}
extern "C" {
    pub fn crypto_secretbox_xchacha20poly1305_easy(
        c: *mut libc::c_uchar,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_secretbox_xchacha20poly1305_open_easy(
        m: *mut libc::c_uchar,
        c: *const libc::c_uchar,
        clen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_secretbox_xchacha20poly1305_detached(
        c: *mut libc::c_uchar,
        mac: *mut libc::c_uchar,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_secretbox_xchacha20poly1305_open_detached(
        m: *mut libc::c_uchar,
        c: *const libc::c_uchar,
        mac: *const libc::c_uchar,
        clen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_pwhash_scryptsalsa208sha256_bytes_min() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_scryptsalsa208sha256_bytes_max() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_scryptsalsa208sha256_passwd_min() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_scryptsalsa208sha256_passwd_max() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_scryptsalsa208sha256_saltbytes() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_scryptsalsa208sha256_strbytes() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_scryptsalsa208sha256_strprefix() -> *const libc::c_char;
}
extern "C" {
    pub fn crypto_pwhash_scryptsalsa208sha256_opslimit_min() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_scryptsalsa208sha256_opslimit_max() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_scryptsalsa208sha256_memlimit_min() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_scryptsalsa208sha256_memlimit_max() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_scryptsalsa208sha256_opslimit_interactive() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_scryptsalsa208sha256_memlimit_interactive() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_scryptsalsa208sha256_opslimit_sensitive() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_scryptsalsa208sha256_memlimit_sensitive() -> usize;
}
extern "C" {
    pub fn crypto_pwhash_scryptsalsa208sha256(
        out: *mut libc::c_uchar,
        outlen: libc::c_ulonglong,
        passwd: *const libc::c_char,
        passwdlen: libc::c_ulonglong,
        salt: *const libc::c_uchar,
        opslimit: libc::c_ulonglong,
        memlimit: usize,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_pwhash_scryptsalsa208sha256_str(
        out: *mut libc::c_char,
        passwd: *const libc::c_char,
        passwdlen: libc::c_ulonglong,
        opslimit: libc::c_ulonglong,
        memlimit: usize,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_pwhash_scryptsalsa208sha256_str_verify(
        str: *const libc::c_char,
        passwd: *const libc::c_char,
        passwdlen: libc::c_ulonglong,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_pwhash_scryptsalsa208sha256_ll(
        passwd: *const u8,
        passwdlen: usize,
        salt: *const u8,
        saltlen: usize,
        N: u64,
        r: u32,
        p: u32,
        buf: *mut u8,
        buflen: usize,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_pwhash_scryptsalsa208sha256_str_needs_rehash(
        str: *const libc::c_char,
        opslimit: libc::c_ulonglong,
        memlimit: usize,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_stream_salsa2012_keybytes() -> usize;
}
extern "C" {
    pub fn crypto_stream_salsa2012_noncebytes() -> usize;
}
extern "C" {
    pub fn crypto_stream_salsa2012_messagebytes_max() -> usize;
}
extern "C" {
    pub fn crypto_stream_salsa2012(
        c: *mut libc::c_uchar,
        clen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_stream_salsa2012_xor(
        c: *mut libc::c_uchar,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    pub fn crypto_stream_salsa2012_keygen(k: *mut libc::c_uchar);
}
extern "C" {
    #[deprecated]
    pub fn crypto_stream_salsa208_keybytes() -> usize;
}
extern "C" {
    #[deprecated]
    pub fn crypto_stream_salsa208_noncebytes() -> usize;
}
extern "C" {
    #[deprecated]
    pub fn crypto_stream_salsa208_messagebytes_max() -> usize;
}
extern "C" {
    #[deprecated]
    pub fn crypto_stream_salsa208(
        c: *mut libc::c_uchar,
        clen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    #[deprecated]
    pub fn crypto_stream_salsa208_xor(
        c: *mut libc::c_uchar,
        m: *const libc::c_uchar,
        mlen: libc::c_ulonglong,
        n: *const libc::c_uchar,
        k: *const libc::c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    #[deprecated]
    pub fn crypto_stream_salsa208_keygen(k: *mut libc::c_uchar);
}
