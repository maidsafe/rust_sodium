#!/bin/sh

# Builds and runs tests for a particular target passed as an argument to this
# script.

set -ex

emulate() {
    case "$TARGET" in
        armv7-linux-androideabi)
            # Use the 64-bit emulator
            emulator64-arm @arm-21 -no-window &
            adb wait-for-device
            adb root
            adb remount
            adb push $TEST_FILE /data/test-file
            adb push testvectors /data/testvectors
            adb shell chmod 777 /data /data/testvectors
            adb shell ls -l /data/testvectors
            adb shell ls -l /data/testvectors/SHA256LongMsg.rsp
            adb shell /data/test-file --nocapture 2>&1 | tee /tmp/out
            grep "^# TODO: est result.* 0 failed" /tmp/out
            ;;

        i686-linux-android)
            # Use the 64-bit emulator
            emulator64-arm @x86-21 -no-window &
            adb wait-for-device
            adb root
            adb remount
            adb push $TEST_FILE /data/test-file
            adb shell chmod 755 /data/test-file
            adb push testvectors /data/testvectors
            adb shell chmod 777 /data /data/testvectors
            adb shell ls -l /data/testvectors
            adb shell ls -l /data/testvectors/SHA256LongMsg.rsp
            adb shell /data/test-file --nocapture 2>&1 | tee /tmp/out
            grep "^test result.* 0 failed" /tmp/out
            ;;

        *)
            exit 1;
            ;;
    esac
}

TARGET=$1

if [ -f /etc/cargo_config ] && [ -d /cargo ]; then cp -f /etc/cargo_config /cargo/config; fi


### Test rust_sodium

# Build the test
cargo test --release --no-run --target=$TARGET --verbose

# Find the file to run
TEST_FILE=$(find target/$TARGET/release -maxdepth 1 -type f -perm -111 -name "rust_sodium-*" | head -1)

emulate


### Test rust_sodium-sys

# Build the test
if [ -f /etc/cargo_config ] && [ -d /cargo ]; then cp -f /etc/cargo_config /cargo/config; fi
cargo test --release --no-run --target=$TARGET --manifest-path=rust_sodium-sys/Cargo.toml --verbose

# Find the file to run
TEST_FILE=$(find rust_sodium-sys/target/$TARGET/release -maxdepth 1 -type f -perm -111 -name "rust_sodium_sys-*" | head -1)

emulate


### Run systest

# Build the test
if [ -f /etc/cargo_config ] && [ -d /cargo ]; then cp -f /etc/cargo_config /cargo/config; fi
cargo build --release --target=$TARGET --manifest-path=systest/Cargo.toml --verbose

# Find the file to run
TEST_FILE=$(find systest/target/$TARGET/release -maxdepth 1 -type f -perm -111 -name "systest-*" | head -1)

emulate
