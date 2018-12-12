# -*- coding: utf-8 -*-

from __future__ import absolute_import
from __future__ import unicode_literals
from builtins import object
from ctypes import *
from .ffi.ontology import CProtocolHandler, CDialogueFacade, CContinueSessionMessage, CEndSessionMessage, \
    CStartSessionMessageAction, CStartSessionMessageNotification, CStringArray, CIntentMessage, CSessionStartedMessage, CSessionQueuedMessage, \
    CSessionEndedMessage, CSessionInitNotification, CSessionInitAction, CActionSessionInit
from .ffi.utils import *
import threading
from time import sleep



class Hermes(object):
    def __init__(self, mqtt_server_address, rust_logs_enabled=False):
        self.mqtt_server_address = mqtt_server_address
        self.rust_logs_enabled = rust_logs_enabled

        self._protocol_handler = POINTER(CProtocolHandler)()
        self._facade = POINTER(CDialogueFacade)()

        # References to callbacks called from C
        self._c_callback_subscribe_intent = []
        self._c_callback_subscribe_intents = None
        self._c_callback_subscribe_session_started = None
        self._c_callback_subscribe_session_queued = None
        self._c_callback_subscribe_session_ended = None

        self._thread = None
        self._thread_terminate = False

    def __enter__(self):
        hermes_protocol_handler_new_mqtt(byref(self._protocol_handler), self.mqtt_server_address)

        hermes_protocol_handler_dialogue_facade(self._protocol_handler,
                                                    byref(self._facade))

        if self.rust_logs_enabled:
            lib.hermes_enable_debug_logs()

        return self

    def __exit__(self, exception_type, exception_val, trace):
        if self._thread is not None:
            self.loop_stop()

        hermes_drop_dialogue_facade(self._facade)

    def _wraps(self, user_callback, callback_argtype, callback_restype, argtype):
        def params_converter(func):
            def called_with_good_params(*args, **kwargs):
                parsed_args = (argtype.from_c_repr(arg.contents) for arg in (args))
                return func(self, *parsed_args)

            return called_with_good_params

        return CFUNCTYPE(callback_restype, POINTER(callback_argtype))(params_converter(user_callback))

    def subscribe_intent(self, intent_name, user_callback_subscribe_intent):
        """
        Registers a callback to be triggered when the intent intent_name is recognized.

        The callback will be called with the following parameters :
            - hermes : the current instance of the Hermes object
            - intentMessage : A python representation of the intent parsed by the NLU engine.

        :param intent_name: the name of the intent to subscribe to.
        :param user_callback_subscribe_intent: the callback that will be executed when intent_name is recognized.
        :return: the current instance of Hermes to allow chaining.
        """
        self._c_callback_subscribe_intent.append(self._wraps(user_callback_subscribe_intent, CIntentMessage, c_void_p,
                                                        IntentMessage))

        number_of_callbacks = len(self._c_callback_subscribe_intent)
        hermes_dialogue_subscribe_intent(self._facade, c_char_p(intent_name.encode('utf-8')), self._c_callback_subscribe_intent[number_of_callbacks - 1]) # We retrieve the last callback we
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
        self._c_callback_subscribe_intents = self._wraps(user_callback_subscribe_intents, CIntentMessage, c_void_p,
                                                         IntentMessage)
        hermes_dialogue_subscribe_intents(self._facade, self._c_callback_subscribe_intents)
        return self

    def subscribe_session_started(self, user_callback_subscribe_session_started):
        """
        Register a callback when the Dialogue Manager starts a new session.

        The callback will be called with the following parameters :
            - hermes : the current instance of the Hermes object
            - sessionStartedMessage : message that the handler receives from the Dialogue Manager when a session is started.

        :param user_callback_subscribe_session_started: the callback to be executed when a new dialogue session is started.
        :return: the current instance of Hermes to allow chaining.
        """
        self._c_callback_subscribe_session_started = self._wraps(user_callback_subscribe_session_started,
                                                                 CSessionStartedMessage, c_void_p,
                                                                 SessionStartedMessage)
        hermes_dialogue_subscribe_session_started(self._facade, self._c_callback_subscribe_session_started)
        return self

    def subscribe_session_queued(self, user_callback_subscribe_session_queued):
        """
        Register a callback when the Dialogue Manager queues the current session.

        The callback will be called with the following parameters :
            - hermes : the current instance of the Hermes object
            - sessionQueuedMessage : message that the handler receives from the Dialogue Manager when a session is queued.

        :param user_callback_subscribe_session_queued: the callback to be executed when a new dialogue session is queued.
        :return: the current instance of Hermes to allow chaining.
        """
        self._c_callback_subscribe_session_queued = self._wraps(user_callback_subscribe_session_queued,
                                                                CSessionQueuedMessage, c_void_p, SessionQueuedMessage)
        hermes_dialogue_subscribe_session_queued(self._facade, self._c_callback_subscribe_session_queued)
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
        self._c_callback_subscribe_session_ended = self._wraps(user_callback_subscribe_session_ended,
                                                               CSessionEndedMessage, c_void_p, SessionEndedMessage)
        hermes_dialogue_subscribe_session_ended(self._facade, self._c_callback_subscribe_session_ended)
        return self

    def publish_continue_session(self, session_id, text, intent_filter):
        """
        Publishes a ContinueSession message to the Dialogue Manage to continue a dialogue session.

        :param session_id: The identifier of the session to be continued.
        :param text: the text the TTS should say to start this additional request of the session.
        :param intent_filter: A list of intents names to restrict the NLU resolution on the answer of this query.
        :return: the current instance of Hermes to allow chaining.
        """
        cContinueSessionMessage = CContinueSessionMessage.build(session_id, text, intent_filter)
        hermes_dialogue_publish_continue_session(self._facade, byref(cContinueSessionMessage))
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
        cEndSessionMessage = CEndSessionMessage.build(session_id, text)
        hermes_dialogue_publish_end_session(self._facade, byref(cEndSessionMessage))
        return self

    def publish_start_session_notification(self, site_id, session_init_value, custom_data):
        """
        Publishes a StartSession message to the Dialogue Manager to initiate a new session.

        This message can be sent by the handler code to programmatically initiate a new session.

        :param site_id: Site where the user started the interaction.
        :param session_init_value: Text the TTS should say.
        :param custom_data: Additional information that can be provided by the handler. Each message related to the new session - sent by the Dialogue Manager - will contain this data.
        :return: the current instance of Hermes to allow chaining.
        """
        init = CSessionInitNotification.build(session_init_value)
        cStartSessionMessage = CStartSessionMessageNotification.build(init, custom_data, site_id)
        hermes_dialogue_publish_start_session(self._facade, byref(cStartSessionMessage))
        return self

    def publish_start_session_action(self, site_id, session_init_text, session_init_intent_filter, session_init_can_be_enqueued, custom_data):
        """
        Publishes a StartSession message to the Dialogue Manager to initiate a new session.

        This message can be sent by the handler code to programmatically initiate a new session.
        The Dialogue Manager will start the session by asking the TTS to say the text (if any)
        and wait for the answer of the end user.


        :param site_id: Site where the user started the interaction.
        :param session_init_text: Text that the TTS should say at the beginning of the session.
        :param session_init_intent_filter: A list of intents names to restrict the NLU resolution on the first query.
        :param session_init_can_be_enqueued: if true, the session will start when there is no pending one on this siteId, if false, the session is just dropped if there is running one.
        :param custom_data: Additional information that can be provided by the handler. Each message related to the new session - sent by the Dialogue Manager - will contain this data.
        :return: the current instance of Hermes to allow chaining.
        """
        init = CSessionInitAction.build(session_init_text, session_init_intent_filter, session_init_can_be_enqueued)
        cStartSessionMessage = CStartSessionMessageAction.build(init, custom_data, site_id)
        hermes_dialogue_publish_start_session(self._facade, byref(cStartSessionMessage))
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
            if (self._thread_terminate):
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

    def loop_stop(self, force=False):
        if self._thread is None:
            return

        self._thread_terminate = True
        if threading.currentThread() != self._thread:
            self._thread.join()
            self._thread = None
