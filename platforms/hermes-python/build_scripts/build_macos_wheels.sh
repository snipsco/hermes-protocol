#!/usr/bin/env bash
set -e -x

# Build the .so file
if ! [[ -z "$(ls -A ../hermes_python/dylib)" ]]; then
   echo "hermes_python/dylib should be empty. Aborting!" && exit 1
fi

CARGO_TARGET_DIR=../target cargo rustc --lib --manifest-path ../../../hermes-mqtt-ffi/Cargo.toml --release -- --crate-type cdylib || exit 1

mv ../target/release/libhermes_mqtt_ffi.dylib ../hermes_python/dylib/

# Build wheel
cd ..
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
cp dist/ wheelhouse/
rm -rf dist/*.whl
cd build_scripts
