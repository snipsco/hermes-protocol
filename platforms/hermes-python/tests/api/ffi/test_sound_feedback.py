import mock
from hermes_python.api.ffi import FFI
from hermes_python.ontology.soundfeedback import SiteMessage


@mock.patch("hermes_python.api.ffi.sound_feedback.utils")
def test_publish_(ffi_utils):
    ffi = FFI(use_json_api=False)
    site_message = SiteMessage("default")
    ffi.sound_feedback.publish_toggle_on(site_message)
    ffi_utils.hermes_sound_feedback_publish_toggle_on.assert_called_once()
