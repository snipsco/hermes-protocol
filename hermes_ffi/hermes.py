# -*- coding: utf-8 -*-

from ctypes import *
from ffi.ontology import CProtocolHandler, CDialogueFacade, CContinueSessionMessage, CEndSessionMessage, CStartSessionMessage, CStringArray
from ffi.utils import *
from time import sleep


class Hermes(object):
    def __init__(self, mqtt_server_address):
        self._protocol_handler = POINTER(CProtocolHandler)()  # TODO: Destroy protocol handler
        self._facade = POINTER(CDialogueFacade)()

        lib.hermes_protocol_handler_new_mqtt(byref(self._protocol_handler), mqtt_server_address)

        lib.hermes_protocol_handler_dialogue_facade(self._protocol_handler,
                                                    byref(self._facade))

        # References to user-defined callbacks
        self._subscribe_intent_callback = None
        self._subscribe_intents_callback = None
        self._subscribe_session_started_callback = None
        self._subscribe_session_queued_callback = None
        self._subscribe_session_ended_callback = None

    def subscribe_intent(self, intent_name, user_callback_subscribe_intent):
        self._subscribe_intent_callback = user_callback_subscribe_intent
        hermes_dialogue_subscribe_intent(self._facade, c_char_p(intent_name), self._subscribe_intent_callback)
        return self

    def subscribe_intents(self, user_callback_subscribe_intents):
        self._subscribe_intents_callback = user_callback_subscribe_intents
        hermes_dialogue_subscribe_intents(self._facade, self._subscribe_intents_callback)
        return self

    def subscribe_session_started(self, user_callback_subscribe_session_started):
        self._subscribe_session_started_callback = user_callback_subscribe_session_started
        hermes_dialogue_subscribe_session_started(self._facade, self._subscribe_session_started_callback)
        return self

    def subscribe_session_queued(self, user_callback_subscribe_session_queued):
        self._subscribe_session_queued_callback = user_callback_subscribe_session_queued
        hermes_dialogue_subscribe_session_started(self._facade, self._subscribe_session_queued_callback)
        return self

    def subscribe_session_ended(self, user_callback_subscribe_session_ended):
        self._subscribe_session_ended_callback = user_callback_subscribe_session_ended
        hermes_dialogue_subscribe_session_started(self._facade, self._subscribe_session_ended_callback)
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
