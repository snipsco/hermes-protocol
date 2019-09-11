# -*- coding: utf-8 -*-
from __future__ import absolute_import
from __future__ import unicode_literals

from ctypes import Structure, c_char_p, POINTER, c_int, pointer, cast
from enum import IntEnum

from ..ontology import CMapStringToStringArray


class InjectionKind(IntEnum):
    ADD = 1
    ADD_FROM_VANILLA = 2


class CInjectionStatusMessage(Structure):
    _fields_ = [("last_injection_date", c_char_p)]


class CInjectionRequestOperation(Structure):
    _fields_ = [("values", POINTER(CMapStringToStringArray)),
                ("kind", c_int)]  # kind is an enum

    @classmethod
    def from_repr(cls, repr):
        kind = repr.kind
        values = pointer(CMapStringToStringArray.from_repr(repr.values))
        return cls(values, kind)


class CInjectionRequestOperations(Structure):
    _fields_ = [("operations", POINTER(POINTER(CInjectionRequestOperation))),
                ("count", c_int)]

    @classmethod
    def from_repr(cls, repr):
        operations = [pointer(CInjectionRequestOperation.from_repr(operation)) for operation in repr]
        count = len(repr)
        array_of_operations = (POINTER(CInjectionRequestOperation) * count)()
        array_of_operations[:] = operations

        return cls(array_of_operations, count)


class CInjectionRequestMessage(Structure):
    _fields_ = [("operations", POINTER(CInjectionRequestOperations)),
                ("lexicon", POINTER(CMapStringToStringArray)),
                ("cross_language", c_char_p),  # Nullable
                ("id", c_char_p)]  # Nullable

    @classmethod
    def from_repr(cls, repr):
        operations = pointer(CInjectionRequestOperations.from_repr(repr.operations))
        lexicon = pointer(CMapStringToStringArray.from_repr(repr.lexicon))
        cross_language = repr.cross_language.encode('utf-8') if repr.cross_language else None
        id = repr.id.encode('utf-8') if repr.id else None
        return cls(operations, lexicon, cross_language, id)


class CInjectionCompleteMessage(Structure):
    _fields_ = [("request_id", c_char_p)]

    @classmethod
    def from_repr(cls, repr):
        request_id = repr.request_id.encode('utf-8')
        return cls(request_id)


