use failure::Fallible;
use hermes::ontology::HermesMessage;

use crate::UserData;

pub type CJsonCallback = unsafe extern "C" fn(*const libc::c_char, *mut libc::c_void);

pub fn json_ptr_to_callback<'de, T>(ptr: Option<CJsonCallback>, user_data: UserData) -> Fallible<hermes::Callback<T>>
where
    T: HermesMessage<'de>,
{
    match ptr {
        Some(ptr) => Ok(hermes::Callback::new(move |payload: &T| {
            let json = serde_json::to_string(&payload).expect("json serialization failed");
            let c_string = std::ffi::CString::new(json).expect("CString::new failed");
            unsafe { ptr(c_string.as_ptr(), user_data.0) }
        })),
        None => failure::bail!("null pointer"),
    }
}

pub fn json_from_slice<'a, T>(v: &'a [u8]) -> Fallible<T>
where
    T: HermesMessage<'a>,
{
    Ok(serde_json::from_slice(v)?)
}

#[macro_export]
macro_rules! generate_facade_publish_json {
    ($c_symbol:ident = $facade:ty:$method:ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn $c_symbol(facade: *const $facade) -> ffi_utils::SNIPS_RESULT {
            ffi_utils::wrap!(unsafe { (*facade).extract() }.$method())
        }
    };

    ($c_symbol:ident = $facade:ty:$method:ident($($filter_name:ident)*)) => {
        #[no_mangle]
        pub extern "C" fn $c_symbol(
            facade: *const $facade,
            $($filter_name: *const libc::c_char,)*
            message: *const libc::c_char,
        ) -> ffi_utils::SNIPS_RESULT {
            fn fun(
                facade: *const $facade,
                $($filter_name: *const libc::c_char,)*
                message: *const libc::c_char,
            ) -> failure::Fallible<()> {
                use std::ffi::CStr;

                let c_str = unsafe { CStr::from_ptr(message) };
                let message = $crate::json_from_slice(c_str.to_bytes())?;

                unsafe { (*facade).extract() }.$method(
                    $(unsafe { CStr::from_ptr($filter_name) }.to_string_lossy().into_owned(),)*
                    message,
                )
            }
            ffi_utils::wrap!(fun(facade, $($filter_name,)* message))
        }
    };
}

#[macro_export]
macro_rules! generate_facade_subscribe_json {
    ($c_symbol:ident = $facade:ty:$method:ident($($filter_name:ident)*)) => {
        #[no_mangle]
        pub extern "C" fn $c_symbol(
            facade: *const $facade,
            $($filter_name: *const libc::c_char,)*
            handler: Option<unsafe extern "C" fn(*const libc::c_char, *mut libc::c_void)>,
        ) -> ffi_utils::SNIPS_RESULT {

            fn fun(facade: *const $facade,
                $($filter_name: *const libc::c_char,)*
                handler: Option<$crate::CJsonCallback>,
            )-> failure::Fallible<()> {
                use std::ffi::CStr;

                let user_data = unsafe { (*facade).user_data().duplicate() };
                let callback = $crate::json_ptr_to_callback(handler, user_data)?;

                unsafe { (*facade).extract() }.$method(
                    $(unsafe { CStr::from_ptr($filter_name) }.to_string_lossy().into_owned(),)*
                    callback,
                )
            }

            ffi_utils::wrap!(fun(facade, $($filter_name,)* handler))
        }
    };
}

#[macro_export]
macro_rules! generate_json_c_symbols {
    () => {
        #[rustfmt::skip]
        pub mod json {
            use super::facades::*;
            use super::LAST_ERROR;

            $crate::generate_facade_publish_json!(hermes_sound_feedback_publish_toggle_on_json = CSoundFeedbackFacade: publish_toggle_on());
            $crate::generate_facade_publish_json!(hermes_sound_feedback_publish_toggle_off_json = CSoundFeedbackFacade: publish_toggle_off());

            $crate::generate_facade_subscribe_json!(hermes_dialogue_subscribe_session_queued_json = CDialogueFacade: subscribe_session_queued());
            $crate::generate_facade_subscribe_json!(hermes_dialogue_subscribe_session_started_json = CDialogueFacade: subscribe_session_started());
            $crate::generate_facade_subscribe_json!(hermes_dialogue_subscribe_intent_json = CDialogueFacade: subscribe_intent(intent_name));
            $crate::generate_facade_subscribe_json!(hermes_dialogue_subscribe_intents_json = CDialogueFacade: subscribe_intents());
            $crate::generate_facade_subscribe_json!(hermes_dialogue_subscribe_intent_not_recognized_json = CDialogueFacade: subscribe_intent_not_recognized());
            $crate::generate_facade_subscribe_json!(hermes_dialogue_subscribe_session_ended_json = CDialogueFacade: subscribe_session_ended());
            $crate::generate_facade_publish_json!(hermes_dialogue_publish_start_session_json = CDialogueFacade: publish_start_session());
            $crate::generate_facade_publish_json!(hermes_dialogue_publish_continue_session_json = CDialogueFacade: publish_continue_session());
            $crate::generate_facade_publish_json!(hermes_dialogue_publish_end_session_json = CDialogueFacade: publish_end_session());

            $crate::generate_facade_publish_json!(hermes_injection_publish_injection_request_json = CInjectionFacade: publish_injection_request());
            $crate::generate_facade_publish_json!(hermes_injection_publish_injection_status_request_json = CInjectionFacade: publish_injection_status_request);
            $crate::generate_facade_subscribe_json!(hermes_injection_subscribe_injection_status_json = CInjectionFacade: subscribe_injection_status());

            #[cfg(feature = "full_bindings")]
            pub mod full_bindings {
                use super::super::facades::full_bindings::*;
                use super::super::LAST_ERROR;

                $crate::generate_facade_subscribe_json!(hermes_hotword_subscribe_detected_json = CHotwordFacade: subscribe_detected(hotword_id));
                $crate::generate_facade_subscribe_json!(hermes_hotword_subscribe_all_detected_json = CHotwordFacade: subscribe_all_detected());

                $crate::generate_facade_publish_json!(hermes_hotword_backend_publish_detected_json = CHotwordBackendFacade: publish_detected(hotword_id));

                $crate::generate_facade_publish_json!(hermes_asr_publish_start_listening_json = CAsrFacade: publish_start_listening());
                $crate::generate_facade_publish_json!(hermes_asr_publish_stop_listening_json = CAsrFacade: publish_stop_listening());
                $crate::generate_facade_subscribe_json!(hermes_asr_subscribe_text_captured_json = CAsrFacade: subscribe_text_captured());
                $crate::generate_facade_subscribe_json!(hermes_asr_subscribe_partial_text_captured_json = CAsrFacade: subscribe_partial_text_captured());

                $crate::generate_facade_subscribe_json!(hermes_asr_backend_publish_start_listening_json = CAsrBackendFacade: subscribe_start_listening());
                $crate::generate_facade_subscribe_json!(hermes_asr_backend_publish_stop_listening_json = CAsrBackendFacade: subscribe_stop_listening());
                $crate::generate_facade_publish_json!(hermes_asr_backend_subscribe_text_captured_json = CAsrBackendFacade: publish_text_captured());
                $crate::generate_facade_publish_json!(hermes_asr_backend_subscribe_partial_text_captured_json = CAsrBackendFacade: publish_partial_text_captured());

                $crate::generate_facade_publish_json!(hermes_tts_publish_say_json = CTtsFacade: publish_say());
                $crate::generate_facade_subscribe_json!(hermes_tts_subscribe_say_finished_json = CTtsFacade: subscribe_say_finished());

                $crate::generate_facade_subscribe_json!(hermes_tts_backend_subscribe_say_json = CTtsBackendFacade: subscribe_say());
                $crate::generate_facade_publish_json!(hermes_tts_backend_publish_say_finished_json = CTtsBackendFacade: publish_say_finished());

                $crate::generate_facade_publish_json!(hermes_nlu_publish_query_json = CNluFacade: publish_query());
                $crate::generate_facade_publish_json!(hermes_nlu_publish_partial_query_json = CNluFacade: publish_partial_query());
                $crate::generate_facade_subscribe_json!(hermes_nlu_subscribe_slot_parsed_json = CNluFacade: subscribe_slot_parsed());
                $crate::generate_facade_subscribe_json!(hermes_nlu_subscribe_intent_parsed_json = CNluFacade: subscribe_intent_parsed());
                $crate::generate_facade_subscribe_json!(hermes_nlu_subscribe_intent_not_recognized_json = CNluFacade: subscribe_intent_not_recognized());

                $crate::generate_facade_subscribe_json!(hermes_nlu_backend_subscribe_query_json = CNluBackendFacade: subscribe_query());
                $crate::generate_facade_subscribe_json!(hermes_nlu_backend_subscribe_partial_query_json = CNluBackendFacade: subscribe_partial_query());
                $crate::generate_facade_publish_json!(hermes_nlu_backend_publish_slot_parsed_json = CNluBackendFacade: publish_slot_parsed());
                $crate::generate_facade_publish_json!(hermes_nlu_backend_publish_intent_parsed_json = CNluBackendFacade: publish_intent_parsed());
                $crate::generate_facade_publish_json!(hermes_nlu_backend_publish_intent_not_recognized_json = CNluBackendFacade: publish_intent_not_recognized());

                $crate::generate_facade_publish_json!(hermes_audio_server_publish_play_bytes_json = CAudioServerFacade: publish_play_bytes());
                $crate::generate_facade_subscribe_json!(hermes_audio_server_subscribe_play_finished_json = CAudioServerFacade: subscribe_play_finished(site_id));
                $crate::generate_facade_subscribe_json!(hermes_audio_server_subscribe_all_play_finished_json = CAudioServerFacade: subscribe_all_play_finished());
                $crate::generate_facade_subscribe_json!(hermes_audio_server_subscribe_audio_frame_json = CAudioServerFacade: subscribe_audio_frame(site_id));

                $crate::generate_facade_subscribe_json!(hermes_audio_server_backend_subscribe_play_bytes_json = CAudioServerBackendFacade: subscribe_play_bytes(site_id));
                $crate::generate_facade_subscribe_json!(hermes_audio_server_backend_subscribe_all_play_bytes_json = CAudioServerBackendFacade: subscribe_all_play_bytes());
                $crate::generate_facade_publish_json!(hermes_audio_server_backend_publish_play_finished_json = CAudioServerBackendFacade: publish_play_finished());
                $crate::generate_facade_publish_json!(hermes_audio_server_backend_publish_audio_frame_json = CAudioServerBackendFacade: publish_audio_frame());

                $crate::generate_facade_publish_json!(hermes_dialogue_backend_publish_session_queued_json = CDialogueBackendFacade: publish_session_queued());
                $crate::generate_facade_publish_json!(hermes_dialogue_backend_publish_session_started_json = CDialogueBackendFacade: publish_session_started());
                $crate::generate_facade_publish_json!(hermes_dialogue_backend_publish_intent_json = CDialogueBackendFacade: publish_intent());
                $crate::generate_facade_publish_json!(hermes_dialogue_backend_publish_intent_not_recognized_json = CDialogueBackendFacade: publish_intent_not_recognized());
                $crate::generate_facade_publish_json!(hermes_dialogue_backend_publish_session_ended_json = CDialogueBackendFacade: publish_session_ended());
                $crate::generate_facade_subscribe_json!(hermes_dialogue_backend_subscribe_start_session_json = CDialogueBackendFacade: subscribe_start_session());
                $crate::generate_facade_subscribe_json!(hermes_dialogue_backend_subscribe_continue_session_json = CDialogueBackendFacade: subscribe_continue_session());
                $crate::generate_facade_subscribe_json!(hermes_dialogue_backend_subscribe_end_session_json = CDialogueBackendFacade: subscribe_end_session());
            }
        }
    };
}
