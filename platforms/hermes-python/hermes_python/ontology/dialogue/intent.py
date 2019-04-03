# -*- coding: utf-8 -*-
from .slot import SlotMap


class IntentMessage(object):
    def __init__(self, session_id, custom_data, site_id, input, intent, slots):
        # type: (str, str, str, str, IntentClassifierResult, SlotMap) -> None
        """
        A python representation of the intent parsed by the NLU engine.

        :param session_id: Identifier of the dialogue session during which this intent was parsed.
        :param custom_data: Custom data passed by the Dialogue Manager in the current dialogue session.
        :param site_id: Site where the user interaction took place.
        :param input: The user input that has generated this intent.
        :param intent: Structured description of the intent classification.
        :param slots: Structured description of the detected slots for this intent if any.
        """
        self.session_id = session_id
        self.custom_data = custom_data
        self.site_id = site_id
        self.input = input
        self.intent = intent  # type : IntentClassifierResult
        self.slots = slots if slots else SlotMap({})  # type: SlotMap

    @classmethod
    def from_c_repr(cls, c_repr):
        session_id = c_repr.session_id.decode('utf-8')
        custom_data = c_repr.custom_data.decode('utf-8') if c_repr.custom_data else None
        site_id = c_repr.site_id.decode('utf-8')
        input = c_repr.input.decode('utf-8')
        intent = IntentClassifierResult.from_c_repr(c_repr.intent.contents)
        if c_repr.slots:  # Slots is now nullable.
            slots = SlotMap.from_c_repr(c_repr.slots.contents)
        else:
            slots = SlotMap({})

        return cls(session_id, custom_data, site_id, input, intent, slots)


class IntentClassifierResult(object):
    def __init__(self, intent_name, confidence_score):
        # type: (str, float) -> None
        """
        Structured description of the intent classification.

        :param intent_name: name of the intent.
        :param confidence_score: confidence_score that the parsed sentence is the `intent_name` intent.
        """
        self.intent_name = intent_name
        self.confidence_score = confidence_score

    @classmethod
    def from_c_repr(cls, c_repr):
        intent_name = c_repr.intent_name.decode('utf-8')
        confidence_score = c_repr.confidence_score
        return cls(intent_name, confidence_score)
