from ctypes import c_char_p, c_int32, c_int64, c_int, c_float, c_uint8, c_void_p, POINTER, pointer, Structure, c_double,\
    byref, cast
from ..ontology import CStringArray


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
        text = text.encode('utf-8') if text else None
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


class CSlotValue(Structure):
    _fields_ = [
        ("value", c_void_p),
        ("value_type", c_int32) # TODO : value_type is an enum
    ]
    @classmethod
    def build(cls, value, value_type):
        return cls(value, value_type)

    @classmethod
    def from_repr(cls, repr):
        if 1 == repr.value_type:  # CUSTOM
            c_repr_custom_value = repr.value.value.encode('utf-8')
            c_repr_value = cast(c_char_p(c_repr_custom_value), c_void_p)
            return cls(c_repr_value, c_int32(repr.value_type))

        elif 2 == repr.value_type:  # NUMBER
            c_repr_number = c_double(repr.value.value)
            cls(byref(c_repr_number), c_int32(repr.value_type))

        elif 4 == repr.value_type:  # INSTANTTIME
            c_repr_instant_time_value = CInstantTimeValue.from_repr(repr.value)
            cls(byref(c_repr_instant_time_value), c_int32(repr.value_type))

        elif 5 == repr.value_type:  # TIMEINTERVAL TODO
            cls(c_void_p, c_int32(repr.value_type))
        elif 6 == repr.value_type:  # AMOUNTOFMONEY TODO
            cls(c_void_p, c_int32(repr.value_type))
        elif 7 == repr.value_type:  # TEMPERATURE TODO
            cls(c_void_p, c_int32(repr.value_type))
        elif 8 == repr.value_type:  # DURATION TODO
            cls(c_void_p, c_int32(repr.value_type))
        elif 9 == repr.value_type:  # PERCENTAGE TODO
            cls(c_void_p, c_int32(repr.value_type))
        elif 10 == repr.value_type:  # MUSICARTIST TODO
            cls(c_void_p, c_int32(repr.value_type))
        elif 11 == repr.value_type:  # MUSICALBUM TODO
            cls(c_void_p, c_int32(repr.value_type))
        elif 12 == repr.value_type:  # MUSICTRACK TODO
            cls(c_void_p, c_int32(repr.value_type))

        else:
            raise Exception("Bad value type. Got : {}".format(repr.value_type))



class CSlot(Structure):
    _fields_ = [
        ("value", CSlotValue),
        ("raw_value", c_char_p),
        ("entity", c_char_p),
        ("slot_name", c_char_p),
        ("range_start", c_int32),
        ("range_end", c_int32),
        ("confidence_score", c_float)
    ]

    @classmethod
    def build(cls, c_slot_value, raw_value, entity, slot_name, range_start, range_end, confidence_score):
        # type: (CSlotValue, str, str, str, int, int, float) -> CSlot
        raw_value = raw_value.encode('utf-8') if raw_value else None
        entity = entity.encode('utf-8') if entity else None
        slot_name = slot_name.encode('utf-8') if slot_name else None
        range_start = range_start
        range_end = range_end
        confidence_score = float(confidence_score) if confidence_score else float(-1)
        return cls(c_slot_value, raw_value, entity, slot_name, range_start, range_end, confidence_score)


class CNluSlot(Structure):
    _fields_ = [
        ("nlu_slot", POINTER(CSlot))
    ]

    @classmethod
    def from_repr(cls, repr):
        # type: NluSlot -> CNluSlot
        c_slot_value = CSlotValue.from_repr(repr.slot_value)
        c_slot = CSlot.build(c_slot_value,
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


class CSlotList(Structure):
    _fields_ = [
        ("slots", POINTER(CSlot)),
        ("size", c_int32)
    ]


class CNluSlotArray(Structure):
    _fields_ = [
        ("entries", POINTER(POINTER(CNluSlot))), # *const *const CNluSlot,
        ("count", c_int)
    ]

    @classmethod
    def from_repr(cls, repr):
        # type: (SlotMap)
        """
        impl CReprOf<Vec<hermes::NluSlot>> for CNluSlotArray {
            fn c_repr_of(input: Vec<hermes::NluSlot>) -> Fallible<Self> {
                let array = Self {
                    count: input.len() as _,
                    entries: Box::into_raw(
                        input
                            .into_iter()
                            .map(|e| CNluSlot::c_repr_of(e).map(|c| c.into_raw_pointer()))
                            .collect::<Fallible<Vec<_>>>()
                            .context("Could not convert map to C Repr")?
                            .into_boxed_slice(),
                    ) as *const *const _,
                };
                Ok(array)
            }
        }
        :param repr:
        :return:
        """

        # We flatten all the slots into a list
        nlu_slots = [nlu_slot for slot_name, nlu_slot_array in repr.items() for nlu_slot in nlu_slot_array]
        count = len(nlu_slots)

        c_nlu_slots = [CNluSlot.from_repr(nlu_slot) for nlu_slot in nlu_slots]
        entries = (CNluSlot * count)(*c_nlu_slots)
        entries = cast(entries, POINTER(POINTER(CNluSlot)))

        return cls(entries, c_int(count))



class CIntentMessage(Structure):
    _fields_ = [("session_id", c_char_p),
                ("custom_data", c_char_p),
                ("site_id", c_char_p),
                ("input", c_char_p),
                ("intent", POINTER(CNluIntentClassifierResult)),
                ("slots", POINTER(CNluSlotArray))]

    @classmethod
    def build(cls, session_id, custom_data, site_id, input, c_intent_classifier_result, c_slot):
        session_id = session_id.encode('utf-8')
        custom_data = custom_data.encode('utf-8') if custom_data else None
        site_id = site_id.encode('utf-8') if site_id else None
        input = input.encode('utf-8') if input else None

        return cls(session_id, custom_data, site_id, input, c_intent_classifier_result, c_slot)

    @classmethod
    def from_repr(cls, repr):
        c_intent_classifier_result = POINTER(CNluIntentClassifierResult)(CNluIntentClassifierResult.from_repr(repr.intent))
        c_slots = POINTER(CNluSlotArray)(CNluSlotArray.from_repr(repr.slots))
        return cls.build(repr.session_id, repr.custom_data, repr.site_id, repr.input, c_intent_classifier_result, c_slots)


class CSessionTermination(Structure):
    _fields_ = [("termination_type", c_int),
                ("data", c_char_p)]

    @classmethod
    def build(cls, termination_type, data):
        data = data.encode('utf-8') if data else None
        return cls(termination_type, data)

    @classmethod
    def from_repr(cls, repr):
        return cls.build(repr.termination_type, repr.data)


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
                ("confidence_score", c_float)]

    @classmethod
    def build(cls, site_id, session_id, input, custom_data, confidence_score):
        site_id = site_id.encode('utf-8')
        session_id = session_id.encode('utf-8')
        input = input.encode('utf-8') if input else None
        custom_data = custom_data.encode('utf-8') if custom_data else None
        confidence_score = float(confidence_score)

        return cls(site_id, session_id, input, custom_data, confidence_score)

    @classmethod
    def from_repr(cls, repr):
        return cls.build(repr.site_id, repr.session_id, repr.input, repr.custom_data, repr.confidence_score)


# Slot Types Structs

class CAmountOfMoneyValue(Structure):
    _fields_ = [("unit", c_char_p),
                ("value", c_float),
                ("precision", c_int)] # TODO : Precision is an enum.


class CTemperatureValue(Structure):
    _fields_ = [("unit", c_char_p),
                ("value", c_float)]


class CInstantTimeValue(Structure):
    _fields_ = [("value", c_char_p),
               ("grain", c_int), # TODO : CGrain is an enum ...
               ("precision", c_int)] # TODO : Precision is an enum ...


class CTimeIntervalValue(Structure):
    _fields_ = [("from_date", c_char_p),
                ("to_date", c_char_p)]


class CDurationValue(Structure):
    _fields_ = [("years", c_int64),
                ("quarters", c_int64),
                ("months", c_int64),
                ("weeks", c_int64),
                ("days", c_int64),
                ("hours", c_int64),
                ("minutes", c_int64),
                ("seconds", c_int64),
                ("precision", c_int)]
