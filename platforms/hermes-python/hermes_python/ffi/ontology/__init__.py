from __future__ import absolute_import
from __future__ import unicode_literals

from ctypes import c_char_p, c_int32, c_void_p, POINTER, Structure


class CStringArray(Structure):
    _fields_ = [
        ("data", POINTER(c_char_p)),
        ("size", c_int32)
    ]


class CProtocolHandler(Structure):
    _fields_ = [("handler", c_void_p)]


