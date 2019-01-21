use failure::Fallible;
use failure::ResultExt;

use ffi_utils::*;

#[repr(C)]
#[derive(Debug)]
pub struct CHotwordDetectedMessage {
    pub site_id: *const libc::c_char,
    pub model_id: *const libc::c_char,
}

unsafe impl Sync for CHotwordDetectedMessage {}

impl CReprOf<hermes::HotwordDetectedMessage> for CHotwordDetectedMessage {
    fn c_repr_of(input: hermes::HotwordDetectedMessage) -> Fallible<Self> {
        Ok(Self {
            site_id: convert_to_c_string!(input.site_id),
            model_id: convert_to_c_string!(input.model_id),
        })
    }
}

impl AsRust<hermes::HotwordDetectedMessage> for CHotwordDetectedMessage {
    fn as_rust(&self) -> Fallible<hermes::HotwordDetectedMessage> {
        Ok(hermes::HotwordDetectedMessage {
            site_id: create_rust_string_from!(self.site_id),
            model_id: create_rust_string_from!(self.model_id),
            model_version: None,
            model_type: None,
            current_sensitivity: None,
            detection_signal_ms: None,
            end_signal_ms: None,
        })
    }
}

impl Drop for CHotwordDetectedMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.site_id);
        take_back_c_string!(self.model_id);
    }
}
