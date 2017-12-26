#![doc(html_logo_url =
           "https://raw.githubusercontent.com/maidsafe/QA/master/Images/maidsafe_logo.png",
       html_favicon_url = "https://maidsafe.net/img/favicon.ico",
       html_root_url = "https://docs.rs/rust_sodium/master/rust_sodium_sys")]

// For explanation of lint checks, run `rustc -W help` or see
// https://github.com/maidsafe/QA/blob/master/Documentation/Rust%20Lint%20Checks.md
#![forbid(exceeding_bitshifts, mutable_transmutes, no_mangle_const_items,
          unknown_crate_types, warnings)]
#![deny(bad_style, deprecated, improper_ctypes, non_shorthand_field_patterns,
        overflowing_literals, plugin_as_library, private_no_mangle_fns, private_no_mangle_statics,
        stable_features, unconditional_recursion, unknown_lints, unused, unused_allocation,
        unused_attributes, unused_comparisons, unused_features, unused_parens, while_true)]
#![warn(unused_extern_crates, unused_import_braces, unused_qualifications, unused_results)]
// Allow `trivial_casts` to cast `u8` to `c_char`, which is `u8` or `i8`, depending on the
// architecture.
#![allow(box_pointers, missing_copy_implementations,
         missing_debug_implementations, missing_docs, non_upper_case_globals, trivial_casts,
         trivial_numeric_casts, unsafe_code, variant_size_differences)]
// TODO: Allow `panic_params` until https://github.com/Manishearth/rust-clippy/issues/768
//       is resolved.
#![cfg_attr(all(feature="cargo-clippy", test), allow(panic_params))]
#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate rand;
#[macro_use]
extern crate unwrap;


use libc::{c_char, c_int, c_ulonglong, c_void, size_t, uint32_t};
use rand::{Rng, SeedableRng, XorShiftRng};
use std::cell::RefCell;
use std::ffi::CString;
use std::rc::Rc;
use std::sync::Mutex;

include!("src/core.rs");

include!("src/crypto_aead_chacha20poly1305.rs");

include!("src/crypto_auth.rs");
include!("src/crypto_auth_hmacsha256.rs");
include!("src/crypto_auth_hmacsha512.rs");
include!("src/crypto_auth_hmacsha512256.rs");

include!("src/crypto_box.rs");
include!("src/crypto_box_curve25519xsalsa20poly1305.rs");

include!("src/crypto_core_hsalsa20.rs");
include!("src/crypto_core_salsa20.rs");
include!("src/crypto_core_salsa2012.rs");
include!("src/crypto_core_salsa208.rs");

include!("src/crypto_generichash.rs");
include!("src/crypto_generichash_blake2b.rs");

include!("src/crypto_hash.rs");
include!("src/crypto_hash_sha256.rs");
include!("src/crypto_hash_sha512.rs");

include!("src/crypto_onetimeauth.rs");
include!("src/crypto_onetimeauth_poly1305.rs");

include!("src/crypto_pwhash_scryptsalsa208sha256.rs");

include!("src/crypto_scalarmult.rs");
include!("src/crypto_scalarmult_curve25519.rs");

include!("src/crypto_secretbox.rs");
include!("src/crypto_secretbox_xsalsa20poly1305.rs");
include!("src/crypto_shorthash_siphash24.rs");
include!("src/crypto_sign_ed25519.rs");

include!("src/crypto_stream.rs");
include!("src/crypto_stream_chacha20.rs");
include!("src/crypto_stream_salsa20.rs");
include!("src/crypto_stream_salsa2012.rs");
include!("src/crypto_stream_salsa208.rs");
include!("src/crypto_stream_xsalsa20.rs");

include!("src/crypto_verify_16.rs");
include!("src/crypto_verify_32.rs");
include!("src/crypto_verify_64.rs");

include!("src/randombytes.rs");
include!("src/init_with_rng.rs");
include!("src/utils.rs");
include!("src/version.rs");
