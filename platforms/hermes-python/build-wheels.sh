#!/usr/bin/env bash
set -e -x

PYTHON_PROJECT_PATH=/io/platforms/hermes-python
WHEELHOUSE=/io/platforms/hermes-python/wheelhouse

# Install Rust
curl https://sh.rustup.rs -sSf | bash -s -- -y
export PATH="/usr/local/bin:$HOME/.cargo/bin:$PATH"

# Build the .so file
cd /io
if ! [[ -z "$(ls -A platforms/hermes-python/hermes_python/dylib)" ]]; then
   echo "hermes_python/dylib should be empty. Aborting!" && exit 1
fi

mkdir -p platforms/hermes-python/hermes_python/dylib
mkdir -p platforms/hermes-python/target

CARGO_TARGET_DIR=$PYTHON_PROJECT_PATH/target cargo rustc --lib --manifest-path hermes-mqtt-ffi/Cargo.toml --release -- --crate-type cdylib || exit 1

# Move .so to correct path : 
mv $PYTHON_PROJECT_PATH/target/release/libhermes_mqtt_ffi.so $PYTHON_PROJECT_PATH/hermes_python/dylib/ 

# Build wheel
cd $PYTHON_PROJECT_PATH
for PYBIN in /opt/python/*/bin; do
	${PYBIN}/python setup.py bdist_wheel -d ${WHEELHOUSE}
done

cd ${WHEELHOUSE}

# Audit wheel
for whl in ${WHEELHOUSE}/*.whl; do
	if [[ ${whl} != *none-any.whl ]]; then
		auditwheel repair ${whl} -w ${WHEELHOUSE}
	fi
done

# Testing of the wheel is disabled for now

#for PYBIN in /opt/python/*/bin; do
#	# Install package
#	${PYBIN}/pip install -v hermes-python --no-index -f ${WHEELHOUSE}
#	# Test
#	${PYBIN}/python -c "from hermes_python.hermes import Hermes"
#done

# Delete non repaired wheels
rm -rf ${WHEELHOUSE}/*-linux_*
# Delete unrelated wheels
rm -rf ${WHEELHOUSE}/*-none-any.whl
