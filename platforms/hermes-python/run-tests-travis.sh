#!/usr/bin/env bash

if ! [[ -z "$(ls -A hermes_python/dylib)" ]]; then
   echo "hermes_python/dylib should be empty. Aborting!" && exit 1
fi

mkdir -p hermes_python/dylib

if ! [[ -z "$(ls -A tests/roundtrip/debug)" ]]; then
   echo "tests/test_ontology.py should be empty. Aborting!" && exit 1
fi

mkdir -p tests/roundtrip/debug

# The artifact were generated in a previous stage of the build
# Let's copy them to appropriate locations

cp ../../target/debug/libhermes_mqtt_ffi.so hermes_python/dylib
cp ../../target/debug/libhermes_ffi_test.so tests/roundtrip/debug


virtualenv --python=python2.7 env27
source env27/bin/activate 
pip install -r requirements/tests.txt
py.test 

virtualenv --python=python3 env3
source env3/bin/activate
pip install -r requirements/lint.txt
mypy --py2 --follow-imports=skip -p hermes_python.ontology

