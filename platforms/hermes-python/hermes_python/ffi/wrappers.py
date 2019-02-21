from ..ffi import lib, LibException
from ctypes import POINTER, c_char_p, string_at, CFUNCTYPE
import json


def wrap_library_call(lib_func):
    """
    This helper function wrap ffi functions so that they raise an Exception when the error code is different than 0.
    """
    def wrapped_library_call(*args, **kwargs):
        return_code = lib_func(*args, **kwargs)
        if return_code > 0:  # An error occured
            empty_string = "".encode('utf-8')
            error_p = POINTER(c_char_p)(c_char_p(empty_string))
            # Retrieve the last error and put it in the memory location error_p points to
            lib.hermes_get_last_error(error_p)
            error_cause = string_at(error_p.contents).decode('utf-8')
            raise LibException(error_cause)
        return return_code

    return wrapped_library_call


def parse_json_string(ptr_to_utf_8_encoded_string):
    return json.loads(string_at(ptr_to_utf_8_encoded_string).decode('utf-8'))


def ffi_function_callback_wrapper(use_json_api, hermes_client, target_handler_return_type, handler_function,
                                  handler_argument_type=None, target_handler_argument_type=None):
    """
    We need to provide the C library a handler function that will be called
    when the event the handler should handle is triggered.
    This handler has `target_handler_return_type`, and has arguments with type `target_handler_argument_type`.

    The goal of this function is to convert a handler written in python (`handler_function`)
    to a C handler with appropriate types.

    Let's go through the arguments:
    :param use_json_api: A flag, if activated, all arguments of handler callback will be python dictionaries.
    :param hermes_client:
    :param target_handler_return_type: The type to which the python handler return type will be converted to.
    :param target_handler_argument_type: Optional (not used if use_json_api is activated). The type to which the python handler arguments will be converted to.
    :param handler_function: a python function
    :param handler_argument_type: Optional (not used if use_json_api is activated). The type of the arguments the handler will be called with.
    :return: A C handler function that will be called when events it is registered to happens.

    """
    if use_json_api:
        def convert_function_arguments(func):
            def convert_arguments_when_invoking_function(*args, **kwargs):
                    parsed_args = (parse_json_string(arg) for arg in (args))
                    return func(hermes_client, *parsed_args)
            return convert_arguments_when_invoking_function
        return CFUNCTYPE(target_handler_return_type, c_char_p)(
            convert_function_arguments(handler_function))
    else:
        def convert_function_arguments(func):
            def convert_arguments_when_invoking_function(*args, **kwargs):
                parsed_args = (handler_argument_type.from_c_repr(arg.contents) for arg in (args))
                return func(hermes_client, *parsed_args)

            return convert_arguments_when_invoking_function
        return CFUNCTYPE(target_handler_return_type, POINTER(target_handler_argument_type))(
                convert_function_arguments(handler_function))


