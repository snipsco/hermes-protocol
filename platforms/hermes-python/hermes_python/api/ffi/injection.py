from ctypes import POINTER, c_char_p, byref, c_void_p

from ...ffi import utils
from ...ffi.wrappers import ffi_function_callback_wrapper
from ...ffi.ontology.facades import CInjectionFacade
from ...ffi.utils import hermes_protocol_handler_injection_facade, hermes_drop_injection_facade
from ...ffi.ontology.injection import CInjectionStatusMessage, CInjectionCompleteMessage

from ...ontology.injection import InjectionStatusMessage, InjectionCompleteMessage


class InjectionFFI(object):
    def __init__(self, use_json_api=True):
        self.use_json_api = use_json_api
        self._facade = POINTER(CInjectionFacade)()

        # References to callbacks called from C
        self._c_callback_subscribe_injection_status = []

        # References to callbacks called from C
        self._c_callback_subscribe_injection_complete = []

    def initialize_facade(self, protocol_handler):
        hermes_protocol_handler_injection_facade(protocol_handler, byref(self._facade))

    def release_facade(self):
        hermes_drop_injection_facade(self._facade)
        self._facade = POINTER(CInjectionFacade)()

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

    def _register_c_handler(self, ffi_function_name, c_handler):
        if self.use_json_api:
            ffi_function_name = ffi_function_name + "_json"

        getattr(utils, ffi_function_name)(
            self._facade,
            c_handler
        )
        return self

    def _call_foreign_function_no_arg(self, foreign_function_name):  # TODO rename
        if self.use_json_api:
            foreign_function_name = foreign_function_name + "_json"

        getattr(utils, foreign_function_name)(self._facade)

    def publish_injection_request(self, message):
        self._call_foreign_function(
            'hermes_injection_publish_injection_request',
            message
        )
        return self

    def publish_injection_status_request(self):
        self._call_foreign_function_no_arg('hermes_injection_publish_injection_status_request')
        return self

    def register_subscribe_injection_status(self, user_defined_callback, hermes_client):
        c_intent_handler_callback = ffi_function_callback_wrapper(use_json_api=self.use_json_api,
                                                                  hermes_client=hermes_client,
                                                                  target_handler_return_type=c_void_p,
                                                                  handler_function=user_defined_callback,
                                                                  handler_argument_type=InjectionStatusMessage,
                                                                  target_handler_argument_type=CInjectionStatusMessage)

        self._c_callback_subscribe_injection_status.append(c_intent_handler_callback)  # Register callback
        number_of_callbacks = len(self._c_callback_subscribe_injection_status)

        self._register_c_handler(
            'hermes_injection_subscribe_injection_status',
            self._c_callback_subscribe_injection_status[
                number_of_callbacks - 1])  # We retrieve the last callback we registered

        return self

    def register_subscribe_injection_complete(self, user_defined_callback, hermes_client):
        c_intent_handler_callback = ffi_function_callback_wrapper(use_json_api=self.use_json_api,
                                                                  hermes_client=hermes_client,
                                                                  target_handler_return_type=c_void_p,
                                                                  handler_function=user_defined_callback,
                                                                  handler_argument_type=InjectionCompleteMessage,
                                                                  target_handler_argument_type=CInjectionCompleteMessage)

        self._c_callback_subscribe_injection_complete.append(c_intent_handler_callback)
        number_of_callbacks = len(self._c_callback_subscribe_injection_complete)

        self._register_c_handler(
            'hermes_injection_subscribe_injection_complete',
            self._c_callback_subscribe_injection_complete[
                number_of_callbacks - 1])  # We retrieve the last callback we registered

        return self
