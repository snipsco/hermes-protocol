#[macro_use]
extern crate failure;

use failure::Fallible;
use ffi_utils::*;
use hermes_ffi::*;
use libc;

generate_error_handling!(hermes_ffi_test_get_last_error);

#[cfg(feature = "json")]
mod json_round_trips;

#[cfg(feature = "json")]
pub use json_round_trips::*;

fn round_trip<T, U>(input: *const T, output: *mut *const T) -> Fallible<()>
where
    T: AsRust<U> + CReprOf<U>,
{
    let input = unsafe { input.as_ref() }.ok_or_else(|| format_err!("unexpected null pointer given as the message"))?;

    let rust_object = input.as_rust()?;

    let raw = T::c_repr_of(rust_object)?;

    let raw_pointer = raw.into_raw_pointer();

    unsafe {
        *output = raw_pointer;
    }
    Ok(())
}

#[no_mangle]
pub extern "C" fn hermes_ffi_test_round_trip_session_queued(
    input: *const hermes_ffi::CSessionQueuedMessage,
    output: *mut *const hermes_ffi::CSessionQueuedMessage,
) -> ffi_utils::SNIPS_RESULT {
    wrap!(round_trip(input, output))
}

#[no_mangle]
pub extern "C" fn hermes_ffi_test_round_trip_session_started(
    input: *const hermes_ffi::CSessionStartedMessage,
    output: *mut *const hermes_ffi::CSessionStartedMessage,
) -> ffi_utils::SNIPS_RESULT {
    wrap!(round_trip(input, output))
}

#[no_mangle]
pub extern "C" fn hermes_ffi_test_round_trip_session_ended(
    input: *const hermes_ffi::CSessionEndedMessage,
    output: *mut *const hermes_ffi::CSessionEndedMessage,
) -> ffi_utils::SNIPS_RESULT {
    wrap!(round_trip(input, output))
}

#[no_mangle]
pub extern "C" fn hermes_ffi_test_round_trip_intent(
    input: *const hermes_ffi::CIntentMessage,
    output: *mut *const hermes_ffi::CIntentMessage,
) -> ffi_utils::SNIPS_RESULT {
    wrap!(round_trip(input, output))
}

#[no_mangle]
pub extern "C" fn hermes_ffi_test_round_trip_intent_not_recognized(
    input: *const hermes_ffi::CIntentNotRecognizedMessage,
    output: *mut *const hermes_ffi::CIntentNotRecognizedMessage,
) -> ffi_utils::SNIPS_RESULT {
    wrap!(round_trip(input, output))
}

#[no_mangle]
pub extern "C" fn hermes_ffi_test_round_trip_start_session(
    input: *const hermes_ffi::CStartSessionMessage,
    output: *mut *const hermes_ffi::CStartSessionMessage,
) -> ffi_utils::SNIPS_RESULT {
    wrap!(round_trip(input, output))
}

#[no_mangle]
pub extern "C" fn hermes_ffi_test_round_trip_continue_session(
    input: *const hermes_ffi::CContinueSessionMessage,
    output: *mut *const hermes_ffi::CContinueSessionMessage,
) -> ffi_utils::SNIPS_RESULT {
    wrap!(round_trip(input, output))
}

#[no_mangle]
pub extern "C" fn hermes_ffi_test_round_trip_end_session(
    input: *const hermes_ffi::CEndSessionMessage,
    output: *mut *const hermes_ffi::CEndSessionMessage,
) -> ffi_utils::SNIPS_RESULT {
    wrap!(round_trip(input, output))
}

#[no_mangle]
pub extern "C" fn hermes_ffi_test_round_trip_injection_request(
    input: *const hermes_ffi::CInjectionRequestMessage,
    output: *mut *const hermes_ffi::CInjectionRequestMessage,
) -> ffi_utils::SNIPS_RESULT {
    wrap!(round_trip(input, output))
}

#[no_mangle]
pub extern "C" fn hermes_ffi_test_round_trip_injection_complete_message(
    input: *const hermes_ffi::CInjectionCompleteMessage,
    output: *mut *const hermes::CInjectionCompleteMessage,
) -> ffi_utils::SNIPS_RESULT {
    wrap!(round_trip(input, output))
}

#[no_mangle]
pub extern "C" fn hermes_ffi_test_round_trip_map_string_to_string_array(
    input: *const hermes_ffi::CMapStringToStringArray,
    output: *mut *const hermes_ffi::CMapStringToStringArray,
) -> ffi_utils::SNIPS_RESULT {
    wrap!(round_trip(input, output))
}

#[no_mangle]
pub extern "C" fn hermes_ffi_test_round_trip_register_sound(
    input: *const hermes_ffi::CRegisterSoundMessage,
    output: *mut *const hermes_ffi::CRegisterSoundMessage,
) -> ffi_utils::SNIPS_RESULT {
    wrap!(round_trip(input, output))
}

#[no_mangle]
pub extern "C" fn hermes_ffi_test_round_trip_dialogue_configure_intent(
    input: *const hermes_ffi::CDialogueConfigureIntent,
    output: *mut *const hermes_ffi::CDialogueConfigureIntent,
) -> ffi_utils::SNIPS_RESULT {
    wrap!(round_trip(input, output))
}

#[no_mangle]
pub extern "C" fn hermes_ffi_test_round_trip_dialogue_configure_intent_array(
    input: *const hermes_ffi::CDialogueConfigureIntentArray,
    output: *mut *const hermes_ffi::CDialogueConfigureIntentArray,
) -> ffi_utils::SNIPS_RESULT {
    wrap!(round_trip(input, output))
}

#[no_mangle]
pub extern "C" fn hermes_ffi_test_round_trip_dialogue_configure(
    input: *const hermes_ffi::CDialogueConfigureMessage,
    output: *mut *const hermes_ffi::CDialogueConfigureMessage,
) -> ffi_utils::SNIPS_RESULT {
    wrap!(round_trip(input, output))
}

#[no_mangle]
pub extern "C" fn hermes_ffi_test_round_trip_asr_token(
    input: *const hermes_ffi::CAsrToken,
    output: *mut *const hermes_ffi::CAsrToken,
) -> ffi_utils::SNIPS_RESULT {
    wrap!(round_trip(input, output))
}

#[no_mangle]
pub extern "C" fn hermes_ffi_test_round_trip_asr_token_array(
    input: *const hermes_ffi::CAsrTokenArray,
    output: *mut *const hermes_ffi::CAsrTokenArray,
) -> ffi_utils::SNIPS_RESULT {
    wrap!(round_trip(input, output))
}

#[no_mangle]
pub extern "C" fn hermes_ffi_test_round_trip_asr_token_double_array(
    input: *const hermes_ffi::CAsrTokenDoubleArray,
    output: *mut *const hermes_ffi::CAsrTokenDoubleArray,
) -> ffi_utils::SNIPS_RESULT {
    wrap!(round_trip(input, output))
}

#[no_mangle]
pub extern "C" fn hermes_ffi_test_round_trip_text_captured(
    input: *const hermes_ffi::CTextCapturedMessage,
    output: *mut *const hermes_ffi::CTextCapturedMessage,
) -> ffi_utils::SNIPS_RESULT {
    wrap!(round_trip(input, output))
}

#[no_mangle]
pub unsafe extern "C" fn hermes_ffi_test_destroy_string(string: *mut libc::c_char) -> SNIPS_RESULT {
    wrap!(std::ffi::CString::from_raw_pointer(string))
}

#[no_mangle]
pub unsafe extern "C" fn hermes_ffi_test_destroy_map_string_to_string_array(
    input: *mut CMapStringToStringArray,
) -> SNIPS_RESULT {
    wrap!(CMapStringToStringArray::drop_raw_pointer(input))
}

#[no_mangle]
pub unsafe extern "C" fn hermes_ffi_test_destroy_asr_token(input: *mut hermes_ffi::CAsrToken) -> SNIPS_RESULT {
    wrap!(hermes_ffi::CAsrToken::drop_raw_pointer(input))
}

#[no_mangle]
pub unsafe extern "C" fn hermes_ffi_test_destroy_asr_token_array(
    input: *mut hermes_ffi::CAsrTokenArray,
) -> SNIPS_RESULT {
    wrap!(hermes_ffi::CAsrTokenArray::drop_raw_pointer(input))
}

#[no_mangle]
pub unsafe extern "C" fn hermes_ffi_test_destroy_asr_token_double_array(
    input: *mut hermes_ffi::CAsrTokenDoubleArray,
) -> SNIPS_RESULT {
    wrap!(hermes_ffi::CAsrTokenDoubleArray::drop_raw_pointer(input))
}

generate_hermes_c_symbols!();
