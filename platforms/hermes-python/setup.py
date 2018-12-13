#! /usr/bin/env python
# encoding: utf-8

import io
import os
import sys
from setuptools import setup, find_packages

from wheel.bdist_wheel import bdist_wheel as _bdist_wheel
from setuptools.command.install import install



class InstallPlatlib(install):
    def finalize_options(self):
        install.finalize_options(self)
        self.install_lib = self.install_platlib

class bdist_wheel(_bdist_wheel, object):
    def finalize_options(self):
        _bdist_wheel.finalize_options(self)
        # noinspection PyAttributeOutsideInit
        self.root_is_pure = False

    def get_tag(self):
        return super(bdist_wheel, self).get_tag()


PACKAGE_NAME = "hermes_python"
ROOT_PATH = os.path.dirname(os.path.abspath(__file__))
PACKAGE_PATH = os.path.join(ROOT_PATH, PACKAGE_NAME)
README = os.path.join(ROOT_PATH, "README.md")
HISTORY = os.path.join(ROOT_PATH, "HISTORY.rst")
VERSION = "__version__"

with io.open(os.path.join(PACKAGE_PATH, VERSION)) as f:
	version = f.readline()

with io.open(README, "rt", encoding="utf8") as f:
	readme = f.read()

with io.open(HISTORY, "rt", encoding="utf8") as f:
	history = f.read()

packages = [p for p in find_packages() if "tests" not in p]

extras_require = {
    "test": [
        "mock",
        "pytest",
        "coverage",
        "pytest-cov",
        "setuptools_rust",
    ],
}

def get_rust_extension_command(argvs):
    if "--plat-name" in argvs:
        return RustExtension(TARGET, CARGO_FILE_PATH, binding=Binding.NoBinding, dinghy=True, rust_x_compile_target="arm-unknown-linux-gnueabihf", dinghy_platform="raspbian")
    return RustExtension(TARGET, CARGO_FILE_PATH, binding=Binding.NoBinding)

setup(
    name=PACKAGE_NAME,
    version=version,
    description='Python bindings for Snips Hermes Protocol',
    long_description=readme + history,
    author='Anthony Reinette',
    author_email='anthony.reinette@snips.ai',
    url='https://github.com/snipsco/snips-platform/tree/main/hermes-ffi-python-extension/hermes-protocol/hermes-ffi/platforms/hermes-python',
    classifiers=[
        'Programming Language :: Python :: 2',
        'Programming Language :: Python :: 2.7',
        'Programming Language :: Python :: 3.5'],
    download_url='',
    license='MIT',
    keywords=['snips'],
    install_requires=['six', 'dotmap', 'future'],
    test_suite="tests",
    extras_require=extras_require,
    packages=packages,
    cmdclass={'bdist_wheel': bdist_wheel, 'install': InstallPlatlib},
    zip_safe=False,
    include_package_data=True,
)
