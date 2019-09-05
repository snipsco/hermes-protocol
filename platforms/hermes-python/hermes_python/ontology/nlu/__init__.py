from typing import List

from collections import defaultdict

# Ever since Python3.3, Mapping was moved to the abc submodule.
try:
    from collections.abc import Mapping  # type: ignore
except ImportError:
    from collections import Mapping  # Python2.7+

from ..slot import Slot


class NluIntentAlternative(object):
    def __init__(self, intent_name, confidence_score, slots):
        # type: (Optional[Text], float, SlotMap) -> None
        self.intent_name = intent_name
        self.confidence_score = confidence_score
        self.slots = slots

    @classmethod
    def from_c_repr(cls, c_repr):
        intent_name = c_repr.intent_name.decode('utf-8') if c_repr.intent_name else None
        confidence_score = float(c_repr.confidence_score)

        if c_repr.slots:  # Slots is now nullable.
            slots = SlotMap.from_c_repr(c_repr.slots.contents)
        else:
            slots = SlotMap({})

        return cls(intent_name, confidence_score, slots)


class NluSlot(object):
    def __init__(self, slot_value, raw_value, alternatives, entity, slot_name, range_start, range_end,
                 confidence_score):
        # type: (SlotValue, str, List[SlotValue], str, str, int, int, float) -> None
        self.slot_value = slot_value
        self.raw_value = raw_value
        self.alternatives = alternatives
        self.entity = entity
        self.slot_name = slot_name
        self.range_start = range_start
        self.range_end = range_end
        self.confidence_score = confidence_score

    def __eq__(self, other):
        self.__dict__ == other.__dict__

    @classmethod
    def from_c_repr(cls, c_repr):
        slot = Slot.from_c_repr(c_repr.nlu_slot[0])

        slot_value = slot.slot_value  # To ensure compatibility, we flatten the data structure ...
        raw_value = slot.raw_value
        alternatives = slot.alternatives
        entity = slot.entity
        slot_name = slot.slot_name
        range_start = slot.range_start
        range_end = slot.range_end
        confidence_score = slot.confidence_score
        return cls(slot_value, raw_value, alternatives, entity, slot_name, range_start, range_end, confidence_score)


class SlotMap(Mapping):
    def __init__(self, data):
        # type: (dict) -> None
        mapping = dict()
        for k, v in data.items():
            mapping[k] = SlotsList(v)
        self.__data = mapping

    def __getattr__(self, item):
        return self.__data.get(item, SlotsList())

    def __getitem__(self, item):
        return self.__data.get(item, SlotsList())

    def __len__(self):
        return self.__data.__len__()

    def __iter__(self):
        return iter(self.__data)

    def __eq__(self, other):
        return self.__data == other.__data

    @classmethod
    def from_c_repr(cls, c_slots_list_repr):
        mapping = defaultdict(SlotsList)

        slots_list_length = c_slots_list_repr.count
        c_slots_array_repr = c_slots_list_repr.entries

        for i in range(slots_list_length):
            nlu_slot = NluSlot.from_c_repr(c_slots_array_repr[i].contents)
            slot_name = nlu_slot.slot_name
            mapping[slot_name].append(nlu_slot)
        return cls(mapping)


class SlotsList(list):
    # An extension to make things easier to reach slot_values that are deeply nested in the IntentMessage datastructure.

    def __eq__(self, other):
        return self.__dict__ == other.__dict__

    def first(self):
        """

        :return:
        """
        if len(self) > 0:
            return self[0].slot_value.value
        else:
            return None

    def all(self):
        """

        :return:
        """
        if len(self) > 0:
            return [element.slot_value.value for element in self]
        else:
            return None
