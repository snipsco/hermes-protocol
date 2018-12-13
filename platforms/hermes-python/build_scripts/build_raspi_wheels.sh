#!/usr/bin/env bash
set -e -x

# Build the .so file
if ! [[ -z "$(ls -A ../hermes_python/dylib)" ]]; then
   echo "hermes_python/dylib should be empty. Aborting!" && exit 1
fi

CARGO_TARGET_DIR=../target cargo dinghy --platform raspbian build -p hermes-mqtt-ffi --release || exit 1

mkdir -p ../hermes_python/dylib
mv ../target/arm-unknown-linux-gnueabihf/release/libhermes_mqtt_ffi.so ../hermes_python/dylib/

# Build wheel
PYTHON_INTERPRETERS=( 'python2.7' 'python3.5')

cd ..
for PYINTERPRETER in "${PYTHON_INTERPRETERS[@]}";
do
	echo $PYINTERPRETER
	virtualenv --python=$PYINTERPRETER env
	source env/bin/activate
	python setup.py bdist_wheel --plat-name linux-armv7l
	python setup.py bdist_wheel --plat-name linux-armv6l
	rm -rf env
done

# Clean up after yourself
rm hermes_python/dylib/*.so

ls -1 dist/
cd build_scripts
