# -*- coding: utf-8 -*-

from __future__ import absolute_import
from __future__ import unicode_literals

from builtins import object
from typing import Optional, Callable, List, Text

from .ontology import MqttOptions

from .ontology.dialogue import ContinueSessionMessage, EndSessionMessage, StartSessionMessage, SessionInitNotification, \
    SessionInitAction, SessionStartedMessage, IntentMessage, SessionQueuedMessage, SessionEndedMessage, \
    IntentNotRecognizedMessage
from .ontology.feedback import SiteMessage
from .ontology.tts import RegisterSoundMessage
from .ontology.injection import InjectionStatusMessage, InjectionRequestMessage
from .api.ffi import FFI

import threading
from time import sleep


class Hermes(object):
    def __init__(self,
                 broker_address=None,
                 rust_logs_enabled=False,
                 mqtt_options=MqttOptions(),
                 use_json_api=False):
        # type: (Optional[Text], bool, MqttOptions, bool) -> None
        """
        :param broker_address: Address of the MQTT broker in the form 'ip:port'
        :param rust_logs_enabled: Enables or Disables stdout logs *(default false)*
        :param mqtt_options: Options to connect to the mqtt broker.
        :param use_json_api: If set to False, hermes-python will use the legacy format for published/subscribed
        messages. This is an upcoming feature.
        """

        self.rust_logs_enabled = rust_logs_enabled
        self.use_json_api = use_json_api

        self.mqtt_options = mqtt_options  # type: MqttOptions
        if broker_address:  # This test is kept for API compatibility reasons.
            self.mqtt_options.broker_address = broker_address

        self.ffi = FFI(use_json_api=use_json_api, rust_logs_enabled=rust_logs_enabled)  # type: FFI

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
        # type: (Text, Callable[[Hermes, IntentMessage], None]) -> Hermes
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
        # type: (Callable[[Hermes, IntentMessage], None]) -> Hermes
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
        # type: (Callable[[Hermes, SessionStartedMessage], None]) -> Hermes
        """
        Register a callback when the Dialogue Manager starts a new session.

        The callback will be called with the following parameters :
            - hermes : the current instance of the Hermes object
            - sessionStartedMessage : message that the handler receives from the Dialogue Manager when a session is started.

        :param user_callback_subscribe_session_started: the callback to be executed when a new dialogue session is started.

        :return: the current instance of Hermes to allow chaining.
        """
        self.ffi.dialogue.register_session_started_handler(user_callback_subscribe_session_started, self)
        return self

    def subscribe_session_queued(self, user_callback_subscribe_session_queued):
        # type: (Callable[[Hermes, SessionQueuedMessage], None]) -> Hermes
        """
        Register a callback when the Dialogue Manager queues the current session.

        The callback will be called with the following parameters :
            - hermes : the current instance of the Hermes object
            - sessionQueuedMessage : message that the handler receives from the Dialogue Manager when a session is queued.

        :param user_callback_subscribe_session_queued: the callback to be executed when a new dialogue session is queued.
        :return: the current instance of Hermes to allow chaining.
        """
        self.ffi.dialogue.register_session_queued_handler(user_callback_subscribe_session_queued, self)
        return self

    def subscribe_session_ended(self, user_callback_subscribe_session_ended):
        # type: (Callable[[Hermes, SessionEndedMessage], None]) -> Hermes
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
        # type: (Callable[[Hermes, IntentNotRecognizedMessage], None]) -> Hermes
        """
        Register a callback when the Dialogue Manager doesn't recognize an intent.

        Note that you need to have initialized a session, (or call publish_continue_session method on an existing
        session) with the intent_not_recognized field set to true.
        Otherwise, the DialogueManager will take care itself of not recognized intent and the callback you registered
        will never be called.

        The callback will be called with the following parameters :
            - hermes: the current instance of the Hermes object
            - intentNotRecognizedMessage : message that the handler receives from the Dialogue Manager when an intent is not recognized.

        :param user_callback_subscribe_intent_not_recognized: the callback executed when an intent is not recognized.

        :return: the current instance of Hermes to allow chaining.


        """
        self.ffi.dialogue.register_intent_not_recognized_handler(user_callback_subscribe_intent_not_recognized, self)
        return self

    def publish_continue_session(self, session_id, text, intent_filter, custom_data=None,
                                 send_intent_not_recognized=False, slot_to_fill=None):
        # type: (Text, Optional[Text], List[Text], Optional[Text], bool, Optional[Text]) -> Hermes

        """
        Publishes a ContinueSession message to the Dialogue Manage to continue a dialogue session.

        :param session_id: The identifier of the session to be continued.
        :param text: the text the TTS should say to start this additional request of the session.
        :param intent_filter: A list of intents names to restrict the NLU resolution on the answer of this query. Can be an empty list.
        :param send_intent_not_recognized: An optional boolean to indicate whether the dialogue manager should handle non recognized intents by itself or sent them as an `IntentNotRecognizedMessage` for the client to handle. This setting applies only to the next conversation turn. The default value is false (and the dialogue manager will handle non recognized intents by itself)
        :param slot_to_fill: is an Optional string. It requires `intent_filter` to contain a single value. If set, the dialogue engine will not run the the intent classification on the user response and will go straight to slot filling, assuming the intent is the one passed in the `intent_filter`, and searching the value of the given slot.

        :return: the current instance of Hermes to allow chaining.
        """
        continue_session_msg = ContinueSessionMessage(session_id, text, intent_filter, custom_data,
                                                      send_intent_not_recognized, slot_to_fill)
        self.ffi.dialogue.publish_continue_session(continue_session_msg)
        return self

    def publish_end_session(self, session_id, text):
        # type: (Optional[Text], Optional[Text]) -> Hermes
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

    def publish_start_session_notification(self, site_id, session_initiation_text, custom_data, text=u""):
        # type: (Optional[Text], Text, Optional[Text], Optional[Text]) -> Hermes
        """
        Publishes a StartSession message to the Dialogue Manager to initiate a new session.

        This message can be sent by the handler code to programmatically initiate a new session.
        Use this type when you only want to inform the user of something without expecting a response.

        :param site_id: Site where the user started the interaction.
        :param session_initiation_text: Text the TTS should say.
        :param custom_data: Additional information that can be provided by the handler. Each message related to the new session - sent by the Dialogue Manager - will contain this data.
        :param text: Text the TTS should say. This parameter was introduced by mistake and shouldn't be used.

        :return: the current instance of Hermes to allow chaining.
        """

        session_init_message = SessionInitNotification(text or session_initiation_text)

        start_session_notification_message = StartSessionMessage(session_init_message, custom_data, site_id)

        self.ffi.dialogue.publish_start_session(start_session_notification_message)
        return self

    def publish_start_session_action(self, site_id, session_init_text, session_init_intent_filter,
                                     session_init_can_be_enqueued, session_init_send_intent_not_recognized,
                                     custom_data):
        # type: (Optional[Text], Optional[Text], List[Text], bool, bool, Optional[Text]) -> Hermes
        """
        Publishes a StartSession message to the Dialogue Manager to initiate a new session.

        This message can be sent by the handler code to programmatically initiate a new session.
        Use this type when you need the end user to respond.
        The Dialogue Manager will start the session by asking the TTS to say the text (if any)
        and wait for the answer of the end user.


        :param site_id: Site where the user started the interaction.
        :param session_init_text: Text that the TTS should say at the beginning of the session.
        :param session_init_intent_filter: A list of intents names to restrict the NLU resolution on the first query.
        :param session_init_can_be_enqueued: if true, the session will start when there is no pending one on this siteId, if false, the session is just dropped if there is running one.
        :param custom_data: Additional information that can be provided by the handler. Each message related to the new session - sent by the Dialogue Manager - will contain this data.

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

    def configure_dialogue(self, configure_message):
        # type: (DialogueConfiguration) -> Hermes
        """
        Publish configuration message for different aspects of the Dialogue system.

        :param configure_message: DialogueConfiguration the configuration that will be sent to the dialogue Manager

        :return: the current instance of Hermes to allow chaining.
        """
        configure_dialogue_messages = configure_message.build()

        for conf in configure_dialogue_messages:
            self.ffi.dialogue.publish_configure(conf)

        return self

    def enable_sound_feedback(self, site_message):
        # type: (SiteMessage) -> Hermes
        """
        Toggles on the sound feedback for the snips platform for a given site_id and optionally for a session_id defined in
        the site_message.

        :param site_message: SiteMessage where the sound feedback will be turned on.

        :return: the current instance of Hermes to allow chaining.
        """
        self.ffi.sound_feedback.publish_toggle_on(site_message)
        return self

    def disable_sound_feedback(self, site_message):
        # type: (SiteMessage) -> Hermes
        """
        Toggles off the sound feedback for the snips platform for a given site_id and optionally for a session_id defined in
        the site_message.

        :param site_message: SiteMessage where the sound feedback will be turned off.

        :return: the current instance of Hermes to allow chaining.
        """
        self.ffi.sound_feedback.publish_toggle_off(site_message)
        return self

    def register_sound(self, sound):
        # type: (RegisterSoundMessage) -> Hermes
        """
        Register a sound that can later be played by the TTS.

        :param sound: a sound to be played by the TTS.
        :return: the current instance of Hermes to allow chaining.
        """
        self.ffi.tts.publish_register_sound(sound)
        return self

    # This method is disabled as long as the injection API is stabilized
    # def subscribe_injection_status(self, injection_status_callback):
    #     # type: (Callable[[Hermes, InjectionStatusMessage], None]) -> Hermes
    #     """
    #     Registers a callback that will be triggered after a injection status message was requested.
    #
    #     Note that you have to request the status of the injection via the `request_injection_status` method of hermes.
    #
    #     The callback will be called with the following parameters :
    #         - hermes: the current instance of the Hermes object
    #         - injection_status : An object that gives you the date of the latest succesful injection.
    #
    #     :param injection_status_callback: the callback executed following a injection status request.
    #     :return: the current instance of Hermes to allow chaining.
    #     """
    #     self.ffi.injection.register_subscribe_injection_status(
    #         injection_status_callback,
    #         self
    #     )
    #     return self

    # This method is disabled as long as the injection API is stabilized
    # def request_injection_status(self):
    #     # type: () -> Hermes
    #     """
    #     Publishes a injection status request to the platform.
    #
    #     Note that this function is asynchronous, and that you retrieve the results of this call if you register a
    #     callback with the `subscribe_injection_status` method.
    #
    #
    #     :return: the current instance of Hermes to allow chaining.
    #     """
    #     self.ffi.injection.publish_injection_status_request()
    #     return self

    def subscribe_injection_complete(self, user_callback_injection_complete):
        # type: (Callable[[Hermes, InjectionCompleteMessage], None]) -> Hermes
        """
        Registers a callback to be triggered when an injection process is completed.

        The callback will be called with the following parameters :
            - hermes : the current instance of the Hermes object
            - injectionCompleteMessage :
                - A python representation of the message of completion for an injection operation (for json_repr set to False)
                - A json representation of the the message of completion for an injection operation (for json_repr set to True)

        :param user_callback_injection_complete: the callback that will be executed when a launched injection operation is completed.
        :type user_callback_injection_complete: Callable[[Hermes, InjectionCompleteMessage]
        :return: the current instance of Hermes to allow chaining.
        """
        self.ffi.injection.register_subscribe_injection_complete(user_callback_injection_complete, self)
        return self

    def request_injection(self, injection_request):
        # type: (InjectionRequestMessage) -> Hermes
        """
        Publishes an injection request to the platform.

        Note that this function is asynchronous. You can check the status of the injection by registering a injection
        complete callback with the `subscribe_injection_complete` method of `hermes`.

        :param injection_request: An object that contains the different injection requests operations.
        :return: the current instance of Hermes to allow chaining.
        """
        self.ffi.injection.publish_injection_request(injection_request)
        return self

    def subscribe_injection_reset_complete(self, user_callback_injection_reset_complete):
        # type: (Callable[[Hermes, InjectionResetCompleteMessage], None]) -> Hermes
        """
        Registers a callback to be triggered when an injection reset process is completed.

        The callback will be called with the following parameters :
            - hermes : the current instance of the Hermes object
            - injectionResetCompleteMessage:
                - A python representation of the message of completion for an injection reset operation (for json_repr set to False)
                - A json representation of the the message of completion for an injection reset operation (for json_repr set to True)

        :param user_callback_injection_reset_complete:
        :type user_callback_injection_reset_complete: Callable[[Hermes, InjectionResetCompleteMessage]
        :return:
        """
        self.ffi.injection.register_subscribe_injection_reset_complete(user_callback_injection_reset_complete, self)
        return self

    def request_injection_reset(self, injection_reset_request):
        # type: (InjectionResetRequestMessage) -> Hermes
        """
        Publishes an injection reset request to the platform.

        Note that this function is asynchronous. You can check the status of the injection reset by registering a injection
        reset complete callback with the `subscribe_injection_reset_complete` method of `hermes`.


        :param injection_reset_request: An request object to reset the injection to its factory settings;
        :return: the current instance of Hermes to allow chaining.
        """
        self.ffi.injection.publish_injection_reset_request(injection_reset_request)
        return self

    def start(self):
        """

        DEPRECATED. This method is just kept for compatibility with previous versions of the library.
        """
        self.loop_forever()

    def loop_forever(self):
        """

        This is a convenience method to loop forever in a blocking fashion.
        """
        while 1:
            if self._thread_terminate:
                break
            sleep(.1)

    def loop_start(self):
        """

        to set a thread running to call a infinite loop for you.
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
