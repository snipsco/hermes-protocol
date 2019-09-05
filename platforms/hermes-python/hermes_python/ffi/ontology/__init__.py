# coding: utf-8
from __future__ import absolute_import
from __future__ import unicode_literals

from ctypes import c_double, c_char_p, c_float, c_int, c_int32, c_int64, c_void_p, c_uint8, POINTER, Structure, pointer, cast, \
    byref
from enum import IntEnum


class CStringArray(Structure):
    _fields_ = [
        ("data", POINTER(c_char_p)),
        ("size", c_int32)
    ]

    @classmethod
    def from_repr(self, repr):
        encoded_strings = [s.encode("utf-8") for s in repr]
        arr = (c_char_p * len(repr))()
        arr[:] = encoded_strings
        return CStringArray(arr, len(repr))

    def into_repr(self):
        return [self.data[i].decode('utf-8') for i in range(self.size)]


class CMapStringToStringArrayEntry(Structure):
    _fields_ = [("key", c_char_p),
                ("value", POINTER(CStringArray))]

    @classmethod
    def from_repr(cls, repr):
        return cls(
            c_char_p(repr[0].encode('utf-8')),
            pointer(CStringArray.from_repr(repr[1]))
        )

    def into_repr(self):
        key = self.key.decode('utf-8')
        list_of_strings = [self.value.contents.data[i].decode('utf-8') for i in range(self.value.contents.size)]
        return key, list_of_strings


class CMapStringToStringArray(Structure):
    _fields_ = [("entries", POINTER(POINTER(CMapStringToStringArrayEntry))),
                ("count", c_int32)]

    @classmethod
    def from_repr(cls, repr):
        input_data_as_list = list(repr.items())
        map_entries = (POINTER(CMapStringToStringArrayEntry) * len(repr))()
        map_entries[:] = [pointer(CMapStringToStringArrayEntry.from_repr(e)) for e in input_data_as_list]

        return cls(
            map_entries,
            len(repr)
        )

    def into_repr(self):
        number_of_entries = self.count
        entries_list = [self.entries[i].contents.into_repr() for i in range(number_of_entries)]
        return dict(entries_list)


class CProtocolHandler(Structure):
    _fields_ = [("handler", c_void_p)]


class CMqttOptions(Structure):
    _fields_ = [("broker_address", c_char_p),
                ("username", c_char_p),
                ("password", c_char_p),
                ("tls_hostname", c_char_p),
                ("tls_ca_file", POINTER(CStringArray)),
                ("tls_ca_path", POINTER(CStringArray)),
                ("tls_client_key", c_char_p),
                ("tls_client_cert", c_char_p),
                ("tls_disable_root_store", c_uint8)]

    @classmethod
    def build(cls, broker_address, username, password, tls_hostname, tls_ca_file, tls_ca_path, tls_client_key,
              tls_client_cert, tls_disable_root_store):
        broker_address = broker_address.encode('utf-8')
        username = username.encode('utf-8') if username else None
        password = password.encode('utf-8') if password else None
        tls_hostname = tls_hostname.encode('utf-8') if tls_hostname else None
        tls_ca_file = tls_ca_file.encode('utf-8') if tls_ca_file else None
        tls_ca_path = tls_ca_path.encode('utf-8') if tls_ca_path else None
        tls_client_key = tls_client_key.encode('utf-8') if tls_client_key else None
        tls_client_cert = tls_client_cert.encode('utf-8') if tls_client_cert else None
        tls_disable_root_store = 1 if tls_disable_root_store else 0  # tls_disable_root_store is a boolean

        return cls(broker_address,
                   username, password,
                   tls_hostname, tls_ca_file, tls_ca_path, tls_client_key, tls_client_cert, tls_disable_root_store)

    @classmethod
    def from_repr(cls, repr):
        return cls.build(repr.broker_address,
                         repr.username, repr.password,
                         repr.tls_hostname, repr.tls_ca_file, repr.tls_ca_path, repr.tls_client_key,
                         repr.tls_client_cert, repr.tls_disable_root_store)


# Slot Types Structs

class CAmountOfMoneyValue(Structure):
    _fields_ = [("unit", c_char_p),
                ("value", c_float),
                ("precision", c_int)]  # TODO : Precision is an enum.


class CTemperatureValue(Structure):
    _fields_ = [("unit", c_char_p),
                ("value", c_float)]


class CInstantTimeValue(Structure):
    _fields_ = [("value", c_char_p),
                ("grain", c_int),  # TODO : CGrain is an enum ...
                ("precision", c_int)]  # TODO : Precision is an enum ...


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


class SlotValueType(IntEnum):
    CUSTOM = 1
    NUMBER = 2
    ORDINAL = 3
    INSTANTTIME = 4
    TIMEINTERVAL = 5
    AMOUNTOFMONEY = 6
    TEMPERATURE = 7
    DURATION = 8
    PERCENTAGE = 9
    MUSICARTIST = 10
    MUSICALBUM = 11
    MUSICTRACK = 12
    CITY = 13
    COUNTRY = 14
    REGION = 15


class Grain(IntEnum):
    YEAR = 0
    QUARTER = 1
    MONTH = 2
    WEEK = 3
    DAY = 4
    HOUR = 5
    MINUTE = 6
    SECOND = 7


class Precision(IntEnum):
    APPROXIMATE = 0
    EXACT = 1


class SNIPS_HERMES_COMPONENT(IntEnum):
    SNIPS_HERMES_COMPONENT_NONE = -1
    SNIPS_HERMES_COMPONENT_AUDIO_SERVER = 1
    SNIPS_HERMES_COMPONENT_HOTWORD = 2
    SNIPS_HERMES_COMPONENT_ASR = 3
    SNIPS_HERMES_COMPONENT_NLU = 4
    SNIPS_HERMES_COMPONENT_DIALOGUE = 5
    SNIPS_HERMES_COMPONENT_TTS = 6
    SNIPS_HERMES_COMPONENT_INJECTION = 7
    SNIPS_HERMES_COMPONENT_CLIENT_APP = 8

    @classmethod
    def from_repr(cls, repr):
        # type: (Option[HermesComponent]) -> SNIPS_HERMES_COMPONENT
        if repr:
            return SNIPS_HERMES_COMPONENT(repr.value)
        else:
            return SNIPS_HERMES_COMPONENT.SNIPS_HERMES_COMPONENT_NONE


class CSlotValue(Structure):
    _fields_ = [
        ("value", c_void_p),
        ("value_type", c_int32)
    ]

    @classmethod
    def build(cls, value, value_type):
        return cls(value, value_type)

    @classmethod
    def from_repr(cls, repr):
        if SlotValueType.CUSTOM == repr.value_type:  # CUSTOM
            c_repr_custom_value = repr.value.value.encode('utf-8')
            c_repr_value = cast(c_char_p(c_repr_custom_value), c_void_p)
            return cls(c_repr_value, c_int32(repr.value_type))

        elif SlotValueType.NUMBER == repr.value_type:  # NUMBER
            c_repr_number = c_double(repr.value.value)
            cls(byref(c_repr_number), c_int32(repr.value_type))

        elif SlotValueType.ORDINAL == repr.value_type:  # ORDINAL
            c_repr_ordinal_value = c_int64(repr.value.value)
            cls(byref(c_repr_ordinal_value), c_int32(repr.value_type))

        elif SlotValueType.INSTANTTIME == repr.value_type:  # INSTANTTIME
            c_repr_instant_time_value = CInstantTimeValue.from_repr(repr.value)
            cls(byref(c_repr_instant_time_value), c_int32(repr.value_type))

        elif SlotValueType.TIMEINTERVAL == repr.value_type:  # TIMEINTERVAL
            cls(c_void_p, c_int32(repr.value_type))
        elif SlotValueType.AMOUNTOFMONEY == repr.value_type:  # AMOUNTOFMONEY
            cls(c_void_p, c_int32(repr.value_type))
        elif SlotValueType.TEMPERATURE == repr.value_type:  # TEMPERATURE
            cls(c_void_p, c_int32(repr.value_type))
        elif SlotValueType.DURATION == repr.value_type:  # DURATION
            cls(c_void_p, c_int32(repr.value_type))
        elif SlotValueType.PERCENTAGE == repr.value_type:  # PERCENTAGE
            cls(c_void_p, c_int32(repr.value_type))
        elif SlotValueType.MUSICARTIST == repr.value_type:  # MUSICARTIST
            cls(c_void_p, c_int32(repr.value_type))
        elif SlotValueType.MUSICALBUM == repr.value_type:  # MUSICALBUM
            cls(c_void_p, c_int32(repr.value_type))
        elif SlotValueType.MUSICTRACK == repr.value_type:  # MUSICTRACK
            cls(c_void_p, c_int32(repr.value_type))
        elif SlotValueType.CITY == repr.value_type:  # CITY
            cls(c_void_p, c_int32(repr.value_type))
        elif SlotValueType.COUNTRY == repr.value_type:  # COUNTRY
            cls(c_void_p, c_int32(repr.value_type))
        elif SlotValueType.REGION == repr.value_type:  # REGION
            cls(c_void_p, c_int32(repr.value_type))

        else:
            raise Exception("Bad value type. Got : {}".format(repr.value_type))


class CSlotValueArray(Structure):
    _fields_ = [
        ("slot_values", POINTER(CSlotValue)),
        ("size", c_int32)]

    @classmethod
    def from_repr(cls, repr):
        # type: (List[SlotValue]) -> CSlotValueArray

        c_slot_values_list = [CSlotValue.from_repr(slot_value) for slot_value in repr]
        entries = (CSlotValue * len(c_slot_values_list))(*c_slot_values_list)
        count = c_int32(len(c_slot_values_list))

        return cls(entries, count)


class CSlot(Structure):
    _fields_ = [
        ("value", POINTER(CSlotValue)),
        ("alternatives", POINTER(CSlotValueArray)),
        ("raw_value", c_char_p),
        ("entity", c_char_p),
        ("slot_name", c_char_p),
        ("range_start", c_int32),
        ("range_end", c_int32),
        ("confidence_score", c_float)
    ]

    @classmethod
    def build(cls, c_slot_value_p, alternatives_p, raw_value, entity, slot_name, range_start, range_end, confidence_score):
        # type: (POINTER(CSlotValue), POINTER(CSlotValueArray), str, str, str, int, int, float) -> CSlot
        raw_value = raw_value.encode('utf-8') if raw_value else None
        entity = entity.encode('utf-8') if entity else None
        slot_name = slot_name.encode('utf-8') if slot_name else None
        range_start = range_start
        range_end = range_end
        confidence_score = float(confidence_score) if confidence_score else float(-1)
        return cls(c_slot_value_p, alternatives_p, raw_value, entity, slot_name, range_start, range_end, confidence_score)

    @classmethod
    def from_repr(cls, repr):
        # type: (Slot) -> CSlot
        c_slot_value = CSlotValue.from_repr(repr.slot_value)
        c_slot_value_p = POINTER(CSlotValue)(c_slot_value)

        alternatives = CSlotValueArray.from_repr(repr.alternatives)
        alternatives_p = POINTER(CSlotValueArray)(alternatives)

        return CSlot.build(c_slot_value_p,
                             alternatives_p,
                             repr.raw_value,
                             repr.entity,
                             repr.slot_name,
                             repr.range_start,
                             repr.range_end,
                             repr.confidence_score)


class CSlotList(Structure):
    _fields_ = [
        ("slots", POINTER(CSlot)),
        ("size", c_int32)
    ]
