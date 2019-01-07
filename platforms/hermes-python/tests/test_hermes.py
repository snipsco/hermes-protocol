from __future__ import unicode_literals
import mock
import pytest

from hermes_python.hermes import Hermes

HOST = "localhost"
DUMMY_INTENT_NAME = "INTENT"

def test_initialization():
    h = Hermes(HOST)
    assert 0 == len(h._c_callback_subscribe_intent)


@mock.patch("hermes_python.hermes.hermes_protocol_handler_dialogue_facade")
@mock.patch("hermes_python.hermes.hermes_protocol_handler_new_mqtt")
def test_context_manager_enter(hermes_protocol_handler_new_mqtt, hermes_protocol_handler_dialogue_facade):
    with Hermes(HOST) as h:
        pass

    hermes_protocol_handler_new_mqtt.assert_called_once()
    hermes_protocol_handler_dialogue_facade.assert_called_once()

@mock.patch("hermes_python.hermes.hermes_protocol_handler_dialogue_facade")
@mock.patch("hermes_python.hermes.hermes_drop_dialogue_facade")
@mock.patch("hermes_python.hermes.hermes_protocol_handler_new_mqtt")
def test_context_manager_exit(hermes_protocol_handler_new_mqtt, hermes_drop_dialogue_facade, hermes_protocol_handler_dialogue_facade):
    with Hermes(HOST) as h:
        pass
    hermes_drop_dialogue_facade.assert_called_once()


@mock.patch("hermes_python.hermes.hermes_protocol_handler_dialogue_facade")
@mock.patch("hermes_python.hermes.hermes_drop_dialogue_facade")
@mock.patch("hermes_python.hermes.hermes_protocol_handler_new_mqtt")
def test_context_manager_catches_exceptions(hermes_protocol_handler_new_mqtt, mocked_hermes_drop_dialogue_facade, hermes_protocol_handler_dialogue_facade):
    hermes_protocol_handler_dialogue_facade.side_effect = Exception("An exception occured!")

    with pytest.raises(Exception):
        with Hermes(HOST) as h:
            pass


@mock.patch("hermes_python.hermes.hermes_dialogue_subscribe_intent")
@mock.patch("hermes_python.hermes.hermes_protocol_handler_dialogue_facade")
@mock.patch("hermes_python.hermes.hermes_drop_dialogue_facade")
@mock.patch("hermes_python.hermes.hermes_protocol_handler_new_mqtt")
def test_subscribe_intent_correctly_registers_callback(hermes_protocol_handler_new_mqtt, hermes_drop_dialogue_facade, hermes_protocol_handler_dialogue_facade, hermes_dialogue_subscribe_intent):
    def user_callback(hermes, intentMessage):
        pass

    with Hermes(HOST) as h:
        h.subscribe_intent(DUMMY_INTENT_NAME, user_callback)
        assert len(h._c_callback_subscribe_intent) == 1

    hermes_protocol_handler_new_mqtt.assert_called_once()
    hermes_dialogue_subscribe_intent.assert_called_once()



@mock.patch("hermes_python.hermes.hermes_dialogue_subscribe_intent")
@mock.patch("hermes_python.hermes.hermes_protocol_handler_dialogue_facade")
@mock.patch("hermes_python.hermes.hermes_drop_dialogue_facade")
@mock.patch("hermes_python.hermes.hermes_protocol_handler_new_mqtt")
def test_subscribe_intent_correctly_registers_two_callbacks_for_same_intent(hermes_protocol_handler_new_mqtt, hermes_drop_dialogue_facade, hermes_protocol_handler_dialogue_facade, hermes_dialogue_subscribe_intent):
    hermes_protocol_handler_new_mqtt.assert_not_called()
    def user_callback_1(hermes, intentMessage):
        pass

    def user_callback_2(hermes, intentMessage):
        pass

    with Hermes(HOST) as h:
        h.subscribe_intent(DUMMY_INTENT_NAME, user_callback_1)
        hermes_protocol_handler_new_mqtt.assert_called_once()
        h.subscribe_intent(DUMMY_INTENT_NAME, user_callback_2)
        hermes_protocol_handler_new_mqtt.assert_called_once()
        assert len(h._c_callback_subscribe_intent) == 2

    hermes_protocol_handler_new_mqtt.assert_called_once()


@mock.patch("hermes_python.hermes.hermes_dialogue_subscribe_intents")
@mock.patch("hermes_python.hermes.hermes_protocol_handler_dialogue_facade")
@mock.patch("hermes_python.hermes.hermes_drop_dialogue_facade")
@mock.patch("hermes_python.hermes.hermes_protocol_handler_new_mqtt")
def test_subscribe_intents_correctly_registers_callback(hermes_protocol_handler_new_mqtt, hermes_drop_dialogue_facade, hermes_protocol_handler_dialogue_facade, hermes_dialogue_subscribe_intents):

    def user_callback(hermes, intentMessage):
        pass

    with Hermes(HOST) as h:
        h.subscribe_intents(user_callback)
        assert h._c_callback_subscribe_intents is not None

    hermes_protocol_handler_new_mqtt.assert_called_once()
    hermes_dialogue_subscribe_intents.assert_called_once()

@mock.patch("hermes_python.hermes.hermes_dialogue_publish_continue_session")
@mock.patch("hermes_python.hermes.hermes_protocol_handler_dialogue_facade")
@mock.patch("hermes_python.hermes.hermes_drop_dialogue_facade")
@mock.patch("hermes_python.hermes.hermes_protocol_handler_new_mqtt")
def test_publish_continue_session(hermes_protocol_handler_new_mqtt, hermes_drop_dialogue_facade, hermes_protocol_handler_dialogue_facade, hermes_dialogue_publish_continue_session):
    with Hermes(HOST) as h:
        h.publish_continue_session("session_id", "text", [])

    hermes_protocol_handler_new_mqtt.assert_called_once()
    hermes_dialogue_publish_continue_session.assert_called_once()
