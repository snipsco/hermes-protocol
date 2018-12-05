# -*- coding: utf-8 -*-
from __future__ import absolute_import
from __future__ import unicode_literals
from glob import glob
import os
from ctypes import cdll, CFUNCTYPE, c_void_p, POINTER
from ..ontology import *

DYLIB_NAME = "libhermes_mqtt_ffi.*"
DYLIB_DIR = os.path.join(os.path.dirname(__file__), "../dylib")
DYLIB_PATH = glob(os.path.join(DYLIB_DIR, DYLIB_NAME))[0]

lib = cdll.LoadLibrary(DYLIB_PATH)

# Shortcuts

hermes_dialogue_publish_continue_session = lib.hermes_dialogue_publish_continue_session
hermes_dialogue_publish_end_session = lib.hermes_dialogue_publish_end_session
hermes_dialogue_publish_start_session = lib.hermes_dialogue_publish_start_session

hermes_dialogue_subscribe_intent = lib.hermes_dialogue_subscribe_intent
hermes_dialogue_subscribe_intents = lib.hermes_dialogue_subscribe_intents
hermes_dialogue_subscribe_session_ended = lib.hermes_dialogue_subscribe_session_ended
hermes_dialogue_subscribe_session_queued = lib.hermes_dialogue_subscribe_session_queued
hermes_dialogue_subscribe_session_started = lib.hermes_dialogue_subscribe_session_started

hermes_drop_asr_backend_facade = lib.hermes_drop_asr_backend_facade
hermes_drop_asr_facade = lib.hermes_drop_asr_facade
hermes_drop_audio_server_backend_facade = lib.hermes_drop_audio_server_backend_facade
hermes_drop_audio_server_facade = lib.hermes_drop_audio_server_facade
hermes_drop_dialogue_backend_facade = lib.hermes_drop_dialogue_backend_facade
hermes_drop_dialogue_facade = lib.hermes_drop_dialogue_facade
hermes_drop_hotword_backend_facade = lib.hermes_drop_hotword_backend_facade
hermes_drop_hotword_facade = lib.hermes_drop_hotword_facade
hermes_drop_nlu_backend_facade = lib.hermes_drop_nlu_backend_facade
hermes_drop_nlu_facade = lib.hermes_drop_nlu_facade
hermes_drop_sound_feedback_backend_facade = lib.hermes_drop_sound_feedback_backend_facade
hermes_drop_sound_feedback_facade = lib.hermes_drop_sound_feedback_facade
hermes_drop_tts_backend_facade = lib.hermes_drop_tts_backend_facade
hermes_drop_tts_facade = lib.hermes_drop_tts_facade

# FFI decorators

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
