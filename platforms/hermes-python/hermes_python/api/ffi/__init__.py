from ctypes import POINTER, c_char_p, byref
from ...ffi.ontology import CProtocolHandler, CMqttOptions
from ...ffi.utils import hermes_protocol_handler_new_mqtt_with_options, hermes_destroy_mqtt_protocol_handler
from ...ffi import utils, lib

from .dialogue import DialogueFFI
from .feedback import SoundFeedBackFFI
from .injection import InjectionFFI
from .tts import TtsFFI


class FFI(object):
    def __init__(self, use_json_api=True, rust_logs_enabled=False):
        self.use_json_api = use_json_api
        self.rust_logs_enabled = rust_logs_enabled

        # API Subsets
        self.dialogue = DialogueFFI(use_json_api)
        self.sound_feedback = SoundFeedBackFFI(use_json_api)
        self.injection = InjectionFFI(use_json_api)
        self.tts = TtsFFI(use_json_api)

        self._protocol_handler = POINTER(CProtocolHandler)()

    def establish_connection(self, mqtt_options):
        c_mqtt_options = CMqttOptions.from_repr(mqtt_options)

        hermes_protocol_handler_new_mqtt_with_options(byref(self._protocol_handler), byref(c_mqtt_options))
        self.initialize_facades()

        if self.rust_logs_enabled:
            lib.hermes_enable_debug_logs()

    def initialize_facades(self):
        self.dialogue.initialize_facade(self._protocol_handler)
        self.sound_feedback.initialize_facade(self._protocol_handler)
        self.injection.initialize_facade(self._protocol_handler)
        self.tts.initialize_facade(self._protocol_handler)

    def release_facades(self):
        self.dialogue.release_facade()
        self.sound_feedback.release_facade()
        self.injection.release_facade()
        self.tts.release_facade()

    def release_connection(self):
        hermes_destroy_mqtt_protocol_handler(self._protocol_handler)
        self._protocol_handler = POINTER(CProtocolHandler)()
        self.release_facades()

    def _call_foreign_function(self, foreign_function_name, function_argument):
        if self.use_json_api:
            foreign_function_name = foreign_function_name + "_json"
            a_json_string = str(function_argument)  # function_argument should be a dict.
            ptr_to_foreign_function_argument = c_char_p(a_json_string.encode('utf-8'))
        else:
            function_argument = function_argument.into_c_repr()
            ptr_to_foreign_function_argument = byref(function_argument)

        getattr(utils, foreign_function_name)(
            self._facade,
            ptr_to_foreign_function_argument
        )
