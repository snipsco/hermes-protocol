from __future__ import absolute_import
from __future__ import unicode_literals

import os
from glob import glob
from ctypes import cdll


class MockedLib(object):  # This class is here in case you need to mock calls to the actual library.
    def __getattr__(self, item):
        return MockedLib()

    def __call__(self, *args, **kwargs):
        raise LibException("Trying to call mocked FFI library")


class LibException(Exception):
    pass


DYLIB_NAME = "libhermes_mqtt_ffi.*"
DYLIB_DIR = os.path.join(os.path.dirname(__file__), "../dylib")
DYLIB_PATHS = glob(os.path.join(DYLIB_DIR, DYLIB_NAME))

lib = cdll.LoadLibrary(DYLIB_PATHS[0]) if len(DYLIB_PATHS) > 0 else MockedLib()
