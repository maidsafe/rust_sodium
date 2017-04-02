# rust_sodium

 **Maintainer:** David Irvine (david.irvine@maidsafe.net)

Rust bindings to [libsodium](https://github.com/jedisct1/libsodium).

This project is largely based on [sodiumoxide](https://github.com/dnaq/sodiumoxide).  Some main
differences are:

* this project builds or downloads libsodium by default, favouring a statically-built, fixed version
  of the native library
* offers the ability to initialise libsodium with a psuedorandom number generator, allowing for
  reproducible data to be generated, which can be useful in the context of tests
* implements a test to ensure the FFI bindings match the native library's interface
* runs CI tests on Windows (AppVeyor), OS X and Linux (Travis)

|Crate|Documentation|Linux/OS X|Windows|Issues|
|:---:|:-----------:|:--------:|:-----:|:----:|
|[![](http://meritbadge.herokuapp.com/rust_sodium)](https://crates.io/crates/rust_sodium)|[![Documentation](https://docs.rs/rust_sodium/badge.svg)](https://docs.rs/rust_sodium)|[![Build Status](https://travis-ci.org/maidsafe/rust_sodium.svg?branch=master)](https://travis-ci.org/maidsafe/rust_sodium)|[![Build status](https://ci.appveyor.com/api/projects/status/kkgtqnx263xgk0c3/branch/master?svg=true)](https://ci.appveyor.com/project/MaidSafe-QA/rust-sodium/branch/master)|[![Stories in Ready](https://badge.waffle.io/maidsafe/rust_sodium.png?label=ready&title=Ready)](https://waffle.io/maidsafe/rust_sodium)|

| [MaidSafe website](https://maidsafe.net) | [SAFE Dev Forum](https://forum.safedev.org) | [SAFE Network Forum](https://safenetforum.org) |
|:----------------------------------------:|:-------------------------------------------:|:----------------------------------------------:|

## Cross-Compiling

### Cross-Compiling for ARM

1. Install dependencies and toolchain:

   ```sh
   sudo apt update
   sudo apt install build-essential gcc-arm-linux-gnueabihf libc6-armhf-cross libc6-dev-armhf-cross -y
   rustup target add armv7-unknown-linux-gnueabihf
   ```

1. Add the following to a [.cargo/config file](http://doc.crates.io/config.html):

   ```
   [target.armv7-unknown-linux-gnueabihf]
   linker = "arm-linux-gnueabihf-gcc"
   ```

1. Build by running:

   ```sh
   cargo build --release --target armv7-unknown-linux-gnueabihf
   ```

### Cross-Compiling for 32-bit Linux

1. Install dependencies and toolchain:

   ```sh
   sudo apt update
   sudo apt install build-essential gcc-multilib -y
   rustup target add i686-unknown-linux-gnu
   ```

1. Build by running:

   ```sh
   cargo build --release --target i686-unknown-linux-gnu
   ```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
