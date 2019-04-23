#!/usr/bin/env bash
set -e

if ! [[ -z "$(ls -A hermes_python/dylib)" ]]; then
   echo "hermes_python/dylib should be empty. Aborting!" && exit 1
fi

if ! [[ -z "$(ls -A tests/roundtrip/debug)" ]]; then
   echo "tests/test_ontology.py should be empty. Aborting!" && exit 1
fi


mkdir -p hermes_python/dylib
mkdir -p tests/roundtrip/debug

if [[ $(uname) == "Linux" ]]; then 
    cp ../../target/release/libhermes_mqtt_ffi.so hermes_python/dylib
    cp ../../target/release/libhermes_ffi_test.so tests/roundtrip/debug
elif [[ $(uname) == "Darwin" ]]; then
    cp ../../target/release/libhermes_mqtt_ffi.dylib hermes_python/dylib
    cp ../../target/release/libhermes_ffi_test.dylib tests/roundtrip/debug
fi

virtualenv --python=python2.7 env27
source env27/bin/activate 
pip install . 
pip install -r requirements/tests.txt
py.test 

virtualenv --python=python3 env3
source env3/bin/activate
pip install -r requirements/lint.txt
mypy --py2 --follow-imports=skip -p hermes_python.ontology


rm -rf env27 env3
rm -rf hermes_python/dylib
rm -rf tests/roundtrip/debug
