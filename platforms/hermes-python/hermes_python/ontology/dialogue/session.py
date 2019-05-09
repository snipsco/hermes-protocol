# -*- coding: utf-8 -*-
from builtins import object
from six.moves import range
from typing import List, Optional

from hermes_python.ffi.ontology.dialogue import CStartSessionMessageAction, CStartSessionMessageNotification, \
    CSessionInitAction, CSessionInitNotification, CEndSessionMessage, CContinueSessionMessage


class SessionInit(object):
    pass


class SessionInitAction(SessionInit):
    def __init__(self, text=None, intent_filter=list(), can_be_enqueued=True, send_intent_not_recognized=False):
        # type:(Optional[str], Optional[List[str]], bool, bool) -> None
        """
        :param text: An optional text to say to the user
        :param intent_filter: An optional list of intent name to restrict the parsing of the user response to
        :param can_be_enqueued: An optional boolean to indicate if the session can be enqueued if it can't be started
        immediately (ie there is another running session on the site). The default value is true
        :param send_intent_not_recognized: An optional boolean to indicate whether the dialogue manager should handle
        non recognized intents by itself or sent them as an `IntentNotRecognizedMessage` for the
        client to handle. This setting applies only to the next conversation turn. The default
        value is false (and the dialogue manager will handle non recognized intents by itself)
        """
        self.text = text
        self.intent_filter = intent_filter
        self.can_be_enqueued = can_be_enqueued
        self.send_intent_not_recognized = send_intent_not_recognized

    def __eq__(self, other):
        return self.__dict__ == other.__dict__

    def into_c_repr(self):
        return CSessionInitAction.build(self.text,
                                        self.intent_filter,
                                        self.can_be_enqueued,
                                        self.send_intent_not_recognized)

    @classmethod
    def from_c_repr(cls, c_repr):
        c_action_session_init = c_repr.value.contents

        intent_filter = []
        intent_filter_length = c_action_session_init.intent_filter.contents.size
        for i in range(intent_filter_length):
            intent_name = c_action_session_init.intent_filter.contents.data[i].decode('utf-8')
            intent_filter.append(intent_name)

        return cls(
            c_action_session_init.text.decode('utf-8') if c_action_session_init.text else None,
            intent_filter,
            True if c_action_session_init.can_be_enqueued > 0 else False,
            True if c_action_session_init.send_intent_not_recognized > 0 else False
        )


class SessionInitNotification(SessionInit):
    def __init__(self, text=""):
        # type:(str) -> None
        """
        The session doesn't expect a response from the user.
        If the session cannot be started, it will enqueued.
        :param text:
        """
        self.text = text

    def into_c_repr(self):
        return CSessionInitNotification.build(self.text)

    @classmethod
    def from_c_repr(cls, c_repr):
        text = c_repr.value.decode('utf-8')
        return cls(text)

    def __eq__(self, other):
        return self.__dict__ == other.__dict__


class StartSessionMessage(object):
    def __init__(self, session_init, custom_data=None, site_id=None):
        # type: (SessionInit, str, str) -> None
        """
        :param session_init: The way this session was created
        :param custom_data: An optional piece of data that will be given back in `IntentMessage`,
        `IntentNotRecognizedMessage`, `SessionQueuedMessage`, `SessionStartedMessage` and `SessionEndedMessage`
        that are related to this session
        :param site_id: The site where the session should be started, a value of `None` will be interpreted as the
        default one
        """
        self.init = session_init
        self.custom_data = custom_data
        self.site_id = site_id

    def into_c_repr(self):
        c_init = self.init.into_c_repr()
        if type(self.init) is SessionInitAction:
            return CStartSessionMessageAction.build(c_init, self.custom_data, self.site_id)
        else:
            return CStartSessionMessageNotification.build(c_init, self.custom_data, self.site_id)

    @classmethod
    def from_c_repr(cls, c_repr):
        if type(c_repr) is CStartSessionMessageNotification:
            session_init = SessionInitNotification.from_c_repr(c_repr.init)
        else:
            session_init = SessionInitAction.from_c_repr(c_repr.init)

        custom_data = c_repr.custom_data.decode('utf-8') if c_repr.custom_data else None
        site_id = c_repr.site_id.decode('utf-8') if c_repr.site_id else None

        return cls(session_init, custom_data, site_id)

    def __eq__(self, other):
        return self.__dict__ == other.__dict__


class SessionStartedMessage(object):
    def __init__(self, session_id, custom_data, site_id, reactivated_from_session_id):
        # type: (str, Optional[str], str, Optional[str]) -> None
        """
        A message that the handler receives from the Dialogue Manager when a session is started.

        :param session_id: Session identifier that was started.
        :param custom_data: Custom data provided in the start session request on.
        :param site_id:  Site where the user interaction is taking place.
        :param reactivated_from_session_id: NOT IMPLEMENTED YET. This feature is coming soon.
        This optional field indicates this session is a reactivation of a previously
        ended session. This is for example provided when the user continues talking to the platform without saying the
        hotword again after a session was ended.
        """
        self.session_id = session_id
        self.custom_data = custom_data
        self.site_id = site_id
        self.reactivated_from_session_id = reactivated_from_session_id

    @classmethod
    def from_c_repr(cls, c_repr):
        session_id = c_repr.session_id.decode('utf-8')
        custom_data = c_repr.custom_data.decode('utf-8') if c_repr.custom_data else None
        site_id = c_repr.site_id.decode('utf-8')
        reactivated_from_session_id = c_repr.reactivated_from_session_id.decode('utf-8') if c_repr.reactivated_from_session_id else None
        return cls(session_id, custom_data, site_id, reactivated_from_session_id)

    def __eq__(self, other):
        return self.__dict__ == other.__dict__


class EndSessionMessage(object):
    def __init__(self, session_id, text=None):
        # type: (str, Optional[str]) -> None
        """
        :param session_id: The id of the session to end
        :param text: An optional text to say to the user before ending the session
        """
        self.session_id = session_id
        self.text = text

    def __eq__(self, other):
        return self.__dict__ == other.__dict__

    @classmethod
    def from_c_repr(cls, c_repr):
        session_id = c_repr.session_id.decode('utf-8')
        text = c_repr.text.decode('utf-8') if c_repr.text else None
        return cls(session_id, text)

    def into_c_repr(self):
        return CEndSessionMessage.build(self.session_id, self.text)


class SessionEndedMessage(object):
    def __init__(self, session_id, custom_data, site_id, termination):
        # type: (str, Optional[str], str, SessionTermination) -> None
        """
        A message that the handler receives from the Dialogue Manager when a session is ended.

        :param session_id: Session identifier that was started.
        :param custom_data: Custom data provided in the start session request on.
        :param site_id: Site where the user interaction is taking place.
        :param termination: Structured description of why the session has been ended.
        """
        self.session_id = session_id
        self.custom_data = custom_data
        self.site_id = site_id
        self.termination = termination

    def __eq__(self, other):
            return self.__dict__ == other.__dict__

    @classmethod
    def from_c_repr(cls, c_repr):
        session_id = c_repr.session_id.decode('utf-8') if c_repr.session_id else None
        custom_data = c_repr.custom_data.decode('utf-8') if c_repr.custom_data else None
        site_id = c_repr.site_id.decode('utf-8') if c_repr.site_id else None
        termination = SessionTermination.from_c_repr(c_repr.termination)
        return cls(session_id, custom_data, site_id, termination)


class SessionQueuedMessage(object):
    def __init__(self, session_id, custom_data, site_id):
        # type: (str, Optional[str], str) -> None
        """
        A message that the handler receives from the Dialogue Manager when a session is queued.

        :param session_id: Session identifier that was started.
        :param custom_data: Custom data provided in the start session request on.
        :param site_id: Site where the user interaction is taking place
        """
        self.session_id = session_id
        self.custom_data = custom_data
        self.site_id = site_id

    @classmethod
    def from_c_repr(cls, c_repr):
        session_id = c_repr.session_id.decode('utf-8') if c_repr.session_id else None
        custom_data = c_repr.custom_data.decode('utf-8') if c_repr.custom_data else None
        site_id = c_repr.site_id.decode('utf-8') if c_repr.site_id else None
        return cls(session_id, custom_data, site_id)

    def __eq__(self, other):
        return self.__dict__ == other.__dict__


class SessionTermination(object):
    def __init__(self, termination_type, data):
        """

        :param termination_type:
        :param data: the reason why the session was ended
        """
        self.termination_type = termination_type
        self.data = data

    @classmethod
    def from_c_repr(cls, c_repr):
        termination_type = c_repr.termination_type
        data = c_repr.data.decode('utf-8') if c_repr.data else None
        return cls(termination_type, data)

    def __eq__(self, other):
        return self.__dict__ == other.__dict__


class ContinueSessionMessage(object):
    def __init__(self, session_id, text, intent_filter, custom_data, send_intent_not_recognized, slot=None):
        # type: (str, str, List[str], Optional[str], bool, Optional[str]) -> None
        """
        :param session_id: Identifier of the dialogue session during which this intent was parsed.
        :param text:
        :param intent_filter: a list of allowed intent names that the dialogue manager will use to filter incoming
        intents. Nullable argument
        :param custom_data: Nullable argument.
        :param send_intent_not_recognized: An optional boolean to indicate whether the dialogue manager should handle
        non recognized intents by itself or sent them as an `IntentNotRecognizedMessage` for the client to handle. This
        setting applies only to the next conversation turn. The default
        value is false (and the dialogue manager will handle non recognized intents by itself)
        :param slot: An optional string, requires `intent_filter` to contain a single value. If set, the dialogue engine
         will not run the the intent classification on the user response and go straight to slot filling, assuming the
         intent is the one passed in the `intent_filter`, and searching the value of the given slot
        """
        self.session_id = session_id
        self.text = text
        self.intent_filter = intent_filter
        self.custom_data = custom_data
        self.slot = slot
        self.send_intent_not_recognized = send_intent_not_recognized

    def __eq__(self, other):
        return self.__dict__ == other.__dict__

    @classmethod
    def from_c_repr(cls, c_repr):
        session_id = c_repr.session_id.decode('utf-8')
        text = c_repr.text.decode('utf-8') if c_repr.text else None

        intent_filter = []
        intent_filter_length = c_repr.intent_filter.contents.size
        for i in range(intent_filter_length):
            intent_name = c_repr.intent_filter.contents.data[i].decode('utf-8')
            intent_filter.append(intent_name)

        custom_data = c_repr.custom_data.decode('utf-8') if c_repr.custom_data else None
        slot = c_repr.slot.decode('utf-8') if c_repr.slot else None
        send_intent_not_recognized = True if c_repr.send_intent_not_recognized > 0 else False

        return cls(session_id, text, intent_filter, custom_data, send_intent_not_recognized, slot)

    def into_c_repr(self):
        return CContinueSessionMessage.build(self.session_id, self.text, self.intent_filter, self.custom_data, self.slot, self.send_intent_not_recognized)


class IntentNotRecognizedMessage(object):
    def __init__(self, site_id, session_id, input, custom_data, confidence_score):
        # type: (str, str, Optional[str], Optional[str], float) -> None
        """
        A message that the handler receives from the Dialogue manager when an intent is not recognized and that the
        session was initialized with the intent_not_recognized flag turned on.

        :param site_id: Site where the user interaction is taking place.
        :param session_id: Session identifier that was started.
        :param input: The user input that has generated this intent. This parameter is nullable
        :param custom_data: Custom data passed by the Dialogue Manager in the current dialogue session.
        This parameter is nullable
        :param confidence_score: Between 0 and 1
        """
        self.site_id = site_id
        self.session_id = session_id
        self.input = input
        self.custom_data = custom_data
        self.confidence_score = confidence_score

    @classmethod
    def from_c_repr(cls, c_repr):
        site_id = c_repr.site_id.decode('utf-8')
        session_id = c_repr.session_id.decode('utf-8')
        input = c_repr.input.decode('utf-8') if c_repr.input else None
        custom_data = c_repr.custom_data.decode('utf-8') if c_repr.custom_data else None
        confidence_score = float(c_repr.confidence_score)

        return cls(site_id, session_id, input, custom_data, confidence_score)

    def __eq__(self, other):
        return self.__dict__ == other.__dict__
