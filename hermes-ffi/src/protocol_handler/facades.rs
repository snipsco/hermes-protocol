use ffi_utils::RawPointerConverter;
use hermes::HermesProtocolHandler;

#[repr(C)]
#[derive(Debug)]
pub struct CProtocolHandler {
    // hides a Box<HermesProtocolHandler>, note the 2 levels (raw pointer + box) to be sure we have
    // a thin pointer here
    pub handler: *const libc::c_void,
    pub user_data: *mut libc::c_void,
}

pub struct UserData(pub *mut libc::c_void);

unsafe impl Send for UserData {}
unsafe impl Sync for UserData {}

impl UserData {
    pub fn duplicate(&self) -> Self {
        Self(self.0)
    }
}

impl CProtocolHandler {
    pub fn new(handler: Box<HermesProtocolHandler>, user_data: *mut libc::c_void) -> Self {
        let user_data = UserData(user_data).into_raw_pointer() as _;
        Self {
            handler: Box::into_raw(Box::new(handler)) as *const libc::c_void,
            user_data,
        }
    }

    pub fn extract(&self) -> &HermesProtocolHandler {
        unsafe { &(**(self.handler as *const Box<HermesProtocolHandler>)) }
    }

    pub fn user_data(&self) -> &UserData {
        unsafe { &(*(self.user_data as *mut UserData)) }
    }

    pub fn destroy(self) {
        let _ = unsafe { Box::from_raw(self.handler as *mut Box<HermesProtocolHandler>) };
    }
}

#[macro_export]
macro_rules! generate_facade_wrapper {
    (
        $wrapper_name:ident for
        $facade:ty,
        $drop_name:ident,
        $getter_name:ident = handler.
        $getter:ident
    ) => {
        #[repr(C)]
        pub struct $wrapper_name {
            // hides a Box<$facade>, note the 2 levels (raw pointer + box) to be sure we have a thin pointer here
            facade: *const libc::c_void,
            user_data: *mut libc::c_void,
        }

        impl $wrapper_name {
            pub fn from(facade: Box<$facade>, user_data: $crate::UserData) -> Self {
                use ffi_utils::RawPointerConverter;

                Self {
                    facade: Box::into_raw(Box::new(facade)) as *const libc::c_void,
                    user_data: user_data.into_raw_pointer() as _,
                }
            }

            pub fn extract(&self) -> &$facade {
                unsafe { &(**(self.facade as *const Box<$facade>)) }
            }

            pub fn user_data(&self) -> &$crate::UserData {
                unsafe { &(*(self.user_data as *mut $crate::UserData)) }
            }
        }

        impl Drop for $wrapper_name {
            fn drop(&mut self) {
                unsafe { Box::from_raw(self.facade as *mut Box<$facade>) };
            }
        }

        $crate::generate_destroy!($drop_name for $wrapper_name);

        #[no_mangle]
        pub extern "C" fn $getter_name(
            handler: *const $crate::CProtocolHandler,
            facade: *mut *const $wrapper_name,
        ) -> ffi_utils::SNIPS_RESULT {
            fn fun(
                handler: *const $crate::CProtocolHandler,
                facade: *mut *const $wrapper_name,
            ) -> failure::Fallible<()> {
                use ffi_utils::RawPointerConverter;

                let pointer = $wrapper_name::into_raw_pointer($wrapper_name::from(unsafe {
                    let handler = (*handler).extract();
                    (*handler).$getter()
                }, unsafe { (*handler).user_data().duplicate() }));
                unsafe { *facade = pointer };
                Ok(())
            }

            ffi_utils::wrap!(fun(handler, facade))
        }
    };
}

#[macro_export]
macro_rules! generate_facade_c_symbols {
    () => {
        pub mod facades {
            use super::LAST_ERROR;

            $crate::generate_facade_wrapper!(CSoundFeedbackFacade for hermes::SoundFeedbackFacade, hermes_drop_sound_feedback_facade, hermes_protocol_handler_sound_feedback_facade = handler.sound_feedback);
            $crate::generate_facade_wrapper!(CDialogueFacade for hermes::DialogueFacade, hermes_drop_dialogue_facade, hermes_protocol_handler_dialogue_facade = handler.dialogue);
            $crate::generate_facade_wrapper!(CInjectionFacade for hermes::InjectionFacade, hermes_drop_injection_facade, hermes_protocol_handler_injection_facade = handler.injection);

            #[cfg(feature = "full_bindings")]
            pub mod full_bindings {
                use super::super::LAST_ERROR;

                $crate::generate_facade_wrapper!(CHotwordFacade for hermes::HotwordFacade, hermes_drop_hotword_facade, hermes_protocol_handler_hotword_facade = handler.hotword);
                $crate::generate_facade_wrapper!(CHotwordBackendFacade for hermes::HotwordBackendFacade, hermes_drop_hotword_backend_facade, hermes_protocol_handler_hotword_backend_facade = handler.hotword_backend);
                $crate::generate_facade_wrapper!(CSoundFeedbackBackendFacade for hermes::SoundFeedbackBackendFacade, hermes_drop_sound_feedback_backend_facade, hermes_protocol_handler_sound_feedback_backend_facade = handler.sound_feedback_backend);
                $crate::generate_facade_wrapper!(CAsrFacade for hermes::AsrFacade, hermes_drop_asr_facade, hermes_protocol_handler_asr_facade = handler.asr);
                $crate::generate_facade_wrapper!(CAsrBackendFacade for hermes::AsrBackendFacade, hermes_drop_asr_backend_facade, hermes_protocol_handler_asr_backend_facade = handler.asr_backend);
                $crate::generate_facade_wrapper!(CTtsFacade for hermes::TtsFacade, hermes_drop_tts_facade, hermes_protocol_handler_tts_facade = handler.tts);
                $crate::generate_facade_wrapper!(CTtsBackendFacade for hermes::TtsBackendFacade, hermes_drop_tts_backend_facade, hermes_protocol_handler_tts_backend_facade = handler.tts_backend);
                $crate::generate_facade_wrapper!(CNluFacade for hermes::NluFacade, hermes_drop_nlu_facade, hermes_protocol_handler_nlu_facade = handler.nlu);
                $crate::generate_facade_wrapper!(CNluBackendFacade for hermes::NluBackendFacade, hermes_drop_nlu_backend_facade, hermes_protocol_handler_nlu_backend_facade = handler.nlu_backend);
                $crate::generate_facade_wrapper!(CAudioServerFacade for hermes::AudioServerFacade, hermes_drop_audio_server_facade, hermes_protocol_handler_audio_server_facade = handler.audio_server);
                $crate::generate_facade_wrapper!(CAudioServerBackendFacade for hermes::AudioServerBackendFacade, hermes_drop_audio_server_backend_facade, hermes_protocol_handler_audio_server_backend_facade = handler.audio_server_backend);
                $crate::generate_facade_wrapper!(CDialogueBackendFacade for hermes::DialogueBackendFacade, hermes_drop_dialogue_backend_facade, hermes_protocol_handler_dialogue_backend_facade = handler.dialogue_backend);
            }
        }
    };
}
