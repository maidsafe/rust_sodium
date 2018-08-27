// Copyright 2018 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under the MIT license <LICENSE-MIT
// http://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.

//! Authenticated Encryption with Additional Data
//! This operation:
//!
//! - Encrypts a message with a key and a nonce to keep it confidential
//! - Computes an authentication tag. This tag is used to make sure that the message,
//!   as well as optional, non-confidential (non-encrypted) data, haven't been tampered with.
//!
//! # Selected primitive
//! `seal()`, `seal_detached()`, `open()` and `open_detached()` are currently
//! an implementation of `chacha20poly1305_ietf`, i.e. the IETF construction defined in
//! <https://tools.ietf.org/html/rfc7539./>
//!
//! # Example (combined mode)
//! ```
//! use rust_sodium::crypto::aead;
//!
//! let k = aead::gen_key();
//! let n = aead::gen_nonce();
//! let m = b"Some plaintext";
//! let ad = b"Some additional data";
//!
//! let c = aead::seal(m, Some(ad), &n, &k);
//! let m2 = aead::open(&c, Some(ad), &n, &k).unwrap();
//!
//! assert_eq!(&m[..], &m2[..]);
//!```
//!
//! # Example (detached mode)
//! ```
//! use rust_sodium::crypto::aead;
//!
//! let k = aead::gen_key();
//! let n = aead::gen_nonce();
//! let mut m = [0x41, 0x42, 0x43, 0x44];
//! let m2 = m.clone();
//! let ad = b"Some additional data";
//!
//! let t = aead::seal_detached(&mut m, Some(ad), &n, &k);
//! aead::open_detached(&mut m, Some(ad), &t, &n, &k).unwrap();
//!
//! assert_eq!(m, m2);
//! ```

pub use self::chacha20poly1305_ietf::*;
#[macro_use]
mod aead_macros;
pub mod chacha20poly1305;
pub mod chacha20poly1305_ietf;
