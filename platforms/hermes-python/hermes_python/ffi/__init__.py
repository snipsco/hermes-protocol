from __future__ import absolute_import
from __future__ import unicode_literals

import os
from glob import glob
from ctypes import cdll

DYLIB_NAME = "libhermes_mqtt_ffi.*"
DYLIB_DIR = os.path.join(os.path.dirname(__file__), "../dylib")
DYLIB_PATH = glob(os.path.join(DYLIB_DIR, DYLIB_NAME))[0]

lib = cdll.LoadLibrary(DYLIB_PATH)

class LibException(Exception):
    pass

