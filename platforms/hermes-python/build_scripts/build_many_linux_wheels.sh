#!/usr/bin/env bash
set -e -x

sudo docker run -v `pwd`:/io quay.io/pypa/manylinux1_x86_64 /io/platforms/hermes-python/build_scripts/build-wheels.sh
