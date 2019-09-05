from ctypes import Structure, c_float, c_int32, c_char_p, POINTER, c_int

class CAsrDecodingDuration(Structure):
    _fields_ = [
        ("start", c_float),
        ("end", c_float)
    ]

    @classmethod
    def from_repr(cls, repr):
        return cls(repr.start, repr.end)


class CAsrToken(Structure):
    _fields_ = [
        ("value", c_char_p),
        ("confidence", c_float),
        ("range_start", c_int32),
        ("range_end", c_int32),
        ("time", CAsrDecodingDuration)
    ]

    @classmethod
    def from_repr(cls, repr):
        value = repr.value.encode('utf-8')
        confidence = c_float(repr.confidence)
        range_start = c_int32(repr.range_start)
        range_end = c_int32(repr.range_end)
        time = CAsrDecodingDuration.from_repr(repr.time)

        return cls(value, confidence, range_start, range_end, time)


class CAsrTokenArray(Structure):
    _fields_ = [
        ("entries", POINTER(POINTER(CAsrToken))),
        ("count", c_int)
    ]

    @classmethod
    def from_repr(cls, repr):
        # type: (List[AsrToken]) -> CAsrTokenArray

        c_asr_tokens = [CAsrToken.from_repr(asr_token) for asr_token in repr]
        c_asr_tokens_pointers = [POINTER(CAsrToken)(c_asr_token) for c_asr_token in c_asr_tokens]

        entries = (POINTER(CAsrToken) * len(c_asr_tokens_pointers))(*c_asr_tokens_pointers)
        count = c_int(len(c_asr_tokens))

        return cls(entries, count)

class CAsrTokenDoubleArray(Structure):
    _fields_ = [
        ("entries", POINTER(POINTER(CAsrTokenArray))),
        ("count", c_int)
    ]

    @classmethod
    def from_repr(cls, repr):
        # type: (List[List[AsrToken]]) -> CAsrTokenDoubleArray
        c_asr_token_arrays = [CAsrTokenArray.from_repr(asr_token_array) for asr_token_array in repr]
        c_asr_token_arrays_pointers = [POINTER(CAsrTokenArray)(c_asr_token_array) for c_asr_token_array in c_asr_token_arrays]

        entries = (POINTER(CAsrTokenArray) * len(c_asr_token_arrays))(*c_asr_token_arrays_pointers)
        count = c_int(len(c_asr_token_arrays))

        return cls(entries, count)