from ctypes import c_char_p, c_int32, c_float, c_void_p, POINTER, Structure


class CStringArray(Structure):
    _fields_ = [
        ("data", POINTER(c_char_p)),
        ("size", c_int32)
    ]


class CProtocolHandler(Structure):
    _fields_ = [("handler", c_void_p)]


class CTtsFacade(Structure):
    _fields_ = [("facade", c_void_p)]


class CDialogueFacade(Structure):
    _fields_ = [("facade", c_void_p)]


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
                ("intent_filter", POINTER(CStringArray))]  # Not sure about this one ...


class CEndSessionMessage(Structure):
    _fields_ = [("session_id", c_char_p),
                ("text", c_char_p)]


class CSessionInitType(Structure):
    _fields_ = []


class CSessionInit(Structure):
    _fields_ = [("init_type", c_int32),
                ("value", c_char_p)]


class CStartSessionMessage(Structure):
    _fields_ = [("init", CSessionInit),
                ("custom_data", c_char_p),
                ("site_id", c_char_p)]


class CIntentClassifierResult(Structure):
    _fields_ = [("intent_name", c_char_p),
                ("probability", c_float)]


class CSlotValue(Structure):
    _fields_ = [
        ("value", c_void_p),
        # TODO : Points to either a *const char, a CNumberValue, a COrdinalValue, a CInstantTimeValue, a CTimeIntervalValue, a CAmountOfMoneyValue, a CTemperatureValue or a CDurationValue depending on value_type
        ("value_type", c_int32)
    ]


class CSlot(Structure):
    _fields_ = [
        ("value", CSlotValue),
        ("raw_value", c_char_p),
        ("entity", c_char_p),
        ("slot_name", c_char_p),
        ("range_start", c_int32),
        ("range_end", c_int32)
    ]


class CSlotList(Structure):
    _fields_ = [
        ("slots", POINTER(CSlot)),
        ("size", c_int32)
    ]


class CIntentMessage(Structure):
    _fields_ = [("session_id", c_char_p),
                ("custom_data", c_char_p),
                ("site_id", c_char_p),
                ("input", c_char_p),
                ("intent", POINTER(CIntentClassifierResult)),
                ("slots", POINTER(CSlotList))]


class CSessionEndedMessage(Structure):
    _fields_ = [("session_id", c_char_p),
                ("custom_data", c_char_p),
                ("termination", c_void_p),
                ("site_id", c_char_p)]


class CSessionQueuedMessage(Structure):
    _fields_ = [("session_id", c_char_p),
                ("custom_data", c_char_p),
                ("site_id", c_char_p)]


class CSessionStartedMessage(Structure):
    _fields_ = [("session_id", c_char_p),
                ("custom_data", c_char_p),
                ("site_id", c_char_p),
                ("reactivated_from_session_id", c_char_p)]
