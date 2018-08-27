// Copyright 2018 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under the MIT license <LICENSE-MIT
// http://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.

//! Secret-key One-time authentication
//!
//! # Security model
//! The `authenticate()` function, viewed as a function
//! of the message for a uniform random key, is designed to meet the
//! standard notion of unforgeability after a single message. After the
//! sender authenticates one message, an attacker cannot find authenticators
//! for any other messages.
//!
//! The sender must not use `authenticate()` to authenticate more than one message
//! under the same key. Authenticators for two messages under the same key should
//! be expected to reveal enough information to allow forgeries of authenticators
//! on other messages.
//!
//! # Selected primitive
//! `authenticate()` is `crypto_onetimeauth_poly1305`, an authenticator specified
//! in [Cryptography in `NaCl`](http://nacl.cr.yp.to/valid.html), Section 9. This
//! authenticator is proven to meet the standard notion of unforgeability after a
//! single message.
//!
//! # Example
//! ```
//! use rust_sodium::crypto::onetimeauth;
//!
//! let key = onetimeauth::gen_key();
//! let data_to_authenticate = b"some data";
//! let tag = onetimeauth::authenticate(data_to_authenticate, &key);
//! assert!(onetimeauth::verify(&tag, data_to_authenticate, &key));
//! ```

pub use self::poly1305::*;
#[path = "../auth/auth_macros.rs"]
#[macro_use]
mod auth_macros;
pub mod poly1305;
