#! /usr/bin/env python
# encoding: utf-8

import io
import os
from setuptools import setup, find_packages
import sys
import subprocess
import shutil

from wheel.bdist_wheel import bdist_wheel as _bdist_wheel
from setuptools.command.install import install

PACKAGE_NAME = "hermes_python"
here = os.path.dirname(os.path.abspath(__file__))
PACKAGE_PATH = os.path.join(here, PACKAGE_NAME)
README = os.path.join(here, "README.rst")
HISTORY = os.path.join(here, "documentation/source/HISTORY.rst")
VERSION = "__version__.py"

DYLIB_PATH = os.path.join(PACKAGE_PATH, "dylib")
SHARED_OBJECT_FILENAME = "libhermes_mqtt_ffi"
SHARED_OBJECT_EXTENSION = ".dylib" if sys.platform.startswith("darwin") else ".so"
SHARED_OBJECT_PATH = os.path.join(DYLIB_PATH, SHARED_OBJECT_FILENAME + SHARED_OBJECT_EXTENSION)


class InstallPlatlib(install):
    def finalize_options(self):
        install.finalize_options(self)
        self.install_lib = self.install_platlib

    def run(self):
        BUILT_SHARED_OBJECT_PATH = os.path.join(
            os.path.normpath(os.path.join(here, "../..")),
            "target/release/{}{}".format(SHARED_OBJECT_FILENAME, SHARED_OBJECT_EXTENSION)
        )

        if not os.path.exists(SHARED_OBJECT_PATH):
            if not os.path.exists(BUILT_SHARED_OBJECT_PATH):
                return_code = subprocess.call(["cargo", "build", "-p", "hermes-mqtt-ffi", "--release"])
                if return_code > 0:
                    raise Exception("Could not compile C bindings")

            shutil.copy(BUILT_SHARED_OBJECT_PATH, DYLIB_PATH)

        install.run(self)


class bdist_wheel(_bdist_wheel, object):
    def finalize_options(self):
        _bdist_wheel.finalize_options(self)
        # noinspection PyAttributeOutsideInit
        self.root_is_pure = False

    def get_tag(self):
        return super(bdist_wheel, self).get_tag()


with io.open(os.path.join(PACKAGE_PATH, VERSION), encoding="utf8") as f:
    about = dict()
    exec(f.read(), about)

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
    ],
}

setup(
    name=about['__title__'],
    version=about['__version__'],
    description=about['__description__'],
    long_description=readme + history,
    author='Anthony Reinette',
    author_email='anthony.reinette@snips.ai',
    project_urls=about['__url__'],
    classifiers=[
        'Programming Language :: Python :: 2',
        'Programming Language :: Python :: 2.7',
        'Programming Language :: Python :: 3.5'],
    download_url='',
    license='MIT',
    keywords=['snips'],
    install_requires=['six', 'future', 'typing', 'enum34'],
    test_suite="tests",
    extras_require=extras_require,
    packages=packages,
    cmdclass={
        'bdist_wheel': bdist_wheel,
        'install': InstallPlatlib},
    command_options={
        'documentation': {
            'project': ('setup.py', 'Hermes Python'),
            'version': ('setup.py', about['__version__']),
            'source_dir': ('setup.py', './documentation/source'),
            'build_dir': ('setup.py', './documentation/build'),
            'builder': ('setup.py', 'doctest rst')
        }
    },
    zip_safe=False,
    include_package_data=True,
)
