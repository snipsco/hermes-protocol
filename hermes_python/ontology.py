# -*- coding: utf-8 -*-

from __future__ import absolute_import
from six.moves import range
class Slot(object):
    def __init__(self, value, raw_value, entity, slot_name, range_start, range_end):
        self.value = None
        self.raw_value = raw_value
        self.entity = entity
        self.slot_name = slot_name
        self.range_start = range_start
        self.range_end = range_end

    @classmethod
    def from_c_repr(cls, c_repr):
        raw_value = c_repr.raw_value
        entity = c_repr.entity
        slot_name = c_repr.slot_name
        range_start = c_repr.range_start
        range_end = c_repr.range_end

        return cls(None, raw_value, entity, slot_name, range_start, range_end)


class SlotMap(object):
    def __init__(self, mapping):
        self.__data = mapping

    def __getattr__(self, name):
        return self.__data.get(name, None)

    @classmethod
    def from_c_repr(cls, c_slots_list_repr):
        mapping = dict()

        slots_list_length = c_slots_list_repr.size
        c_slots_array_repr = c_slots_list_repr.slots

        for i in range(slots_list_length):
            slot = Slot.from_c_repr(c_slots_array_repr[i])
            mapping[slot.slot_name] = slot
        return cls(mapping)


class IntentMessage(object):
    def __init__(self, session_id, custom_data, site_id, input, intent, slots):
        self.session_id = session_id
        self.custom_data = custom_data
        self.site_id = site_id
        self.input = input
        self.intent = intent
        self.slots = slots

    @classmethod
    def from_c_repr(cls, c_repr):
        session_id = c_repr.session_id
        custom_data = c_repr.custom_data
        site_id = c_repr.site_id
        input = c_repr.input
        intent = IntentClassifierResult.from_c_repr(c_repr.intent.contents)
        slots = SlotMap.from_c_repr(c_repr.slots.contents)  # TODO : Handle no slot case !
        return cls(session_id, custom_data, site_id, input, intent, slots)


class IntentClassifierResult(object):
    def __init__(self, intent_name, probability):
        self.intent_name = intent_name
        self.probability = probability

    @classmethod
    def from_c_repr(cls, c_repr):
        intent_name = c_repr.intent_name
        probability = c_repr.probability
        return cls(intent_name, probability)


class SessionEndedMessage(object):
    def __init__(self, session_id, custom_data, site_id):
        self.session_id = session_id
        self.custom_data = custom_data
        self.site_id = site_id

    @classmethod
    def from_c_repr(cls, c_repr):
        session_id = c_repr.session_id
        custom_data = c_repr.custom_data
        site_id = c_repr.site_id
        return cls(session_id, custom_data, site_id)


class SessionQueuedMessage(object):
    def __init__(self, session_id, custom_data, site_id):
        self.session_id = session_id
        self.custom_data = custom_data
        self.site_id = site_id

    @classmethod
    def from_c_repr(cls, c_repr):
        session_id = c_repr.session_id
        custom_data = c_repr.custom_data
        site_id = c_repr.site_id
        return cls(session_id, custom_data, site_id)


class SessionStartedMessage(object):
    def __init__(self, session_id, custom_data, site_id, reactivated_from_session_id):
        self.session_id = session_id
        self.custom_data = custom_data
        self.site_id = site_id
        self.reactivated_from_session_id = reactivated_from_session_id

    @classmethod
    def from_c_repr(cls, c_repr):
        session_id = c_repr.session_id
        custom_data = c_repr.custom_data
        site_id = c_repr.site_id
        reactivated_from_session_id = c_repr.reactivated_from_session_id
        return cls(session_id, custom_data, site_id, reactivated_from_session_id)
