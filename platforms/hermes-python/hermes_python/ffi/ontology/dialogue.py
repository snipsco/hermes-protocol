from ctypes import c_char_p, c_int32, c_int64, c_int, c_float, c_uint8, c_void_p, POINTER, pointer, Structure, c_double,\
    byref, cast
from enum import IntEnum

from ..ontology import CStringArray, SlotValueType, Grain, Precision, SNIPS_HERMES_COMPONENT
from .nlu import CNluIntentAlternativeArray, CNluSlotArray, CNluIntentClassifierResult
from .asr import CAsrTokenDoubleArray


class CSayMessage(Structure):
    _fields_ = [("text", c_char_p),
                ("lang", c_char_p),
                ("id", c_char_p),
                ("site_id", c_char_p),
                ("session_id", c_char_p)]


class CSayFinishedMessage(Structure):
    _fields_ = [("id", POINTER(c_char_p)),
                ("session_id", POINTER(c_char_p))]


class CContinueSessionMessage(Structure):
    _fields_ = [("session_id", c_char_p),
                ("text", c_char_p),
                ("intent_filter", POINTER(CStringArray)),
                ("custom_data", c_char_p),
                ("slot", c_char_p),
                ("send_intent_not_recognized", c_uint8)]

    @classmethod
    def build(cls, session_id, text, intent_filter, custom_data, slot=None, send_intent_not_recognized=False):
        session_id = session_id.encode('utf-8')
        text = text.encode('utf-8')
        intent_filter = [intent_filter_item.encode('utf-8') for intent_filter_item in intent_filter]

        c_intent_filter = CStringArray()
        c_intent_filter.size = c_int(len(intent_filter))
        c_intent_filter.data = (c_char_p * len(intent_filter))(*intent_filter)

        custom_data = custom_data.encode('utf-8') if custom_data else None
        slot = slot.encode('utf-8') if slot else None
        send_intent_not_recognized = 1 if send_intent_not_recognized else 0  # send_intent_not_recognized is a boolean

        cContinueSessionMessage = cls(session_id, text, pointer(c_intent_filter), custom_data, slot, send_intent_not_recognized)
        return cContinueSessionMessage

    @classmethod
    def from_repr(cls, repr):
        session_id = repr.session_id
        text = repr.text
        intent_filter = repr.intent_filter
        custom_data = repr.custom_data
        slot = repr.slot
        send_intent_not_recognized = repr.send_intent_not_recognized

        return cls.build(session_id, text, intent_filter, custom_data, slot, send_intent_not_recognized)


class CEndSessionMessage(Structure):
    _fields_ = [("session_id", c_char_p),
                ("text", c_char_p)]

    @classmethod
    def build(cls, session_id, text):
        b_text = text.encode('utf-8') if text else None
        b_session_id = session_id.encode('utf-8') if session_id else None
        return cls(b_session_id, b_text)

    @classmethod
    def from_repr(cls, py_repr):
        return py_repr.into_c_repr()


SNIPS_SESSION_INIT_TYPE_ACTION = 1
SNIPS_SESSION_INIT_TYPE_NOTIFICATION = 2


class CSessionInit(Structure):
    _fields_ = [("init_type", c_int32)]  # 1 : Action, 2: Notification


class CActionSessionInit(Structure):
    _fields_ = [("text", c_char_p),  # Nullable
                ("intent_filter", POINTER(CStringArray)),  # Nullable
                ("can_be_enqueued", c_uint8),
                ("send_intent_not_recognized", c_uint8)] \

    @classmethod
    def build(cls, text, intent_filter, can_be_enqueued_boolean, send_intent_not_recognized):
        text = text.encode('utf-8') if text else None
        intent_filter = [intent_filter_item.encode('utf-8') for intent_filter_item in intent_filter]

        c_intent_filter = CStringArray()
        c_intent_filter.size = c_int(len(intent_filter))
        c_intent_filter.data = (c_char_p * len(intent_filter))(*intent_filter)

        can_be_enqueued = 1 if can_be_enqueued_boolean else 0
        send_intent_not_recognized = 1 if send_intent_not_recognized else 0  # send_intent_not_recognized is a boolean

        return cls(text, pointer(c_intent_filter), can_be_enqueued, send_intent_not_recognized)


class CSessionInitAction(CSessionInit):
    _fields_ = [("value", POINTER(CActionSessionInit))]

    @classmethod
    def build(cls, text, intent_filter, can_be_enqueued_boolean, send_intent_not_recognized):
        cActionSessionInit = CActionSessionInit.build(text, intent_filter, can_be_enqueued_boolean, send_intent_not_recognized)
        return cls(c_int(SNIPS_SESSION_INIT_TYPE_ACTION), pointer(cActionSessionInit))


class CSessionInitNotification(CSessionInit):
    _fields_ = [("value", c_char_p)]

    @classmethod
    def build(cls, value):
        encoded_value = value.encode('utf-8')
        return cls(c_int(SNIPS_SESSION_INIT_TYPE_NOTIFICATION), encoded_value)


class CStartSessionMessageAction(Structure):
    _fields_ = [("init", CSessionInitAction),
                ("custom_data", c_char_p),
                ("site_id", c_char_p)]

    @classmethod
    def build(cls, init, custom_data, site_id):
        custom_data = custom_data.encode('utf-8') if custom_data else None
        site_id = site_id.encode('utf-8') if site_id else None
        return cls(init, custom_data, site_id)

    @classmethod
    def from_repr(cls, repr):
        return repr.into_c_repr()


class CStartSessionMessageNotification(Structure):
    _fields_ = [("init", CSessionInitNotification),
                ("custom_data", c_char_p),
                ("site_id", c_char_p)]

    @classmethod
    def build(cls, init, custom_data, site_id):
        custom_data = custom_data.encode('utf-8') if custom_data else None
        site_id = site_id.encode('utf-8') if site_id else None
        return cls(init, custom_data, site_id)

    @classmethod
    def from_repr(cls, repr):
        return repr.into_c_repr()


class CIntentMessage(Structure):
    _fields_ = [("session_id", c_char_p),
                ("custom_data", c_char_p),
                ("site_id", c_char_p),
                ("input", c_char_p),
                ("intent", POINTER(CNluIntentClassifierResult)),
                ("slots", POINTER(CNluSlotArray)),
                ("alternatives", POINTER(CNluIntentAlternativeArray)),
                ("asr_tokens", POINTER(CAsrTokenDoubleArray)),
                ("asr_confidence", c_float)]

    @classmethod
    def build(cls, session_id, custom_data, site_id, input, c_intent_classifier_result, c_slots_p, c_intent_alternative_array_p, c_asr_token_double_array_p, asr_confidence):
        session_id = session_id.encode('utf-8')
        custom_data = custom_data.encode('utf-8') if custom_data else None
        site_id = site_id.encode('utf-8') if site_id else None
        input = input.encode('utf-8') if input else None

        return cls(session_id, custom_data, site_id, input, c_intent_classifier_result, c_slots_p, c_intent_alternative_array_p, c_asr_token_double_array_p, asr_confidence)

    @classmethod
    def from_repr(cls, repr):
        c_intent_classifier_result = POINTER(CNluIntentClassifierResult)(CNluIntentClassifierResult.from_repr(repr.intent))
        c_slots = POINTER(CNluSlotArray)(CNluSlotArray.from_repr(repr.slots))
        c_intent_alternatives = POINTER(CNluIntentAlternativeArray)(CNluIntentAlternativeArray.from_repr(repr.alternatives))
        c_asr_token_double_array_p = POINTER(CAsrTokenDoubleArray)(CAsrTokenDoubleArray.from_repr(repr.asr_tokens))
        asr_confidence = c_float(repr.asr_confidence)
        return cls.build(repr.session_id, repr.custom_data, repr.site_id, repr.input, c_intent_classifier_result, c_slots, c_intent_alternatives, c_asr_token_double_array_p, asr_confidence)


class SNIPS_SESSION_TERMINATION_TYPE(IntEnum):
    SNIPS_SESSION_TERMINATION_TYPE_NOMINAL = 1
    SNIPS_SESSION_TERMINATION_TYPE_SITE_UNAVAILABLE = 2
    SNIPS_SESSION_TERMINATION_TYPE_ABORTED_BY_USER = 3
    SNIPS_SESSION_TERMINATION_TYPE_INTENT_NOT_RECOGNIZED = 4
    SNIPS_SESSION_TERMINATION_TYPE_TIMEOUT = 5
    SNIPS_SESSION_TERMINATION_TYPE_ERROR = 6


class CSessionTermination(Structure):
    _fields_ = [("termination_type", c_int),
                ("data", c_char_p),
                ("component", c_int)]

    _enum_types_ = [("termination_type", SNIPS_SESSION_TERMINATION_TYPE),
                    ("component", SNIPS_HERMES_COMPONENT)]

    @classmethod
    def build(cls, termination_type, data, component):
        termination_type = termination_type.into_c_repr()
        data = data.encode('utf-8') if data else None
        component = SNIPS_HERMES_COMPONENT.from_repr(component)
        return cls(termination_type, data, component)

    @classmethod
    def from_repr(cls, repr):
        # type:(SessionTermination) -> CSessionTermination
        component = repr.termination_type.component if repr.termination_type.component else None
        return cls.build(repr.termination_type, repr.data, component)


class CSessionEndedMessage(Structure):
    _fields_ = [("session_id", c_char_p),
                ("custom_data", c_char_p),
                ("termination", CSessionTermination),
                ("site_id", c_char_p)]

    @classmethod
    def build(cls, session_id, custom_data, c_termination_repr, site_id):
        session_id = session_id.encode('utf-8')
        custom_data = custom_data.encode('utf-8') if custom_data else None
        site_id = site_id.encode('utf-8')
        termination = c_termination_repr
        return cls(session_id, custom_data, termination, site_id)

    @classmethod
    def from_repr(cls, repr):
        return cls.build(repr.session_id, repr.custom_data, CSessionTermination.from_repr(repr.termination), repr.site_id)


class CSessionQueuedMessage(Structure):
    _fields_ = [("session_id", c_char_p),
                ("custom_data", c_char_p),
                ("site_id", c_char_p)]

    @classmethod
    def build(cls, session_id, custom_data, site_id):
        session_id = session_id.encode('utf-8')
        custom_data = custom_data.encode('utf-8') if custom_data else None
        site_id = site_id.encode('utf-8')
        return cls(session_id, custom_data, site_id)

    @classmethod
    def from_repr(cls, repr):
        return cls.build(repr.session_id, repr.custom_data, repr.site_id)


class CSessionStartedMessage(Structure):
    _fields_ = [("session_id", c_char_p),
                ("custom_data", c_char_p),
                ("site_id", c_char_p),
                ("reactivated_from_session_id", c_char_p)]

    @classmethod
    def build(cls, session_id, custom_data, site_id, reactivated_from_session_id):
        session_id = session_id.encode('utf-8')
        custom_data = custom_data.encode('utf-8') if custom_data else None
        site_id = site_id.encode('utf-8')
        reactivated_from_session_id = reactivated_from_session_id.encode('utf-8') if reactivated_from_session_id else None
        return cls(session_id, custom_data, site_id, reactivated_from_session_id)

    @classmethod
    def from_repr(cls, repr):
        return cls.build(repr.session_id, repr.custom_data, repr.site_id, repr.reactivated_from_session_id)


class CIntentNotRecognizedMessage(Structure):
    _fields_ = [("site_id", c_char_p),
                ("session_id", c_char_p),
                ("input", c_char_p),  # Nullable
                ("custom_data", c_char_p),  # Nullable
                ("alternatives", POINTER(CNluIntentAlternativeArray)),  # Nullable
                ("confidence_score", c_float)]

    @classmethod
    def build(cls, site_id, session_id, input, custom_data, c_intent_alternative_array, confidence_score):
        site_id = site_id.encode('utf-8')
        session_id = session_id.encode('utf-8')
        input = input.encode('utf-8') if input else None
        custom_data = custom_data.encode('utf-8') if custom_data else None
        confidence_score = float(confidence_score)

        return cls(site_id, session_id, input, custom_data, c_intent_alternative_array, confidence_score)

    @classmethod
    def from_repr(cls, repr):
        alternatives_p = POINTER(CNluIntentAlternativeArray)(CNluIntentAlternativeArray.from_repr(repr.alternatives))
        return cls.build(repr.site_id, repr.session_id, repr.input, repr.custom_data, alternatives_p, repr.confidence_score)


class CDialogueConfigureIntent(Structure):
    _fields_ = [
        ("intent_id", c_char_p),
        ("enable", c_uint8)]

    @classmethod
    def from_repr(cls, repr):
        # type: (DialogueConfigureIntent) -> CDialogueConfigureIntent
        return cls.build(repr.intent_id, repr.enable)

    @classmethod
    def build(cls, intent_id, enable):
        # type: (str, bool) -> CDialogueConfigureIntent
        intent_id = intent_id.encode('utf-8')
        enable = c_uint8(1) if enable else c_uint8(0)

        return cls(intent_id, enable)


class CDialogueConfigureIntentArray(Structure):
    _fields_ = [
        ("entries", POINTER(POINTER(CDialogueConfigureIntent))),
        ("count", c_int32)]

    @classmethod
    def build(cls, intents):
        # type: (List[DialogueConfigureIntent]) -> CDialogueConfigureIntentArray
        c_dialogue_configure_intents = [CDialogueConfigureIntent.from_repr(dialogue_configure_intent) for
                                        dialogue_configure_intent in intents]

        c_dialogue_configure_intents = [POINTER(CDialogueConfigureIntent)(intent) for intent in
                                        c_dialogue_configure_intents]

        entries = (POINTER(CDialogueConfigureIntent) * len(intents))(*c_dialogue_configure_intents)
        entries = cast(entries, POINTER(POINTER(CDialogueConfigureIntent)))
        count = c_int32(len(intents))

        return cls(entries, count)

    @classmethod
    def from_repr(cls, repr):
        return cls.build(repr)


class CDialogueConfigureMessage(Structure):
    _fields_ = [("site_id", c_char_p),  # site_id is nullable.
                ("intents", POINTER(CDialogueConfigureIntentArray))]

    @classmethod
    def from_repr(cls, repr):
        return cls.build(repr.site_id, repr.intents)

    @classmethod
    def build(cls, site_id, intents):
        # type: (str, List[DialogueConfigureIntent]) -> CDialogueConfigureMessage
        site_id = site_id.encode('utf-8') if site_id else None
        c_dialogue_configure_intent_array = CDialogueConfigureIntentArray.build(intents)
        c_dialogue_configure_intent_array_p = POINTER(CDialogueConfigureIntentArray)(c_dialogue_configure_intent_array)
        return cls(site_id, c_dialogue_configure_intent_array_p)