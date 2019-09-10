# -*- coding: utf-8 -*-
from typing import List, Optional, Text
from ..nlu import SlotMap, NluIntentAlternative
from ..asr import AsrToken


class IntentMessage(object):
    def __init__(self, session_id, custom_data, site_id, input, intent, slots, alternatives, asr_tokens, asr_confidence):
        # type: (Text, Text, Text, Text, IntentClassifierResult, SlotMap, List[NluIntentAlternative], List[List[AsrToken]], float) -> None
        """
        A python representation of the intent parsed by the NLU engine.

        :param session_id: Identifier of the dialogue session during which this intent was parsed.
        :type session_id: Text
        :param custom_data: Custom data passed by the Dialogue Manager in the current dialogue session.
        :type custom_data: Text
        :param site_id: Site where the user interaction took place.
        :type site_id: Text
        :param input: The user input that has generated this intent.
        :type input: Text
        :param intent: Structured description of the intent classification.
        :type intent: IntentClassifierResult
        :param slots: Structured description of the detected slots for this intent if any.
        :type slots: SlotMap
        :param alternatives: A list of alternatives intent resolutions
        :type alternatives: List[NluIntentAlternative]
        :param asr_tokens: The tokens detected by the ASR. The first list represents the different ASR invocations
        :type asr_tokens: List[List[AsrToken]]
        :param asr_confidence: Confidence of the asr capture
        :type asr_confidence: float

        """
        self.session_id = session_id
        self.custom_data = custom_data
        self.site_id = site_id
        self.input = input
        self.intent = intent  # type : IntentClassifierResult
        self.slots = slots if slots else SlotMap({})  # type: SlotMap
        self.alternatives = alternatives  # type: List[NluIntentAlternative]
        self.asr_tokens = asr_tokens  # type: List[List[AsrToken]]
        self.asr_confidence = asr_confidence


    @classmethod
    def from_c_repr(cls, c_repr):
        session_id = c_repr.session_id.decode('utf-8')
        custom_data = c_repr.custom_data.decode('utf-8') if c_repr.custom_data else None
        site_id = c_repr.site_id.decode('utf-8')
        input = c_repr.input.decode('utf-8')
        asr_confidence = float(c_repr.asr_confidence)
        intent = IntentClassifierResult.from_c_repr(c_repr.intent.contents)
        if c_repr.slots:  # Slots is now nullable.
            slots = SlotMap.from_c_repr(c_repr.slots.contents)
        else:
            slots = SlotMap({})

        alternatives = list()

        if c_repr.alternatives:
            intent_alternatives_length = c_repr.alternatives.contents.count
            c_intent_alternatives_array_repr = c_repr.alternatives.contents.entries

            for i in range(intent_alternatives_length):
                intent_alternative = NluIntentAlternative.from_c_repr(c_intent_alternatives_array_repr[i].contents)
                alternatives.append(intent_alternative)

        asr_token_arrays = list()
        c_asr_token_arrays_length = c_repr.asr_tokens.contents.count
        c_asr_token_arrays_repr = c_repr.asr_tokens.contents.entries

        for i in range(c_asr_token_arrays_length):
            c_asr_token_array = c_asr_token_arrays_repr[i].contents
            asr_token_array = [AsrToken.from_c_repr(c_asr_token_array.entries[i].contents) for i in range(c_asr_token_array.count)]
            asr_token_arrays.append(asr_token_array)

        return cls(session_id, custom_data, site_id, input, intent, slots, alternatives, asr_token_arrays, asr_confidence)


class IntentClassifierResult(object):
    def __init__(self, intent_name, confidence_score):
        # type: (Text, float) -> None
        """
        Structured description of the intent classification.

        :param intent_name: name of the intent.
        :type intent_name: Text
        :param confidence_score: confidence_score that the parsed sentence is the `intent_name` intent.
        :type confidence_score: float
        """
        self.intent_name = intent_name
        self.confidence_score = confidence_score

    @classmethod
    def from_c_repr(cls, c_repr):
        intent_name = c_repr.intent_name.decode('utf-8')
        confidence_score = c_repr.confidence_score
        return cls(intent_name, confidence_score)


class IntentNotRecognizedMessage(object):
    def __init__(self, site_id, session_id, input, custom_data, confidence_score, alternatives):
        # type: (Text, Text, Optional[Text], Optional[Text], float, List[NluIntentAlternative]) -> None
        """
        A message that the handler receives from the Dialogue manager when an intent is not recognized and that the
        session was initialized with the intent_not_recognized flag turned on.

        :param site_id: Site where the user interaction is taking place.
        :type site_id: Text
        :param session_id: Session identifier that was started.
        :type session_id: Text
        :param input: The user input that has generated this intent. This parameter is nullable
        :type input: Optional[Text]
        :param custom_data: Custom data passed by the Dialogue Manager in the current dialogue session. This parameter is nullable
        :type custom_data: Optional[Text]
        :param confidence_score: Between 0 and 1
        :type confidence_score: float
        :param alternatives: A list of alternatives intent resolutions
        :type alternatives: List[NluIntentAlternatives]
        """
        self.site_id = site_id
        self.session_id = session_id
        self.input = input
        self.custom_data = custom_data
        self.confidence_score = confidence_score
        self.alternatives = alternatives

    @classmethod
    def from_c_repr(cls, c_repr):
        site_id = c_repr.site_id.decode('utf-8')
        session_id = c_repr.session_id.decode('utf-8')
        input = c_repr.input.decode('utf-8') if c_repr.input else None
        custom_data = c_repr.custom_data.decode('utf-8') if c_repr.custom_data else None
        confidence_score = float(c_repr.confidence_score)

        alternatives = []
        if c_repr.alternatives:
            intent_alternatives_length = c_repr.alternatives.contents.count
            c_intent_alternatives_array_repr = c_repr.alternatives.contents.entries

            for i in range(intent_alternatives_length):
                intent_alternative = NluIntentAlternative.from_c_repr(c_intent_alternatives_array_repr[i].contents)
                alternatives.append(intent_alternative)

        return cls(site_id, session_id, input, custom_data, confidence_score, alternatives)

    def __eq__(self, other):
        return self.__dict__ == other.__dict__