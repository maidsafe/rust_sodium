#!/bin/sh

# Builds and runs tests for a particular target passed as an argument to this
# script.

set -ex

TARGET=$1

case "$TARGET" in
  *-apple-*)
    # Download the iOS test harness
    curl -vv -L https://github.com/carllerche/ios-test-harness/releases/download/v0.1.0/libiosharness-$TARGET.a > libiosharness.a;

    rustc -O ./ci/ios/deploy_and_run_on_ios_simulator.rs;


    ### Test rust_sodium

    # Build the test
    cargo rustc --release --tests --target $TARGET --verbose -- \
        -L . \
        -C link-args="-mios-simulator-version-min=7.0 -e _ios_main -liosharness";

    # Find the file to run
    TEST_FILE=$(find $TARGET/release -maxdepth 1 -type f -perm -111 -name "rust_sodium-*" | head -1);

    ./deploy_and_run_on_ios_simulator $TEST_FILE;


    ### Test rust_sodium-sys

    cp libiosharness.a rust_sodium-sys/libiosharness.a

    # Build the test
    cargo rustc --release --tests --target $TARGET --manifest-path rust_sodium-sys/Cargo.toml --verbose -- \
        -L . \
        -C link-args="-mios-simulator-version-min=7.0 -e _ios_main -liosharness";

    # Find the file to run
    TEST_FILE=$(find $TARGET/release -maxdepth 1 -type f -perm -111 -name "rust_sodium_sys-*" | head -1);

    ./deploy_and_run_on_ios_simulator $TEST_FILE;


    ### Run systest

    cp libiosharness.a systest/libiosharness.a

    # Build the test
    cargo rustc --release --target $TARGET --manifest-path systest/Cargo.toml --verbose -- \
        -L . \
        -C link-args="-mios-simulator-version-min=7.0 -e _ios_main -liosharness";

    find . -type f -name systest-*

    # Find the file to run
    TEST_FILE=$(find $TARGET/release/deps -maxdepth 1 -type f -perm -111 -name "systest-*" | head -1);

    ./deploy_and_run_on_ios_simulator $TEST_FILE;

    ;;

  *)
    echo "unsupported target $TARGET";
    exit 1;
    ;;
esac
