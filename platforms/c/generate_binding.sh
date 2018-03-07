#!/bin/bash -e

cargo +nightly clean -p hermes-ffi
cargo +nightly rustc -p hermes-ffi -- -Z unstable-options --pretty=expanded > hermes_ffi.rs
cbindgen -l c -o hermes_ffi.h hermes_ffi.rs
rm hermes_ffi.rs
