# coding: utf-8

from __future__ import absolute_import
from __future__ import unicode_literals

from ctypes import cdll, CFUNCTYPE, c_void_p, c_char_p, POINTER, pointer, string_at
import os
from glob import glob
import sys
import pytest

import hermes_python

from hermes_python.ontology.nlu import NluSlot, SlotMap, NluIntentAlternative
from hermes_python.ontology.dialogue import IntentMessage, IntentClassifierResult
from hermes_python.ontology.slot import SlotValue, CustomValue
from hermes_python.ontology.asr import AsrDecodingDuration, AsrToken

from hermes_python.ffi.ontology import CMapStringToStringArray
from hermes_python.ffi.ontology.asr import CAsrDecodingDuration, CAsrToken
from hermes_python.ffi.ontology.dialogue import CSessionQueuedMessage, CSessionStartedMessage, CSessionEndedMessage, \
    CIntentNotRecognizedMessage, CContinueSessionMessage, CStartSessionMessageNotification, CStartSessionMessageAction, \
    CEndSessionMessage, CDialogueConfigureMessage

from hermes_python.ontology.injection import InjectionRequestMessage, AddInjectionRequest, \
    AddFromVanillaInjectionRequest
from hermes_python.ffi.ontology.injection import CInjectionRequestMessage

from hermes_python.ontology.tts import RegisterSoundMessage
from hermes_python.ffi.ontology.tts import CRegisterSoundMessage

from ..ffi.test_ontology import wav_data

DYLIB_NAME = "libhermes_ffi_test" + (".dylib" if sys.platform == "darwin" else ".so")
DYLIB_DIR = os.path.join(os.path.dirname(__file__), "./debug")
DYLIB_PATH = glob(os.path.join(DYLIB_DIR, DYLIB_NAME))[0]

lib = cdll.LoadLibrary(DYLIB_PATH)


class LibException(Exception):
    pass


class MapStringToStringArray(object):  # This is just a helper class, just used in roundtrip tests.
    @classmethod
    def from_c_repr(cls, c_repr):
        return c_repr.into_repr()


def dispatch_to_ffi_function(pointer_to_c_struct, c_struct_type):
    if c_struct_type is CSessionQueuedMessage:
        print(c_struct_type)
        return lib.hermes_drop_session_queued_message(pointer_to_c_struct)
    elif c_struct_type is CSessionStartedMessage:
        print(c_struct_type)
        return lib.hermes_drop_session_started_message(pointer_to_c_struct)
    elif c_struct_type is CSessionEndedMessage:
        print(c_struct_type)
        return lib.hermes_drop_session_ended_message(pointer_to_c_struct)
    elif c_struct_type is CIntentNotRecognizedMessage:
        type(c_struct_type)
        return lib.hermes_drop_intent_not_recognized_message(pointer_to_c_struct)
    elif c_struct_type is CContinueSessionMessage:
        type(c_struct_type)
        return lib.hermes_drop_continue_session_message(pointer_to_c_struct)
    elif c_struct_type is CStartSessionMessageNotification:
        type(c_struct_type)
        return lib.hermes_drop_start_session_message(pointer_to_c_struct)
    elif c_struct_type is CStartSessionMessageAction:
        type(c_struct_type)
        return lib.hermes_drop_start_session_message(pointer_to_c_struct)
    elif c_struct_type is CEndSessionMessage:
        type(c_struct_type)
        return lib.hermes_drop_end_session_message(pointer_to_c_struct)
    elif c_struct_type is CDialogueConfigureMessage:
        type(c_struct_type)
        return lib.hermes_drop_dialogue_configure_message(pointer_to_c_struct)
    else:
        raise Exception("Cannot drop struct of type {}".format(c_struct_type))


def drop_structure(ptr_to_c_struct, C_struct_type):
    destruction = dispatch_to_ffi_function(ptr_to_c_struct, C_struct_type)
    if destruction > 0:
        wrap_c_error()


def wrap_c_error():
    error_p = POINTER(c_char_p)(c_char_p("".encode('utf-8')))
    lib.hermes_ffi_test_get_last_error(error_p)
    raise LibException(string_at(error_p.contents).decode('utf-8'))


def get_round_trip_data_structure(py_ontology_object_instance, C_Ontology_Type, Python_Ontology_Class,
                                  round_trip_function):
    c_repr_object = C_Ontology_Type.from_repr(py_ontology_object_instance)
    pointer_c_repr = pointer(c_repr_object)
    output_pointer = pointer(c_void_p())

    # Send it for round trip to Rust
    result = round_trip_function(pointer_c_repr, output_pointer)
    if result > 0:
        wrap_c_error()

    # Deserialize Rust result into C representation
    round_trip_c_repr_object = C_Ontology_Type.from_address(output_pointer.contents.value)

    # Deserialize into Python object
    round_trip_python_ontology_object = Python_Ontology_Class.from_c_repr(round_trip_c_repr_object)
    return round_trip_python_ontology_object


def test_hermes_ffi_test_round_trip_session_queued():
    # Initialize deserialized python object
    session_queued_message = hermes_python.ontology.dialogue.SessionQueuedMessage("session_id", "custom_daté",
                                                                                  "site_id")

    round_trip_session_queued_message = get_round_trip_data_structure(
        session_queued_message,
        hermes_python.ffi.ontology.dialogue.CSessionQueuedMessage,
        hermes_python.ontology.dialogue.SessionQueuedMessage,
        lib.hermes_ffi_test_round_trip_session_queued
    )

    assert session_queued_message == round_trip_session_queued_message


def test_hermes_ffi_test_round_trip_session_started():
    session_started_message = hermes_python.ontology.dialogue.SessionStartedMessage("session_id", "custom_data",
                                                                                    "site_id", "reactivated")
    round_trip_session_started_message = get_round_trip_data_structure(
        session_started_message,
        hermes_python.ffi.ontology.dialogue.CSessionStartedMessage,
        hermes_python.ontology.dialogue.SessionStartedMessage,
        lib.hermes_ffi_test_round_trip_session_started
    )

    assert session_started_message == round_trip_session_started_message


class TestSessionEndedRoundTrip(object):
    def test_hermes_ffi_test_round_trip_session_ended_for_error(self):
        session_termination_type_error = hermes_python.ontology.dialogue.SessionTerminationTypeError("dataé")
        session_termination = hermes_python.ontology.dialogue.SessionTermination(session_termination_type_error,
                                                                                 "dataé")
        session_ended_message = hermes_python.ontology.dialogue.SessionEndedMessage("session_id", "custom_data",
                                                                                    "site_id",
                                                                                    session_termination)
        round_trip_session_ended_message = get_round_trip_data_structure(
            session_ended_message,
            hermes_python.ffi.ontology.dialogue.CSessionEndedMessage,
            hermes_python.ontology.dialogue.SessionEndedMessage,
            lib.hermes_ffi_test_round_trip_session_ended
        )

        assert session_ended_message.termination.data == round_trip_session_ended_message.termination.data
        assert session_ended_message.termination == round_trip_session_ended_message.termination
        assert session_ended_message == round_trip_session_ended_message

    def test_hermes_ffi_test_round_trip_session_ended(self):
        session_termination_type_nominal = hermes_python.ontology.dialogue.SessionTerminationTypeNominal()
        session_termination = hermes_python.ontology.dialogue.SessionTermination(session_termination_type_nominal,
                                                                                 None)
        session_ended_message = hermes_python.ontology.dialogue.SessionEndedMessage("session_id", "custom_data",
                                                                                    "site_id",
                                                                                    session_termination)
        round_trip_session_ended_message = get_round_trip_data_structure(
            session_ended_message,
            hermes_python.ffi.ontology.dialogue.CSessionEndedMessage,
            hermes_python.ontology.dialogue.SessionEndedMessage,
            lib.hermes_ffi_test_round_trip_session_ended
        )

        assert session_ended_message.termination.data == round_trip_session_ended_message.termination.data
        assert session_ended_message.termination == round_trip_session_ended_message.termination
        assert session_ended_message == round_trip_session_ended_message


class TestIntentNotRecognized(object):

    def test_hermes_ffi_test_round_trip_intent_not_recognized_1(self):
        intent_not_recognized_message = hermes_python.ontology.dialogue.IntentNotRecognizedMessage("site_id", "session_id",
                                                                                                   "input", "custom_data",
                                                                                                   0.5, [])
        round_trip_intent_not_recognized_message = get_round_trip_data_structure(
            intent_not_recognized_message,
            hermes_python.ffi.ontology.dialogue.CIntentNotRecognizedMessage,
            hermes_python.ontology.dialogue.IntentNotRecognizedMessage,
            lib.hermes_ffi_test_round_trip_intent_not_recognized
        )

        assert intent_not_recognized_message == round_trip_intent_not_recognized_message

    @pytest.mark.skip(reason="skipped until slotmap equality resolved.")
    def test_hermes_ffi_test_round_trip_intent_not_recognized_2(self):
        slot_value = SlotValue(1, CustomValue("hello 🎁"))
        nlu_slot = NluSlot(slot_value, "raw_value", [slot_value, slot_value], "entity", "slotName", 0, 2, 0.2)
        slot_map = SlotMap({"slotName": [nlu_slot]})

        intent_alternative = hermes_python.ontology.nlu.NluIntentAlternative("intent1", .2, slot_map)

        intent_not_recognized_message = hermes_python.ontology.dialogue.IntentNotRecognizedMessage("site_id", "session_id",
                                                                                                       "input", "custom_data",
                                                                                                       0.5, [intent_alternative])
        round_trip_intent_not_recognized_message = get_round_trip_data_structure(
            intent_not_recognized_message,
            hermes_python.ffi.ontology.dialogue.CIntentNotRecognizedMessage,
            hermes_python.ontology.dialogue.IntentNotRecognizedMessage,
            lib.hermes_ffi_test_round_trip_intent_not_recognized
        )

        assert intent_not_recognized_message == round_trip_intent_not_recognized_message


class TestContinueSessionRoundTrip(object):
    def test_hermes_ffi_test_round_trip_continue_session(self):
        continue_session_message = hermes_python.ontology.dialogue.ContinueSessionMessage("session_id",
                                                                                          "this is a text",
                                                                                          ["test"], "custom_data", True)
        round_trip_continue_session_message = get_round_trip_data_structure(
            continue_session_message,
            hermes_python.ffi.ontology.dialogue.CContinueSessionMessage,
            hermes_python.ontology.dialogue.ContinueSessionMessage,
            lib.hermes_ffi_test_round_trip_continue_session
        )

        assert continue_session_message == round_trip_continue_session_message

    def test_hermes_ffi_test_round_trip_continue_session_2(self):
        continue_session_message = hermes_python.ontology.dialogue.ContinueSessionMessage(
            "session_id",
            "The text that will be said out loud",
            [],
            None,
            False)

        round_trip_continue_session_message = get_round_trip_data_structure(
            continue_session_message,
            hermes_python.ffi.ontology.dialogue.CContinueSessionMessage,
            hermes_python.ontology.dialogue.ContinueSessionMessage,
            lib.hermes_ffi_test_round_trip_continue_session
        )

        assert continue_session_message == round_trip_continue_session_message


class TestStartSessionNoticationRoundtrip(object):
    def test_hermes_ffi_test_round_trip_start_session_notification_1(self):
        session_init = hermes_python.ontology.dialogue.SessionInitNotification("testing")

        start_session_message = hermes_python.ontology.dialogue.StartSessionMessage(session_init, None, None)

        round_trip_start_session_message = get_round_trip_data_structure(
            start_session_message,
            hermes_python.ffi.ontology.dialogue.CStartSessionMessageNotification,
            hermes_python.ontology.dialogue.StartSessionMessage,
            lib.hermes_ffi_test_round_trip_start_session
        )

        assert start_session_message == round_trip_start_session_message

    def test_hermes_ffi_test_round_trip_start_session_notification_2(self):
        session_init = hermes_python.ontology.dialogue.SessionInitNotification()

        start_session_message = hermes_python.ontology.dialogue.StartSessionMessage(session_init, None, None)

        round_trip_start_session_message = get_round_trip_data_structure(
            start_session_message,
            hermes_python.ffi.ontology.dialogue.CStartSessionMessageNotification,
            hermes_python.ontology.dialogue.StartSessionMessage,
            lib.hermes_ffi_test_round_trip_start_session
        )

        assert start_session_message == round_trip_start_session_message

    def test_hermes_ffi_test_round_trip_start_session_notification_3(self):
        session_init = hermes_python.ontology.dialogue.SessionInitNotification()
        custom_data = "blabla"

        start_session_message = hermes_python.ontology.dialogue.StartSessionMessage(session_init, custom_data, None)

        round_trip_start_session_message = get_round_trip_data_structure(
            start_session_message,
            hermes_python.ffi.ontology.dialogue.CStartSessionMessageNotification,
            hermes_python.ontology.dialogue.StartSessionMessage,
            lib.hermes_ffi_test_round_trip_start_session
        )

        assert start_session_message == round_trip_start_session_message
        assert round_trip_start_session_message.custom_data == "blabla"

    def test_hermes_ffi_test_round_trip_start_session_notification_4(self):
        session_init = hermes_python.ontology.dialogue.SessionInitNotification()
        site_id = "room"

        start_session_message = hermes_python.ontology.dialogue.StartSessionMessage(session_init, None, site_id)

        round_trip_start_session_message = get_round_trip_data_structure(
            start_session_message,
            hermes_python.ffi.ontology.dialogue.CStartSessionMessageNotification,
            hermes_python.ontology.dialogue.StartSessionMessage,
            lib.hermes_ffi_test_round_trip_start_session
        )

        assert start_session_message == round_trip_start_session_message


class TestStartSessionActionRoundtrip(object):
    def test_hermes_ffi_test_round_trip_start_session_action_1(self):
        session_init = hermes_python.ontology.dialogue.SessionInitAction("testing")
        start_session_message = hermes_python.ontology.dialogue.StartSessionMessage(session_init, None, None)

        round_trip_start_session_message = get_round_trip_data_structure(
            start_session_message,
            hermes_python.ffi.ontology.dialogue.CStartSessionMessageAction,
            hermes_python.ontology.dialogue.StartSessionMessage,
            lib.hermes_ffi_test_round_trip_start_session
        )

        assert start_session_message == round_trip_start_session_message

    def test_hermes_ffi_test_round_trip_start_session_action_2(self):
        session_init = hermes_python.ontology.dialogue.SessionInitAction(intent_filter=["intent1"])
        start_session_message = hermes_python.ontology.dialogue.StartSessionMessage(session_init, None, None)

        round_trip_start_session_message = get_round_trip_data_structure(
            start_session_message,
            hermes_python.ffi.ontology.dialogue.CStartSessionMessageAction,
            hermes_python.ontology.dialogue.StartSessionMessage,
            lib.hermes_ffi_test_round_trip_start_session
        )

        assert start_session_message == round_trip_start_session_message

    def test_hermes_ffi_test_round_trip_start_session_action_3(self):
        session_init = hermes_python.ontology.dialogue.SessionInitAction(can_be_enqueued=False)
        start_session_message = hermes_python.ontology.dialogue.StartSessionMessage(session_init, None, None)

        round_trip_start_session_message = get_round_trip_data_structure(
            start_session_message,
            hermes_python.ffi.ontology.dialogue.CStartSessionMessageAction,
            hermes_python.ontology.dialogue.StartSessionMessage,
            lib.hermes_ffi_test_round_trip_start_session
        )

        assert start_session_message == round_trip_start_session_message

    def test_hermes_ffi_test_round_trip_start_session_action_4(self):
        session_init = hermes_python.ontology.dialogue.SessionInitAction(send_intent_not_recognized=True)
        start_session_message = hermes_python.ontology.dialogue.StartSessionMessage(session_init, None, None)

        round_trip_start_session_message = get_round_trip_data_structure(
            start_session_message,
            hermes_python.ffi.ontology.dialogue.CStartSessionMessageAction,
            hermes_python.ontology.dialogue.StartSessionMessage,
            lib.hermes_ffi_test_round_trip_start_session
        )

        assert start_session_message == round_trip_start_session_message
        assert start_session_message.init.send_intent_not_recognized == round_trip_start_session_message.init.send_intent_not_recognized


class TestStartEndSessionRoundtrip(object):
    def test_hermes_ffi_test_round_trip_end_session_1(self):
        end_session_message = hermes_python.ontology.dialogue.EndSessionMessage("session_id")
        round_trip_end_session_message = get_round_trip_data_structure(
            end_session_message,
            hermes_python.ffi.ontology.dialogue.CEndSessionMessage,
            hermes_python.ontology.dialogue.EndSessionMessage,
            lib.hermes_ffi_test_round_trip_end_session
        )

        assert end_session_message == round_trip_end_session_message

    def test_hermes_ffi_test_round_trip_end_session_2(self):
        end_session_message = hermes_python.ontology.dialogue.EndSessionMessage("session_id", "hello there 🤗")
        round_trip_end_session_message = get_round_trip_data_structure(
            end_session_message,
            hermes_python.ffi.ontology.dialogue.CEndSessionMessage,
            hermes_python.ontology.dialogue.EndSessionMessage,
            lib.hermes_ffi_test_round_trip_end_session
        )

        assert end_session_message == round_trip_end_session_message


class TestDialogueConfigureRoundTrip(object):
    def test_hermes_ffi_test_round_trip_dialogue_configure(self):
        intent1 = hermes_python.ontology.dialogue.DialogueConfigureIntent("intent1", True)
        intent2 = hermes_python.ontology.dialogue.DialogueConfigureIntent("intent1", True)
        intent3 = hermes_python.ontology.dialogue.DialogueConfigureIntent("intent1", True)
        dialogue_configure = \
            hermes_python.ontology.dialogue.DialogueConfigureMessage("kitchen", [intent1, intent2, intent3])

        round_trip_dialogue_configure = get_round_trip_data_structure(
            dialogue_configure,
            hermes_python.ffi.ontology.dialogue.CDialogueConfigureMessage,
            hermes_python.ontology.dialogue.DialogueConfigureMessage,
            lib.hermes_ffi_test_round_trip_dialogue_configure
        )

        assert dialogue_configure == round_trip_dialogue_configure

    def test_hermes_ffi_test_round_trip_dialogue_configure_default_site_id(self):
        intent1 = hermes_python.ontology.dialogue.DialogueConfigureIntent("intent1", True)
        intent2 = hermes_python.ontology.dialogue.DialogueConfigureIntent("intent1", True)
        intent3 = hermes_python.ontology.dialogue.DialogueConfigureIntent("intent1", True)
        dialogue_configure = \
            hermes_python.ontology.dialogue.DialogueConfigureMessage(None, [intent1, intent2, intent3])

        round_trip_dialogue_configure = get_round_trip_data_structure(
            dialogue_configure,
            hermes_python.ffi.ontology.dialogue.CDialogueConfigureMessage,
            hermes_python.ontology.dialogue.DialogueConfigureMessage,
            lib.hermes_ffi_test_round_trip_dialogue_configure
        )

        assert dialogue_configure == round_trip_dialogue_configure


def test_hermes_ffi_test_round_trip_dialogue_configure_intent():
    dialogue_configure_intent = hermes_python.ontology.dialogue.DialogueConfigureIntent("intent1", True)
    round_trip_dialogue_configure_intent = get_round_trip_data_structure(
        dialogue_configure_intent,
        hermes_python.ffi.ontology.dialogue.CDialogueConfigureIntent,
        hermes_python.ontology.dialogue.DialogueConfigureIntent,
        lib.hermes_ffi_test_round_trip_dialogue_configure_intent
    )

    assert dialogue_configure_intent == round_trip_dialogue_configure_intent


def test_hermes_ffi_test_round_trip_dialogue_configure_intent_array():
    intent_1 = hermes_python.ontology.dialogue.DialogueConfigureIntent("intent1", True)
    intent_2 = hermes_python.ontology.dialogue.DialogueConfigureIntent("intent2", True)

    dialogue_configure_intent_array = hermes_python.ontology.dialogue.DialogueConfigureIntentArray([intent_1, intent_2])
    assert len(dialogue_configure_intent_array) == 2

    round_trip_dialogue_configure_intent_array = get_round_trip_data_structure(
        dialogue_configure_intent_array,
        hermes_python.ffi.ontology.dialogue.CDialogueConfigureIntentArray,
        hermes_python.ontology.dialogue.DialogueConfigureIntentArray,
        lib.hermes_ffi_test_round_trip_dialogue_configure_intent_array
    )

    assert dialogue_configure_intent_array == round_trip_dialogue_configure_intent_array


class TestInjection(object):
    def test_injection_request_message_roundtrip(self):
        input_request_1 = AddInjectionRequest({"key": ["hello", "world", "✨"]})
        input_request_2 = AddFromVanillaInjectionRequest({"key": ["hello", "moon", "👽"]})
        operations = [input_request_1, input_request_2]
        lexicon = {"key": ["i", "am a", "lexicon ⚠️"]}
        injection_request = InjectionRequestMessage(operations, lexicon)

        round_trip_injection_request = get_round_trip_data_structure(
            injection_request,
            hermes_python.ffi.ontology.injection.CInjectionRequestMessage,
            hermes_python.ontology.injection.InjectionRequestMessage,
            lib.hermes_ffi_test_round_trip_injection_request
        )

        assert injection_request == round_trip_injection_request

    def test_injection_complete_message_roundtrip(self):
        injection_complete = hermes_python.ontology.injection.InjectionCompleteMessage("request_id")

        round_trip_injection_complete_message = get_round_trip_data_structure(
            injection_complete,
            hermes_python.ffi.ontology.injection.CInjectionCompleteMessage,
            hermes_python.ontology.injection.InjectionCompleteMessage,
            lib.hermes_ffi_test_round_trip_injection_complete
        )

        assert injection_complete == round_trip_injection_complete_message

    def test_injection_reset_request_message_roundtrip(self):
        injection_reset___message = hermes_python.ontology.injection.InjectionResetRequestMessage("request_id")

        round_trip_injection_reset__message = get_round_trip_data_structure(
            injection_reset___message,
            hermes_python.ffi.ontology.injection.CInjectionResetRequestMessage,
            hermes_python.ontology.injection.InjectionResetRequestMessage,
            lib.hermes_ffi_test_round_trip_injection_reset_request
        )

        assert injection_reset___message == round_trip_injection_reset__message

    def test_injection_reset_request_message_roundtrip(self):
        injection_reset___message = hermes_python.ontology.injection.InjectionResetCompleteMessage("request_id")

        round_trip_injection_reset__message = get_round_trip_data_structure(
            injection_reset___message,
            hermes_python.ffi.ontology.injection.CInjectionResetCompleteMessage,
            hermes_python.ontology.injection.InjectionResetCompleteMessage,
            lib.hermes_ffi_test_round_trip_injection_reset_complete
        )

        assert injection_reset___message == round_trip_injection_reset__message


class TestMapStringToStringArray(object):
    def test_basic(self):
        d = {"key1": ["value1", "value2"], "key2": ["👽", "🛸", "🌍"]}
        round_trip_d = get_round_trip_data_structure(
            d,
            CMapStringToStringArray,
            MapStringToStringArray,  # <- This class is just used for roundtrips.
            lib.hermes_ffi_test_round_trip_map_string_to_string_array
        )

        assert d == round_trip_d


def test_tts_register_sound_message_roundtrip(wav_data):
    register_sound = RegisterSoundMessage("yolo.wav", wav_data)

    round_trip_register_sound = get_round_trip_data_structure(
        register_sound,
        CRegisterSoundMessage,
        RegisterSoundMessage,
        lib.hermes_ffi_test_round_trip_register_sound
    )

    assert round_trip_register_sound == register_sound


class TestNluSlotArray(object):
    def test_hermes_ffi_test_round_trip_nlu_slot_array_empty(self):
        slot_map = SlotMap(dict())

        round_trip_slot_map = get_round_trip_data_structure(
            slot_map,
            hermes_python.ffi.ontology.nlu.CNluSlotArray,
            SlotMap,
            lib.hermes_ffi_test_round_trip_nlu_slot_array
        )

        assert slot_map.items() == round_trip_slot_map.items()

    def test_hermes_ffi_test_round_trip_nlu_slot_array_with_slots(self):
        slot_value = SlotValue(1, CustomValue("hello 🎁"))
        nlu_slot = NluSlot(slot_value, "raw_value", [slot_value, slot_value], "entity", "slotName", 0, 2, 0.2)
        slot_map = SlotMap({"slotName": [nlu_slot]})

        round_trip_slot_map = get_round_trip_data_structure(
            slot_map,
            hermes_python.ffi.ontology.nlu.CNluSlotArray,
            SlotMap,
            lib.hermes_ffi_test_round_trip_nlu_slot_array
        )

        assert slot_map.slotName.first().value == round_trip_slot_map.slotName.first().value


class TestIntentAlternative(object):
    def test_hermes_ffi_test_round_trip_intent_alternative(self):
        intent_alternative = hermes_python.ontology.nlu.NluIntentAlternative("intent_name", float(0.2), SlotMap(dict()))

        round_trip_intent_alternative = get_round_trip_data_structure(
            intent_alternative,
            hermes_python.ffi.ontology.nlu.CNluIntentAlternative,
            hermes_python.ontology.nlu.NluIntentAlternative,
            lib.hermes_ffi_test_round_trip_nlu_intent_alternative
        )

        assert intent_alternative.intent_name == round_trip_intent_alternative.intent_name
        assert len(intent_alternative.slots) == len(round_trip_intent_alternative.slots)

    def test_hermes_ffi_test_round_trip_intent_alternative_with_slots(self):
        first_slot_value = SlotValue(1, CustomValue("hello 🎁"))
        second_slot_value = SlotValue(1, CustomValue("hell o 🎁"))
        third_slot_value = SlotValue(1, CustomValue("hel o 🎁"))

        nlu_slot = NluSlot(first_slot_value, "raw_value", [second_slot_value, third_slot_value], "entity", "searchWeather", 0, 2, .2)

        slotMap = SlotMap({"searchWeather": [nlu_slot]})

        intent_alternative = hermes_python.ontology.nlu.NluIntentAlternative("intent_name", float(0.2), slotMap)

        round_trip_intent_alternative = get_round_trip_data_structure(
            intent_alternative,
            hermes_python.ffi.ontology.nlu.CNluIntentAlternative,
            hermes_python.ontology.nlu.NluIntentAlternative,
            lib.hermes_ffi_test_round_trip_nlu_intent_alternative
        )
        assert intent_alternative.intent_name == round_trip_intent_alternative.intent_name
        assert len(intent_alternative.slots) == len(round_trip_intent_alternative.slots)


class TestIntentAlternativeArray(object):
    def test_hermes_ffi_test_round_trip_nlu_intent_alternative_array(self):

        slot_value = SlotValue(1, CustomValue("hello 🎁"))
        nlu_slot = NluSlot(slot_value, "raw_value", [slot_value, slot_value], "entity", "slotName", 0, 2, 0.2)
        slot_map_1 = SlotMap({"slotName": [nlu_slot]})

        slot_map_2 = SlotMap({})

        nlu_intent_alternative_1 = hermes_python.ontology.nlu.NluIntentAlternative("intent1", .2, slot_map_1)
        nlu_intent_alternative_2 = hermes_python.ontology.nlu.NluIntentAlternative("intent2", .3, slot_map_2)

        intent_alternatives = [nlu_intent_alternative_1, nlu_intent_alternative_2]

        c_repr_object = hermes_python.ffi.ontology.nlu.CNluIntentAlternativeArray.from_repr(intent_alternatives)
        pointer_c_repr = pointer(c_repr_object)
        output_pointer = pointer(c_void_p())

        # Send it for round trip to Rust
        result = lib.hermes_ffi_test_round_trip_nlu_intent_alternative_array(pointer_c_repr, output_pointer)
        if result > 0:
            wrap_c_error()

        # Deserialize Rust result into C representation
        round_trip_c_repr_object = hermes_python.ffi.ontology.nlu.CNluIntentAlternativeArray.from_address(output_pointer.contents.value)

        # Deserialize into Python object
        round_trip_python_ontology_object = [NluIntentAlternative.from_c_repr(round_trip_c_repr_object.entries[i].contents) for i in range(round_trip_c_repr_object.count)]

        assert len(round_trip_python_ontology_object) == len(intent_alternatives)


class TestIntent(object):
    @staticmethod
    def get_slot_map():
        slot_value_1 = SlotValue(1, CustomValue("hello 🎁"))
        slot_value_2 = SlotValue(1, CustomValue("helloo 🎁"))
        slot_value_3 = SlotValue(1, CustomValue("hellooo 🎁"))
        search_weather_nlu = NluSlot(slot_value_1, "hello", [slot_value_2, slot_value_3], "entity", "searchWeather", 0, 2, 0.2)
        return SlotMap({"searchWeather": [search_weather_nlu]})

    @staticmethod
    def get_intent_alternatives():
        slot_map_1 = TestIntent.get_slot_map()
        slot_map_2 = TestIntent.get_slot_map()
        nlu_intent_alternative_1 = hermes_python.ontology.nlu.NluIntentAlternative("intent1", .2, slot_map_1)
        nlu_intent_alternative_2 = hermes_python.ontology.nlu.NluIntentAlternative("intent2", .3, slot_map_2)

        return [nlu_intent_alternative_1, nlu_intent_alternative_2]


    @staticmethod
    def get_asr_tokens_matrix():
        decoding_duration_1 = AsrDecodingDuration(.2, .4)
        asr_token_1 = AsrToken("hello", .1, 0, 1, decoding_duration_1)

        decoding_duration_2 = AsrDecodingDuration(.2, .4)
        asr_token_2 = AsrToken("my", .1, 0, 1, decoding_duration_2)

        decoding_duration_3 = AsrDecodingDuration(.2, .4)
        asr_token_3 = AsrToken("friend", .1, 0, 1, decoding_duration_3)

        asr_tokens_1 = [asr_token_1, asr_token_2, asr_token_3]
        asr_tokens_2 = [asr_token_3, asr_token_2, asr_token_1]

        return [asr_tokens_1, asr_tokens_2]

    def test_hermes_ffi_test_round_trip_intent(self):
        slot_map = TestIntent.get_slot_map()
        intent_classifier_result = IntentClassifierResult("searchWeather", 0.2)
        alternatives = TestIntent.get_intent_alternatives()
        asr_tokens = TestIntent.get_asr_tokens_matrix()

        intent_message = IntentMessage(
            "session_id", "custom_data", "site_id", "input",
            intent_classifier_result,
            slot_map,
            alternatives,
            asr_tokens,
            .1)

        round_trip_intent_message = get_round_trip_data_structure(
            intent_message,
            hermes_python.ffi.ontology.dialogue.CIntentMessage,
            hermes_python.ontology.dialogue.IntentMessage,
            lib.hermes_ffi_test_round_trip_intent
        )

        assert round_trip_intent_message.input == intent_message.input
        assert round_trip_intent_message.input == intent_message.input
        assert round_trip_intent_message.input == intent_message.input


class TestAsrTokens(object):

    @staticmethod
    def get_asr_tokens_from_c_asr_token_array(c_asr_token_array):
        return [AsrToken.from_c_repr(c_asr_token_array.entries[i].contents) for i in range(c_asr_token_array.count)]

    def test_hermes_ffi_test_round_trip_asr_token(self):
        decoding_duration = AsrDecodingDuration(.2, .4)
        asr_token = AsrToken("hello", .1, 0, 1, decoding_duration)

        round_trip_asr_token = get_round_trip_data_structure(asr_token,
                                      hermes_python.ffi.ontology.asr.CAsrToken,
                                      hermes_python.ontology.asr.AsrToken,
                                      lib.hermes_ffi_test_round_trip_asr_token)

        assert asr_token.value == round_trip_asr_token.value
        assert asr_token.range_start == round_trip_asr_token.range_start

    def test_hermes_ffi_test_round_trip_asr_token_array(self):
        decoding_duration_1 = AsrDecodingDuration(.2, .4)
        asr_token_1 = AsrToken("hello", .1, 0, 1, decoding_duration_1)

        decoding_duration_2 = AsrDecodingDuration(.2, .4)
        asr_token_2 = AsrToken("my", .1, 0, 1, decoding_duration_2)

        decoding_duration_3 = AsrDecodingDuration(.2, .4)
        asr_token_3 = AsrToken("friend", .1, 0, 1, decoding_duration_3)

        asr_tokens = [asr_token_1, asr_token_2, asr_token_3]

        c_repr_object = hermes_python.ffi.ontology.asr.CAsrTokenArray.from_repr(asr_tokens)
        pointer_c_repr = pointer(c_repr_object)
        output_pointer = pointer(c_void_p())

        # Send it for round trip to Rust
        result = lib.hermes_ffi_test_round_trip_asr_token_array(pointer_c_repr, output_pointer)
        if result > 0:
            wrap_c_error()

        # Deserialize Rust result into C representation
        round_trip_c_repr_object = hermes_python.ffi.ontology.asr.CAsrTokenArray.from_address(output_pointer.contents.value)

        # Deserialize into Python object
        round_trip_python_ontology_object = TestAsrTokens.get_asr_tokens_from_c_asr_token_array(round_trip_c_repr_object)

        assert len(round_trip_python_ontology_object) == len(asr_tokens)

    def test_hermes_ffi_test_round_trip_asr_token_double_array(self):
        decoding_duration_1 = AsrDecodingDuration(.2, .4)
        asr_token_1 = AsrToken("hello", .1, 0, 1, decoding_duration_1)

        decoding_duration_2 = AsrDecodingDuration(.2, .4)
        asr_token_2 = AsrToken("my", .1, 0, 1, decoding_duration_2)

        decoding_duration_3 = AsrDecodingDuration(.2, .4)
        asr_token_3 = AsrToken("friend", .1, 0, 1, decoding_duration_3)

        asr_tokens_1 = [asr_token_1, asr_token_2, asr_token_3]
        asr_tokens_2 = [asr_token_3, asr_token_2, asr_token_1]

        asr_tokens_matrix = [asr_tokens_1, asr_tokens_2]

        c_repr_object = hermes_python.ffi.ontology.asr.CAsrTokenDoubleArray.from_repr(asr_tokens_matrix)
        pointer_c_repr = pointer(c_repr_object)
        output_pointer = pointer(c_void_p())

        # Send it for round trip to Rust
        result = lib.hermes_ffi_test_round_trip_asr_token_double_array(pointer_c_repr, output_pointer)
        if result > 0:
            wrap_c_error()

        # Deserialize Rust result into C representation
        round_trip_c_repr_object = hermes_python.ffi.ontology.asr.CAsrTokenDoubleArray.from_address(output_pointer.contents.value)

        # Deserialize into Python object
        round_trip_asr_tokens_matrix = [TestAsrTokens.get_asr_tokens_from_c_asr_token_array(round_trip_c_repr_object.entries[i].contents) for i in range(round_trip_c_repr_object.count)]

        for asr_tokens_row, round_trip_asr_tokens_row in zip(asr_tokens_matrix, round_trip_asr_tokens_matrix):
            assert len(round_trip_asr_tokens_row) == len(round_trip_asr_tokens_row)

