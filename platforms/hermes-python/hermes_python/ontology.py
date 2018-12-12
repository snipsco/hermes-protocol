# -*- coding: utf-8 -*-

from __future__ import absolute_import
from __future__ import unicode_literals
from builtins import range
from builtins import object
from collections import defaultdict
from six.moves import range
from dotmap import DotMap

from ctypes import string_at, c_double
from .ffi.ontology import CAmountOfMoneyValue, CTemperatureValue, CInstantTimeValue, CTimeIntervalValue, CDurationValue


class IntentMessage(object):
    def __init__(self, session_id, custom_data, site_id, input, intent, slots):
        """
        A python representation of the intent parsed by the NLU engine.

        :param session_id: Identifier of the dialogue session during which this intent was parsed.
        :param custom_data: Custom data passed by the Dialogue Manager in the current dialogue session.
        :param site_id: Site where the user interaction took place.
        :param input: The user input that has generated this intent.
        :param intent: Structured description of the intent classification.
        :param slots: Structured description of the detected slots for this intent if any.
        """
        self.session_id = session_id
        self.custom_data = custom_data
        self.site_id = site_id
        self.input = input
        self.intent = intent
        self.slots = slots

    @classmethod
    def from_c_repr(cls, c_repr):
        session_id = c_repr.session_id.decode('utf-8')
        custom_data = c_repr.custom_data.decode('utf-8') if c_repr.custom_data else None
        site_id = c_repr.site_id.decode('utf-8')
        input = c_repr.input.decode('utf-8')
        intent = IntentClassifierResult.from_c_repr(c_repr.intent.contents)
        slots = SlotMap.from_c_repr(c_repr.slots.contents)
        return cls(session_id, custom_data, site_id, input, intent, slots)

class IntentClassifierResult(object):
    def __init__(self, intent_name, probability):
        """
        Structured description of the intent classification.

        :param intent_name: name of the intent.
        :param probability: probability that the parsed sentence is the `intent_name` intent.
        """
        self.intent_name = intent_name
        self.probability = probability

    @classmethod
    def from_c_repr(cls, c_repr):
        intent_name = c_repr.intent_name.decode('utf-8')
        probability = c_repr.probability
        return cls(intent_name, probability)


class SlotMap(DotMap):
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


class SlotsList(list):  # An extension to make things easier to reach slot_values that are deeply nested in the IntentMessage datastructure.
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

class NluSlot(object):
    def __init__(self, confidence, slot_value, raw_value, entity, slot_name, range_start, range_end):
        self.confidence = confidence
        self.slot_value = slot_value
        self.raw_value = raw_value
        self.entity = entity
        self.slot_name = slot_name
        self.range_start = range_start
        self.range_end = range_end

    @classmethod
    def from_c_repr(cls, c_repr):
        confidence = c_repr.confidence
        slot = Slot.from_c_repr(c_repr.nlu_slot[0])

        slot_value = slot.slot_value  # To ensure compatibility, we flatten the data structure ...
        raw_value = slot.raw_value
        entity = slot.entity
        slot_name = slot.slot_name
        range_start = slot.range_start
        range_end = slot.range_end
        return cls(confidence, slot_value, raw_value, entity, slot_name, range_start, range_end)

class Slot(object):
    def __init__(self, slot_value, raw_value, entity, slot_name, range_start, range_end):
        """
        Deprecated.

        This is kept for compatibility reasons.
        Structured description of a detected slot.

        :param slot_value: an slotValue object that represents the value of the parsed slot.
        :param raw_value: the raw value of the slot, not parsed.
        :param entity:
        :param slot_name: name of the slot.
        :param range_start: index at which the slot begins.
        :param range_end: index at which the slot ends.
        """
        self.slot_value = slot_value
        self.raw_value = raw_value
        self.entity = entity
        self.slot_name = slot_name
        self.range_start = range_start
        self.range_end = range_end

    @classmethod
    def from_c_repr(cls, c_repr):
        slot_value = SlotValue.from_c_repr(c_repr.value)
        raw_value = c_repr.raw_value.decode('utf-8')
        entity = c_repr.entity.decode('utf-8')
        slot_name = c_repr.slot_name.decode('utf-8')
        range_start = c_repr.range_start
        range_end = c_repr.range_end

        return cls(slot_value, raw_value, entity, slot_name, range_start, range_end)


class SlotValue(object):
    def __init__(self, value_type, value):
        """
        A structured representation of values a slot can take.

        :param value_type: a constant that defines the type of the Slot Value between : Custom, Number, Ordinal, InstantTime, TimeInterval, AmountOfMoney, Temperature, Duration and Percentage.
        :param value: The parsed value according to the value_type of the slot.
        """
        self.value_type = value_type
        self.value = value

    @classmethod
    def from_c_repr(cls, c_repr):
        value_type = c_repr.value_type

        if 1 == value_type:  # CUSTOM
            c_repr_custom_value = c_repr.value
            string_value = string_at(c_repr_custom_value).decode('utf-8')
            value = CustomValue(string_value)
        elif 2 == value_type: # NUMBER
            c_repr_number = c_double.from_address(c_repr.value)
            number = c_repr_number.value
            value = NumberValue(number)
        elif 4 == value_type: # INSTANTTIME # TODO : Encoding here
            c_repr_instant_time_value = CInstantTimeValue.from_address(c_repr.value)
            value = InstantTimeValue.from_c_repr(c_repr_instant_time_value)
        elif 5 == value_type: # TIMEINTERVAL # TODO : Encoding here
            c_repr_time_interval_value = CTimeIntervalValue.from_address(c_repr.value)
            value = TimeIntervalValue.from_c_repr(c_repr_time_interval_value)
        elif 6 == value_type: # AMOUNTOFMONEY # TODO : Encoding
            c_repr_amount_of_money_value = CAmountOfMoneyValue.from_address(c_repr.value)
            value = AmountOfMoneyValue.from_c_repr(c_repr_amount_of_money_value)
        elif 7 == value_type: # TEMPERATURE # TODO : Encoding
            c_repr_temperature_value = CTemperatureValue.from_address(c_repr.value)
            value = TemperatureValue.from_c_repr(c_repr_temperature_value)
        elif 8 == value_type: # DURATION # TODO : Encoding
            c_repr_duration_value = CDurationValue.from_address(c_repr.value)
            value = DurationValue.from_c_repr(c_repr_duration_value)
        elif 9 == value_type: # PERCENTAGE
            c_repr_percentage = c_double.from_address(c_repr.value)
            value = PercentageValue(c_repr_percentage.value)
        elif 10 == value_type:  # MUSICARTIST  # I FORGOT TO DECODE TO UTF-8
            c_repr_music_artist_value = c_repr.value
            string_value = string_at(c_repr_music_artist_value).decode('utf-8')
            value = MusicArtistValue(string_value)
        elif 11 == value_type:  # MUSICALBUM
            c_repr_music_album_value = c_repr.value
            string_value = string_at(c_repr_music_album_value).decode('utf-8')
            value = MusicAlbumValue(string_value)
        elif 12 == value_type:  # MUSICTRACK
            c_repr_music_artist_value = c_repr.value
            string_value = string_at(c_repr_music_artist_value).decode('utf-8')
            value = MusicTrackValue(string_value)

        else:
            raise Exception("Bad value type. Got : {}".format(value_type))

        return cls(value_type, value)



class SessionStartedMessage(object):
    def __init__(self, session_id, custom_data, site_id, reactivated_from_session_id):
        """
        A message that the handler receives from the Dialogue Manager when a session is started.

        :param session_id: Session identifier that was started.
        :param custom_data: Custom data provided in the start session request on.
        :param site_id:  Site where the user interaction is taking place.
        :param reactivated_from_session_id: This field is left blank voluntarily.
        """
        self.session_id = session_id
        self.custom_data = custom_data
        self.site_id = site_id
        self.reactivated_from_session_id = reactivated_from_session_id

    @classmethod
    def from_c_repr(cls, c_repr):
        session_id = c_repr.session_id.decode('utf-8') if c_repr.session_id else None
        custom_data = c_repr.custom_data.decode('utf-8') if c_repr.custom_data else None
        site_id = c_repr.site_id.decode('utf-8') if c_repr.site_id else None
        reactivated_from_session_id = c_repr.reactivated_from_session_id.decode('utf-8') if c_repr.reactivated_from_session_id else None
        return cls(session_id, custom_data, site_id, reactivated_from_session_id)

    def __eq__(self, other):
        return self.__dict__ == other.__dict__


class SessionEndedMessage(object):
    def __init__(self, session_id, custom_data, site_id, termination):
        """
        A message that the handler receives from the Dialogue Manager when a session is ended.

        :param session_id: Session identifier that was started.
        :param custom_data: Custom data provided in the start session request on.
        :param site_id: Site where the user interaction is taking place.
        :param termination: Structured description of why the session has been ended.
        """
        self.session_id = session_id
        self.custom_data = custom_data
        self.site_id = site_id
        self.termination = termination

    @classmethod
    def from_c_repr(cls, c_repr):
        session_id = c_repr.session_id.decode('utf-8') if c_repr.session_id else None
        custom_data = c_repr.custom_data.decode('utf-8') if c_repr.custom_data else None
        site_id = c_repr.site_id.decode('utf-8') if c_repr.site_id else None
        termination = SessionTermination.from_c_repr(c_repr.termination)
        return cls(session_id, custom_data, site_id, termination)

    def __eq__(self, other):
        return self.__dict__ == other.__dict__

class SessionQueuedMessage(object):
    def __init__(self, session_id, custom_data, site_id):
        """
        A message that the handler receives from the Dialogue Manager when a session is queued.

        :param session_id: Session identifier that was started.
        :param custom_data: Custom data provided in the start session request on.
        :param site_id: Site where the user interaction is taking place
        """
        self.session_id = session_id
        self.custom_data = custom_data
        self.site_id = site_id

    @classmethod
    def from_c_repr(cls, c_repr):
        session_id = c_repr.session_id.decode('utf-8') if c_repr.session_id else None
        custom_data = c_repr.custom_data.decode('utf-8') if c_repr.custom_data else None
        site_id = c_repr.site_id.decode('utf-8') if c_repr.site_id else None
        return cls(session_id, custom_data, site_id)

    def __eq__(self, other):
        return self.__dict__ == other.__dict__


class SessionTermination(object):
    def __init__(self, termination_type, data):
        """

        :param termination_type:
        :param data: the reason why the session was ended
        """
        self.termination_type = termination_type
        self.data = data

    @classmethod
    def from_c_repr(cls, c_repr):
        termination_type = c_repr.termination_type
        data = c_repr.data.decode('utf-8') if c_repr.data else None
        return cls(termination_type, data)

    def __eq__(self, other):
        return self.__dict__ == other.__dict__

class CustomValue(object):
    def __init__(self, string_value):
        """
        A structured representation of Custom Value slot type.

        :param string_value: a string value
        """
        self.value = string_value


class NumberValue(object):
    def __init__(self, value):
        """
        A structured representation of Number Value slot type.

        :param value:
        """
        self.value = value



class AmountOfMoneyValue(object):
    def __init__(self, unit, value, precision):
        """
        A structured representation of a slot type that represents an amount of money.

        :param unit: monetary unit.
        :param value: the amount of money in unit.
        :param precision: numerical precision.
        """
        self.unit = unit
        self.value = value
        self.precision = precision

    @classmethod
    def from_c_repr(cls, c_repr):
        unit = c_repr.unit.decode('utf-8')
        value = c_repr.value
        precision = c_repr.precision

        return cls(unit, value, precision)


class TemperatureValue(object):
    def __init__(self, unit, value):
        """
        A structured representation of a slot type that represents a temperature.

        :param unit: unit used to measure the temperature.
        :param value: value expressed in unit unit.
        """
        self.unit = unit
        self.value = value

    @classmethod
    def from_c_repr(cls, c_repr):
        unit = c_repr.unit.decode('utf-8')
        value = c_repr.value
        return cls(unit, value)


class InstantTimeValue(object):
    def __init__(self, value, grain, precision):
        """
        A structured representation of a slot type that represents a date.

        :param value:
        :param grain:
        :param precision:
        """
        self.value = value
        self.grain = grain
        self.precision = precision

    @classmethod
    def from_c_repr(cls, c_repr):
        value = c_repr.value
        grain = c_repr.grain
        precision = c_repr.precision

        return cls(value, grain, precision)


class TimeIntervalValue(object):
    def __init__(self, from_date, to_date):
        """
        A structured representation of a slot type that represents a time interval.

        :param from_date: date at which starts the interval.
        :param to_date: date at which the interval ends.
        """
        self.from_date = from_date
        self.to_date = to_date

    @classmethod
    def from_c_repr(cls, c_repr):
        from_date = c_repr.from_date
        to_date = c_repr.to_date
        return cls(from_date, to_date)


class DurationValue(object):
    def __init__(self, years, quarters, months, weeks, days, hours, minutes, seconds, precision):
        """
        A structured representation of a slot type that represents a duration.

        :param years: number of years the duration lasts.
        :param quarters: number of quarters the duration lasts.
        :param months: number of months the duration lasts.
        :param weeks: number of weeks the duration lasts.
        :param days: number of days the duration lasts.
        :param hours: number of hours the duration lasts.
        :param minutes: number of minutes the duration lasts.
        :param seconds: number of seconds the duration lasts.
        :param precision:
        """
        self.years = years
        self.quarters = quarters
        self.months = months
        self.weeks = weeks
        self.days = days
        self.hours = hours
        self.minutes = minutes
        self.seconds = seconds
        self.precision = precision

    @classmethod
    def from_c_repr(cls, c_repr):
        years = c_repr.years
        quarters = c_repr.quarters
        months = c_repr.months
        weeks = c_repr.weeks
        days = c_repr.days
        hours = c_repr.hours
        minutes = c_repr.minutes
        seconds = c_repr.seconds
        precision = c_repr.precision
        return cls(years, quarters, months, weeks, days, hours, minutes, seconds, precision)


class PercentageValue(object):
    def __init__(self, value):
        """
        A structured representation of Percentage Value slot type.

        :param value:
        """
        self.value = value


class MusicArtistValue(object):
    def __init__(self, string_value):
        """
        A structured representation of Percentage Value slot type.
        A structured representation of Custom Value slot type.
        :param value:
       :param string_value: a string value
        """
        self.value = string_value


class MusicAlbumValue(object):
    def __init__(self, string_value):
        """
        A structured representation of Custom Value slot type.

        :param string_value: a string value
        """
        self.value = string_value


class MusicTrackValue(object):
    def __init__(self, string_value):
        """
        A structured representation of Custom Value slot type.

        :param string_value: a string value
        """
        self.value = string_value
