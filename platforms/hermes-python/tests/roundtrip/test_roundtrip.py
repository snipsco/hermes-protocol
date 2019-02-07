# coding: utf-8

from __future__ import absolute_import
from __future__ import unicode_literals

from ctypes import cdll, CFUNCTYPE, c_void_p, c_char_p, POINTER, pointer, string_at
import os
from glob import glob

import hermes_python

DYLIB_NAME = "libhermes_ffi_test.so"
DYLIB_DIR = os.path.join(os.path.dirname(__file__), "./debug")
DYLIB_PATH = glob(os.path.join(DYLIB_DIR, DYLIB_NAME))[0]

lib = cdll.LoadLibrary(DYLIB_PATH)

class LibException(Exception):
    pass


def get_round_trip_data_structure(py_ontology_object_instance, C_Ontology_Type, Python_Ontology_Class, round_trip_function):
    c_repr_object = C_Ontology_Type.from_repr(py_ontology_object_instance)

    pointer_c_repr = pointer(c_repr_object)
    output_pointer = pointer(c_void_p())

    # Send it for round trip to Rust
    result = round_trip_function(pointer_c_repr, output_pointer)

    if result > 0:
        error_p = POINTER(c_char_p)(c_char_p("".encode('utf-8')))
        lib.hermes_ffi_test_get_last_error(error_p)
        raise Exception(string_at(error_p.contents).decode('utf-8'))


    # Deserialize Rust result into C representation
    round_trip_c_repr_object = C_Ontology_Type.from_address(output_pointer.contents.value)

    # Deserialize into Python object
    round_trip_python_ontology_object = Python_Ontology_Class.from_c_repr(round_trip_c_repr_object)
    return round_trip_python_ontology_object


def test_hermes_ffi_test_round_trip_session_queued():
    # Initialize deserialized python object
    session_queued_message = hermes_python.ontology.SessionQueuedMessage("session_id", "custom_daté", "site_id")

    round_trip_session_queued_message = get_round_trip_data_structure(
        session_queued_message,
        hermes_python.ffi.ontology.CSessionQueuedMessage,
        hermes_python.ontology.SessionQueuedMessage,
        lib.hermes_ffi_test_round_trip_session_queued
    )

    assert session_queued_message == round_trip_session_queued_message


def test_hermes_ffi_test_round_trip_session_started():
    session_started_message = hermes_python.ontology.SessionStartedMessage("session_id", "custom_data", "site_id", "reactivated")
    round_trip_session_started_message = get_round_trip_data_structure(
        session_started_message,
        hermes_python.ffi.ontology.CSessionStartedMessage,
        hermes_python.ontology.SessionStartedMessage,
        lib.hermes_ffi_test_round_trip_session_started
    )

    assert session_started_message == round_trip_session_started_message


def test_hermes_ffi_test_round_trip_session_ended_for_error(): # TODO : Move Termination type to dedicated enum
    SNIPS_SESSION_TERMINATION_TYPE_ERROR = 6
    session_termination = hermes_python.ontology.SessionTermination(SNIPS_SESSION_TERMINATION_TYPE_ERROR, "dataé")
    session_ended_message = hermes_python.ontology.SessionEndedMessage("session_id", "custom_data", "site_id", session_termination)
    round_trip_session_ended_message = get_round_trip_data_structure(
        session_ended_message,
        hermes_python.ffi.ontology.CSessionEndedMessage,
        hermes_python.ontology.SessionEndedMessage,
        lib.hermes_ffi_test_round_trip_session_ended
    )

    assert session_ended_message.termination.data == round_trip_session_ended_message.termination.data
    assert session_ended_message.termination == round_trip_session_ended_message.termination
    assert session_ended_message == round_trip_session_ended_message


def test_hermes_ffi_test_round_trip_session_ended(): # TODO : Move Termination type to dedicated enum
    SNIPS_SESSION_TERMINATION_TYPE_NOMINAL = 1
    session_termination = hermes_python.ontology.SessionTermination(SNIPS_SESSION_TERMINATION_TYPE_NOMINAL, None)
    session_ended_message = hermes_python.ontology.SessionEndedMessage("session_id", "custom_data", "site_id", session_termination)
    round_trip_session_ended_message = get_round_trip_data_structure(
        session_ended_message,
        hermes_python.ffi.ontology.CSessionEndedMessage,
        hermes_python.ontology.SessionEndedMessage,
        lib.hermes_ffi_test_round_trip_session_ended
    )

    assert session_ended_message.termination.data == round_trip_session_ended_message.termination.data
    assert session_ended_message.termination == round_trip_session_ended_message.termination
    assert session_ended_message == round_trip_session_ended_message


def test_hermes_ffi_test_round_trip_intent_not_recognized():
    intent_not_recognized_message = hermes_python.ontology.IntentNotRecognizedMessage("site_id", "session_id", "input", "custom_data", 0.5)
    round_trip_intent_not_recognized_message = get_round_trip_data_structure(
        intent_not_recognized_message,
        hermes_python.ffi.ontology.CIntentNotRecognizedMessage,
        hermes_python.ontology.IntentNotRecognizedMessage,
        lib.hermes_ffi_test_round_trip_intent_not_recognized
    )

    assert intent_not_recognized_message == round_trip_intent_not_recognized_message


def test_hermes_ffi_test_round_trip_continue_session():
    continue_session_message = hermes_python.ontology.ContinueSessionMessage("session_id", "this is a text", ["test"], "custom_data", True)
    round_trip_continue_session_message = get_round_trip_data_structure(
        continue_session_message,
        hermes_python.ffi.ontology.CContinueSessionMessage,
        hermes_python.ontology.ContinueSessionMessage,
        lib.hermes_ffi_test_round_trip_continue_session
    )

    assert continue_session_message == round_trip_continue_session_message

# TODO : Uncomment tests one by one.
"""
def test_hermes_ffi_test_round_trip_intent():
    intent_message = hermes_python.ontology.IntentMessage(
        "session_id",
        "custom_data",
        "site_id",
        "input",
        hermes_python.ontology.IntentClassifierResult("intent_name", float(0.1)),
        hermes_python.ontology.SlotMap())
    
    
    round_trip_intent_message = get_round_trip_data_structure(
        intent_message,
        hermes_python.ffi.ontology.CIntentMessage,
        hermes_python.ontology.IntentMessage,
        lib.hermes_ffi_test_round_trip_intent
    )

    assert intent_message.session_id == round_trip_intent_message.session_id
    assert intent_message.intent.intent_name == round_trip_intent_message.intent.intent_name
    assert intent_message == round_trip_intent_message


def test_hermes_ffi_test_round_trip_intent_not_recognized():
    intent_not_recognized_message = hermes_python.ontology.Intent
    round_trip__ = get_round_trip_data_structure(
        _,
        __,
        ___,
        lib.hermes_ffi_test_round_trip_intent_not_recognized
    )

    assert




def test_hermes_ffi_test_round_trip_start_session():
    start_session_message = hermes_python.ontology
    round_trip__ = get_round_trip_data_structure(
        _,
        __,
        ___,
        lib.hermes_ffi_test_round_trip_start_session
    )

    assert





def test_hermes_ffi_test_round_trip_end_session():
    end_session_message = hermes_python.ontology
    round_trip__ = get_round_trip_data_structure(
        _,
        __,
        ___,
        lib.hermes_ffi_test_round_trip_end_session
    )

    assert
"""
