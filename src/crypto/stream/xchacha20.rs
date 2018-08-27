// Copyright 2018 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under the MIT license <LICENSE-MIT
// http://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.

//! `xchacha20`. The same construction as `xsalsa20` but using
//! `chacha20` instead of `salsa20` as the underlying stream cipher.
//! This cipher is conjectured to meet the standard notion of
//! unpredictability.

use crate::ffi::{
    crypto_stream_xchacha20, crypto_stream_xchacha20_KEYBYTES, crypto_stream_xchacha20_NONCEBYTES,
    crypto_stream_xchacha20_xor, crypto_stream_xchacha20_xor_ic,
};

stream_module!(
    crypto_stream_xchacha20,
    crypto_stream_xchacha20_xor,
    crypto_stream_xchacha20_xor_ic,
    crypto_stream_xchacha20_KEYBYTES as usize,
    crypto_stream_xchacha20_NONCEBYTES as usize
);
