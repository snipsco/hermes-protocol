# coding: utf-8

from __future__ import absolute_import
from __future__ import unicode_literals

from ctypes import cdll, CFUNCTYPE, c_void_p, POINTER, pointer
import os
from glob import glob

import hermes_python

DYLIB_NAME = "libhermes_ffi_test.dylib"
DYLIB_DIR = os.path.join(os.path.dirname(__file__), "./debug")
DYLIB_PATH = glob(os.path.join(DYLIB_DIR, DYLIB_NAME))[0]

lib = cdll.LoadLibrary(DYLIB_PATH)


def get_round_trip_data_structure(py_ontology_object_instance, C_Ontology_Type, Python_Ontology_Class, round_trip_function):
    c_repr_object = C_Ontology_Type.from_repr(py_ontology_object_instance)

    pointer_c_repr = pointer(c_repr_object)
    output_pointer = pointer(c_void_p())

    # Send it for round trip to Rust
    round_trip_function(pointer_c_repr, output_pointer)

    # Deserialize Rust result into C representation
    round_trip_c_repr_object = C_Ontology_Type.from_address(output_pointer.contents.value)

    # Deserialize into Python object
    round_trip_python_ontology_object = Python_Ontology_Class.from_c_repr(round_trip_c_repr_object)
    return round_trip_python_ontology_object


def test_hermes_ffi_test_round_trip_session_queued():
    # Initialize deserialized python object
    session_queued_message = hermes_python.ontology.SessionQueuedMessage("session_id", "custom_data", "site_id")

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

"""
def test_hermes_ffi_test_round_trip_session_ended():
    session_termination = hermes_python.ontology.SessionTermination(1, "data")
    session_ended_message = hermes_python.ontology.SessionEndedMessage("session_id", "custom_data", "site_id", session_termination)
    round_trip_session_ended_message = get_round_trip_data_structure(
        session_ended_message,
        hermes_python.ffi.ontology.CSessionEndedMessage,
        hermes_python.ontology.SessionEndedMessage,
        lib.hermes_ffi_test_round_trip_session_ended
    )

    assert session_ended_message == round_trip_session_ended_message


def test_hermes_ffi_test_round_trip_intent():
    intent_message = hermes_python.ontology.IntentMessage(
        "session_id",
        "custom_data",
        "site_id",
        "input",
        hermes_python.ontology.IntentClassifierResult("intent_name", float(0.1)),
        hermes_python.ontology.SlotMap())
    
    
    round_trip__ = get_round_trip_data_structure(
        _,
        __,
        ___,
        lib.hermes_ffi_test_round_trip_intent
    )

    assert



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



def test_hermes_ffi_test_round_trip_continue_session():
    continue_session_message = hermes_python.ontology
    round_trip__ = get_round_trip_data_structure(
        _,
        __,
        ___,
        lib.hermes_ffi_test_round_trip_continue_session
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