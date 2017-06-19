# rust_sodium - Change Log

## rust_sodium-sys [0.3.1]
- Added fallback URL for Windows libsodium artefacts.

## [0.3.0]
- Ported several updates from sodiumoxide
- Upgraded libsodium version to 1.0.12
- Changed the default feature behaviour for rust_sodium-sys to download and unpack/build libsodium
- For Linux distros, only enable PIE for Ubuntu >= 15.04
- Added support for MSVC builds
- Changed to use rust 1.17 stable
- Updated CI script to run cargo_install from QA

## [0.2.0]
- Default to serde instead of rustc-serialize
- rustfmt 0.8.1
- enforce min powershell major version of 4 for compilation on Windows

## [0.1.2]
- Fix Windows build scripts by using curl.
- Fix ARM build by allowing the `trivial_casts` lint.
- Print build commands on failure.
- [Upstream pull - Make vector manipulation more efficient.](https://github.com/dnaq/sodiumoxide/commit/f509c90de1a5825abf67e1d8cd8cd70a35b91880)
- Added `init()` to every test.
- Updated dependencies.
- Added standard MaidSafe lint checks and fixed resulting warnings.

## [0.1.1]
- Bugfix for missed renaming of feature gate.

## [0.1.0]
- Initial fork from sodiumoxide including changes to build script.
- Added `init_with_rng()` to allow sodiumoxide_extras crate to be deprecated.
