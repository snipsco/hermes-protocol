from ctypes import c_char_p, pointer
from hermes_python.ffi.ontology import CMapStringToStringArray, CMapStringToStringArrayEntry, CStringArray


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
    input_data_as_list = list(input_data.items())

    map_entries = (CMapStringToStringArrayEntry * len(input_data))()
    test = [ CMapStringToStringArrayEntry(e[0].encode('utf-8'), pointer(CStringArray.from_repr(e[1]))) for e in input_data_as_list ]
    map_entries[:] = test

    serialized_data = CMapStringToStringArray(
        map_entries,
        len(input_data)
    )

    deserialized_data = serialized_data.into_repr()
    assert deserialized_data == input_data




