// Copyright 2018 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under the MIT license <LICENSE-MIT
// http://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.

//! Libsodium version functions

use crate::ffi;
use std::ffi::CStr;

/// `version_string()` returns the version string from libsodium.
pub fn version_string() -> &'static str {
    let version = unsafe { CStr::from_ptr(ffi::sodium_version_string()) };
    unwrap!(version.to_str())
}

/// `version_major()` returns the major version from libsodium.
pub fn version_major() -> usize {
    unsafe { ffi::sodium_library_version_major() as usize }
}

/// `version_minor()` returns the minor version from libsodium.
pub fn version_minor() -> usize {
    unsafe { ffi::sodium_library_version_minor() as usize }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_version_string() {
        use crate::version::version_string;
        unwrap!(crate::init());
        assert!(!version_string().is_empty());
    }
}
