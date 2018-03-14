# -*- coding: utf-8 -*-

from ctypes import *
from ffi.ontology import CProtocolHandler, CDialogueFacade, CContinueSessionMessage, CEndSessionMessage, \
    CStartSessionMessage, CStringArray, CIntentMessage, CSessionStartedMessage, CSessionQueuedMessage, \
    CSessionEndedMessage
from ffi.utils import *
from time import sleep


class Hermes(object):
    def __init__(self, mqtt_server_address):
        self.mqtt_server_address = mqtt_server_address

        self._protocol_handler = POINTER(CProtocolHandler)()
        self._facade = POINTER(CDialogueFacade)()

        # References to callbacks called from C
        self._c_subscribe_intent_callback = None
        self._c_callback_subscribe_intents = None
        self._c_callback_subscribe_session_started = None
        self._c_callback_subscribe_session_queued = None
        self._c_callback_subscribe_session_ended = None

    def __enter__(self):
        lib.hermes_protocol_handler_new_mqtt(byref(self._protocol_handler), self.mqtt_server_address)

        lib.hermes_protocol_handler_dialogue_facade(self._protocol_handler,
                                                    byref(self._facade))

        return self

    def __exit__(self, exception_type, exception_val, trace):
        hermes_drop_dialogue_facade(self._facade)

        return True

    def _wraps(self, user_callback, callback_argtype, callback_restype, argtype):
        def params_converter(func):
            def called_with_good_params(*args, **kwargs):
                parsed_args = (IntentMessage.from_c_repr(arg.contents) for arg in (args))
                return func(self, *parsed_args)

            return called_with_good_params

        return CFUNCTYPE(callback_restype, POINTER(callback_argtype))(params_converter(user_callback))

    def subscribe_intent(self, intent_name, user_callback_subscribe_intent):
        self._c_callback_subscribe_intent = self._wraps(user_callback_subscribe_intent, CIntentMessage, c_void_p,
                                                        IntentMessage)
        hermes_dialogue_subscribe_intent(self._facade, c_char_p(intent_name), self._c_callback_subscribe_intent)
        return self

    def subscribe_intents(self, user_callback_subscribe_intents):
        self._c_callback_subscribe_intents = self._wraps(user_callback_subscribe_intents, CIntentMessage, c_void_p,
                                                         IntentMessage)
        hermes_dialogue_subscribe_intents(self._facade, self._c_callback_subscribe_intents)
        return self

    def subscribe_session_started(self, user_callback_subscribe_session_started):
        self._c_callback_subscribe_session_started = self._wraps(user_callback_subscribe_session_started,
                                                                 CSessionStartedMessage, c_void_p,
                                                                 SessionStartedMessage)
        hermes_dialogue_subscribe_session_started(self._facade, self._c_callback_subscribe_session_started)
        return self

    def subscribe_session_queued(self, user_callback_subscribe_session_queued):
        self._c_callback_subscribe_session_queued = self._wraps(user_callback_subscribe_session_queued,
                                                                CSessionQueuedMessage, c_void_p, SessionQueuedMessage)
        hermes_dialogue_subscribe_session_started(self._facade, self._c_callback_subscribe_session_queued)
        return self

    def subscribe_session_ended(self, user_callback_subscribe_session_ended):
        self._c_callback_subscribe_session_ended = self._wraps(user_callback_subscribe_session_ended,
                                                               CSessionEndedMessage, c_void_p, SessionEndedMessage)
        hermes_dialogue_subscribe_session_started(self._facade, self._c_callback_subscribe_session_ended)
        return self

    def publish_continue_session(self, session_id, text, intent_filter):
        cContinueSessionMessage = CContinueSessionMessage.build(session_id, text, intent_filter)
        hermes_dialogue_publish_continue_session(self._facade, byref(cContinueSessionMessage))
        return self

    def publish_end_session(self, session_id, text):
        cEndSessionMessage = CEndSessionMessage(session_id, text)
        hermes_dialogue_publish_end_session(self._facade, byref(cEndSessionMessage))
        return self

    def publish_start_session(self, custom_data, site_id, value):
        cStartSessionMessage = CStartSessionMessage.build(custom_data, site_id, value)
        hermes_dialogue_publish_start_session(self._facade, byref(cStartSessionMessage))
        return self

    def start(self):
        while 1:
            sleep(.1)
