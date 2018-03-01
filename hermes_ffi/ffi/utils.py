# -*- coding: utf-8 -*-
from glob import glob
import os
from ctypes import cdll, CFUNCTYPE, c_void_p, POINTER
from ..ontology import *

DYLIB_NAME = "libhermes_mqtt_ffi.dy*"
DYLIB_DIR = os.path.join(os.path.dirname(__file__), "../dylib")
DYLIB_PATH = glob(os.path.join(DYLIB_DIR, DYLIB_NAME))[0]

lib = cdll.LoadLibrary(DYLIB_PATH)


# FFI decorators for callbacks

def user_callback(argument_type):
    def callback(user_callback):
        def sanitized(*args):
            parsed_args = (argument_type.from_c_repr(arg.contents) for arg in (args))
            result = user_callback(*parsed_args)
            return result

        return sanitized

    return callback


def subscribe_callback(argument_type):
    def decorate(func):
        return CFUNCTYPE(c_void_p, POINTER(argument_type))(func)

    return decorate



# Shortcuts

hermes_dialogue_publish_continue_session = lib.hermes_dialogue_publish_continue_session
hermes_dialogue_publish_end_session = lib.hermes_dialogue_publish_end_session
hermes_dialogue_publish_start_session = lib.hermes_dialogue_publish_start_session

hermes_dialogue_subscribe_intent = lib.hermes_dialogue_subscribe_intent
hermes_dialogue_subscribe_intents = lib.hermes_dialogue_subscribe_intents
hermes_dialogue_subscribe_session_ended = lib.hermes_dialogue_subscribe_session_ended
hermes_dialogue_subscribe_session_queued = lib.hermes_dialogue_subscribe_session_queued
hermes_dialogue_subscribe_session_started = lib.hermes_dialogue_subscribe_session_started



# Unparametrized decorators
# Those are kept if the generic solution doesn't work.

"""
def callback(user_callback):
    def sanitized(*args):
        parsed_args = (IntentMessage.from_c_repr(arg.contents) for arg in (args))
        result = user_callback(*parsed_args)
        return result

    return sanitized
"""
