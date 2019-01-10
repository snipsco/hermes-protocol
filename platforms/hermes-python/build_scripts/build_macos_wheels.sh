#!/usr/bin/env bash
# This script should be ran from the root of the repository
set -e -x


# Build the .so file
if ! [[ -z "$(ls -A platforms/hermes-python/hermes_python/dylib)" ]]; then
   echo "hermes_python/dylib should be empty. Aborting!" && exit 1
fi

CARGO_TARGET_DIR=platforms/hermes-python/target cargo rustc --lib --manifest-path hermes-mqtt-ffi/Cargo.toml --release -- --crate-type cdylib || exit 1

mv platforms/hermes-python/target/release/libhermes_mqtt_ffi.dylib platforms/hermes-python/hermes_python/dylib/

# Build wheel
cd platforms/hermes-python/
# Build wheel
PYTHON_INTERPRETERS=( 'python2.7' 'python3.6' 'python3.7')

for PYINTERPRETER in "${PYTHON_INTERPRETERS[@]}";
do
	echo $PYINTERPRETER
	virtualenv --python=$PYINTERPRETER env
	source env/bin/activate
	python setup.py bdist_wheel 
	rm -rf env
done

# Clean up after yourself
rm hermes_python/dylib/*.dylib

ls -1 dist/
mkdir -p wheelhouse
cp dist/*.whl wheelhouse/
rm -rf dist/*.whl
cd ../..
