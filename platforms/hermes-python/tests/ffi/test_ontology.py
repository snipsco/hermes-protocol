# coding: utf-8
import pytest
import os

from hermes_python.ffi.ontology import CMapStringToStringArray, CMapStringToStringArrayEntry, CStringArray
from hermes_python.ffi.ontology.injection import CInjectionRequestMessage
from hermes_python.ontology.injection import AddInjectionRequest, InjectionRequestMessage
from hermes_python.ffi.ontology.tts import CRegisterSoundMessage
from hermes_python.ontology.tts import RegisterSoundMessage


@pytest.fixture(scope="package")
def wav_data():
    import wave
    wav = wave.open(os.path.join(os.path.dirname(__file__), "../data/test.wav"), mode="rb")
    nb_of_frames = wav.getnframes()
    frames = wav.readframes(nb_of_frames)
    wav.close()
    return list(frames)


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


def test_serde_RegisterSoundMessage(wav_data):
    register_sound = RegisterSoundMessage("test", wav_data)

    serialized = CRegisterSoundMessage.from_repr(register_sound)
    deserialized = RegisterSoundMessage.from_c_repr(serialized)

    assert deserialized == register_sound
