# coding: utf-8

from __future__ import absolute_import
from __future__ import unicode_literals

from ctypes import cdll, CFUNCTYPE, c_void_p, c_char_p, POINTER, pointer, string_at
import os
from glob import glob
import sys

import hermes_python
from hermes_python.ontology.dialogue import IntentMessage, IntentClassifierResult, SlotMap, NluSlot, SlotValue, \
    CustomValue
from hermes_python.ffi.ontology.dialogue import CSessionQueuedMessage, CSessionStartedMessage, CSessionEndedMessage, \
    CIntentNotRecognizedMessage, CContinueSessionMessage, CStartSessionMessageNotification, CStartSessionMessageAction, \
    CEndSessionMessage, CDialogueConfigureMessage

from hermes_python.ontology.injection import InjectionRequestMessage, AddInjectionRequest
from hermes_python.ffi.ontology.injection import CInjectionRequestMessage

DYLIB_NAME = "libhermes_ffi_test" + (".dylib" if sys.platform == "darwin" else ".so")
DYLIB_DIR = os.path.join(os.path.dirname(__file__), "./debug")
DYLIB_PATH = glob(os.path.join(DYLIB_DIR, DYLIB_NAME))[0]

lib = cdll.LoadLibrary(DYLIB_PATH)


class LibException(Exception):
    pass


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
    def test_hermes_ffi_test_round_trip_session_ended_for_error(self):  # TODO : Move Termination type to dedicated enum
        SNIPS_SESSION_TERMINATION_TYPE_ERROR = 6
        session_termination = hermes_python.ontology.dialogue.SessionTermination(SNIPS_SESSION_TERMINATION_TYPE_ERROR,
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

    def test_hermes_ffi_test_round_trip_session_ended(self):  # TODO : Move Termination type to dedicated enum
        SNIPS_SESSION_TERMINATION_TYPE_NOMINAL = 1
        session_termination = hermes_python.ontology.dialogue.SessionTermination(SNIPS_SESSION_TERMINATION_TYPE_NOMINAL,
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


def test_hermes_ffi_test_round_trip_intent_not_recognized():
    intent_not_recognized_message = hermes_python.ontology.dialogue.IntentNotRecognizedMessage("site_id", "session_id",
                                                                                               "input", "custom_data",
                                                                                               0.5)
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


class TestInjectionRoundtrip(object):
    def test_injection_request_message_roundtrip(self):

        input_request_1 = AddInjectionRequest({"key": ["hello", "world", "✨"]})
        input_request_2 = AddInjectionRequest({"key": ["hello", "moon", "👽"]})
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


"""
def test_hermes_ffi_test_round_trip_intent():
    slot_value = SlotValue(1, CustomValue("hello :) 🎁"))
    search_weather_nlu = NluSlot(0.2, slot_value, "hello", "proutEntity", "searchWeather", 0, 2)
    slotMap = SlotMap({"searchWeather": [search_weather_nlu]})
    intent_classifier_result = IntentClassifierResult("searchWeather", 0.2)
    intent_message = IntentMessage("session_id", "custom_data", "site_id", "input", intent_classifier_result, slotMap)
    round_trip_intent_message = get_round_trip_data_structure(
        intent_message,
        hermes_python.ffi.ontology.dialogue.CIntentMessage,
        hermes_python.ontology.dialogue.IntentMessage,
        lib.hermes_ffi_test_round_trip_intent
    )

    assert intent_message == round_trip_intent_message



# TODO : Missing tests.

def test_hermes_ffi_test_round_trip_intent_not_recognized():
    pass
"""
