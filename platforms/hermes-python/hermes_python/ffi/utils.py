# -*- coding: utf-8 -*-
from __future__ import absolute_import
from __future__ import unicode_literals
from glob import glob
import os
from builtins import bytes

from ctypes import cdll, CFUNCTYPE, c_void_p, c_char_p, POINTER, Structure, c_uint8
from .ontology import CStringArray
from ..ontology import *

DYLIB_NAME = "libhermes_mqtt_ffi.*"
DYLIB_DIR = os.path.join(os.path.dirname(__file__), "../dylib")
DYLIB_PATH = glob(os.path.join(DYLIB_DIR, DYLIB_NAME))[0]

lib = cdll.LoadLibrary(DYLIB_PATH)

# Mqtt options

class CMqttOptions(Structure):
    _fields_ = [("broker_address", c_char_p),
                ("username", c_char_p),
                ("password", c_char_p),
                ("tls_hostname", c_char_p),
                ("tls_ca_file", POINTER(CStringArray)),
                ("tls_ca_path", POINTER(CStringArray)),
                ("tls_client_key", c_char_p),
                ("tls_client_cert", c_char_p),
                ("tls_disable_root_store", c_uint8)]

    @classmethod
    def build(cls, broker_address, username, password, tls_hostname, tls_ca_file, tls_ca_path, tls_client_key, tls_client_cert, tls_disable_root_store):
        broker_address = broker_address.encode('utf-8')
        username = username.encode('utf-8') if username else None
        password = password.encode('utf-8') if password else None
        tls_hostname = tls_hostname.encode('utf-8') if tls_hostname else None
        tls_ca_file = tls_ca_file.encode('utf-8') if tls_ca_file else None
        tls_ca_path = tls_ca_path.encode('utf-8') if tls_ca_path else None
        tls_client_key = tls_client_key.encode('utf-8') if tls_client_key else None
        tls_client_cert = tls_client_cert.encode('utf-8') if tls_client_cert else None
        tls_disable_root_store = 1 if tls_disable_root_store else 0  # tls_disable_root_store is a boolean

        return cls(broker_address,
                   username, password,
                   tls_hostname, tls_ca_file, tls_ca_path, tls_client_key, tls_client_cert, tls_disable_root_store)

    @classmethod
    def from_repr(cls, repr):
        return cls.build(repr.broker_address,
                         repr.username, repr.password,
                         repr.tls_hostname, repr.tls_ca_file, repr.tls_ca_path, repr.tls_client_key, repr.tls_client_cert, repr.tls_disable_root_store)


class MqttOptions(object):
    def __init__(self,
                 broker_address="localhost:1883",
                 username=None, password=None,
                 tls_hostname=None, tls_ca_file=None, tls_ca_path=None, tls_client_key=None, tls_client_cert=None, tls_disable_root_store=False):
        """
        :param broker_address: Address of the MQTT broker in the form 'ip:port'
        :param username: Username to use on the broker. Nullable
        :param password: Password to use on the broker. Nullable
        :param tls_hostname: Hostname to use for the TLS configuration. Nullable, setting a value enables TLS
        :param tls_ca_file: CA files to use if TLS is enabled. Nullable
        :param tls_ca_path: CA path to use if TLS is enabled. Nullable
        :param tls_client_key: Client key to use if TLS is enabled. Nullable
        :param tls_client_cert: Client cert to use if TLS is enabled. Nullable
        :param tls_disable_root_store: Boolean indicating if the root store should be disabled if TLS is enabled.
        """
        self.broker_address = broker_address

        self.username = username
        self.password = password

        self.tls_hostname = tls_hostname
        self.tls_ca_file = tls_ca_file
        self.tls_ca_path = tls_ca_path
        self.tls_client_key = tls_client_key
        self.tls_client_cert = tls_client_cert
        self.tls_disable_root_store = tls_disable_root_store


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
hermes_dialogue_subscribe_intent_not_recognized = wrap_library_call(lib.hermes_dialogue_subscribe_intent_not_recognized)

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
hermes_protocol_handler_new_mqtt_with_options = wrap_library_call(lib.hermes_protocol_handler_new_mqtt_with_options)
hermes_protocol_handler_dialogue_facade = wrap_library_call(lib.hermes_protocol_handler_dialogue_facade)

