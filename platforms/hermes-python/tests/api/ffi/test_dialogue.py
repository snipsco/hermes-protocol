from __future__ import unicode_literals
import mock
import pytest

from hermes_python.api.ffi.dialogue import DialogueFFI
from hermes_python.ontology.dialogue import StartSessionMessage, SessionInitAction, SessionInitNotification, \
    ContinueSessionMessage, EndSessionMessage, DialogueConfigureMessage, DialogueConfigureIntent

DUMMY_INTENT_NAME = "INTENT"


@pytest.fixture()
def dialogue_ffi():
    return DialogueFFI()


@mock.patch("hermes_python.api.ffi.dialogue.utils")
class TestDialogueMethodsCallsUnderlyingFFIfunctions:
    def test_subscribe_intent_correctly_registers_callback(self, ffi_utils, dialogue_ffi):
        def user_callback(_, __):
            pass

        hermes_client = mock.Mock()
        dialogue_ffi.register_subscribe_intent_handler(DUMMY_INTENT_NAME, user_callback, hermes_client)

        assert len(dialogue_ffi._c_callback_subscribe_intent) == 1
        ffi_utils.hermes_dialogue_subscribe_intent_json.assert_called_once()

    def test_subscribe_intent_correctly_registers_two_callbacks_for_same_intent(self, ffi_utils, dialogue_ffi):
        def user_callback_1(hermes, intentMessage):
            pass

        def user_callback_2(hermes, intentMessage):
            pass

        hermes_client = mock.Mock()

        dialogue_ffi.register_subscribe_intent_handler(DUMMY_INTENT_NAME, user_callback_1, hermes_client)
        dialogue_ffi.register_subscribe_intent_handler(DUMMY_INTENT_NAME, user_callback_2, hermes_client)
        assert len(dialogue_ffi._c_callback_subscribe_intent) == 2
        assert ffi_utils.hermes_dialogue_subscribe_intent_json.call_count == 2

    def test_successful_registration_c_handler_callback(self, ffi_utils):
        dialogue_ffi = DialogueFFI()
        c_handler = mock.Mock()

        dialogue_ffi._register_c_intent_handler('test_function', DUMMY_INTENT_NAME, c_handler)
        ffi_utils.test_function_json.assert_called_once()

        ffi_without_json_api = DialogueFFI(use_json_api=False)
        ffi_without_json_api._register_c_intent_handler('test_function', DUMMY_INTENT_NAME, c_handler)
        ffi_utils.test_function.assert_called_once()

    def test_subscribe_intents_correctly_registers_callback(self, ffi_utils, dialogue_ffi):
        def user_callback(_, __):
            pass

        hermes_client = mock.Mock()
        dialogue_ffi.register_subscribe_intents_handler(user_callback, hermes_client)

        assert dialogue_ffi._c_callback_subscribe_intents is not None
        ffi_utils.hermes_dialogue_subscribe_intents_json.assert_called_once()

    def test_subscribe_session_started_correctly_registers_callback(self, ffi_utils, dialogue_ffi):
        def user_callback(_, __):
            pass

        hermes_client = mock.Mock()

        dialogue_ffi.register_session_started_handler(user_callback, hermes_client)

        assert dialogue_ffi._c_callback_subscribe_session_started is not None
        ffi_utils.hermes_dialogue_subscribe_session_started_json.assert_called_once()

    def test_subscribe_session_queued_correctly_registers_callback(self, ffi_utils, dialogue_ffi):
        def user_callback(_, __):
            pass

        hermes_client = mock.Mock()

        dialogue_ffi.register_session_queued_handler(user_callback, hermes_client)

        assert dialogue_ffi._c_callback_subscribe_session_queued is not None
        ffi_utils.hermes_dialogue_subscribe_session_queued_json.assert_called_once()

    def test_subscribe_session_ended_correctly_registers_callback(self, ffi_utils, dialogue_ffi):
        def user_callback(hermes, intentMessage):
            pass

        hermes_client = mock.Mock()

        dialogue_ffi.register_session_ended_handler(user_callback, hermes_client)

        assert dialogue_ffi._c_callback_subscribe_session_ended is not None
        ffi_utils.hermes_dialogue_subscribe_session_ended_json.assert_called_once()

    def test_subscribe_intent_not_recognized_correctly_registers_callback(self, ffi_utils, dialogue_ffi):
        def user_callback(hermes, intentMessage):
            pass

        hermes_client = mock.Mock()

        dialogue_ffi.register_intent_not_recognized_handler(user_callback, hermes_client)

        assert dialogue_ffi._c_callback_subscribe_intent_not_recognized is not None

        ffi_utils.hermes_dialogue_subscribe_intent_not_recognized_json.assert_called_once()

    def test_publish_start_session_with_action_success(self, ffi_utils):
        dialogue_ffi = DialogueFFI(use_json_api=False)
        session_init = SessionInitAction()
        start_session_message_with_action = StartSessionMessage(session_init, custom_data=None, site_id=None)

        dialogue_ffi.publish_start_session(start_session_message_with_action)
        ffi_utils.hermes_dialogue_publish_start_session.assert_called_once()

    def test_publish_start_session_with_action_success_json(self, ffi_utils, dialogue_ffi):
        start_session_message_with_action = {"test": "test"}
        dialogue_ffi.publish_start_session(start_session_message_with_action)
        ffi_utils.hermes_dialogue_publish_start_session_json.assert_called_once()

    def test_publish_start_session_with_notification_success(self, ffi_utils):
        ffi = DialogueFFI(use_json_api=False)

        session_init = SessionInitNotification("hello world!")
        start_session_message_with_notification = StartSessionMessage(session_init, custom_data=None, site_id=None)

        ffi.publish_start_session(start_session_message_with_notification)
        ffi_utils.hermes_dialogue_publish_start_session.assert_called_once()

    def test_publish_start_session_with_notification_success_json(self, ffi_utils, dialogue_ffi):
        start_session_message_with_notification = {"test": "test"}
        dialogue_ffi.publish_start_session(start_session_message_with_notification)
        ffi_utils.hermes_dialogue_publish_start_session_json.assert_called_once()

    def test_publish_continue_session_success(self, ffi_utils):
        dialogue_ffi = DialogueFFI(use_json_api=False)
        continue_session_message = ContinueSessionMessage("session_id",
                                                          "text",
                                                          "intent_filter",
                                                          "custom_data",
                                                          False)

        dialogue_ffi.publish_continue_session(continue_session_message)
        ffi_utils.hermes_dialogue_publish_continue_session.assert_called_once()

    def test_publish_continue_session_success_json(self, ffi_utils, dialogue_ffi):
        continue_session_message = {"test": "test"}
        dialogue_ffi.publish_continue_session(continue_session_message)

        ffi_utils.hermes_dialogue_publish_continue_session_json.assert_called_once()

    def test_publish_end_session_success(self, ffi_utils):
        dialogue_ffi = DialogueFFI(use_json_api=False)
        end_session_message = EndSessionMessage("session_id", "I end the session with this text")
        dialogue_ffi.publish_end_session(end_session_message)

        ffi_utils.hermes_dialogue_publish_end_session.assert_called_once()

    def test_publish_end_session_success_json(self, ffi_utils, dialogue_ffi):
        end_session_message = {"session_id": "session_id", "text": "ok"}
        dialogue_ffi.publish_end_session(end_session_message)

        ffi_utils.hermes_dialogue_publish_end_session_json.assert_called_once()

    def test_configure_dialogue(self, ffi_utils):
        dialogue_ffi = DialogueFFI(use_json_api=False)
        intent_config = DialogueConfigureIntent("dummy_intent", False)
        dialogue_configure_message = DialogueConfigureMessage(None, [intent_config])
        dialogue_ffi.publish_configure(dialogue_configure_message)

        ffi_utils.hermes_dialogue_publish_configure.assert_called_once()
