# -*- coding: utf-8 -*-

from __future__ import absolute_import
from __future__ import unicode_literals
from builtins import object
from .ontology import MqttOptions
from .ontology.dialogue import ContinueSessionMessage, EndSessionMessage, StartSessionMessage, SessionInitNotification, \
    SessionInitAction
from .api.ffi import FFI

import threading
from time import sleep


class Hermes(object):
    def __init__(self,
                 broker_address=None,
                 rust_logs_enabled=False,
                 mqtt_options=MqttOptions(),
                 use_json_api=False):
        """
        :param broker_address: Address of the MQTT broker in the form 'ip:port'
        :param rust_logs_enabled: Enables or Disables stdout logs *(default false)*
        :param mqtt_options: Options to connect to the mqtt broker.
        :param use_json_api: If set to False, hermes-python will use the legacy format for published/subscribed
        messages. This is an upcoming feature.
        """

        self.rust_logs_enabled = rust_logs_enabled
        self.use_json_api = use_json_api

        self.mqtt_options = mqtt_options
        if broker_address:  # This test is kept for API compatibility reasons.
            self.mqtt_options.broker_address = broker_address

        self.ffi = FFI(use_json_api=use_json_api)

        self._thread = None
        self._thread_terminate = False

    def __enter__(self):
        return self.connect()

    def __exit__(self, exception_type, exception_val, trace):
        if not exception_type:
            return self.disconnect()
        return False

    def connect(self):
        self.ffi.establish_connection(self.mqtt_options)
        return self

    def disconnect(self):
        if self._thread is not None:
            self.loop_stop()

        self.ffi.release_connection()
        return self

    def subscribe_intent(self, intent_name, user_callback_subscribe_intent):
        """
        Registers a callback to be triggered when the intent intent_name is recognized.

        The callback will be called with the following parameters :
            - hermes : the current instance of the Hermes object
            - intentMessage :
                - A python representation of the intent parsed by the NLU engine (for json_repr set to False)
                - A json representation of the intent parsed by the NLU engine. (for json_repr set to True)

        :param intent_name: the name of the intent to subscribe to.
        :param user_callback_subscribe_intent: the callback that will be executed when intent_name is recognized.
        :return: the current instance of Hermes to allow chaining.
        """

        self.ffi.dialogue.register_subscribe_intent_handler(intent_name, user_callback_subscribe_intent, self)
        return self

    def subscribe_intents(self, user_callback_subscribe_intents):
        """
        Register a callback to be triggered everytime an intent is recognized.

        The callback will be called with the following parameters :
            - hermes : the current instance of the Hermes object
            - intentMessage : A python representation of the intent parsed by the NLU engine.


        :param user_callback_subscribe_intents: The callback to be executed when any intent is parsed by the platform.
        :return: the current instance of Hermes to allow chaining.
        """
        self.ffi.dialogue.register_subscribe_intents_handler(user_callback_subscribe_intents, self)
        return self

    def subscribe_session_started(self, user_callback_subscribe_session_started):
        """
        Register a callback when the Dialogue Manager starts a new session.

        The callback will be called with the following parameters :
            - hermes : the current instance of the Hermes object
            - sessionStartedMessage : message that the handler receives from the Dialogue Manager when a session is
            started.

        :param user_callback_subscribe_session_started: the callback to be executed when a new dialogue session is
        started.
        :return: the current instance of Hermes to allow chaining.
        """
        self.ffi.dialogue.register_session_started_handler(user_callback_subscribe_session_started, self)
        return self

    def subscribe_session_queued(self, user_callback_subscribe_session_queued):
        """
        Register a callback when the Dialogue Manager queues the current session.

        The callback will be called with the following parameters :
            - hermes : the current instance of the Hermes object
            - sessionQueuedMessage : message that the handler receives from the Dialogue Manager when a session is
            queued.

        :param user_callback_subscribe_session_queued: the callback to be executed when a new dialogue session is queued.
        :return: the current instance of Hermes to allow chaining.
        """
        self.ffi.dialogue.register_session_queued_handler(user_callback_subscribe_session_queued, self)
        return self

    def subscribe_session_ended(self, user_callback_subscribe_session_ended):
        """
        Register a callback when the Dialogue Manager ends a session.

        The callback will be called with the following parameters :
            - hermes : the current instance of the Hermes object
            - sessionEndedMessage : message that the handler receives from the Dialogue Manager when a session is ended.

        :param user_callback_subscribe_session_ended: the callback to be executed when a new dialogue session is ended.
        :return: the current instance of Hermes to allow chaining.
        """
        self.ffi.dialogue.register_session_ended_handler(user_callback_subscribe_session_ended, self)
        return self

    def subscribe_intent_not_recognized(self, user_callback_subscribe_intent_not_recognized):
        """
        Register a callback when the Dialogue Manager doesn't recognize an intent.

        Note that you need to have initialized a session, (or call publish_continue_session method on an existing
        session) with the intent_not_recognized field set to true.
        Otherwise, the DialogueManager will take care itself of not recognized intent and the callback you registered
        will never be called.

        The callback will be called with the following parameters :
            - hermes: the current instance of the Hermes object
            - intentNotRecognizedMessage : message that the handler receives from the Dialogue Manager when an intent
            is not recognized.

        :param user_callback_subscribe_intent_not_recognized: the callback executed when an intent is not recognized.
        :return: the current instance of Hermes to allow chaining.
        """
        self.ffi.dialogue.register_intent_not_recognized_handler(user_callback_subscribe_intent_not_recognized, self)
        return self

    def publish_continue_session(self, session_id, text, intent_filter, custom_data=None,
                                 send_intent_not_recognized=False):
        """
        Publishes a ContinueSession message to the Dialogue Manage to continue a dialogue session.

        :param session_id: The identifier of the session to be continued.
        :param text: the text the TTS should say to start this additional request of the session.
        :param intent_filter: A list of intents names to restrict the NLU resolution on the answer of this query.
        :param send_intent_not_recognized: An optional boolean to indicate whether the dialogue manager should handle
        non recognized intents by itself or sent them as an `IntentNotRecognizedMessage` for the client to handle.
        This setting applies only to the next conversation turn. The default value is false
        (and the dialogue manager will handle non recognized intents by itself)
        :return: the current instance of Hermes to allow chaining.
        """
        continue_session_msg = ContinueSessionMessage(session_id, text, intent_filter, custom_data,
                                                      send_intent_not_recognized)
        self.ffi.dialogue.publish_continue_session(continue_session_msg)
        return self

    def publish_end_session(self, session_id, text):
        """
        Publishes a EndSession message to the Dialogue Manager to end a dialogue session.

        When the handler received the intents it needs, or when the handler wants to explicitly end a running session,
        it should send this endSession message with the given session_id.

        :param session_id: Session identifier to end.
        :param text: The text the TTS should say to end the session.
        :return: the current instance of Hermes to allow chaining.
        """
        end_session_message = EndSessionMessage(session_id, text)
        self.ffi.dialogue.publish_end_session(end_session_message)
        return self

    def publish_start_session_notification(self, site_id, session_init_value, custom_data, text=""):
        """
        Publishes a StartSession message to the Dialogue Manager to initiate a new session.

        This message can be sent by the handler code to programmatically initiate a new session.
        Use this type when you only want to inform the user of something without expecting a response.

        :param site_id: Site where the user started the interaction.
        :param session_init_value: Text the TTS should say.
        :param custom_data: Additional information that can be provided by the handler. Each message related to
        the new session - sent by the Dialogue Manager - will contain this data.
        :return: the current instance of Hermes to allow chaining.
        """
        session_init_message = SessionInitNotification(text)
        start_session_notification_message = StartSessionMessage(session_init_message, custom_data, site_id)

        self.ffi.dialogue.publish_start_session(start_session_notification_message)
        return self

    def publish_start_session_action(self, site_id, session_init_text, session_init_intent_filter,
                                     session_init_can_be_enqueued, session_init_send_intent_not_recognized,
                                     custom_data):
        """
        Publishes a StartSession message to the Dialogue Manager to initiate a new session.

        This message can be sent by the handler code to programmatically initiate a new session.
        Use this type when you need the end user to respond.
        The Dialogue Manager will start the session by asking the TTS to say the text (if any)
        and wait for the answer of the end user.


        :param site_id: Site where the user started the interaction.
        :param session_init_text: Text that the TTS should say at the beginning of the session.
        :param session_init_intent_filter: A list of intents names to restrict the NLU resolution on the first query.
        :param session_init_can_be_enqueued: if true, the session will start when there is no pending one
        on this siteId, if false, the session is just dropped if there is running one.
        :param custom_data: Additional information that can be provided by the handler. Each message related
        to the new session - sent by the Dialogue Manager - will contain this data.
        :return: the current instance of Hermes to allow chaining.
        """
        session_init_message = SessionInitAction(
            session_init_text,
            session_init_intent_filter,
            session_init_can_be_enqueued,
            session_init_send_intent_not_recognized)
        start_session_action_message = StartSessionMessage(session_init_message, custom_data, site_id)
        self.ffi.dialogue.publish_start_session(start_session_action_message)
        return self

    def start(self):
        """
        DEPRECATED. This method is just kept for compatibility with previous versions of the library.
        :return:
        """
        self.loop_forever()

    def loop_forever(self):
        """
        This is a convenience method to loop forever in a blocking fashion.
        :return: None
        """
        while 1:
            if self._thread_terminate:
                break
            sleep(.1)

    def loop_start(self):
        """
        to set a thread running to call a infinite loop for you.
        :return: None
        """
        self._thread_terminate = False
        self._thread = threading.Thread(target=self.loop_forever)
        self._thread.daemon = True
        self._thread.start()

    def loop_stop(self):
        if self._thread is None:
            return

        self._thread_terminate = True
        if threading.currentThread() != self._thread:
            self._thread.join()
            self._thread = None
