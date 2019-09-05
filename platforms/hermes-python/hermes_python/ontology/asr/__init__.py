class AsrDecodingDuration(object):
    def __init__(self, start, end):
        # type: (float, float) -> None
        self.start = start
        self.end = end

    def __eq__(self, other):
        return self.__dict__ == other.__dict__

    @classmethod
    def from_c_repr(cls, c_repr):
        return cls(float(c_repr.start), float(c_repr.end))


class AsrToken(object):
    def __init__(self, value, confidence, range_start, range_end, time):
        # type: (str, float, int, int, AsrDecodingDuration) -> None
        self.value = value
        self.confidence = confidence
        self.range_start = range_start
        self.range_end = range_end
        self.time = time

    def __eq__(self, other):
        return self.__dict__ == other.__dict__

    @classmethod
    def from_c_repr(cls, c_repr):
        value = c_repr.value.decode('utf-8')
        confidence = c_repr.confidence
        range_start = int(c_repr.range_start)
        range_end = int(c_repr.range_end)
        time = AsrDecodingDuration.from_c_repr(c_repr.time)

        return cls(value, confidence, range_start, range_end, time)