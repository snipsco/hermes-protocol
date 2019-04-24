import mock
from hermes_python.api.ffi.feedback import SoundFeedBackFFI
from hermes_python.ontology.feedback import SiteMessage


@mock.patch("hermes_python.api.ffi.feedback.utils")
def test_publish_toggle_on(ffi_utils):
    ffi = SoundFeedBackFFI(use_json_api=False)
    site_message = SiteMessage("default")
    ffi.publish_toggle_on(site_message)
    ffi_utils.hermes_sound_feedback_publish_toggle_on.assert_called_once()


@mock.patch("hermes_python.api.ffi.feedback.utils")
def test_publish_toggle_off(ffi_utils):
    ffi = SoundFeedBackFFI(use_json_api=False)
    site_message = SiteMessage("default")
    ffi.publish_toggle_off(site_message)
    ffi_utils.hermes_sound_feedback_publish_toggle_off.assert_called_once()
