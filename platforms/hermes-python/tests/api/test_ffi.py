from __future__ import unicode_literals
import mock
import pytest

from hermes_python.api.ffi import FFI
from hermes_python.ontology import MqttOptions
from hermes_python.ontology.dialogue import StartSessionMessage, SessionInitAction, SessionInitNotification, \
    ContinueSessionMessage, EndSessionMessage

HOST = "localhost"
DUMMY_INTENT_NAME = "INTENT"

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
    ffi.dialogue = mocked_dialogue_ffi

    ffi.establish_connection(mqtt_opts)

    hermes_protocol_handler_new_mqtt.assert_called_once()
    ffi.dialogue.initialize_facade.assert_called_once()

@mock.patch("hermes_python.api.ffi.hermes_protocol_handler_new_mqtt_with_options")
def test_release_connection_calls_api_subsets(hermes_protocol_handler_new_mqtt):
    ffi = FFI()
    mqtt_opts = MqttOptions()

    # Here, you have to mock every API subset of Hermes Protocol
    mocked_dialogue_ffi = mock.Mock()
    ffi.dialogue = mocked_dialogue_ffi

    ffi.release_connection()

    ffi.dialogue.release_facade.assert_called_once()

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
    ffi.dialogue.register_subscribe_intent_handler(DUMMY_INTENT_NAME, user_callback)

    assert len(ffi.dialogue._c_callback_subscribe_intent) == 1
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
    ffi.dialogue.register_subscribe_intent_handler(DUMMY_INTENT_NAME, user_callback_1)
    ffi.dialogue.register_subscribe_intent_handler(DUMMY_INTENT_NAME, user_callback_2)
    assert len(ffi.dialogue._c_callback_subscribe_intent) == 2

    hermes_protocol_handler_new_mqtt.assert_called_once()  # connection is established
    hermes_protocol_handler_dialogue_facade.assert_called_once()  # connection is established
    assert ffi_utils.hermes_dialogue_subscribe_intent_json.call_count == 2


@mock.patch("hermes_python.api.ffi.utils")
def test_successful_registration_c_handler_callback(utils):
    ffi = FFI()
    c_handler = mock.Mock()

    ffi.dialogue._register_c_intent_handler('test_function', DUMMY_INTENT_NAME, c_handler)
    utils.test_function_json.assert_called_once()

    ffi_without_json_api = FFI(use_json_api=False)
    ffi_without_json_api.dialogue._register_c_intent_handler('test_function', DUMMY_INTENT_NAME, c_handler)
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

    ffi.dialogue.register_subscribe_intents_handler(user_callback)

    assert ffi.dialogue._c_callback_subscribe_intents is not None

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

    ffi.dialogue.register_session_started_handler(user_callback)

    assert ffi.dialogue._c_callback_subscribe_session_started is not None

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

    ffi.dialogue.register_session_queued_handler(user_callback)

    assert ffi.dialogue._c_callback_subscribe_session_queued is not None

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

    ffi.dialogue.register_session_ended_handler(user_callback)

    assert ffi.dialogue._c_callback_subscribe_session_ended is not None

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

    ffi.dialogue.register_intent_not_recognized_handler(user_callback)

    assert ffi.dialogue._c_callback_subscribe_intent_not_recognized is not None

    hermes_protocol_handler_new_mqtt.assert_called_once()  # connection is established
    hermes_protocol_handler_dialogue_facade.assert_called_once()  # connection is established
    ffi_utils.hermes_dialogue_subscribe_intent_not_recognized_json.assert_called_once()

@mock.patch("hermes_python.api.ffi.hermes_protocol_handler_dialogue_facade")
@mock.patch("hermes_python.api.ffi.hermes_protocol_handler_new_mqtt_with_options")
@mock.patch("hermes_python.api.ffi.utils")
def test_publish_start_session_with_action_success(ffi_utils, hermes_protocol_handler_new_mqtt, hermes_protocol_handler_dialogue_facade):
    ffi = FFI(use_json_api=False)
    mqtt_opts = MqttOptions()
    ffi.establish_connection(mqtt_opts)

    session_init = SessionInitAction()
    start_session_message_with_action = StartSessionMessage(session_init, custom_data=None, site_id=None)

    ffi.dialogue.publish_start_session(start_session_message_with_action)
    ffi_utils.hermes_dialogue_publish_start_session.assert_called_once()


@mock.patch("hermes_python.api.ffi.hermes_protocol_handler_dialogue_facade")
@mock.patch("hermes_python.api.ffi.hermes_protocol_handler_new_mqtt_with_options")
@mock.patch("hermes_python.api.ffi.utils")
def test_publish_start_session_with_action_success_json(ffi_utils, hermes_protocol_handler_new_mqtt, hermes_protocol_handler_dialogue_facade):
    ffi = FFI()
    mqtt_opts = MqttOptions()
    ffi.establish_connection(mqtt_opts)

    start_session_message_with_action = {"test": "test"}
    ffi.dialogue.publish_start_session(start_session_message_with_action)
    ffi_utils.hermes_dialogue_publish_start_session_json.assert_called_once()


@mock.patch("hermes_python.api.ffi.hermes_protocol_handler_dialogue_facade")
@mock.patch("hermes_python.api.ffi.hermes_protocol_handler_new_mqtt_with_options")
@mock.patch("hermes_python.api.ffi.utils")
def test_publish_start_session_with_notification_success(ffi_utils, hermes_protocol_handler_new_mqtt, hermes_protocol_handler_dialogue_facade):
    ffi = FFI(use_json_api=False)
    mqtt_opts = MqttOptions()
    ffi.establish_connection(mqtt_opts)

    session_init = SessionInitNotification("hello world!")
    start_session_message_with_notification = StartSessionMessage(session_init, custom_data=None, site_id=None)

    ffi.dialogue.publish_start_session(start_session_message_with_notification)
    ffi_utils.hermes_dialogue_publish_start_session.assert_called_once()


@mock.patch("hermes_python.api.ffi.hermes_protocol_handler_dialogue_facade")
@mock.patch("hermes_python.api.ffi.hermes_protocol_handler_new_mqtt_with_options")
@mock.patch("hermes_python.api.ffi.utils")
def test_publish_start_session_with_notification_success_json(ffi_utils, hermes_protocol_handler_new_mqtt, hermes_protocol_handler_dialogue_facade):
    ffi = FFI()
    mqtt_opts = MqttOptions()
    ffi.establish_connection(mqtt_opts)

    start_session_message_with_notification = {"test": "test"}
    ffi.dialogue.publish_start_session(start_session_message_with_notification)
    ffi_utils.hermes_dialogue_publish_start_session_json.assert_called_once()


@mock.patch("hermes_python.api.ffi.hermes_protocol_handler_dialogue_facade")
@mock.patch("hermes_python.api.ffi.hermes_protocol_handler_new_mqtt_with_options")
@mock.patch("hermes_python.api.ffi.utils")
def test_publish_continue_session_success(ffi_utils, hermes_protocol_handler_new_mqtt, hermes_protocol_handler_dialogue_facade):
    ffi = FFI(use_json_api=False)
    mqtt_opts = MqttOptions()
    ffi.establish_connection(mqtt_opts)

    continue_session_message = ContinueSessionMessage("session_id",
                                                      "text",
                                                      "intent_filter",
                                                      "custom_data",
                                                      False)

    ffi.dialogue.publish_continue_session(continue_session_message)
    ffi_utils.hermes_dialogue_publish_continue_session.assert_called_once()

@mock.patch("hermes_python.api.ffi.hermes_protocol_handler_dialogue_facade")
@mock.patch("hermes_python.api.ffi.hermes_protocol_handler_new_mqtt_with_options")
@mock.patch("hermes_python.api.ffi.utils")
def test_publish_continue_session_success_json(ffi_utils, hermes_protocol_handler_new_mqtt, hermes_protocol_handler_dialogue_facade):
    ffi = FFI()
    mqtt_opts = MqttOptions()
    ffi.establish_connection(mqtt_opts)

    continue_session_message = {"test": "test"}
    ffi.dialogue.publish_continue_session(continue_session_message)

    ffi_utils.hermes_dialogue_publish_continue_session_json.assert_called_once()


@mock.patch("hermes_python.api.ffi.hermes_protocol_handler_dialogue_facade")
@mock.patch("hermes_python.api.ffi.hermes_protocol_handler_new_mqtt_with_options")
@mock.patch("hermes_python.api.ffi.utils")
def test_publish_end_session_success(ffi_utils, hermes_protocol_handler_new_mqtt, hermes_protocol_handler_dialogue_facade):
    ffi = FFI(use_json_api=False)
    mqtt_opts = MqttOptions()
    ffi.establish_connection(mqtt_opts)

    end_session_message = EndSessionMessage("session_id", "I end the session with this text")
    ffi.dialogue.publish_end_session(end_session_message)

    ffi_utils.hermes_dialogue_publish_end_session.assert_called_once()


@mock.patch("hermes_python.api.ffi.hermes_protocol_handler_dialogue_facade")
@mock.patch("hermes_python.api.ffi.hermes_protocol_handler_new_mqtt_with_options")
@mock.patch("hermes_python.api.ffi.utils")
def test_publish_end_session_success_json(ffi_utils, hermes_protocol_handler_new_mqtt, hermes_protocol_handler_dialogue_facade):
    ffi = FFI()
    mqtt_opts = MqttOptions()
    ffi.establish_connection(mqtt_opts)

    end_session_message = {"session_id": "session_id", "text": "ok"}
    ffi.dialogue.publish_end_session(end_session_message)

    ffi_utils.hermes_dialogue_publish_end_session_json.assert_called_once()

