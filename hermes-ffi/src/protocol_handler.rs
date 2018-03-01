use hermes;
use hermes::{HermesProtocolHandler, ResultExt};
use hermes_mqtt;
use libc;
use std::sync::Mutex;

lazy_static! {
    static ref LAST_ERROR: Mutex<String> = Mutex::new("".to_string());
}

macro_rules! wrap {
    ($e:expr) => { match $e {
        Ok(_) => HERMES_RESULT::OK,
        Err(e) => {
            use error_chain::ChainedError;
            let msg = e.display_chain().to_string();
            eprintln!("{}", msg);
            match LAST_ERROR.lock() {
                Ok(mut guard) => *guard = msg,
                Err(_) => () /* curl up and cry */
            }
            HERMES_RESULT::KO
        }
    }}
}


#[repr(C)]
#[derive(Debug)]
pub enum HERMES_RESULT {
    KO = 0,
    OK = 1,
}

#[repr(C)]
pub struct CProtocolHandler {
    handler: *const HermesProtocolHandler
}

#[no_mangle]
pub extern "C" fn hermes_protocol_handler_new_mqtt(handler: *mut *const CProtocolHandler) -> HERMES_RESULT {
    // TODO move that elsewhere, and destructor
    fn doit(handler: *mut *const CProtocolHandler) -> hermes::Result<()>{
        let cph = CProtocolHandler { handler: Box::into_raw(Box::new(hermes_mqtt::MqttHermesProtocolHandler::new("localhost:1883")?)) };
        let ptr = Box::into_raw(Box::new(cph));
        unsafe {
            *handler = ptr;
        }
        Ok(())
    }
    wrap!(doit(handler))
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
        pub extern "C" fn $drop_name(wrapper : *mut $wrapper_name) -> HERMES_RESULT {
            unsafe { Box::from_raw(wrapper) };
            HERMES_RESULT::OK
        }

        #[no_mangle]
        pub extern "C" fn $getter_name(handler: *const CProtocolHandler, facade: *mut *const $wrapper_name) -> HERMES_RESULT {
            fn fun(handler: *const CProtocolHandler, facade: *mut *const $wrapper_name) -> hermes::Result<()> {
                let pointer = Box::into_raw(Box::new($wrapper_name::from(unsafe { (*(*handler).handler).$getter() })));
                unsafe { *facade = pointer };
                Ok(())
            }

            wrap!(fun(handler, facade))
        }

    };
}

macro_rules! generate_facade_publish {
    ($c_symbol:ident = $facade:ty[$method:ident($arg:ty)]) => {
        #[no_mangle]
        pub extern "C" fn $c_symbol(facade : *const $facade, message : *const $arg) -> HERMES_RESULT {
            fn fun(facade : *const $facade, message : *const $arg) -> hermes::Result<()> {
                let message = convert(message)?;
                unsafe {(*facade).extract().$method(message)}
            }

            wrap!(fun(facade, message))
        }
    };
}


macro_rules! generate_facade_subscribe {
    ($c_symbol:ident = $facade:ty[$method:ident($arg:ty)]) => {
        #[no_mangle]
        pub extern "C" fn $c_symbol(facade: *const $facade, handler: Option<unsafe extern "C" fn(*const $arg)>) -> HERMES_RESULT {
            fn fun(facade: *const $facade, handler: Option<unsafe extern "C" fn(*const $arg)>) -> hermes::Result<()> {
                let callback = ptr_to_callback(handler)?;
                unsafe { (*facade).extract().$method(callback) }
            }

            wrap!(fun(facade, handler))
        }
    };
}

generate_facade_wrapper!(CHotwordFacade for hermes::HotwordFacade,
                         hermes_drop_hotword_facade,
                         hermes_protocol_handler_hotword_facade = handler.hotword);

generate_facade_wrapper!(CHotwordBackendFacade for hermes::HotwordBackendFacade,
                         hermes_drop_hotword_backend_facade,
                         hermes_protocol_handler_hotword_backend_facade = handler.hotword_backend);

generate_facade_wrapper!(CTtsFacade for hermes::TtsFacade,
                         hermes_drop_tts_facade,
                         hermes_protocol_handler_tts_facade = handler.tts);
generate_facade_publish!(hermes_tts_publish_say = CTtsFacade[publish_say(::CSayMessage)]);


generate_facade_wrapper!(CTtsBackendFacade for hermes::TtsBackendFacade,
                         hermes_drop_tts_backend_facade,
                         hermes_protocol_handler_tts_backend_facade = handler.tts_backend);
generate_facade_subscribe!(hermes_tts_backend_subscribe_say = CTtsBackendFacade[subscribe_say(::CSayMessage)]);

generate_facade_wrapper!(CDialogueFacade for hermes::DialogueFacade,
                         hermes_drop_dialogue_facade,
                         hermes_protocol_handler_dialogue_facade = handler.dialogue);
generate_facade_subscribe!(hermes_dialogue_subscribe_session_queued = CDialogueFacade[subscribe_session_queued(::CSessionQueuedMessage)]);
generate_facade_subscribe!(hermes_dialogue_subscribe_session_started = CDialogueFacade[subscribe_session_started(::CSessionStartedMessage)]);
generate_facade_subscribe!(hermes_dialogue_subscribe_intents = CDialogueFacade[subscribe_intents(::CIntentMessage)]);
generate_facade_subscribe!(hermes_dialogue_subscribe_session_ended = CDialogueFacade[subscribe_session_ended(::CSessionEndedMessage)]);
generate_facade_publish!(hermes_dialogue_publish_start_session = CDialogueFacade[publish_start_session(::CStartSessionMessage)]);
generate_facade_publish!(hermes_dialogue_publish_continue_session = CDialogueFacade[publish_continue_session(::CContinueSessionMessage)]);
generate_facade_publish!(hermes_dialogue_publish_end_session = CDialogueFacade[publish_end_session(::CEndSessionMessage)]);




generate_facade_wrapper!(CDialogueBackendFacade for hermes::DialogueBackendFacade,
                         hermes_drop_dialogue_backend_facade,
                         hermes_protocol_handler_dialogue_backend_facade = handler.dialogue_backend);












fn convert<T, U: ::ToRustStruct<T>>(raw : *const U) -> hermes::Result<T> {
    unsafe { (*raw).to_rust_struct() }
}

fn ptr_to_callback<T,U>(ptr: Option<unsafe extern "C" fn(*const U)>) -> hermes::Result<hermes::Callback<T>>
    where
        T: Clone + Sync,
        U: ::CreateRawFrom<T> + Sync + 'static{
    if let Some(ptr) =ptr  {
        Ok(hermes::Callback::new(move |payload: &T| {
            let param = Box::into_raw(Box::new(U::create_raw_from(payload.clone()).unwrap()));
            unsafe { ptr(param) }
        }))
    } else {
        bail!("null pointer")
    }
}

#[no_mangle]
pub extern "C" fn hermes_get_last_error(error: *mut *const libc::c_char) -> HERMES_RESULT {
    wrap!(get_last_error(error))
}


fn get_last_error(error: *mut *const libc::c_char) -> hermes::Result<()> {
    point_to_string(error, LAST_ERROR.lock()?.clone())
}

fn point_to_string(pointer: *mut *const libc::c_char, string: String) -> hermes::Result<()> {
    unsafe { *pointer = convert_to_c_string!(string) }
    Ok(())
}

