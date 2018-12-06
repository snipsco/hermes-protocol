#!/usr/bin/env bash

if ! [[ -z "$(ls -A hermes_python/dylib)" ]]; then
   echo "hermes_python/dylib should be empty. Aborting!" && exit 1
fi

mkdir -p hermes_python/dylib

if [[ $(uname) == "Linux" ]]; then
    CARGO_TARGET_DIR=./target cargo rustc --lib --manifest-path ../../hermes-mqtt-ffi/Cargo.toml --release -- --crate-type cdylib || exit 1
    mv ./target/release/libhermes_mqtt_ffi.so hermes_python/dylib
elif [[ $(uname) == "Darwin" ]]; then
    CARGO_TARGET_DIR=./target cargo rustc --lib --manifest-path ../../hermes-mqtt-ffi/Cargo.toml --release -- --crate-type cdylib -C link-arg=-undefined -C link-arg=dynamic_lookup || exit 1
    mv target/release/libhermes_mqtt_ffi.dylib hermes_python/dylib
fi

virtualenv env
source env/bin/activate 
pip install -r requirements/tests.txt
py.test 
