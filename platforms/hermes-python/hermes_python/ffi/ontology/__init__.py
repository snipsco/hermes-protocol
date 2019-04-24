# coding: utf-8
from __future__ import absolute_import
from __future__ import unicode_literals

from ctypes import c_char_p, c_int32, c_void_p, c_uint8, POINTER, Structure, pointer, cast
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
