# coding: utf-8
from __future__ import unicode_literals
import mock
import pytest

from hermes_python.api.ffi.injection import InjectionFFI
from hermes_python.ontology.injection import InjectionRequestMessage, AddInjectionRequest, InjectionResetRequestMessage, InjectionResetCompleteMessage, InjectionCompleteMessage

@mock.patch("hermes_python.api.ffi.injection.utils")
class TestInjectionFFICallsUnderlyingFFIFunctions:
    def test_subscribe_injection_status_callback(self, ffi_utils):
        def injection_status_callback(_):
            pass

        injection_ffi = InjectionFFI(use_json_api=False)
        hermes_client = mock.Mock()

        injection_ffi.register_subscribe_injection_status(injection_status_callback, hermes_client)

        assert len(injection_ffi._c_callback_subscribe_injection_status) == 1
        ffi_utils.hermes_injection_subscribe_injection_status.assert_called_once()

    def test_publish_injection_request(self, ffi_utils):
        injection_ffi = InjectionFFI(use_json_api=False)

        input_request_1 = AddInjectionRequest({"key": ["hello", "world", "✨"]})
        input_request_2 = AddInjectionRequest({"key": ["hello", "moon", "👽"]})
        operations = [input_request_1, input_request_2]
        lexicon = {"key": ["i", "am a", "lexicon ⚠️"]}

        message = InjectionRequestMessage(operations, lexicon)
        injection_ffi.publish_injection_request(message)

        ffi_utils.hermes_injection_publish_injection_request.assert_called_once()

    def test_publish_injection_request_status(self, ffi_utils):
        injection_ffi = InjectionFFI(use_json_api=False)
        injection_ffi.publish_injection_status_request()
        ffi_utils.hermes_injection_publish_injection_status_request.assert_called_once()

    def test_subscribe_injection_complete_callback(self, ffi_utils):
        def injection_complete_callback():
            pass

        injection_ffi = InjectionFFI(use_json_api=False)
        hermes_client = mock.Mock()

        injection_ffi.register_subscribe_injection_complete(injection_complete_callback(), hermes_client)

        assert len(injection_ffi._c_callback_subscribe_injection_complete) == 1
        ffi_utils.hermes_injection_subscribe_injection_complete.assert_called_once()

    def test_subscribe_injection_reset_complete_callback(self, ffi_utils):
        def injection_complete_callback():
            pass

        injection_ffi = InjectionFFI(use_json_api=False)
        hermes_client = mock.Mock()

        injection_ffi.register_subscribe_injection_reset_complete(injection_complete_callback(), hermes_client)

        assert len(injection_ffi._c_callback_subscribe_injection_reset_complete) == 1
        ffi_utils.hermes_injection_subscribe_injection_reset_complete.assert_called_once()

    def test_publish_injection_reset_request(self, ffi_utils):
        injection_ffi = InjectionFFI(use_json_api=False)

        message = InjectionResetRequestMessage("request_id")

        injection_ffi.publish_injection_reset_request(message)
        ffi_utils.hermes_injection_publish_injection_reset_request.assert_called_once()

