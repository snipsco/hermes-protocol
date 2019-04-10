from __future__ import unicode_literals
import mock

from hermes_python.api.ffi import FFI
from hermes_python.ontology import MqttOptions

HOST = "localhost"


def test_initialization():
    h = FFI()
    assert 0 == len(h.dialogue._c_callback_subscribe_intent)


def test_initialization_use_json_api_by_default():
    h = FFI()
    assert h.use_json_api


@mock.patch("hermes_python.api.ffi.hermes_protocol_handler_new_mqtt_with_options")
def test_establish_connection_calls_api_subsets(hermes_protocol_handler_new_mqtt):
    ffi = FFI()
    mqtt_opts = MqttOptions()

    # Here, you have to mock every API subset of Hermes Protocol
    mocked_dialogue_ffi = mock.Mock()
    mocked_sound_feedback_ffi = mock.Mock()
    ffi.dialogue = mocked_dialogue_ffi
    ffi.sound_feedback = mocked_sound_feedback_ffi

    ffi.establish_connection(mqtt_opts)

    hermes_protocol_handler_new_mqtt.assert_called_once()
    ffi.dialogue.initialize_facade.assert_called_once()
    ffi.sound_feedback.initialize_facade.assert_called_once()


def test_release_connection_calls_api_subsets():
    ffi = FFI()

    # Here, you have to mock every API subset of Hermes Protocol
    mocked_dialogue_ffi = mock.Mock()
    mocked_sound_feedback_ffi = mock.Mock()
    ffi.dialogue = mocked_dialogue_ffi
    ffi.sound_feedback = mocked_sound_feedback_ffi

    ffi.release_connection()

    ffi.dialogue.release_facade.assert_called_once()
    ffi.sound_feedback.release_facade.assert_called_once()


@mock.patch("hermes_python.api.ffi.hermes_protocol_handler_new_mqtt_with_options")
@mock.patch("hermes_python.api.ffi.DialogueFFI")
@mock.patch("hermes_python.api.ffi.SoundFeedBackFFI")
def test_establishing_successful_connection(soundfeedbackFFI, dialogueFFI, hermes_protocol_handler_new_mqtt):
    ffi = FFI()
    mqtt_opts = MqttOptions()
    ffi.establish_connection(mqtt_opts)

    hermes_protocol_handler_new_mqtt.assert_called_once()
    ffi.dialogue.initialize_facade.assert_called_once()
    ffi.sound_feedback.initialize_facade.assert_called_once()


@mock.patch("hermes_python.api.ffi.SoundFeedBackFFI")
@mock.patch("hermes_python.api.ffi.DialogueFFI")
@mock.patch("hermes_python.api.ffi.hermes_protocol_handler_new_mqtt_with_options")
def test_release_connection_sucessful(hermes_protocol_handler_new_mqtt, dialogueFFI, soundfeedbackFFI):
    ffi = FFI()
    mqtt_opts = MqttOptions()

    ffi.establish_connection(mqtt_opts)
    ffi.release_connection()

    hermes_protocol_handler_new_mqtt.assert_called_once()

    ffi.dialogue.initialize_facade.assert_called_once()
    ffi.dialogue.release_facade.assert_called_once()
    ffi.sound_feedback.initialize_facade.assert_called_once()
    ffi.sound_feedback.release_facade.assert_called_once()



