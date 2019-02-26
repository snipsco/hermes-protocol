#!/bin/bash

set -ex

rustup default nightly
cbindgen -c hermes-mqtt-ffi/cbindgen_full.toml -o platforms/c/libsnips_hermes_full.h hermes-mqtt-ffi -v
cbindgen -c hermes-mqtt-ffi/cbindgen_json.toml -o platforms/c/libsnips_hermes_json.h hermes-mqtt-ffi -v
cbindgen -c hermes-mqtt-ffi/cbindgen.toml -o platforms/c/libsnips_hermes.h hermes-mqtt-ffi -v
rustup default stable
