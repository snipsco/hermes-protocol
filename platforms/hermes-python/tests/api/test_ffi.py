from __future__ import unicode_literals
import mock
import pytest

from hermes_python.api.ffi import FFI
from hermes_python.ffi.utils import MqttOptions

HOST = "localhost"
DUMMY_INTENT_NAME = "INTENT"

def test_initialization():
    h = FFI()
    assert 0 == len(h._c_callback_subscribe_intent)

def test_initialization_use_json_api_by_default():
        h = FFI()
        assert h.use_json_api

@mock.patch("hermes_python.api.ffi.hermes_protocol_handler_dialogue_facade")
@mock.patch("hermes_python.api.ffi.hermes_protocol_handler_new_mqtt_with_options")
def test_establishing_successful_connection(hermes_protocol_handler_new_mqtt, hermes_protocol_handler_dialogue_facade):
    ffi = FFI()
    mqtt_opts = MqttOptions()
    ffi.establish_connection(mqtt_opts)

    hermes_protocol_handler_new_mqtt.assert_called_once()
    hermes_protocol_handler_dialogue_facade.assert_called_once()


@mock.patch("hermes_python.api.ffi.hermes_protocol_handler_dialogue_facade")
@mock.patch("hermes_python.api.ffi.hermes_drop_dialogue_facade")
@mock.patch("hermes_python.api.ffi.hermes_protocol_handler_new_mqtt_with_options")
def test_release_connection_sucessful(hermes_protocol_handler_new_mqtt, hermes_drop_dialogue_facade,
                                      hermes_protocol_handler_dialogue_facade):
    ffi = FFI()
    mqtt_opts = MqttOptions()

    ffi.establish_connection(mqtt_opts)
    ffi.release_connection()

    hermes_protocol_handler_new_mqtt.assert_called_once()
    hermes_protocol_handler_dialogue_facade.assert_called_once()
    hermes_drop_dialogue_facade.assert_called_once()


@mock.patch("hermes_python.api.ffi.hermes_protocol_handler_dialogue_facade")
@mock.patch("hermes_python.api.ffi.hermes_protocol_handler_new_mqtt_with_options")
@mock.patch("hermes_python.api.ffi.utils")
def test_subscribe_intent_correctly_registers_callback(ffi_utils, hermes_protocol_handler_new_mqtt,
                                                       hermes_protocol_handler_dialogue_facade):
    def user_callback(hermes, intentMessage):
        pass

    ffi = FFI()
    mqtt_opts = MqttOptions()

    ffi.establish_connection(mqtt_opts)
    ffi.register_subscribe_intent_handler(DUMMY_INTENT_NAME, user_callback)

    assert len(ffi._c_callback_subscribe_intent) == 1
    hermes_protocol_handler_new_mqtt.assert_called_once()  # connection is established
    hermes_protocol_handler_dialogue_facade.assert_called_once()  # connection is established
    ffi_utils.hermes_dialogue_subscribe_intent_json.assert_called_once()


@mock.patch("hermes_python.api.ffi.hermes_protocol_handler_dialogue_facade")
@mock.patch("hermes_python.api.ffi.hermes_protocol_handler_new_mqtt_with_options")
@mock.patch("hermes_python.api.ffi.utils")
def test_subscribe_intent_correctly_registers_two_callbacks_for_same_intent(ffi_utils, hermes_protocol_handler_new_mqtt, hermes_protocol_handler_dialogue_facade):
    def user_callback_1(hermes, intentMessage):
        pass

    def user_callback_2(hermes, intentMessage):
        pass


    ffi = FFI()
    mqtt_opts = MqttOptions()

    ffi.establish_connection(mqtt_opts)
    ffi.register_subscribe_intent_handler(DUMMY_INTENT_NAME, user_callback_1)
    ffi.register_subscribe_intent_handler(DUMMY_INTENT_NAME, user_callback_2)
    assert len(ffi._c_callback_subscribe_intent) == 2

    hermes_protocol_handler_new_mqtt.assert_called_once()  # connection is established
    hermes_protocol_handler_dialogue_facade.assert_called_once()  # connection is established
    assert ffi_utils.hermes_dialogue_subscribe_intent_json.call_count == 2


@mock.patch("hermes_python.api.ffi.utils")
def test_successful_registration_c_handler_callback(utils):
    ffi = FFI()
    c_handler = mock.Mock()

    ffi._register_c_intent_handler('test_function', DUMMY_INTENT_NAME, c_handler)
    utils.test_function_json.assert_called_once()

    ffi_without_json_api = FFI(use_json_api=False)
    ffi_without_json_api._register_c_intent_handler('test_function', DUMMY_INTENT_NAME, c_handler)
    utils.test_function.assert_called_once()


@mock.patch("hermes_python.api.ffi.hermes_protocol_handler_dialogue_facade")
@mock.patch("hermes_python.api.ffi.hermes_protocol_handler_new_mqtt_with_options")
@mock.patch("hermes_python.api.ffi.utils")
def test_subscribe_intents_correctly_registers_callback(ffi_utils, hermes_protocol_handler_new_mqtt, hermes_protocol_handler_dialogue_facade):
    def user_callback(hermes, intentMessage):
        pass

    ffi = FFI()
    mqtt_opts = MqttOptions()
    ffi.establish_connection(mqtt_opts)

    ffi.register_subscribe_intents_handler(user_callback)

    assert ffi._c_callback_subscribe_intents is not None

    hermes_protocol_handler_new_mqtt.assert_called_once()  # connection is established
    hermes_protocol_handler_dialogue_facade.assert_called_once()  # connection is established
    ffi_utils.hermes_dialogue_subscribe_intents_json.assert_called_once()


@mock.patch("hermes_python.api.ffi.hermes_protocol_handler_dialogue_facade")
@mock.patch("hermes_python.api.ffi.hermes_protocol_handler_new_mqtt_with_options")
@mock.patch("hermes_python.api.ffi.utils")
def test_subscribe_session_started_correctly_registers_callback(ffi_utils, hermes_protocol_handler_new_mqtt, hermes_protocol_handler_dialogue_facade):
    def user_callback(hermes, intentMessage):
        pass

    ffi = FFI()
    mqtt_opts = MqttOptions()
    ffi.establish_connection(mqtt_opts)

    ffi.register_session_started_handler(user_callback)

    assert ffi._c_callback_subscribe_session_started is not None

    hermes_protocol_handler_new_mqtt.assert_called_once()  # connection is established
    hermes_protocol_handler_dialogue_facade.assert_called_once()  # connection is established
    ffi_utils.hermes_dialogue_subscribe_session_started_json.assert_called_once()


@mock.patch("hermes_python.api.ffi.hermes_protocol_handler_dialogue_facade")
@mock.patch("hermes_python.api.ffi.hermes_protocol_handler_new_mqtt_with_options")
@mock.patch("hermes_python.api.ffi.utils")
def test_subscribe_session_queued_correctly_registers_callback(ffi_utils, hermes_protocol_handler_new_mqtt, hermes_protocol_handler_dialogue_facade):
    def user_callback(hermes, intentMessage):
        pass

    ffi = FFI()
    mqtt_opts = MqttOptions()
    ffi.establish_connection(mqtt_opts)

    ffi.register_session_queued_handler(user_callback)

    assert ffi._c_callback_subscribe_session_queued is not None

    hermes_protocol_handler_new_mqtt.assert_called_once()  # connection is established
    hermes_protocol_handler_dialogue_facade.assert_called_once()  # connection is established
    ffi_utils.hermes_dialogue_subscribe_session_queued_json.assert_called_once()


@mock.patch("hermes_python.api.ffi.hermes_protocol_handler_dialogue_facade")
@mock.patch("hermes_python.api.ffi.hermes_protocol_handler_new_mqtt_with_options")
@mock.patch("hermes_python.api.ffi.utils")
def test_subscribe_session_ended_correctly_registers_callback(ffi_utils, hermes_protocol_handler_new_mqtt, hermes_protocol_handler_dialogue_facade):
    def user_callback(hermes, intentMessage):
        pass

    ffi = FFI()
    mqtt_opts = MqttOptions()
    ffi.establish_connection(mqtt_opts)

    ffi.register_session_ended_handler(user_callback)

    assert ffi._c_callback_subscribe_session_ended is not None

    hermes_protocol_handler_new_mqtt.assert_called_once()  # connection is established
    hermes_protocol_handler_dialogue_facade.assert_called_once()  # connection is established
    ffi_utils.hermes_dialogue_subscribe_session_ended_json.assert_called_once()

@mock.patch("hermes_python.api.ffi.hermes_protocol_handler_dialogue_facade")
@mock.patch("hermes_python.api.ffi.hermes_protocol_handler_new_mqtt_with_options")
@mock.patch("hermes_python.api.ffi.utils")
def test_subscribe_intent_not_recognized_correctly_registers_callback(ffi_utils, hermes_protocol_handler_new_mqtt, hermes_protocol_handler_dialogue_facade):
    def user_callback(hermes, intentMessage):
        pass

    ffi = FFI()
    mqtt_opts = MqttOptions()
    ffi.establish_connection(mqtt_opts)

    ffi.register_intent_not_recognized_handler(user_callback)

    assert ffi._c_callback_subscribe_intent_not_recognized is not None

    hermes_protocol_handler_new_mqtt.assert_called_once()  # connection is established
    hermes_protocol_handler_dialogue_facade.assert_called_once()  # connection is established
    ffi_utils.hermes_dialogue_subscribe_intent_not_recognized_json.assert_called_once()

