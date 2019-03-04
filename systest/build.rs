// Copyright 2018 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under the MIT license <LICENSE-MIT
// http://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.

extern crate ctest;
#[macro_use]
extern crate unwrap;

use std::env;

fn main() {
    let include = unwrap!(env::var("DEP_SODIUM_INCLUDE"));
    let mut cfg = ctest::TestGenerator::new();

    cfg.header("sodium.h")
        .header("sodium/core.h")
        .header("sodium/crypto_aead_aes256gcm.h")
        .header("sodium/crypto_aead_chacha20poly1305.h")
        .header("sodium/crypto_aead_xchacha20poly1305.h")
        .header("sodium/crypto_auth.h")
        .header("sodium/crypto_auth_hmacsha256.h")
        .header("sodium/crypto_auth_hmacsha512256.h")
        .header("sodium/crypto_auth_hmacsha512.h")
        .header("sodium/crypto_box_curve25519xchacha20poly1305.h")
        .header("sodium/crypto_box_curve25519xsalsa20poly1305.h")
        .header("sodium/crypto_box.h")
        .header("sodium/crypto_core_ed25519.h")
        .header("sodium/crypto_core_hchacha20.h")
        .header("sodium/crypto_core_hsalsa20.h")
        .header("sodium/crypto_core_salsa2012.h")
        .header("sodium/crypto_core_salsa208.h")
        .header("sodium/crypto_core_salsa20.h")
        .header("sodium/crypto_generichash_blake2b.h")
        .header("sodium/crypto_generichash.h")
        .header("sodium/crypto_hash.h")
        .header("sodium/crypto_hash_sha256.h")
        .header("sodium/crypto_hash_sha512.h")
        .header("sodium/crypto_kdf_blake2b.h")
        .header("sodium/crypto_kdf.h")
        .header("sodium/crypto_kx.h")
        .header("sodium/crypto_onetimeauth.h")
        .header("sodium/crypto_onetimeauth_poly1305.h")
        .header("sodium/crypto_pwhash_argon2id.h")
        .header("sodium/crypto_pwhash_argon2i.h")
        .header("sodium/crypto_pwhash.h")
        .header("sodium/crypto_pwhash_scryptsalsa208sha256.h")
        .header("sodium/crypto_scalarmult_curve25519.h")
        .header("sodium/crypto_scalarmult_ed25519.h")
        .header("sodium/crypto_scalarmult.h")
        .header("sodium/crypto_secretbox.h")
        .header("sodium/crypto_secretbox_xchacha20poly1305.h")
        .header("sodium/crypto_secretbox_xsalsa20poly1305.h")
        .header("sodium/crypto_secretstream_xchacha20poly1305.h")
        .header("sodium/crypto_shorthash.h")
        .header("sodium/crypto_shorthash_siphash24.h")
        .header("sodium/crypto_sign_ed25519.h")
        .header("sodium/crypto_sign_edwards25519sha512batch.h")
        .header("sodium/crypto_sign.h")
        .header("sodium/crypto_stream_chacha20.h")
        .header("sodium/crypto_stream.h")
        .header("sodium/crypto_stream_salsa2012.h")
        .header("sodium/crypto_stream_salsa208.h")
        .header("sodium/crypto_stream_salsa20.h")
        .header("sodium/crypto_stream_xchacha20.h")
        .header("sodium/crypto_stream_xsalsa20.h")
        .header("sodium/crypto_verify_16.h")
        .header("sodium/crypto_verify_32.h")
        .header("sodium/crypto_verify_64.h")
        .header("sodium/export.h")
        .header("sodium/randombytes.h")
        .header("sodium/randombytes_salsa20_random.h")
        .header("sodium/randombytes_sysrandom.h")
        .header("sodium/runtime.h")
        .header("sodium/utils.h")
        .header("sodium/version.h");
    cfg.include(&include);
    cfg.type_name(|s, _| s.to_string());
    cfg.skip_field(|_, field| field.starts_with("__bindgen_padding_"));
    cfg.skip_type(|_| true);
    cfg.skip_const(|s| {
        s.ends_with("_PRIMITIVE") || s.ends_with("_STRPREFIX") || s == "SODIUM_VERSION_STRING"
    });
    if cfg!(target_env = "msvc") {
        // Suppress "warning C4324: 'crypto_generichash_blake2b_state': structure was padded due to
        // alignment specifier"
        cfg.flag("/wd4324");
    }
    cfg.generate("../rust_sodium-sys/src/lib.rs", "all.rs");
}
