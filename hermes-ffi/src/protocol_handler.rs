use hermes;
use hermes::HermesProtocolHandler;
use hermes_mqtt;
use libc;

use failure::ResultExt;
use hermes::ResultExt as HResultExt;
use ffi_utils:: {RawPointerConverter, RawBorrow, CReprOf, AsRust, LAST_ERROR, C_RESULT};

use ontology::*;

#[repr(C)]
pub struct CProtocolHandler {
    handler: *const HermesProtocolHandler
}

#[no_mangle]
pub extern "C" fn hermes_protocol_handler_new_mqtt(handler: *mut *const CProtocolHandler) -> C_RESULT {
    // TODO move that elsewhere, and destructor
    fn doit(handler: *mut *const CProtocolHandler) -> hermes::Result<()>{
        let cph = CProtocolHandler { handler: Box::into_raw(Box::new(hermes_mqtt::MqttHermesProtocolHandler::new("localhost:1883")?)) };
        let ptr = CProtocolHandler::into_raw_pointer(cph);
        unsafe {
            *handler = ptr;
        }
        Ok(())
    }
    wrap!(doit(handler))
}

#[no_mangle]
pub extern "C" fn hermes_get_last_error(error: *mut *const libc::c_char) -> C_RESULT {
    wrap!(get_last_error(error))
}


fn get_last_error(error: *mut *const libc::c_char) -> hermes::Result<()> {
    point_to_string(error, LAST_ERROR.lock()?.clone())
}


// TODO remove and directly use the ffi_utils version once we get rid of error-chain
fn point_to_string(pointer: *mut *const libc::c_char, string: String) -> hermes::Result<()> {
    Ok(::ffi_utils::point_to_string(pointer, string).compat().chain_err(||"could not convert to C Repr")?)
}

fn convert<T, U: AsRust<T>>(raw : *const U) -> hermes::Result<T> {
    unsafe { (*raw).as_rust().compat().chain_err(||"could not convert pointer to rust struct") }
}

fn ptr_to_callback<T,U>(ptr: Option<unsafe extern "C" fn(*const U)>) -> hermes::Result<hermes::Callback<T>>
    where
        T: Clone + Sync,
        U: CReprOf<T> + Sync + 'static{
    if let Some(ptr) = ptr  {
        Ok(hermes::Callback::new(move |payload: &T| {
            let param = Box::into_raw(Box::new(U::c_repr_of(payload.clone()).unwrap()));
            unsafe { ptr(param) }
        }))
    } else {
        // TODO replace this non-sense by a bail! once we get rid of error-chain
        Err(format_err!("null pointer")).compat().chain_err(|| "got a null pointer")
    }
}


macro_rules! generate_facade_wrapper {
    ($wrapper_name:ident for $facade:ty, $drop_name:ident, $getter_name:ident = handler.$getter:ident) => {
        #[repr(C)]
        pub struct $wrapper_name {
            facade: *const $facade
        }

        impl $wrapper_name {
            pub fn from(facade: Box<$facade>) -> Self {
                Self { facade: Box::into_raw(facade) }
            }

            pub fn extract(&self) -> &$facade {
                unsafe { &(*self.facade) }
            }
        }

        impl Drop for $wrapper_name {
            fn drop(&mut self) {
                unsafe { Box::from_raw(self.facade as *mut $facade) };
            }
        }

        #[no_mangle]
        pub extern "C" fn $drop_name(wrapper : *mut $wrapper_name) -> C_RESULT {
            let _ = unsafe { $wrapper_name::from_raw_pointer(wrapper) };
            C_RESULT::OK
        }

        #[no_mangle]
        pub extern "C" fn $getter_name(handler: *const CProtocolHandler, facade: *mut *const $wrapper_name) -> C_RESULT {
            fn fun(handler: *const CProtocolHandler, facade: *mut *const $wrapper_name) -> hermes::Result<()> {
                let pointer = $wrapper_name::into_raw_pointer($wrapper_name::from(unsafe { (*(*handler).handler).$getter() }));
                unsafe { *facade = pointer };
                Ok(())
            }

            wrap!(fun(handler, facade))
        }

    };
}

macro_rules! generate_facade_publish {
    ($c_symbol:ident = $facade:ty:$method:ident($( + $qualifier_name:ident : $qualifier:ty as $qualifier_raw:ty,)* $arg:ty)) => {
        #[no_mangle]
        pub extern "C" fn $c_symbol(facade : *const $facade, $($qualifier_name : *const $qualifier_raw,)* message : *const $arg) -> C_RESULT {
            fn fun(facade : *const $facade, $($qualifier_name : *const $qualifier_raw,)* message : *const $arg) -> hermes::Result<()> {
                let message = convert(message)?;
                // TODO remove all these .compat().chain_err() when we get rid of error-chain
                unsafe {(*facade).extract().$method($(<$qualifier as RawBorrow<$qualifier_raw>>::raw_borrow($qualifier_name).compat().chain_err(|| "could not borrow")?.as_rust().compat().chain_err(|| "could not convert to rust")?,)* message)}
            }

            wrap!(fun(facade, $($qualifier_name,)* message))
        }
    };
}


macro_rules! generate_facade_subscribe {
    ($c_symbol:ident = $facade:ty:$method:ident($( $filter_name:ident : $filter:ty as $filter_raw:ty,)* | $arg:ty | )) => {
        #[no_mangle]
        pub extern "C" fn $c_symbol(facade: *const $facade, $($filter_name : *const $filter_raw,)* handler: Option<unsafe extern "C" fn(*const $arg)>) -> C_RESULT {
            fn fun(facade: *const $facade, $($filter_name : *const $filter_raw,)* handler: Option<unsafe extern "C" fn(*const $arg)>) -> hermes::Result<()> {
                let callback = ptr_to_callback(handler)?;
                // TODO remove all these .compat().chain_err() when we get rid of error-chain
                unsafe { (*facade).extract().$method($(<$filter as RawBorrow<$filter_raw>>::raw_borrow($filter_name).compat().chain_err(|| "could not borrow")?.as_rust().compat().chain_err(|| "could not convert to rust")?,)* callback) }
            }

            wrap!(fun(facade, $($filter_name,)* handler))
        }
    };
}

generate_facade_wrapper!(CHotwordFacade for hermes::HotwordFacade,
                         hermes_drop_hotword_facade,
                         hermes_protocol_handler_hotword_facade = handler.hotword);
generate_facade_subscribe!(hermes_hotword_subscribe_detected = CHotwordFacade:subscribe_detected(hotword_id: ::std::ffi::CStr as libc::c_char, |CHotwordDetectedMessage|));
generate_facade_subscribe!(hermes_hotword_subscribe_all_detected = CHotwordFacade:subscribe_all_detected(|CHotwordDetectedMessage|));

generate_facade_wrapper!(CHotwordBackendFacade for hermes::HotwordBackendFacade,
                         hermes_drop_hotword_backend_facade,
                         hermes_protocol_handler_hotword_backend_facade = handler.hotword_backend);
generate_facade_publish!(hermes_hotword_backend_publish_detected = CHotwordBackendFacade:publish_detected(+hotword_id: ::std::ffi::CStr as libc::c_char, CHotwordDetectedMessage));

generate_facade_wrapper!(CSoundFeedbackFacade for hermes::SoundFeedbackFacade,
                         hermes_drop_sound_feedback_facade,
                         hermes_protocol_handler_sound_feedback_facade = handler.sound_feedback);

generate_facade_wrapper!(CSoundFeedbackBackendFacade for hermes::SoundFeedbackBackendFacade,
                         hermes_drop_sound_feedback_backend_facade,
                         hermes_protocol_handler_sound_feedback_backend_facade = handler.sound_feedback_backend);

generate_facade_wrapper!(CAsrFacade for hermes::AsrFacade,
                         hermes_drop_asr_facade,
                         hermes_protocol_handler_asr_facade = handler.asr);
generate_facade_publish!(hermes_asr_publish_start_listening = CAsrFacade:publish_start_listening(CSiteMessage));
generate_facade_publish!(hermes_asr_publish_stop_listening = CAsrFacade:publish_stop_listening(CSiteMessage));
generate_facade_subscribe!(hermes_asr_subscribe_text_captured = CAsrFacade:subscribe_text_captured(|CTextCapturedMessage|));
generate_facade_subscribe!(hermes_asr_subscribe_partial_text_captured = CAsrFacade:subscribe_partial_text_captured(|CTextCapturedMessage|));

generate_facade_wrapper!(CAsrBackendFacade for hermes::AsrBackendFacade,
                         hermes_drop_asr_backend_facade,
                         hermes_protocol_handler_asr_backend_facade = handler.asr_backend);
generate_facade_subscribe!(hermes_asr_bakend_publish_start_listening = CAsrBackendFacade:subscribe_start_listening(|CSiteMessage|));
generate_facade_subscribe!(hermes_asr_bakend_publish_stop_listening = CAsrBackendFacade:subscribe_stop_listening(|CSiteMessage|));
generate_facade_publish!(hermes_asr_bakend_subscribe_text_captured = CAsrBackendFacade:publish_text_captured(CTextCapturedMessage));
generate_facade_publish!(hermes_asr_bakend_subscribe_partial_text_captured = CAsrBackendFacade:publish_partial_text_captured(CTextCapturedMessage));

generate_facade_wrapper!(CTtsFacade for hermes::TtsFacade,
                         hermes_drop_tts_facade,
                         hermes_protocol_handler_tts_facade = handler.tts);
generate_facade_publish!(hermes_tts_publish_say = CTtsFacade:publish_say(CSayMessage));
generate_facade_subscribe!(hermes_tts_subscribe_say_finished = CTtsFacade:subscribe_say_finished(|CSayFinishedMessage|));

generate_facade_wrapper!(CTtsBackendFacade for hermes::TtsBackendFacade,
                         hermes_drop_tts_backend_facade,
                         hermes_protocol_handler_tts_backend_facade = handler.tts_backend);
generate_facade_subscribe!(hermes_tts_backend_subscribe_say = CTtsBackendFacade:subscribe_say(|CSayMessage|));
generate_facade_publish!(hermes_tts_backend_publish_say_finished = CTtsBackendFacade:publish_say_finished(CSayFinishedMessage));

generate_facade_wrapper!(CNluFacade for hermes::NluFacade,
                         hermes_drop_nlu_facade,
                         hermes_protocol_handler_nlu_facade = handler.nlu);
generate_facade_publish!(hermes_nlu_publish_query = CNluFacade:publish_query(CNluQueryMessage));
generate_facade_publish!(hermes_nlu_publish_partial_query = CNluFacade:publish_partial_query(CNluSlotQueryMessage));
generate_facade_subscribe!(hermes_nlu_subscribe_slot_parsed = CNluFacade:subscribe_slot_parsed(|CNluSlotMessage|));
generate_facade_subscribe!(hermes_nlu_subscribe_intent_parsed = CNluFacade:subscribe_intent_parsed(|CNluIntentMessage|));
generate_facade_subscribe!(hermes_nlu_subscribe_intent_not_recognized = CNluFacade:subscribe_intent_not_recognized(|CNluIntentNotRecognizedMessage|));

generate_facade_wrapper!(CNluBackendFacade for hermes::NluBackendFacade,
                         hermes_drop_nlu_backend_facade,
                         hermes_protocol_handler_nlu_backend_facade = handler.nlu_backend);
generate_facade_subscribe!(hermes_nlu_backend_subscribe_query = CNluBackendFacade:subscribe_query(|CNluQueryMessage|));
generate_facade_subscribe!(hermes_nlu_backend_subscribe_partial_query = CNluBackendFacade:subscribe_partial_query(|CNluSlotQueryMessage|));
generate_facade_publish!(hermes_nlu_backend_publish_slot_parsed = CNluBackendFacade:publish_slot_parsed(CNluSlotMessage));
generate_facade_publish!(hermes_nlu_backend_publish_intent_parsed = CNluBackendFacade:publish_intent_parsed(CNluIntentMessage));
generate_facade_publish!(hermes_nlu_backend_publish_intent_not_recognized = CNluBackendFacade:publish_intent_not_recognized(CNluIntentNotRecognizedMessage));

generate_facade_wrapper!(CAudioServerFacade for hermes::AudioServerFacade,
                         hermes_drop_audio_server_facade,
                         hermes_protocol_handler_audio_server_facade = handler.audio_server);

generate_facade_publish!(hermes_audio_server_publish_play_bytes = CAudioServerFacade:publish_play_bytes(CPlayBytesMessage));
generate_facade_subscribe!(hermes_audio_server_subscribe_play_finished = CAudioServerFacade:subscribe_play_finished(site_id: ::std::ffi::CStr as libc::c_char, |CPlayFinishedMessage|));
generate_facade_subscribe!(hermes_audio_server_subscribe_all_play_finished = CAudioServerFacade:subscribe_all_play_finished(|CPlayFinishedMessage|));
generate_facade_subscribe!(hermes_audio_server_subscribe_audio_frame = CAudioServerFacade:subscribe_audio_frame(site_id: ::std::ffi::CStr as libc::c_char, |CAudioFrameMessage|));

generate_facade_wrapper!(CAudioServerBackendFacade for hermes::AudioServerBackendFacade,
                         hermes_drop_audio_server_backend_facade,
                         hermes_protocol_handler_audio_server_backend_facade = handler.audio_server_backend);
generate_facade_subscribe!(hermes_audio_server_backend_subscribe_play_bytes = CAudioServerBackendFacade:subscribe_play_bytes(site_id: ::std::ffi::CStr as libc::c_char,|CPlayBytesMessage|));
generate_facade_subscribe!(hermes_audio_server_backend_subscribe_all_play_bytes = CAudioServerBackendFacade:subscribe_all_play_bytes(|CPlayBytesMessage|));
generate_facade_publish!(hermes_audio_server_backend_publish_play_finished = CAudioServerBackendFacade:publish_play_finished(CPlayFinishedMessage));
generate_facade_publish!(hermes_audio_server_backend_publish_audio_frame = CAudioServerBackendFacade:publish_audio_frame(CAudioFrameMessage));

generate_facade_wrapper!(CDialogueFacade for hermes::DialogueFacade,
                         hermes_drop_dialogue_facade,
                         hermes_protocol_handler_dialogue_facade = handler.dialogue);
generate_facade_subscribe!(hermes_dialogue_subscribe_session_queued = CDialogueFacade:subscribe_session_queued(|CSessionQueuedMessage|));
generate_facade_subscribe!(hermes_dialogue_subscribe_session_started = CDialogueFacade:subscribe_session_started(|CSessionStartedMessage|));
generate_facade_subscribe!(hermes_dialogue_subscribe_intent = CDialogueFacade:subscribe_intent(intent_name: ::std::ffi::CStr as libc::c_char, |CIntentMessage|));
generate_facade_subscribe!(hermes_dialogue_subscribe_intents = CDialogueFacade:subscribe_intents(|CIntentMessage|));
generate_facade_subscribe!(hermes_dialogue_subscribe_session_ended = CDialogueFacade:subscribe_session_ended(|CSessionEndedMessage|));
generate_facade_publish!(hermes_dialogue_publish_start_session = CDialogueFacade:publish_start_session(CStartSessionMessage));
generate_facade_publish!(hermes_dialogue_publish_continue_session = CDialogueFacade:publish_continue_session(CContinueSessionMessage));
generate_facade_publish!(hermes_dialogue_publish_end_session = CDialogueFacade:publish_end_session(CEndSessionMessage));

generate_facade_wrapper!(CDialogueBackendFacade for hermes::DialogueBackendFacade,
                         hermes_drop_dialogue_backend_facade,
                         hermes_protocol_handler_dialogue_backend_facade = handler.dialogue_backend);
generate_facade_publish!(hermes_dialogue_backend_publish_session_queued = CDialogueBackendFacade:publish_session_queued(CSessionQueuedMessage));
generate_facade_publish!(hermes_dialogue_backend_publish_session_started = CDialogueBackendFacade:publish_session_started(CSessionStartedMessage));
generate_facade_publish!(hermes_dialogue_backend_publish_intent = CDialogueBackendFacade:publish_intent(CIntentMessage));
generate_facade_publish!(hermes_dialogue_backend_publish_session_ended = CDialogueBackendFacade:publish_session_ended(CSessionEndedMessage));
generate_facade_subscribe!(hermes_dialogue_backend_subscribe_start_session = CDialogueBackendFacade:subscribe_start_session(|CStartSessionMessage|));
generate_facade_subscribe!(hermes_dialogue_backend_subscribe_continue_session = CDialogueBackendFacade:subscribe_continue_session(|CContinueSessionMessage|));
generate_facade_subscribe!(hermes_dialogue_backend_subscribe_end_session = CDialogueBackendFacade:subscribe_end_session(|CEndSessionMessage|));




