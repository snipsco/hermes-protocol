# coding: utf-8
from __future__ import unicode_literals
import ctypes
import pytest
from hermes_python.ffi.utils import wrap_library_call
from hermes_python.ffi.wrappers import parse_json_string

SUCCESS_EXIT_CODE = 0
ERROR_EXIT_CODE = 1

def test_wrap_library_call_raises_expection_for_error_return_code():
    def test_func():
        return ERROR_EXIT_CODE

    with pytest.raises(Exception):
        wrap_library_call(test_func)()


def test_wrap_library_call_doesnt_raises_expection_for_error_return_code():
    def test_func():
        return SUCCESS_EXIT_CODE

    assert wrap_library_call(test_func)() == SUCCESS_EXIT_CODE


def test_parsing_of_json_string_deserialization():
    a_string = '{"message": "my name is René 🤗"}'
    ptr_to_a_string = ctypes.c_char_p(a_string.encode('utf-8'))
    deserialized_string = parse_json_string(ptr_to_a_string)

    assert deserialized_string.get('message') == 'my name is René 🤗'
