#!/usr/bin/env bash
set -e -x

PROJECT_NAME=$1
PROJECT_PATH=/io/${PROJECT_NAME}
WHEELHOUSE=/io/wheelhouse
PYTHON_PROJECT_PATH=${PROJECT_PATH}/platforms/hermes-python

PYBIN=/opt/python/cp27-cp27m/bin

cd /io

# Build wheel
${PYBIN}/python setup.py bdist_wheel -d ${WHEELHOUSE}

cd ${WHEELHOUSE}
whl=hermes_python-0.1.24-cp27-cp27m-linux_x86_64.whl

# Audit wheel
auditwheel repair ${whl} -w ${WHEELHOUSE}

PYBIN=/opt/python/cp27-cp27m/bin
fixed_whl=hermes_python-0.1.24-cp27-cp27m-manylinux1_x86_64.whl
# Test
${PYBIN}/pip install ${fixed_whl}
${PYBIN}/python -c "import hermes_python.hermes.Hermes"
