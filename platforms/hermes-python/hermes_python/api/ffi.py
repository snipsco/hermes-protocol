from ctypes import POINTER, c_void_p, c_char_p, byref
from ..ffi.ontology import CProtocolHandler, CMqttOptions
from ..ffi.ontology.facades import CDialogueFacade
from ..ffi.ontology.dialogue import CIntentMessage, CSessionStartedMessage, \
    CSessionQueuedMessage, CSessionEndedMessage, CIntentNotRecognizedMessage
from ..ffi.utils import hermes_protocol_handler_new_mqtt_with_options, \
    hermes_protocol_handler_dialogue_facade, hermes_drop_dialogue_facade
from ..ffi.wrappers import ffi_function_callback_wrapper
from ..ffi import utils, lib
from ..ontology.dialogue import IntentMessage, SessionStartedMessage, SessionQueuedMessage, SessionEndedMessage, \
    IntentNotRecognizedMessage


class FFI(object):
    def __init__(self, use_json_api=True, rust_logs_enabled=False):
        self.use_json_api = use_json_api
        self.rust_logs_enabled = rust_logs_enabled

        # API Subsets
        self.dialogue = DialogueFFI(use_json_api)
        self.audio = AudioFFI(use_json_api)
        self.injection = InjectionFFI(use_json_api)

        self._protocol_handler = POINTER(CProtocolHandler)()

    def establish_connection(self, mqtt_options):
        c_mqtt_options = CMqttOptions.from_repr(mqtt_options)

        hermes_protocol_handler_new_mqtt_with_options(byref(self._protocol_handler), byref(c_mqtt_options))
        self.initialize_facades()

        if self.rust_logs_enabled:
            lib.hermes_enable_debug_logs()

    def initialize_facades(self):
        self.dialogue.initialize_facade(self._protocol_handler)

    def release_facades(self):
        self.dialogue.release_facade()

    def release_connection(self):
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


class DialogueFFI(object):
    def __init__(self, use_json_api=True):
        self.use_json_api = use_json_api
        self._facade = POINTER(CDialogueFacade)()

        # References to callbacks called from C
        self._c_callback_subscribe_intent = []
        self._c_callback_subscribe_intents = None
        self._c_callback_subscribe_session_started = None
        self._c_callback_subscribe_session_queued = None
        self._c_callback_subscribe_session_ended = None
        self._c_callback_subscribe_intent_not_recognized = None

    def initialize_facade(self, protocol_handler):
        hermes_protocol_handler_dialogue_facade(protocol_handler, byref(self._facade))

    def release_facade(self):
        hermes_drop_dialogue_facade(self._facade)
        self._facade = POINTER(CDialogueFacade)()

    def register_subscribe_intent_handler(self, intent_name, user_defined_callback):
        c_intent_handler_callback = ffi_function_callback_wrapper(use_json_api=self.use_json_api,
                                                                  hermes_client=self,
                                                                  target_handler_return_type=c_void_p,
                                                                  handler_function=user_defined_callback,
                                                                  handler_argument_type=IntentMessage,
                                                                  target_handler_argument_type=CIntentMessage)
        self._c_callback_subscribe_intent.append(c_intent_handler_callback)  # Register callback
        number_of_callbacks = len(self._c_callback_subscribe_intent)

        self._register_c_intent_handler(
            'hermes_dialogue_subscribe_intent',
            intent_name,
            self._c_callback_subscribe_intent[number_of_callbacks - 1])  # We retrieve the last callback we registered

        return self

    def register_subscribe_intents_handler(self, user_defined_callback):
        c_handler_callback = ffi_function_callback_wrapper(
            use_json_api=self.use_json_api,
            hermes_client=self,
            target_handler_return_type=c_void_p,
            handler_function=user_defined_callback,
            handler_argument_type=IntentMessage,
            target_handler_argument_type=CIntentMessage)

        self._c_callback_subscribe_intents = c_handler_callback

        self._register_c_handler(
            'hermes_dialogue_subscribe_intents',
            self._c_callback_subscribe_intents
        )

        return self

    def register_session_started_handler(self, user_defined_callback):
        c_handler_callback = ffi_function_callback_wrapper(
            use_json_api=self.use_json_api,
            hermes_client=self,
            target_handler_return_type=c_void_p,
            handler_function=user_defined_callback,
            handler_argument_type=SessionStartedMessage,
            target_handler_argument_type=CSessionStartedMessage)

        self._c_callback_subscribe_session_started = c_handler_callback
        self._register_c_handler(
            'hermes_dialogue_subscribe_session_started',
            self._c_callback_subscribe_session_started
        )
        return self

    def register_session_queued_handler(self, user_defined_callback):
        c_handler_callback = ffi_function_callback_wrapper(
            use_json_api=self.use_json_api,
            hermes_client=self,
            target_handler_return_type=c_void_p,
            handler_function=user_defined_callback,
            handler_argument_type=SessionQueuedMessage,
            target_handler_argument_type=CSessionQueuedMessage)

        self._c_callback_subscribe_session_queued = c_handler_callback
        self._register_c_handler(
            'hermes_dialogue_subscribe_session_queued',
            self._c_callback_subscribe_session_queued
        )
        return self

    def register_session_ended_handler(self, user_defined_callback):
        c_handler_callback = ffi_function_callback_wrapper(
            use_json_api=self.use_json_api,
            hermes_client=self,
            target_handler_return_type=c_void_p,
            handler_function=user_defined_callback,
            handler_argument_type=SessionEndedMessage,
            target_handler_argument_type=CSessionEndedMessage)

        self._c_callback_subscribe_session_ended = c_handler_callback
        self._register_c_handler(
            'hermes_dialogue_subscribe_session_ended',
            self._c_callback_subscribe_session_ended
        )
        return self

    def register_intent_not_recognized_handler(self, user_defined_callback):
        c_handler_callback = ffi_function_callback_wrapper(
            use_json_api=self.use_json_api,
            hermes_client=self,
            target_handler_return_type=c_void_p,
            handler_function=user_defined_callback,
            handler_argument_type=IntentNotRecognizedMessage,
            target_handler_argument_type=CIntentNotRecognizedMessage)

        self._c_callback_subscribe_intent_not_recognized = c_handler_callback
        self._register_c_handler(
            'hermes_dialogue_subscribe_intent_not_recognized',
            self._c_callback_subscribe_intent_not_recognized
        )
        return self

    def publish_continue_session(self, message):
        self._call_foreign_function(
            'hermes_dialogue_publish_continue_session',
            message
        )
        return self

    def publish_end_session(self, message):
        self._call_foreign_function(
            'hermes_dialogue_publish_end_session',
            message
        )
        return self

    def publish_start_session(self, message):
        self._call_foreign_function(
            'hermes_dialogue_publish_start_session',
            message
        )
        return self

    def _register_c_handler(self, ffi_function_name, c_handler):
        if self.use_json_api:
            ffi_function_name = ffi_function_name + "_json"

        getattr(utils, ffi_function_name)(
            self._facade,
            c_handler
        )
        return self

    def _register_c_intent_handler(self, ffi_function_name, intent_name, c_handler):
        if self.use_json_api:
            ffi_function_name = ffi_function_name + "_json"

        getattr(utils, ffi_function_name)(
            self._facade,
            c_char_p(intent_name.encode('utf-8')),
            c_handler
        )
        return self

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


# API subset stubs


class InjectionFFI(object):
    def __init__(self, use_json_api=True):
        pass


class AudioFFI(object):
    def __init__(self, use_json_api=True):
        pass
