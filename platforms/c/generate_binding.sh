#!/bin/bash -e

cargo +nightly clean -p hermes-mqtt-ffi
cargo +nightly rustc -p hermes-mqtt-ffi -- -Z unstable-options --pretty=expanded > hermes_ffi.rs
cbindgen -l c -o hermes_ffi.h hermes_ffi.rs
rm hermes_ffi.rs
