# coding: utf-8
from __future__ import unicode_literals
import mock

from hermes_python.api.ffi.tts import TtsFFI
from hermes_python.ontology.tts import RegisterSoundMessage


@mock.patch("hermes_python.api.ffi.tts.utils")
class TestInjectionFFICallsUnderlyingFFIFunctions:
    def test_publish_register_sound(self, ffi_utils):
        tts_ffi = TtsFFI(use_json_api=False)

        message = RegisterSoundMessage("sound_id", b'')

        tts_ffi.publish_register_sound(message)
        ffi_utils.hermes_tts_publish_register_sound.assert_called_once()
