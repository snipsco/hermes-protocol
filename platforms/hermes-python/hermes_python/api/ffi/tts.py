from ctypes import POINTER, c_char_p, byref

from ...ffi import utils
from ...ffi.ontology.facades import CTtsFacade
from ...ffi.utils import hermes_protocol_handler_tts_facade, hermes_drop_tts_facade


class TtsFFI(object):
    def __init__(self, use_json_api=True):
        self.use_json_api = use_json_api
        self._facade = POINTER(CTtsFacade)()

    def initialize_facade(self, protocol_handler):
        hermes_protocol_handler_tts_facade(protocol_handler, byref(self._facade))

    def release_facade(self):
        hermes_drop_tts_facade(self._facade)
        self._facade = POINTER(CTtsFacade)()

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

    def publish_register_sound(self, message):
        self._call_foreign_function(
            'hermes_tts_publish_register_sound',
            message
        )
        return self
