#! /usr/bin/env python
# encoding: utf-8

import io
import os
from setuptools import setup, find_packages
import sys
import subprocess
import shutil
from setuptools import Command

from wheel.bdist_wheel import bdist_wheel as _bdist_wheel
from setuptools.command.install import install

PACKAGE_NAME = "hermes_python"
here = os.path.dirname(os.path.abspath(__file__))
PACKAGE_PATH = os.path.join(here, PACKAGE_NAME)
README = os.path.join(here, "README.rst")
HISTORY = os.path.join(here, "documentation/source/HISTORY.rst")
VERSION = "__version__.py"


def log(level, msg):
    levels = {
        'info': '\033[36m',
        'warning': '\033[33m',
        'error': '\033[31m',
        'success': '\033[32m'
    }
    print(levels[level] + msg + '\033[39m')

class InstallPlatlib(install):
    def finalize_options(self):
        install.finalize_options(self)
        self.install_lib = self.install_platlib

class HermesExtension(Command):
    DYLIB_PATH = os.path.join(PACKAGE_PATH, "dylib")
    SHARED_OBJECT_FILENAME = "libhermes_mqtt_ffi"
    SHARED_OBJECT_EXTENSION = ".dylib" if sys.platform.startswith("darwin") else ".so"
    SHARED_OBJECT_PATH = os.path.join(DYLIB_PATH, SHARED_OBJECT_FILENAME + SHARED_OBJECT_EXTENSION)

    description = "Builds the compiled extension hermes-mqtt-ffi required by hermes-python." \
                  "or includes an already compiled extension if the path to the latter is provided. " \
                  "\n" \
                  "Requires cargo + rust to be installed : \n" \
                  "=> please visit : https://www.rustup.rs"

    user_options = [
        ('include-extension=', None,
         'path to the compiled Hermes extension. If provided, it will not build the extension'),
    ]

    BUILD_HERMES_EXTENSION_COMMAND = ["cargo", "build", "-p", "hermes-mqtt-ffi", "--release"]


    def initialize_options(self):
        log("info", "preparing hermes extension building step")
        self.include_extension = None

    def finalize_options(self):
        if self.include_extension:  # Try to include an pre-compiled hermes extension.
            if self.include_extension == "default":  # We look in the default places :
                log("warning",
                    "assuming the extension is pre-compiled. Will look up for the extensions in default paths. ")
                if not os.path.exists(self.SHARED_OBJECT_PATH):  # if not in hermes_python/dylib/
                    log('error', "Did not find compiled extension in : {}".format(self.SHARED_OBJECT_PATH))

                    BUILT_SHARED_OBJECT_PATH = os.path.join(
                        os.path.normpath(os.path.join(here, "../..")),
                        "target/release/{}{}".format(self.SHARED_OBJECT_FILENAME, self.SHARED_OBJECT_EXTENSION)
                    )

                    if not os.path.exists(BUILT_SHARED_OBJECT_PATH):  # if not in ../../target/release/
                        log("error", "Did not find compiled extensions under : {}".format(
                            BUILT_SHARED_OBJECT_PATH))
                        raise Exception("Could not find any pre-compiled hermes extension in : {} nor {} ..."
                                        "The commands you previously ran, did not output compiled extensions."
                                        "You can build extensions with the command : '{}'"
                                        .format(self.SHARED_OBJECT_PATH,
                                                BUILT_SHARED_OBJECT_PATH,
                                                " ".join(self.BUILD_HERMES_EXTENSION_COMMAND)))
                    else:
                        log("success", "Found extension at : {}".format(BUILT_SHARED_OBJECT_PATH))
                        self.include_extension = BUILT_SHARED_OBJECT_PATH
                else:
                    log("success", "Found extension at : {}".format(self.SHARED_OBJECT_PATH))
                    self.include_extension = self.SHARED_OBJECT_PATH

            else:
                if not os.path.exists(self.include_extension):  # We check that the provided extension exists
                    raise Exception("the provided path to the compiled extension : {} doesn't exists ...")

            self.include_extension = os.path.normpath(self.include_extension)
        else:
            log("warning", "No path to pre-compiled extension provided. "
                           "Proceeding to compile the extension in release mode.")

    def run(self):
        log("warning", 10 * "=" + " Compiling Hermes Extension Step " + 10 * "=")

        if not self.include_extension:
            return_code = subprocess.call(["cargo", "build", "-p", "hermes-mqtt-ffi", "--release"])
            if return_code > 0:
                raise Exception("Could not compile C bindings, the command : '{}' "
                                "exited with a non-zero error code ... "
                                .format(" ".join(self.BUILD_HERMES_EXTENSION_COMMAND)))
            else:
                BUILT_SHARED_OBJECT_PATH = os.path.join(
                    os.path.normpath(os.path.join(here, "../..")),
                    "target/release/{}{}".format(self.SHARED_OBJECT_FILENAME, self.SHARED_OBJECT_EXTENSION)
                )
                log("success", "Done compiling hermes extension !")
                self.include_extension = BUILT_SHARED_OBJECT_PATH

        log("error", "about to look into : {}".format(self.include_extension))
        if os.path.samefile(self.include_extension, self.DYLIB_PATH):
            log("error", "got here")
            shutil.copy(self.include_extension, self.DYLIB_PATH)
        log("success", "Copied {} -> {}".format(self.include_extension, self.DYLIB_PATH))

        log("warning", 10 * "=" + " End of Compiling Hermes Extension Step " + 10 * "=")


class bdist_wheel(_bdist_wheel, HermesExtension):
    user_options = _bdist_wheel.user_options + HermesExtension.user_options

    def initialize_options(self):
        HermesExtension.initialize_options(self)
        _bdist_wheel.initialize_options(self)

    def finalize_options(self):
        _bdist_wheel.finalize_options(self)
        HermesExtension.finalize_options(self)
        # noinspection PyAttributeOutsideInit
        self.root_is_pure = False

    def run(self):
        HermesExtension.run(self)
        _bdist_wheel.run(self)

    def get_tag(self):
        return _bdist_wheel.get_tag(self)


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
        'install': InstallPlatlib,
        'hermes_extension': HermesExtension},
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
