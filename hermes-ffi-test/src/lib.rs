#[macro_use]
extern crate failure;
#[macro_use]
extern crate ffi_utils;
extern crate hermes;
#[macro_use]
extern crate hermes_ffi;
#[macro_use]
extern crate lazy_static;
extern crate libc;

use hermes_ffi::*;

use ffi_utils::*;


generate_error_handling!(hermes_ffi_test_get_last_error);

fn round_trip<T, U>(input : *const T, output : *mut *const T) -> hermes::Result<()>
    where T: AsRust<U> + CReprOf<U> {
    let input = unsafe { input.as_ref() }
        .ok_or_else(|| format_err!("unexpected null pointer given as the message"))?;


    let rust_object = input
        .as_rust()?;

    let raw = T::c_repr_of(rust_object)?;

    let raw_pointer = raw.into_raw_pointer();

    unsafe { *output = raw_pointer; }
    Ok(())
}


#[no_mangle]
pub extern "C" fn hermes_ffi_test_round_trip_session_queued(
    input: *const hermes_ffi::CSessionQueuedMessage, output:
    *mut *const hermes_ffi::CSessionQueuedMessage) -> ffi_utils::SNIPS_RESULT {
    wrap!(round_trip(input, output))
}

#[no_mangle]
pub extern "C" fn hermes_ffi_test_round_trip_session_started(
    input: *const hermes_ffi::CSessionStartedMessage, output:
    *mut *const hermes_ffi::CSessionStartedMessage) -> ffi_utils::SNIPS_RESULT {
    wrap!(round_trip(input, output))
}

#[no_mangle]
pub extern "C" fn hermes_ffi_test_round_trip_session_ended(
    input: *const hermes_ffi::CSessionEndedMessage, output:
    *mut *const hermes_ffi::CSessionEndedMessage) -> ffi_utils::SNIPS_RESULT {
    wrap!(round_trip(input, output))
}

#[no_mangle]
pub extern "C" fn hermes_ffi_test_round_trip_intent(
    input: *const hermes_ffi::CIntentMessage, output:
    *mut *const hermes_ffi::CIntentMessage) -> ffi_utils::SNIPS_RESULT {
    wrap!(round_trip(input, output))
}

#[no_mangle]
pub extern "C" fn hermes_ffi_test_round_trip_start_session(
    input: *const hermes_ffi::CStartSessionMessage, output:
    *mut *const hermes_ffi::CStartSessionMessage) -> ffi_utils::SNIPS_RESULT {
    wrap!(round_trip(input, output))
}

#[no_mangle]
pub extern "C" fn hermes_ffi_test_round_trip_continue_session(
    input: *const hermes_ffi::CContinueSessionMessage, output:
    *mut *const hermes_ffi::CContinueSessionMessage) -> ffi_utils::SNIPS_RESULT {
    wrap!(round_trip(input, output))
}

#[no_mangle]
pub extern "C" fn hermes_ffi_test_round_trip_end_session(
    input: *const hermes_ffi::CEndSessionMessage, output:
    *mut *const hermes_ffi::CEndSessionMessage) -> ffi_utils::SNIPS_RESULT {
    wrap!(round_trip(input, output))
}

#[no_mangle]
pub extern "C" fn hermes_ffi_test_destroy_string(string: *mut libc::c_char) -> SNIPS_RESULT {
    wrap!(unsafe { ::std::ffi::CString::from_raw_pointer(string) })
}


generate_hermes_c_symbols!();



