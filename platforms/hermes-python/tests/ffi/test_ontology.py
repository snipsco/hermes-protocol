# coding: utf-8

from hermes_python.ffi.ontology import CMapStringToStringArray, CMapStringToStringArrayEntry, CStringArray
from hermes_python.ffi.ontology.injection import CInjectionRequestMessage
from hermes_python.ontology.injection import AddInjectionRequest, InjectionRequestMessage


def test_serde_CStringArray():
    input = ["i", "am", "groot", "🌱"]
    serialized_data = CStringArray.from_repr(input)
    deserialized_data = serialized_data.into_repr()

    assert input == deserialized_data


def test_serde_CMapStringToStringArrayEntry():
    input_data = ("key", ["hello", "world", "🌍"])
    serialized_data = CMapStringToStringArrayEntry.from_repr(input_data)
    deserialized_data = serialized_data.into_repr()

    assert input_data[0] == deserialized_data[0]
    assert input_data[1] == deserialized_data[1]


def test_serde_CMapStringToStringArray():
    input_data = {'key1': ['hello', 'world', '🌍'], 'key2': ['hello', 'moon', '👽']}
    serialized_data = CMapStringToStringArray.from_repr(input_data)
    deserialized_data = serialized_data.into_repr()
    assert deserialized_data == input_data


def test_serde_InjectionRequestMessage():
    input_request_1 = AddInjectionRequest({"key": ["hello", "world", "✨"]})
    input_request_2 = AddInjectionRequest({"key": ["hello", "moon", "👽"]})
    operations = [input_request_1, input_request_2]
    lexicon = {"key": ["i", "am a", "lexicon ⚠️"]}

    request = InjectionRequestMessage(operations, lexicon)
    serialized = CInjectionRequestMessage.from_repr(request)
    deserialized = InjectionRequestMessage.from_c_repr(serialized)

    assert request.lexicon == deserialized.lexicon
    assert len(request.operations) == len(deserialized.operations)
    assert request.operations[0].values == deserialized.operations[0].values
