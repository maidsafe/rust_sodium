// Copyright 2018 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under the MIT license <LICENSE-MIT
// http://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.

//! A lot of applications and programming language implementations have been
//! recently found to be vulnerable to denial-of-service attacks when a hash
//! function with weak security guarantees, like Murmurhash 3, was used to
//! construct a hash table.
//!
//! In order to address this, Sodium provides the `shorthash()` function.
//! This very fast hash functions outputs short, but unpredictable
//! (without knowing the secret key) values suitable for picking a list in
//! a hash table for a given key.
//!
//! # Selected primitive
//! `shorthash()` is currently an implementation of `SipHash-2-4` as specified in
//! [`SipHash`: a fast short-input PRF](https://131002.net/siphash/)
//!
//! # Example
//! ```
//! # #![allow(unused_variables)]
//! use rust_sodium::crypto::shorthash;
//!
//! let key = shorthash::gen_key();
//! let data_to_hash = b"some data";
//! let digest = shorthash::shorthash(data_to_hash, &key);
//! ```

pub use self::siphash24::*;
pub mod siphash24;
