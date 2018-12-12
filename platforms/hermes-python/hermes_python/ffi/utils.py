# -*- coding: utf-8 -*-
from __future__ import absolute_import
from __future__ import unicode_literals
from glob import glob
import os
from ctypes import cdll, CFUNCTYPE, c_void_p, c_char_p, POINTER
from ..ontology import *

DYLIB_NAME = "libhermes_mqtt_ffi.*"
DYLIB_DIR = os.path.join(os.path.dirname(__file__), "../dylib")
DYLIB_PATH = glob(os.path.join(DYLIB_DIR, DYLIB_NAME))[0]

lib = cdll.LoadLibrary(DYLIB_PATH)

# FFI decorators
class LibException(Exception):
    pass


def wrap_library_call(lib_func):
    def wrapped_library_call(*args, **kwargs):
        return_code = lib_func(*args, **kwargs)
        if return_code > 0:  # An error occured
            empty_string = "".encode('utf-8')
            error_p = POINTER(c_char_p)(c_char_p(empty_string))
            lib.hermes_get_last_error(error_p) # Retrieve the last error and put it in the memory location error_p points to
            error_cause = string_at(error_p.contents).decode('utf-8')
            raise LibException(error_cause)
            return return_code
        return return_code

    return wrapped_library_call


def hermes_wrap(func):
    def wrapper(self, *args, **kwargs):
        return func(self, *args, **kwargs)

    return wrapper


def user_callback(argument_type):
    def callback(user_callback):
        def sanitized(self, *args):
            parsed_args = (argument_type.from_c_repr(arg.contents) for arg in (args))
            result = user_callback(self, *parsed_args)
            return result

        return sanitized

    return callback


def subscribe_callback(argument_type):
    def decorate(func):
        return CFUNCTYPE(c_void_p, POINTER(argument_type))(func)

    return decorate

# Shortcuts


hermes_dialogue_publish_continue_session = wrap_library_call(lib.hermes_dialogue_publish_continue_session)
hermes_dialogue_publish_end_session = wrap_library_call(lib.hermes_dialogue_publish_end_session)
hermes_dialogue_publish_start_session = wrap_library_call(lib.hermes_dialogue_publish_start_session)

hermes_dialogue_subscribe_intent = wrap_library_call(lib.hermes_dialogue_subscribe_intent)
hermes_dialogue_subscribe_intents = wrap_library_call(lib.hermes_dialogue_subscribe_intents)
hermes_dialogue_subscribe_session_ended = wrap_library_call(lib.hermes_dialogue_subscribe_session_ended)
hermes_dialogue_subscribe_session_queued = wrap_library_call(lib.hermes_dialogue_subscribe_session_queued)
hermes_dialogue_subscribe_session_started = wrap_library_call(lib.hermes_dialogue_subscribe_session_started)

hermes_drop_asr_backend_facade = wrap_library_call(lib.hermes_drop_asr_backend_facade)
hermes_drop_asr_facade = wrap_library_call(lib.hermes_drop_asr_facade)
hermes_drop_audio_server_backend_facade = wrap_library_call(lib.hermes_drop_audio_server_backend_facade)
hermes_drop_audio_server_facade = wrap_library_call(lib.hermes_drop_audio_server_facade)
hermes_drop_dialogue_backend_facade = wrap_library_call(lib.hermes_drop_dialogue_backend_facade)
hermes_drop_dialogue_facade = wrap_library_call(lib.hermes_drop_dialogue_facade)
hermes_drop_hotword_backend_facade = wrap_library_call(lib.hermes_drop_hotword_backend_facade)
hermes_drop_hotword_facade = wrap_library_call(lib.hermes_drop_hotword_facade)
hermes_drop_nlu_backend_facade = wrap_library_call(lib.hermes_drop_nlu_backend_facade)
hermes_drop_nlu_facade = wrap_library_call(lib.hermes_drop_nlu_facade)
hermes_drop_sound_feedback_backend_facade = wrap_library_call(lib.hermes_drop_sound_feedback_backend_facade)
hermes_drop_sound_feedback_facade = wrap_library_call(lib.hermes_drop_sound_feedback_facade)
hermes_drop_tts_backend_facade = wrap_library_call(lib.hermes_drop_tts_backend_facade)
hermes_drop_tts_facade = wrap_library_call(lib.hermes_drop_tts_facade)

hermes_protocol_handler_new_mqtt = wrap_library_call(lib.hermes_protocol_handler_new_mqtt)
hermes_protocol_handler_dialogue_facade = wrap_library_call(lib.hermes_protocol_handler_dialogue_facade)