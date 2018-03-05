#![doc(html_logo_url =
           "https://raw.githubusercontent.com/maidsafe/QA/master/Images/maidsafe_logo.png",
       html_favicon_url = "https://maidsafe.net/img/favicon.ico",
       html_root_url = "https://docs.rs/rust_sodium-sys")]

#![forbid(exceeding_bitshifts, mutable_transmutes, no_mangle_const_items,
          unknown_crate_types, warnings)]
#![deny(deprecated, improper_ctypes, non_shorthand_field_patterns,
        overflowing_literals, plugin_as_library, private_no_mangle_fns, private_no_mangle_statics,
        stable_features, unconditional_recursion, unknown_lints, unused, unused_allocation,
        unused_attributes, unused_comparisons, unused_features, unused_parens, while_true)]
#![warn(unused_extern_crates, unused_import_braces, unused_qualifications, unused_results)]
// Allow `trivial_casts` to cast `u8` to `c_char`, which is `u8` or `i8`, depending on the
// architecture.
#![allow(bad_style, box_pointers, missing_copy_implementations,
         missing_debug_implementations, missing_docs, non_upper_case_globals, trivial_casts,
         trivial_numeric_casts, unsafe_code, variant_size_differences)]

#![cfg_attr(feature = "cargo-clippy", allow(decimal_literal_representation, unreadable_literal))]

#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate rand;
#[macro_use]
extern crate unwrap;

use rand::{Rng, SeedableRng, XorShiftRng};
use std::cell::RefCell;
use std::ffi::CString;
use std::rc::Rc;
use std::sync::Mutex;

lazy_static! {
    static ref INIT_RESULT: Mutex<Option<i32>> = Mutex::new(None);
    static ref RANDOM_BYTES_IMPL: Mutex<RandomBytesImpl> = Mutex::new(RandomBytesImpl::default());
}

thread_local!(static RNG: Rc<RefCell<XorShiftRng>> =
    Rc::new(RefCell::new(XorShiftRng::from_seed(unwrap!(RANDOM_BYTES_IMPL.lock()).seed))));

struct RandomBytesImpl {
    function_pointers: randombytes_implementation,
    name: CString,
    seed: [u32; 4],
}

impl Default for RandomBytesImpl {
    fn default() -> RandomBytesImpl {
        let seed = [
            rand::random(),
            rand::random(),
            rand::random(),
            rand::random(),
        ];
        RandomBytesImpl {
            function_pointers: randombytes_implementation::default(),
            name: unwrap!(CString::new("Rust XorShiftRng")),
            seed: seed,
        }
    }
}

// Bindgen generated file.  Generated using the following commands:
// ```
// REGEX="crypto.*|randombytes.*|sodium.*|SODIUM.*"
//
// bindgen sodium.h -o bindgen.rs --ctypes-prefix=libc --distrust-clang-mangling \
// --no-rustfmt-bindings --generate=functions,types,vars --whitelist-function=$REGEX \
// --whitelist-type=$REGEX --whitelist-var=$REGEX
//
// sed -ie 's/&'\''static \[u8; [0-9]*usize\] = \(b".*\\0"\)/*const libc::c_char = (\1 as *const \
// libc::c_uchar) as *const libc::c_char/g' bindgen.rs
// ```
//
// Further manual adjustments are usually needed when upgrading the libsodium version to accommodate
// for e.g.
//   * deprecated libsodium items
//   * https://github.com/rust-lang-nursery/rust-bindgen/issues/511 generating incorrect rust code
//   * applying #[repr(align(...))]
// However, these should show up when running the systest, which should also be reviewed and updated
// when upgrading the libsodium version.
include!("bindgen.rs");

impl Default for randombytes_implementation {
    fn default() -> randombytes_implementation {
        randombytes_implementation {
            implementation_name: Some(implementation_name),
            random: Some(random),
            stir: None,
            uniform: None,
            buf: Some(buf),
            close: None,
        }
    }
}

extern "C" fn implementation_name() -> *const libc::c_char {
    unwrap!(RANDOM_BYTES_IMPL.lock()).name.as_ptr()
}

extern "C" fn random() -> u32 {
    RNG.with(|rng| rng.borrow_mut().gen())
}

extern "C" fn buf(buf: *mut libc::c_void, size: usize) {
    unsafe {
        let ptr = buf as *mut u8;
        let rng_ptr = RNG.with(|rng| Rc::clone(rng));
        let rng = &mut *rng_ptr.borrow_mut();
        for i in 0..size {
            *ptr.offset(i as isize) = rng.gen();
        }
    }
}

/// Sets [libsodium's `randombytes_implementation`][1] to use a
/// [Rust `Rng` implementation](../rand/trait.Rng.html) and initialises libsodium.
///
/// This allows a seeded PRNG to be used for example, which can be helpful in test scenarios when
/// predictable results may be preferred.
///
/// This function is safe to call multiple times concurrently from different threads.  It will
/// either always return `Ok` or will always return `Err`.
///
/// The error will contain either `-1` or `1`.  If the error is `-1`, the initialisation of
/// libsodium has failed.  If the error is `1`, libsodium has been successfully initialised
/// elsewhere (e.g. via [`rust_sodium::init()`][2]) but this means that our attempt to apply this
/// seeded RNG to libsodium has not been actioned.
///
/// Each `rust_sodium` function which uses the random generator in a new thread will cause a new
/// thread-local instance of the PRNG to be created.  Each such instance will be seeded with the
/// same value, meaning for example that two newly-spawned threads calling `box_::gen_keypair()`
/// will generate identical keys.
///
/// [1]: https://download.libsodium.org/doc/advanced/custom_rng.html
/// [2]: https://docs.rs/rust_sodium/*/rust_sodium/fn.init.html
pub fn init_with_rng<T: Rng>(rng: &mut T) -> Result<(), i32> {
    let init_result = &mut *unwrap!(INIT_RESULT.lock());
    if let Some(ref existing_result) = *init_result {
        return if *existing_result == 0 {
            Ok(())
        } else {
            Err(*existing_result)
        };
    }
    let mut sodium_result;
    let seed = [rng.gen(), rng.gen(), rng.gen(), rng.gen()];
    {
        let random_bytes = &mut *unwrap!(RANDOM_BYTES_IMPL.lock());
        random_bytes.seed = seed;
        sodium_result =
            unsafe { randombytes_set_implementation(&mut random_bytes.function_pointers) };
    }
    if sodium_result == 0 {
        sodium_result = unsafe { sodium_init() };
    }
    // Since `sodium_init()` makes a call to `buf()`, reset the thread-local `RNG` so that it yields
    // consistent results with calls from new threads.
    RNG.with(|rng| *rng.borrow_mut() = XorShiftRng::from_seed(seed));
    *init_result = Some(sodium_result);
    match sodium_result {
        0 => Ok(()),
        result => Err(result),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use libc::*;

    #[test]
    fn generichash_statebytes() {
        assert!(unsafe { crypto_generichash_statebytes() } > 0);
    }

    #[test]
    fn generichash() {
        let mut out = [0u8; crypto_generichash_BYTES as usize];
        let m = [0u8; 64];
        let key = [0u8; crypto_generichash_KEYBYTES as usize];

        assert_eq!(
            unsafe {
                crypto_generichash(
                    out.as_mut_ptr(),
                    out.len(),
                    m.as_ptr(),
                    m.len() as u64,
                    key.as_ptr(),
                    key.len(),
                )
            },
            0
        );
    }

    #[test]
    fn generichash_multipart() {
        use std::mem;

        let mut out = [0u8; crypto_generichash_BYTES as usize];
        let m = [0u8; 64];
        let key = [0u8; crypto_generichash_KEYBYTES as usize];

        let mut st = vec![0u8; unsafe { crypto_generichash_statebytes() }];
        let pst =
            unsafe { mem::transmute::<*mut u8, *mut crypto_generichash_state>(st.as_mut_ptr()) };

        assert_eq!(
            unsafe { crypto_generichash_init(pst, key.as_ptr(), key.len(), out.len()) },
            0
        );

        assert_eq!(
            unsafe { crypto_generichash_update(pst, m.as_ptr(), m.len() as u64) },
            0
        );

        assert_eq!(
            unsafe { crypto_generichash_update(pst, m.as_ptr(), m.len() as u64) },
            0
        );

        assert_eq!(
            unsafe { crypto_generichash_final(pst, out.as_mut_ptr(), out.len()) },
            0
        );
    }

    #[test]
    fn generichash_blake2b() {
        let mut out = [0u8; crypto_generichash_blake2b_BYTES as usize];
        let m = [0u8; 64];
        let key = [0u8; crypto_generichash_blake2b_KEYBYTES as usize];

        assert_eq!(
            unsafe {
                crypto_generichash_blake2b(
                    out.as_mut_ptr(),
                    out.len(),
                    m.as_ptr(),
                    m.len() as u64,
                    key.as_ptr(),
                    key.len(),
                )
            },
            0
        );
    }

    #[test]
    fn generichash_blake2b_salt_personal() {
        let mut out = [0u8; crypto_generichash_blake2b_BYTES as usize];
        let m = [0u8; 64];
        let key = [0u8; crypto_generichash_blake2b_KEYBYTES as usize];
        let salt = [0u8; crypto_generichash_blake2b_SALTBYTES as usize];
        let personal = [0u8; crypto_generichash_blake2b_PERSONALBYTES as usize];

        assert_eq!(
            unsafe {
                crypto_generichash_blake2b_salt_personal(
                    out.as_mut_ptr(),
                    out.len(),
                    m.as_ptr(),
                    m.len() as u64,
                    key.as_ptr(),
                    key.len(),
                    salt.as_ptr(),
                    personal.as_ptr(),
                )
            },
            0
        );
    }

    #[test]
    fn pwhash_scryptsalsa208sha256_str() {
        let password = "Correct Horse Battery Staple";
        let mut hashed_password = [0; crypto_pwhash_scryptsalsa208sha256_STRBYTES as usize];
        let ret_hash = unsafe {
            crypto_pwhash_scryptsalsa208sha256_str(
                hashed_password.as_mut_ptr(),
                password.as_ptr() as *const c_char,
                password.len() as c_ulonglong,
                c_ulonglong::from(crypto_pwhash_scryptsalsa208sha256_OPSLIMIT_INTERACTIVE),
                crypto_pwhash_scryptsalsa208sha256_MEMLIMIT_INTERACTIVE as size_t,
            )
        };
        assert!(ret_hash == 0);
        let ret_verify = unsafe {
            crypto_pwhash_scryptsalsa208sha256_str_verify(
                hashed_password.as_ptr(),
                password.as_ptr() as *const c_char,
                password.len() as c_ulonglong,
            )
        };
        assert!(ret_verify == 0);
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn pwhash_scryptsalsa208sha256_ll_1() {
        // See https://www.tarsnap.com/scrypt/scrypt.pdf Page 16
        let password = "";
        let salt = "";
        let n = 16;
        let r = 1;
        let p = 1;
        let mut buf = [0u8; 64];
        let expected = [0x77, 0xd6, 0x57, 0x62, 0x38, 0x65, 0x7b, 0x20, 0x3b, 0x19, 0xca, 0x42,
            0xc1, 0x8a, 0x04, 0x97, 0xf1, 0x6b, 0x48, 0x44, 0xe3, 0x07, 0x4a, 0xe8, 0xdf, 0xdf,
            0xfa, 0x3f, 0xed, 0xe2, 0x14, 0x42, 0xfc, 0xd0, 0x06, 0x9d, 0xed, 0x09, 0x48, 0xf8,
            0x32, 0x6a, 0x75, 0x3a, 0x0f, 0xc8, 0x1f, 0x17, 0xe8, 0xd3, 0xe0, 0xfb, 0x2e, 0x0d,
            0x36, 0x28, 0xcf, 0x35, 0xe2, 0x0c, 0x38, 0xd1, 0x89, 0x06, ];
        let ret = unsafe {
            crypto_pwhash_scryptsalsa208sha256_ll(
                password.as_ptr(),
                password.len() as size_t,
                salt.as_ptr(),
                salt.len() as size_t,
                n,
                r,
                p,
                buf.as_mut_ptr(),
                buf.len() as size_t,
            )
        };
        assert!(ret == 0);
        assert!(buf[0..] == expected[0..]);
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn pwhash_scryptsalsa208sha256_ll_2() {
        // See https://www.tarsnap.com/scrypt/scrypt.pdf Page 16
        let password = "password";
        let salt = "NaCl";
        let n = 1024;
        let r = 8;
        let p = 16;
        let mut buf = [0u8; 64];
        let expected = [0xfd, 0xba, 0xbe, 0x1c, 0x9d, 0x34, 0x72, 0x00, 0x78, 0x56, 0xe7, 0x19,
            0x0d, 0x01, 0xe9, 0xfe, 0x7c, 0x6a, 0xd7, 0xcb, 0xc8, 0x23, 0x78, 0x30, 0xe7, 0x73,
            0x76, 0x63, 0x4b, 0x37, 0x31, 0x62, 0x2e, 0xaf, 0x30, 0xd9, 0x2e, 0x22, 0xa3, 0x88,
            0x6f, 0xf1, 0x09, 0x27, 0x9d, 0x98, 0x30, 0xda, 0xc7, 0x27, 0xaf, 0xb9, 0x4a, 0x83,
            0xee, 0x6d, 0x83, 0x60, 0xcb, 0xdf, 0xa2, 0xcc, 0x06, 0x40, ];
        let ret = unsafe {
            crypto_pwhash_scryptsalsa208sha256_ll(
                password.as_ptr(),
                password.len() as size_t,
                salt.as_ptr(),
                salt.len() as size_t,
                n,
                r,
                p,
                buf.as_mut_ptr(),
                buf.len() as size_t,
            )
        };
        assert!(ret == 0);
        assert!(buf[0..] == expected[0..]);
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn pwhash_scryptsalsa208sha256_ll_3() {
        // See https://www.tarsnap.com/scrypt/scrypt.pdf Page 16
        let password = "pleaseletmein";
        let salt = "SodiumChloride";
        let n = 16_384;
        let r = 8;
        let p = 1;
        let mut buf = [0u8; 64];
        let expected = [0x70, 0x23, 0xbd, 0xcb, 0x3a, 0xfd, 0x73, 0x48, 0x46, 0x1c, 0x06, 0xcd,
            0x81, 0xfd, 0x38, 0xeb, 0xfd, 0xa8, 0xfb, 0xba, 0x90, 0x4f, 0x8e, 0x3e, 0xa9, 0xb5,
            0x43, 0xf6, 0x54, 0x5d, 0xa1, 0xf2, 0xd5, 0x43, 0x29, 0x55, 0x61, 0x3f, 0x0f, 0xcf,
            0x62, 0xd4, 0x97, 0x05, 0x24, 0x2a, 0x9a, 0xf9, 0xe6, 0x1e, 0x85, 0xdc, 0x0d, 0x65,
            0x1e, 0x40, 0xdf, 0xcf, 0x01, 0x7b, 0x45, 0x57, 0x58, 0x87, ];
        let ret = unsafe {
            crypto_pwhash_scryptsalsa208sha256_ll(
                password.as_ptr(),
                password.len() as size_t,
                salt.as_ptr(),
                salt.len() as size_t,
                n,
                r,
                p,
                buf.as_mut_ptr(),
                buf.len() as size_t,
            )
        };
        assert!(ret == 0);
        assert!(buf[0..] == expected[0..]);
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn pwhash_scryptsalsa208sha256_ll_4() {
        // See https://www.tarsnap.com/scrypt/scrypt.pdf Page 16
        let password = "pleaseletmein";
        let salt = "SodiumChloride";
        let n = 1_048_576;
        let r = 8;
        let p = 1;
        let mut buf = [0u8; 64];
        let expected = [0x21, 0x01, 0xcb, 0x9b, 0x6a, 0x51, 0x1a, 0xae, 0xad, 0xdb, 0xbe, 0x09,
            0xcf, 0x70, 0xf8, 0x81, 0xec, 0x56, 0x8d, 0x57, 0x4a, 0x2f, 0xfd, 0x4d, 0xab, 0xe5,
            0xee, 0x98, 0x20, 0xad, 0xaa, 0x47, 0x8e, 0x56, 0xfd, 0x8f, 0x4b, 0xa5, 0xd0, 0x9f,
            0xfa, 0x1c, 0x6d, 0x92, 0x7c, 0x40, 0xf4, 0xc3, 0x37, 0x30, 0x40, 0x49, 0xe8, 0xa9,
            0x52, 0xfb, 0xcb, 0xf4, 0x5c, 0x6f, 0xa7, 0x7a, 0x41, 0xa4, ];
        let ret = unsafe {
            crypto_pwhash_scryptsalsa208sha256_ll(
                password.as_ptr(),
                password.len() as size_t,
                salt.as_ptr(),
                salt.len() as size_t,
                n,
                r,
                p,
                buf.as_mut_ptr(),
                buf.len() as size_t,
            )
        };
        assert!(ret == 0);
        assert!(buf[0..] == expected[0..]);
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn seeded_init_with_rng() {
        use std::thread::Builder;
        let mut rng = XorShiftRng::from_seed([0, 1, 2, 3]);
        unwrap!(init_with_rng(&mut rng));

        // Initialise again - should succeed.
        unwrap!(init_with_rng(&mut rng));

        let expected_public_key = [116, 196, 172, 118, 77, 124, 253, 254, 156, 51, 141, 193, 20,
            160, 227, 232, 231, 20, 24, 151, 207, 45, 202, 250, 85, 96, 206, 144, 170, 185, 192,
            101, ];
        let expected_secret_key = [24, 74, 130, 137, 89, 75, 193, 8, 153, 136, 7, 141, 220, 198,
            207, 232, 228, 74, 189, 36, 9, 209, 239, 95, 69, 207, 163, 2, 37, 237, 255, 64, ];
        let mut public_key = [0u8; crypto_box_curve25519xsalsa20poly1305_PUBLICKEYBYTES as usize];
        let mut secret_key = [0u8; crypto_box_curve25519xsalsa20poly1305_SECRETKEYBYTES as usize];
        unsafe {
            assert_eq!(
                crypto_box_curve25519xsalsa20poly1305_keypair(
                    public_key.as_mut_ptr(),
                    secret_key.as_mut_ptr(),
                ),
                0
            );
        }
        assert_eq!(expected_public_key, public_key);
        assert_eq!(expected_secret_key, secret_key);

        let child1 = unwrap!(Builder::new().name("child1".to_string()).spawn(move || {
            let mut public_key = [0; crypto_box_curve25519xsalsa20poly1305_PUBLICKEYBYTES as usize];
            let mut secret_key = [0; crypto_box_curve25519xsalsa20poly1305_SECRETKEYBYTES as usize];
            unsafe {
                assert_eq!(
                    crypto_box_curve25519xsalsa20poly1305_keypair(
                        public_key.as_mut_ptr(),
                        secret_key.as_mut_ptr(),
                    ),
                    0
                );
            }
            assert_eq!(expected_public_key, public_key);
            assert_eq!(expected_secret_key, secret_key);
        }));
        let child2 = unwrap!(Builder::new().name("child2".to_string()).spawn(move || {
            let mut public_key = [0; crypto_box_curve25519xsalsa20poly1305_PUBLICKEYBYTES as usize];
            let mut secret_key = [0; crypto_box_curve25519xsalsa20poly1305_SECRETKEYBYTES as usize];
            unsafe {
                assert_eq!(
                    crypto_box_curve25519xsalsa20poly1305_keypair(
                        public_key.as_mut_ptr(),
                        secret_key.as_mut_ptr(),
                    ),
                    0
                );
            }
            assert_eq!(expected_public_key, public_key);
            assert_eq!(expected_secret_key, secret_key);
        }));
        unwrap!(child1.join());
        unwrap!(child2.join());
    }
}
