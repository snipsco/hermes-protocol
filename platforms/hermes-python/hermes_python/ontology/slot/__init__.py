# -*- coding: utf-8 -*-
from typing import Any, Text, List
from builtins import object
from six.moves import range

from ctypes import string_at, c_double, c_int64

from hermes_python.ffi.ontology import SlotValueType, Grain, Precision
from hermes_python.ffi.ontology import CAmountOfMoneyValue, CTemperatureValue, CInstantTimeValue, \
    CTimeIntervalValue, CDurationValue


class Slot(object):
    def __init__(self, slot_value, raw_value, alternatives, entity, slot_name, range_start, range_end,
                 confidence_score):
        # type: (SlotValue, str, List[SlotValue], str, str, int, int, float) -> None
        """
        Deprecated.

        This is kept for compatibility reasons.
        Structured description of a detected slot.

        :param slot_value: an slotValue object that represents the value of the parsed slot.
        :param raw_value: the raw value of the slot, not parsed.
        :param alternatives: alternatives slots
        :param entity:
        :param slot_name: name of the slot.
        :param range_start: index at which the slot begins.
        :param range_end: index at which the slot ends.
        :param confidence_score: between 0 and 1.
        """
        self.slot_value = slot_value
        self.raw_value = raw_value
        self.alternatives = alternatives
        self.entity = entity
        self.slot_name = slot_name
        self.range_start = range_start
        self.range_end = range_end
        self.confidence_score = confidence_score

    @classmethod
    def from_c_repr(cls, c_repr):
        slot_value = SlotValue.from_c_repr(c_repr.value.contents)
        raw_value = c_repr.raw_value.decode('utf-8')
        entity = c_repr.entity.decode('utf-8')
        slot_name = c_repr.slot_name.decode('utf-8')
        range_start = c_repr.range_start
        range_end = c_repr.range_end
        confidence_score = c_repr.confidence_score

        alternatives = list()

        if c_repr.alternatives:
            alternatives_length = c_repr.alternatives.contents.size
            c_alternatives_array_repr = c_repr.alternatives.contents.slot_values
            alternatives = [SlotValue.from_c_repr(c_alternatives_array_repr[i]) for i in range(alternatives_length)]

        return cls(slot_value, raw_value, alternatives, entity, slot_name, range_start, range_end, confidence_score)


class SlotValue(object):
    def __init__(self, value_type, value):
        # type: (int, Any) -> None
        """
        A structured representation of values a slot can take.

        :param value_type: a constant that defines the type of the Slot Value.
        :param value: The parsed value according to the value_type of the slot.
        """
        self.value_type = value_type
        self.value = value

    @classmethod
    def from_c_repr(cls, c_repr):
        value_type = c_repr.value_type

        if SlotValueType.CUSTOM == value_type:  # CUSTOM
            c_repr_custom_value = c_repr.value
            string_value = string_at(c_repr_custom_value).decode('utf-8')
            value = CustomValue(string_value)
        elif SlotValueType.NUMBER == value_type:  # NUMBER
            c_repr_number = c_double.from_address(c_repr.value)
            number = c_repr_number.value
            value = NumberValue(number)
        elif SlotValueType.ORDINAL == value_type:  # ORDINAL
            c_repr_number = c_int64.from_address(c_repr.value)
            number = c_repr_number.value
            value = OrdinalValue(number)
        elif SlotValueType.INSTANTTIME == value_type:  # INSTANTTIME
            c_repr_instant_time_value = CInstantTimeValue.from_address(c_repr.value)
            value = InstantTimeValue.from_c_repr(c_repr_instant_time_value)
        elif SlotValueType.TIMEINTERVAL == value_type:  # TIMEINTERVAL
            c_repr_time_interval_value = CTimeIntervalValue.from_address(c_repr.value)
            value = TimeIntervalValue.from_c_repr(c_repr_time_interval_value)
        elif SlotValueType.AMOUNTOFMONEY == value_type:  # AMOUNTOFMONEY
            c_repr_amount_of_money_value = CAmountOfMoneyValue.from_address(c_repr.value)
            value = AmountOfMoneyValue.from_c_repr(c_repr_amount_of_money_value)
        elif SlotValueType.TEMPERATURE == value_type:  # TEMPERATURE
            c_repr_temperature_value = CTemperatureValue.from_address(c_repr.value)
            value = TemperatureValue.from_c_repr(c_repr_temperature_value)
        elif SlotValueType.DURATION == value_type:  # DURATION
            c_repr_duration_value = CDurationValue.from_address(c_repr.value)
            value = DurationValue.from_c_repr(c_repr_duration_value)
        elif SlotValueType.PERCENTAGE == value_type:  # PERCENTAGE
            c_repr_percentage = c_double.from_address(c_repr.value)
            value = PercentageValue(c_repr_percentage.value)
        elif SlotValueType.MUSICARTIST == value_type:  # MUSICARTIST
            c_repr_music_artist_value = c_repr.value
            string_value = string_at(c_repr_music_artist_value).decode('utf-8')
            value = MusicArtistValue(string_value)
        elif SlotValueType.MUSICALBUM == value_type:  # MUSICALBUM
            c_repr_music_album_value = c_repr.value
            string_value = string_at(c_repr_music_album_value).decode('utf-8')
            value = MusicAlbumValue(string_value)
        elif SlotValueType.MUSICTRACK == value_type:  # MUSICTRACK
            c_repr_music_track_value = c_repr.value
            string_value = string_at(c_repr_music_track_value).decode('utf-8')
            value = MusicTrackValue(string_value)
        elif SlotValueType.CITY == value_type:  # CITY
            c_repr_city_value = c_repr.value
            string_value = string_at(c_repr_city_value).decode('utf-8')
            value = CityValue(string_value)
        elif SlotValueType.COUNTRY == value_type:  # COUNTRY
            c_repr_country_value = c_repr.value
            string_value = string_at(c_repr_country_value).decode('utf-8')
            value = CountryValue(string_value)
        elif SlotValueType.REGION == value_type:  # REGION
            c_repr_regoin_value = c_repr.value
            string_value = string_at(c_repr_regoin_value).decode('utf-8')
            value = RegionValue(string_value)

        else:
            raise Exception("Bad value type. Got : {}".format(value_type))

        return cls(value_type, value)


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


class OrdinalValue(object):
    def __init__(self, value):
        """
        A structured representation of Ordinal Value slot type.

        :param value: an integer value.
        """
        self.value = value


class AmountOfMoneyValue(object):
    def __init__(self, unit, value, precision):
        # type: (Text, Any, Precision) -> None
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
        unit = c_repr.unit.decode('utf-8') if c_repr.unit else None
        value = c_repr.value
        precision = Precision(c_repr.precision)

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
        unit = c_repr.unit.decode('utf-8') if c_repr.unit else None
        value = c_repr.value
        return cls(unit, value)


class InstantTimeValue(object):
    def __init__(self, value, grain, precision):
        # type: (Any, Grain, Precision) -> None
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
        value = c_repr.value.decode('utf-8')
        grain = Grain(c_repr.grain)
        precision = Precision(c_repr.precision)

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
        from_date = c_repr.from_date.decode('utf-8') if c_repr.from_date else None
        to_date = c_repr.to_date.decode('utf-8') if c_repr.to_date else None
        return cls(from_date, to_date)


class DurationValue(object):
    def __init__(self, years, quarters, months, weeks, days, hours, minutes, seconds, precision):
        # type: (int, int, int, int, int, int, int, int, Precision ) -> None
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
        precision = Precision(c_repr.precision)
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


class CityValue(object):
    def __init__(self, string_value):
        """
        A structured representation of City Value slot type.

        :param string_value: a string value
        """
        self.value = string_value


class CountryValue(object):
    def __init__(self, string_value):
        """
        A structured representation of Country Value slot type.

        :param string_value: a string value
        """
        self.value = string_value


class RegionValue(object):
    def __init__(self, string_value):
        """
        A structured representation of Region Value slot type.

        :param string_value: a string value
        """
        self.value = string_value
