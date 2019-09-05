from ctypes import c_char_p, c_int, c_int32, c_float, POINTER, Structure, pointer, cast

from ..ontology import CSlot, CSlotValue, CSlotValueArray


class CNluSlot(Structure):
    _fields_ = [
        ("nlu_slot", POINTER(CSlot))
    ]

    @classmethod
    def from_repr(cls, repr):
        # type: (NluSlot) -> CNluSlot

        c_slot_value_p = POINTER(CSlotValue)(CSlotValue.from_repr(repr.slot_value))
        alternatives_p = POINTER(CSlotValueArray)(CSlotValueArray.from_repr(repr.alternatives))

        c_slot = CSlot.build(c_slot_value_p,
                             alternatives_p,
                             repr.raw_value,
                             repr.entity,
                             repr.slot_name,
                             repr.range_start,
                             repr.range_end,
                             repr.confidence_score)

        return cls(POINTER(CSlot)(c_slot))

    @classmethod
    def build(cls, nlu_slot):
        slot_p = POINTER(CSlot)(nlu_slot)
        return cls(slot_p)


class CNluIntentClassifierResult(Structure):
    _fields_ = [("intent_name", c_char_p),
                ("confidence_score", c_float)]

    @classmethod
    def build(cls, intent_name, confidence_score):
        intent_name = intent_name.encode('utf-8')
        confidence_score = float(confidence_score)
        return cls(intent_name, confidence_score)

    @classmethod
    def from_repr(cls, repr):
        return cls.build(repr.intent_name, repr.confidence_score)


class CNluSlotArray(Structure):
    _fields_ = [
        ("entries", POINTER(POINTER(CNluSlot))),
        ("count", c_int)
    ]

    @classmethod
    def from_repr(cls, repr):
        # type: (SlotMap) -> CNluSlotArray


        # We flatten all the slots into a list
        nlu_slots = [nlu_slot for slot_name, nlu_slot_array in repr.items() for nlu_slot in nlu_slot_array]

        c_nlu_slots = [CNluSlot.from_repr(nlu_slot) for nlu_slot in nlu_slots]
        c_nlu_slots_pointers = [POINTER(CNluSlot)(c_nlu_slot) for c_nlu_slot in c_nlu_slots]

        entries = (POINTER(CNluSlot) * len(c_nlu_slots_pointers))(*c_nlu_slots_pointers)
        count = c_int(len(nlu_slots))

        return cls(entries, count)


class CNluIntentAlternative(Structure):
    _fields_ = [
        ("intent_name", c_char_p),
        ("slots", POINTER(CNluSlotArray)),
        ("confidence_score", c_float)
    ]

    @classmethod
    def from_repr(cls, repr):
        intent_name = repr.intent_name.encode('utf-8')
        confidence_score = c_float(repr.confidence_score)
        c_slots = POINTER(CNluSlotArray)(CNluSlotArray.from_repr(repr.slots))
        return cls(intent_name, c_slots, confidence_score)


class CNluIntentAlternativeArray(Structure):
    _fields_ = [
        ("entries", POINTER(POINTER(CNluIntentAlternative))),
        ("count", c_int)
    ]

    @classmethod
    def from_repr(cls, repr):
        c_nlu_intent_alternatives = [CNluIntentAlternative.from_repr(alt) for alt in repr]
        c_nlu_intent_alternatives_pointers = [POINTER(CNluIntentAlternative)(c_nlu_intent_alternative) for c_nlu_intent_alternative in c_nlu_intent_alternatives]

        entries = (POINTER(CNluIntentAlternative) * len(c_nlu_intent_alternatives))(*c_nlu_intent_alternatives_pointers)
        count = c_int(len(c_nlu_intent_alternatives_pointers))

        return cls(entries, count)

