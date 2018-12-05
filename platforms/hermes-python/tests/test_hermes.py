from __future__ import unicode_literals
import mock
import pytest

from hermes_python.hermes import Hermes

HOST = "localhost"
DUMMY_INTENT_NAME = "INTENT"

def test_initialization():
    h = Hermes(HOST)
    assert 0 == len(h._c_callback_subscribe_intent)


@mock.patch("hermes_python.hermes.lib")
def test_context_manager_enter(mocked_lib):
    with Hermes(HOST) as h:
        pass

    mocked_lib.hermes_protocol_handler_new_mqtt.assert_called_once()
    mocked_lib.hermes_protocol_handler_dialogue_facade.assert_called_once()


@mock.patch("hermes_python.hermes.lib")
def test_context_manager_exit(mocked_lib):
    with Hermes(HOST) as h:
        pass
    mocked_lib.hermes_drop_dialogue_facade.assert_called_once()


@mock.patch("hermes_python.hermes.hermes_drop_dialogue_facade")
@mock.patch("hermes_python.hermes.lib")
def test_context_manager_catches_exceptions(mocked_lib, mocked_hermes_drop_dialogue_facade):
    mocked_lib.hermes_protocol_handler_dialogue_facade.side_effect = Exception("An exception occured!")

    with pytest.raises(Exception):
        with Hermes(HOST) as h:
            pass


@mock.patch("hermes_python.hermes.lib")
def test_subscribe_intent_correctly_registers_callback(mocked_lib):

    def user_callback(hermes, intentMessage):
        pass

    with Hermes(HOST) as h:
        h.subscribe_intent(DUMMY_INTENT_NAME, user_callback)
        assert len(h._c_callback_subscribe_intent) == 1

    mocked_lib.hermes_protocol_handler_new_mqtt.assert_called_once()
    mocked_lib.hermes_dialogue_subscribe_intent.assert_called_once()


@mock.patch("hermes_python.hermes.lib")
def test_subscribe_intent_correctly_registers_two_callbacks_for_same_intent(mocked_lib):

    def user_callback_1(hermes, intentMessage):
        pass

    def user_callback_2(hermes, intentMessage):
        pass

    with Hermes(HOST) as h:
        h.subscribe_intent(DUMMY_INTENT_NAME, user_callback_1)
        h.subscribe_intent(DUMMY_INTENT_NAME, user_callback_2)
        assert len(h._c_callback_subscribe_intent) == 2

    mocked_lib.hermes_protocol_handler_new_mqtt.assert_called_once()



@mock.patch("hermes_python.hermes.lib")
def test_subscribe_intents_correctly_registers_callback(mocked_lib):

    def user_callback(hermes, intentMessage):
        pass

    with Hermes(HOST) as h:
        h.subscribe_intents(user_callback)
        assert h._c_callback_subscribe_intents is not None

    mocked_lib.hermes_protocol_handler_new_mqtt.assert_called_once()
    mocked_lib.hermes_dialogue_subscribe_intents.assert_called_once()



