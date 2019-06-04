use failure::Fallible;
use ffi_utils::*;
use hermes;
use serde_json;

use crate::LAST_ERROR;

fn round_trip_json<'de, T>(input: *const libc::c_char, output: *mut *const libc::c_char) -> Fallible<()>
where
    T: hermes::HermesMessage<'de>,
{
    let input = unsafe { std::ffi::CStr::from_ptr(input) }.to_str()?;

    let rust_object = serde_json::from_str::<T>(&input)?;

    let new_string = serde_json::to_string(&rust_object)?;

    point_to_string(output, new_string)?;

    Ok(())
}

macro_rules! round_trip_json {
    ($c_symbol:ident, $repr_type:ty) => {
        #[no_mangle]
        pub extern "C" fn $c_symbol(
            input: *const libc::c_char,
            output: *mut *const libc::c_char,
        ) -> ffi_utils::SNIPS_RESULT {
            wrap!(round_trip_json::<$repr_type>(input, output))
        }
    };
}

round_trip_json!(
    hermes_ffi_test_round_trip_session_queued_json,
    hermes::SessionQueuedMessage
);

round_trip_json!(
    hermes_ffi_test_round_trip_session_started_json,
    hermes::SessionStartedMessage
);

round_trip_json!(
    hermes_ffi_test_round_trip_session_ended_json,
    hermes::SessionEndedMessage
);

round_trip_json!(hermes_ffi_test_round_trip_intent_json, hermes::IntentMessage);

round_trip_json!(
    hermes_ffi_test_round_trip_intent_not_recognized_json,
    hermes::IntentNotRecognizedMessage
);

round_trip_json!(
    hermes_ffi_test_round_trip_start_session_json,
    hermes::StartSessionMessage
);

round_trip_json!(
    hermes_ffi_test_round_trip_continue_session_json,
    hermes::ContinueSessionMessage
);

round_trip_json!(hermes_ffi_test_round_trip_end_session_json, hermes::EndSessionMessage);

round_trip_json!(
    hermes_ffi_test_round_trip_injection_request_json,
    hermes::InjectionRequestMessage
);

round_trip_json!(
    hermes_ffi_test_round_trip_injection_complete_json,
    hermes::InjectionCompleteMessage
);

round_trip_json!(
    hermes_ffi_test_round_trip_injection_reset_request_json,
    hermes::InjectionResetRequestMessage
);

round_trip_json!(
    hermes_ffi_test_round_trip_injection_reset_complete_json,
    hermes::InjectionResetCompleteMessage
);

round_trip_json!(
    hermes_ffi_test_round_trip_register_sound_json,
    hermes::RegisterSoundMessage
);

round_trip_json!(
    hermes_ffi_test_round_trip_dialogue_configure_json,
    hermes::DialogueConfigureMessage
);

round_trip_json!(
    hermes_ffi_test_round_trip_text_captured_json,
    hermes::TextCapturedMessage
);
