use failure::Fallible;
use ffi_utils::CReprOf;

use crate::protocol_handler::UserData;

pub fn structure_ptr_to_callback<T, U>(
    ptr: Option<unsafe extern "C" fn(*const U, *mut libc::c_void)>,
    user_data: UserData,
) -> Fallible<hermes::Callback<T>>
where
    T: Clone + Sync,
    U: CReprOf<T> + Sync + 'static,
{
    if let Some(ptr) = ptr {
        Ok(hermes::Callback::new(move |payload: &T| {
            let param = Box::into_raw(Box::new(U::c_repr_of(payload.clone()).unwrap()));
            unsafe { ptr(param, user_data.0) }
        }))
    } else {
        Err(failure::format_err!("null pointer"))
    }
}

#[macro_export]
macro_rules! generate_destroy {
    ($c_symbol:ident for $cstruct:ty) => {
        #[no_mangle]
        pub unsafe extern "C" fn $c_symbol(cstruct: *const $cstruct) -> ffi_utils::SNIPS_RESULT {
            use ffi_utils::RawPointerConverter;

            let _ = <$cstruct as RawPointerConverter<$cstruct>>::from_raw_pointer(cstruct);
            ffi_utils::SNIPS_RESULT::SNIPS_RESULT_OK
        }
    };
}

#[macro_export]
macro_rules! generate_facade_publish {
    ($c_symbol:ident = $facade:ty:$method:ident($( + $qualifier_name:ident : $qualifier:ty as $qualifier_raw:ty,)* $arg:ty)) => {
        #[no_mangle]
        pub extern "C" fn $c_symbol(facade : *const $facade, $($qualifier_name : *const $qualifier_raw,)* message : *const $arg) -> ffi_utils::SNIPS_RESULT {
            fn fun(facade : *const $facade, $($qualifier_name : *const $qualifier_raw,)* message : *const $arg) -> failure::Fallible<()> {
                use ffi_utils::{AsRust, RawBorrow};

                let message = unsafe { (*message).as_rust() }?;
                unsafe {(*facade).extract().$method($(<$qualifier as RawBorrow<$qualifier_raw>>::raw_borrow($qualifier_name)?.as_rust()?,)* message)}
            }

            ffi_utils::wrap!(fun(facade, $($qualifier_name,)* message))
        }
    };
    ($c_symbol:ident = $facade:ty:$method:ident($( + $qualifier_name:ident : $qualifier:ty as $qualifier_raw:ty,)*)) => {
        #[no_mangle]
        pub extern "C" fn $c_symbol(facade : *const $facade, $($qualifier_name : *const $qualifier_raw,)*) -> ffi_utils::SNIPS_RESULT {
            fn fun(facade : *const $facade, $($qualifier_name : *const $qualifier_raw,)*) -> failure::Fallible<()> {
                use ffi_utils::{AsRust, RawBorrow};

                unsafe {(*facade).extract().$method($(<$qualifier as RawBorrow<$qualifier_raw>>::raw_borrow($qualifier_name)?.as_rust()?,)*)}
            }

            ffi_utils::wrap!(fun(facade, $($qualifier_name,)*))
        }
    };
}

#[macro_export]
macro_rules! generate_facade_subscribe {
    ($c_symbol:ident = $facade:ty:$method:ident($( $filter_name:ident : $filter:ty as $filter_raw:ty,)* | $arg:ty|)) => {
        #[no_mangle]
        pub extern "C" fn $c_symbol(facade: *const $facade, $($filter_name : *const $filter_raw,)* handler: Option<unsafe extern "C" fn(*const $arg, *mut libc::c_void)>) -> ffi_utils::SNIPS_RESULT {
            fn fun(facade: *const $facade, $($filter_name : *const $filter_raw,)* handler: Option<unsafe extern "C" fn(*const $arg, *mut libc::c_void)>) -> failure::Fallible<()> {
                use ffi_utils::{AsRust, RawBorrow};

                let user_data = unsafe { (*facade).user_data().duplicate() };
                let callback = $crate::structure_ptr_to_callback(handler, user_data)?;
                unsafe { (*facade).extract().$method($(<$filter as RawBorrow<$filter_raw>>::raw_borrow($filter_name)?.as_rust()?,)* callback) }
            }

            ffi_utils::wrap!(fun(facade, $($filter_name,)* handler))
        }
    };
}

#[macro_export]
macro_rules! generate_structures_c_symbols {
    () => {
        pub mod structures {
            use super::LAST_ERROR;
            use super::facades::*;
            use hermes_ffi::ontology::*;

            $crate::generate_facade_publish!(hermes_sound_feedback_publish_toggle_on = CSoundFeedbackFacade: publish_toggle_on(CSiteMessage));
            $crate::generate_facade_publish!(hermes_sound_feedback_publish_toggle_off = CSoundFeedbackFacade: publish_toggle_off(CSiteMessage));

            $crate::generate_facade_subscribe!(hermes_dialogue_subscribe_session_queued = CDialogueFacade: subscribe_session_queued(|CSessionQueuedMessage|));
            $crate::generate_facade_subscribe!(hermes_dialogue_subscribe_session_started = CDialogueFacade: subscribe_session_started(|CSessionStartedMessage|));
            $crate::generate_facade_subscribe!(hermes_dialogue_subscribe_intent = CDialogueFacade: subscribe_intent(intent_name: std::ffi::CStr as libc::c_char, |CIntentMessage| ));
            $crate::generate_facade_subscribe!(hermes_dialogue_subscribe_intents = CDialogueFacade: subscribe_intents(|CIntentMessage|));
            $crate::generate_facade_subscribe!(hermes_dialogue_subscribe_intent_not_recognized = CDialogueFacade: subscribe_intent_not_recognized(|CIntentNotRecognizedMessage| ));
            $crate::generate_facade_subscribe!(hermes_dialogue_subscribe_session_ended = CDialogueFacade: subscribe_session_ended(|CSessionEndedMessage|));
            $crate::generate_facade_publish!(hermes_dialogue_publish_start_session = CDialogueFacade: publish_start_session(CStartSessionMessage));
            $crate::generate_facade_publish!(hermes_dialogue_publish_continue_session = CDialogueFacade: publish_continue_session(CContinueSessionMessage));
            $crate::generate_facade_publish!(hermes_dialogue_publish_end_session = CDialogueFacade: publish_end_session(CEndSessionMessage));

            $crate::generate_facade_publish!(hermes_injection_publish_injection_request = CInjectionFacade: publish_injection_request(CInjectionRequestMessage));
            $crate::generate_facade_publish!(hermes_injection_publish_injection_status_request = CInjectionFacade: publish_injection_status_request());
            $crate::generate_facade_subscribe!(hermes_injection_subscribe_injection_status = CInjectionFacade: subscribe_injection_status(|CInjectionStatusMessage|));

            $crate::generate_destroy!(hermes_drop_intent_message for CIntentMessage);
            $crate::generate_destroy!(hermes_drop_intent_not_recognized_message for CIntentNotRecognizedMessage);
            $crate::generate_destroy!(hermes_drop_session_started_message for CSessionStartedMessage);
            $crate::generate_destroy!(hermes_drop_session_queued_message for CSessionQueuedMessage);
            $crate::generate_destroy!(hermes_drop_session_ended_message for CSessionEndedMessage);
            $crate::generate_destroy!(hermes_drop_version_message for CVersionMessage);
            $crate::generate_destroy!(hermes_drop_error_message for CErrorMessage);
            $crate::generate_destroy!(hermes_drop_injection_status_message for CInjectionStatusMessage);

            #[cfg(feature = "full_bindings")]
            pub mod full_bindings {
                use super::super::LAST_ERROR;
                use super::super::facades::full_bindings::*;
                use hermes_ffi::ontology::*;

                $crate::generate_facade_subscribe!(hermes_hotword_subscribe_detected = CHotwordFacade: subscribe_detected(hotword_id: std::ffi::CStr as libc::c_char, |CHotwordDetectedMessage|));
                $crate::generate_facade_subscribe!(hermes_hotword_subscribe_all_detected = CHotwordFacade: subscribe_all_detected(|CHotwordDetectedMessage|));

                $crate::generate_facade_publish!(hermes_hotword_backend_publish_detected = CHotwordBackendFacade: publish_detected( + hotword_id: std::ffi::CStr as libc::c_char, CHotwordDetectedMessage));

                $crate::generate_facade_publish!(hermes_asr_publish_start_listening = CAsrFacade: publish_start_listening(CAsrStartListeningMessage));
                $crate::generate_facade_publish!(hermes_asr_publish_stop_listening = CAsrFacade: publish_stop_listening(CSiteMessage));
                $crate::generate_facade_subscribe!(hermes_asr_subscribe_text_captured = CAsrFacade: subscribe_text_captured(|CTextCapturedMessage|));
                $crate::generate_facade_subscribe!(hermes_asr_subscribe_partial_text_captured = CAsrFacade: subscribe_partial_text_captured(|CTextCapturedMessage|));

                $crate::generate_facade_subscribe!(hermes_asr_backend_publish_start_listening = CAsrBackendFacade: subscribe_start_listening(|CAsrStartListeningMessage|));
                $crate::generate_facade_subscribe!(hermes_asr_backend_publish_stop_listening = CAsrBackendFacade: subscribe_stop_listening(|CSiteMessage|));
                $crate::generate_facade_publish!(hermes_asr_backend_subscribe_text_captured = CAsrBackendFacade: publish_text_captured(CTextCapturedMessage));
                $crate::generate_facade_publish!(hermes_asr_backend_subscribe_partial_text_captured = CAsrBackendFacade: publish_partial_text_captured(CTextCapturedMessage));

                $crate::generate_facade_publish!(hermes_tts_publish_say = CTtsFacade: publish_say(CSayMessage));
                $crate::generate_facade_subscribe!(hermes_tts_subscribe_say_finished = CTtsFacade: subscribe_say_finished(|CSayFinishedMessage|));

                $crate::generate_facade_subscribe!(hermes_tts_backend_subscribe_say = CTtsBackendFacade: subscribe_say(|CSayMessage|));
                $crate::generate_facade_publish!(hermes_tts_backend_publish_say_finished = CTtsBackendFacade: publish_say_finished(CSayFinishedMessage));

                $crate::generate_facade_publish!(hermes_nlu_publish_query = CNluFacade: publish_query(CNluQueryMessage));
                $crate::generate_facade_publish!(hermes_nlu_publish_partial_query = CNluFacade: publish_partial_query(CNluSlotQueryMessage));
                $crate::generate_facade_subscribe!(hermes_nlu_subscribe_slot_parsed = CNluFacade: subscribe_slot_parsed(|CNluSlotMessage|));
                $crate::generate_facade_subscribe!(hermes_nlu_subscribe_intent_parsed = CNluFacade: subscribe_intent_parsed(|CNluIntentMessage|));
                $crate::generate_facade_subscribe!(hermes_nlu_subscribe_intent_not_recognized = CNluFacade: subscribe_intent_not_recognized(|CNluIntentNotRecognizedMessage|));

                $crate::generate_facade_subscribe!(hermes_nlu_backend_subscribe_query = CNluBackendFacade: subscribe_query(|CNluQueryMessage|));
                $crate::generate_facade_subscribe!(hermes_nlu_backend_subscribe_partial_query = CNluBackendFacade: subscribe_partial_query(|CNluSlotQueryMessage|));
                $crate::generate_facade_publish!(hermes_nlu_backend_publish_slot_parsed = CNluBackendFacade: publish_slot_parsed(CNluSlotMessage));
                $crate::generate_facade_publish!(hermes_nlu_backend_publish_intent_parsed = CNluBackendFacade: publish_intent_parsed(CNluIntentMessage));
                $crate::generate_facade_publish!(hermes_nlu_backend_publish_intent_not_recognized = CNluBackendFacade: publish_intent_not_recognized(CNluIntentNotRecognizedMessage));

                $crate::generate_facade_publish!(hermes_audio_server_publish_play_bytes = CAudioServerFacade: publish_play_bytes(CPlayBytesMessage));
                $crate::generate_facade_subscribe!(hermes_audio_server_subscribe_play_finished = CAudioServerFacade: subscribe_play_finished(site_id: std::ffi::CStr as libc::c_char, |CPlayFinishedMessage|));
                $crate::generate_facade_subscribe!(hermes_audio_server_subscribe_all_play_finished = CAudioServerFacade: subscribe_all_play_finished(|CPlayFinishedMessage|));
                $crate::generate_facade_subscribe!(hermes_audio_server_subscribe_audio_frame = CAudioServerFacade: subscribe_audio_frame(site_id: std::ffi::CStr as libc::c_char, |CAudioFrameMessage|));

                $crate::generate_facade_subscribe!(hermes_audio_server_backend_subscribe_play_bytes = CAudioServerBackendFacade: subscribe_play_bytes(site_id: std::ffi::CStr as libc::c_char, |CPlayBytesMessage|));
                $crate::generate_facade_subscribe!(hermes_audio_server_backend_subscribe_all_play_bytes = CAudioServerBackendFacade: subscribe_all_play_bytes(|CPlayBytesMessage|));
                $crate::generate_facade_publish!(hermes_audio_server_backend_publish_play_finished = CAudioServerBackendFacade: publish_play_finished(CPlayFinishedMessage));
                $crate::generate_facade_publish!(hermes_audio_server_backend_publish_audio_frame = CAudioServerBackendFacade: publish_audio_frame(CAudioFrameMessage));

                $crate::generate_facade_publish!(hermes_dialogue_backend_publish_session_queued = CDialogueBackendFacade: publish_session_queued(CSessionQueuedMessage));
                $crate::generate_facade_publish!(hermes_dialogue_backend_publish_session_started = CDialogueBackendFacade: publish_session_started(CSessionStartedMessage));
                $crate::generate_facade_publish!(hermes_dialogue_backend_publish_intent = CDialogueBackendFacade: publish_intent(CIntentMessage));
                $crate::generate_facade_publish!(hermes_dialogue_backend_publish_intent_not_recognized = CDialogueBackendFacade: publish_intent_not_recognized(CIntentNotRecognizedMessage));
                $crate::generate_facade_publish!(hermes_dialogue_backend_publish_session_ended = CDialogueBackendFacade: publish_session_ended(CSessionEndedMessage));
                $crate::generate_facade_subscribe!(hermes_dialogue_backend_subscribe_start_session = CDialogueBackendFacade: subscribe_start_session(|CStartSessionMessage|));
                $crate::generate_facade_subscribe!(hermes_dialogue_backend_subscribe_continue_session = CDialogueBackendFacade: subscribe_continue_session(|CContinueSessionMessage|));
                $crate::generate_facade_subscribe!(hermes_dialogue_backend_subscribe_end_session = CDialogueBackendFacade: subscribe_end_session(|CEndSessionMessage|));

                $crate::generate_destroy!(hermes_drop_site_message for CSiteMessage);
                $crate::generate_destroy!(hermes_drop_hotword_detected_message for CHotwordDetectedMessage);
                $crate::generate_destroy!(hermes_drop_text_captured_message for CTextCapturedMessage);
                $crate::generate_destroy!(hermes_drop_nlu_query_message for CNluQueryMessage);
                $crate::generate_destroy!(hermes_drop_nlu_slot_query_message for CNluSlotQueryMessage);
                $crate::generate_destroy!(hermes_drop_play_bytes_message for CPlayBytesMessage);
                $crate::generate_destroy!(hermes_drop_audio_frame_message for CAudioFrameMessage);
                $crate::generate_destroy!(hermes_drop_play_finished_message for CPlayFinishedMessage);
                $crate::generate_destroy!(hermes_drop_say_message for CSayMessage);
                $crate::generate_destroy!(hermes_drop_say_finished_message for CSayFinishedMessage);
                $crate::generate_destroy!(hermes_drop_nlu_slot_message for CNluSlotMessage);
                $crate::generate_destroy!(hermes_drop_nlu_intent_not_recognized_message for CNluIntentNotRecognizedMessage);
                $crate::generate_destroy!(hermes_drop_nlu_intent_message for CNluIntentMessage);
                $crate::generate_destroy!(hermes_drop_start_session_message for CStartSessionMessage);
                $crate::generate_destroy!(hermes_drop_continue_session_message for CContinueSessionMessage);
                $crate::generate_destroy!(hermes_drop_end_session_message for CEndSessionMessage);
                $crate::generate_destroy!(hermes_drop_injection_request_message for CInjectionRequestMessage);
            }
        }
    };
}
