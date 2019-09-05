use std::ptr::null;
use std::slice;

use failure::format_err;
use failure::Fallible;
use failure::ResultExt;
use ffi_utils::*;

use crate::ontology::asr::CAsrTokenDoubleArray;
use crate::ontology::nlu::{CNluIntentClassifierResult, CNluSlotArray};
use crate::CNluIntentAlternativeArray;

#[repr(C)]
#[derive(Debug)]
pub struct CIntentMessage {
    pub session_id: *const libc::c_char,
    /// Nullable
    pub custom_data: *const libc::c_char,
    pub site_id: *const libc::c_char,
    pub input: *const libc::c_char,
    pub intent: *const CNluIntentClassifierResult,
    /// Nullable
    pub slots: *const CNluSlotArray,
    /// Nullable
    pub alternatives: *const CNluIntentAlternativeArray,
    ///// Nullable
    //pub speaker_hypotheses: *const CSpeakerIdArray,
    /// Nullable, the first array level represents the asr invocation, the second one the tokens
    pub asr_tokens: *const CAsrTokenDoubleArray,
    /// Note: this value is optional. Any value not in [0,1] should be ignored.
    pub asr_confidence: libc::c_float,
}

unsafe impl Sync for CIntentMessage {}

impl CIntentMessage {
    pub fn from(input: hermes::IntentMessage) -> Fallible<Self> {
        Self::c_repr_of(input)
    }
}

impl CReprOf<hermes::IntentMessage> for CIntentMessage {
    fn c_repr_of(input: hermes::IntentMessage) -> Fallible<Self> {
        Ok(Self {
            session_id: convert_to_c_string!(input.session_id),
            custom_data: convert_to_nullable_c_string!(input.custom_data),
            site_id: convert_to_c_string!(input.site_id),
            input: convert_to_c_string!(input.input),
            intent: CNluIntentClassifierResult::c_repr_of(input.intent)?.into_raw_pointer(),
            slots: if !input.slots.is_empty() {
                CNluSlotArray::c_repr_of(input.slots)?.into_raw_pointer()
            } else {
                null()
            },
            alternatives: if let Some(alternatives) = input.alternatives {
                CNluIntentAlternativeArray::c_repr_of(alternatives)?.into_raw_pointer()
            } else {
                null()
            },
            /*speaker_hypotheses: if let Some(speaker_hypotheses) = input.speaker_hypotheses {
                CSpeakerIdArray::c_repr_of(speaker_hypotheses)?.into_raw_pointer()
            } else {
                null()
            },*/
            asr_tokens: if let Some(asr_tokens) = input.asr_tokens {
                CAsrTokenDoubleArray::c_repr_of(asr_tokens)?.into_raw_pointer()
            } else {
                null()
            },
            asr_confidence: if let Some(asr_confidence) = input.asr_confidence {
                asr_confidence
            } else {
                -1.0
            },
        })
    }
}

impl AsRust<hermes::IntentMessage> for CIntentMessage {
    fn as_rust(&self) -> Fallible<hermes::IntentMessage> {
        Ok(hermes::IntentMessage {
            session_id: create_rust_string_from!(self.session_id),
            custom_data: create_optional_rust_string_from!(self.custom_data),
            site_id: create_rust_string_from!(self.site_id),
            input: create_rust_string_from!(self.input),
            speaker_hypotheses: None,
            /* match unsafe { self.speaker_hypotheses.as_ref() } {
                Some(speaker_hypotheses) => {
                    Some(unsafe { CSpeakerIdArray::raw_borrow(speaker_hypotheses)? }.as_rust()?)
                }
                None => None,
            }*/
            asr_tokens: if self.asr_tokens.is_null() {
                None
            } else {
                Some(unsafe { CAsrTokenDoubleArray::raw_borrow(self.asr_tokens) }?.as_rust()?)
            },
            asr_confidence: if self.asr_confidence >= 0.0 && self.asr_confidence <= 1.0 {
                Some(self.asr_confidence)
            } else {
                None
            },
            intent: unsafe { CNluIntentClassifierResult::raw_borrow(self.intent) }?.as_rust()?,
            slots: if !self.slots.is_null() {
                unsafe { CNluSlotArray::raw_borrow(self.slots) }?.as_rust()?
            } else {
                vec![]
            },
            alternatives: if !self.alternatives.is_null() {
                Some(unsafe { CNluIntentAlternativeArray::raw_borrow(self.alternatives) }?.as_rust()?)
            } else {
                None
            },
        })
    }
}

impl Drop for CIntentMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.session_id);
        take_back_nullable_c_string!(self.custom_data);
        take_back_c_string!(self.site_id);
        take_back_c_string!(self.input);
        let _ = unsafe { CNluIntentClassifierResult::drop_raw_pointer(self.intent) };
        if !self.slots.is_null() {
            let _ = unsafe { CNluSlotArray::drop_raw_pointer(self.slots) };
        }
        if !self.asr_tokens.is_null() {
            let _ = unsafe { CAsrTokenDoubleArray::drop_raw_pointer(self.asr_tokens) };
        }
        if !self.alternatives.is_null() {
            let _ = unsafe { CNluIntentAlternativeArray::drop_raw_pointer(self.alternatives) };
        }
        /*if !self.speaker_hypotheses.is_null() {
            let _ = unsafe { CSpeakerIdArray::drop_raw_pointer(self.speaker_hypotheses) };
        }*/
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CIntentNotRecognizedMessage {
    pub site_id: *const libc::c_char,
    pub session_id: *const libc::c_char,
    /// Nullable
    pub input: *const libc::c_char,
    /// Nullable
    pub custom_data: *const libc::c_char,
    /// Nullable
    pub alternatives: *const CNluIntentAlternativeArray,
    ///// Nullable
    //pub speaker_hypotheses: *const CSpeakerIdArray,
    pub confidence_score: libc::c_float,
}

unsafe impl Sync for CIntentNotRecognizedMessage {}

impl CReprOf<hermes::IntentNotRecognizedMessage> for CIntentNotRecognizedMessage {
    fn c_repr_of(input: hermes::IntentNotRecognizedMessage) -> Fallible<Self> {
        Ok(Self {
            site_id: convert_to_c_string!(input.site_id),
            session_id: convert_to_c_string!(input.session_id),
            input: convert_to_nullable_c_string!(input.input),
            alternatives: if let Some(alternatives) = input.alternatives {
                CNluIntentAlternativeArray::c_repr_of(alternatives)?.into_raw_pointer()
            } else {
                null()
            },
            /*speaker_hypotheses: if let Some(speaker_hypotheses) = input.speaker_hypotheses {
                CSpeakerIdArray::c_repr_of(speaker_hypotheses)?.into_raw_pointer()
            } else {
                null()
            },*/
            custom_data: convert_to_nullable_c_string!(input.custom_data),
            confidence_score: input.confidence_score,
        })
    }
}

impl AsRust<hermes::IntentNotRecognizedMessage> for CIntentNotRecognizedMessage {
    fn as_rust(&self) -> Fallible<hermes::IntentNotRecognizedMessage> {
        Ok(hermes::IntentNotRecognizedMessage {
            site_id: create_rust_string_from!(self.site_id),
            session_id: create_rust_string_from!(self.session_id),
            input: create_optional_rust_string_from!(self.input),
            speaker_hypotheses: None,
            /* match unsafe { self.speaker_hypotheses.as_ref() } {
                Some(speaker_hypotheses) => {
                    Some(unsafe { CSpeakerIdArray::raw_borrow(speaker_hypotheses)? }.as_rust()?)
                }
                None => None,
            }*/
            custom_data: create_optional_rust_string_from!(self.custom_data),
            alternatives: if !self.alternatives.is_null() {
                Some(unsafe { CNluIntentAlternativeArray::raw_borrow(self.alternatives) }?.as_rust()?)
            } else {
                None
            },
            confidence_score: self.confidence_score,
        })
    }
}

impl Drop for CIntentNotRecognizedMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.site_id);
        take_back_c_string!(self.session_id);
        take_back_nullable_c_string!(self.input);
        take_back_nullable_c_string!(self.custom_data);
        /*if !self.speaker_hypotheses.is_null() {
            let _ = unsafe { CSpeakerIdArray::drop_raw_pointer(self.speaker_hypotheses) };
        }*/
    }
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum SNIPS_SESSION_INIT_TYPE {
    SNIPS_SESSION_INIT_TYPE_ACTION = 1,
    SNIPS_SESSION_INIT_TYPE_NOTIFICATION = 2,
}

impl SNIPS_SESSION_INIT_TYPE {
    pub fn from(slot_value: &hermes::SessionInit) -> Self {
        match *slot_value {
            hermes::SessionInit::Notification { .. } => SNIPS_SESSION_INIT_TYPE::SNIPS_SESSION_INIT_TYPE_NOTIFICATION,
            hermes::SessionInit::Action { .. } => SNIPS_SESSION_INIT_TYPE::SNIPS_SESSION_INIT_TYPE_ACTION,
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct CActionSessionInit {
    /// Nullable
    text: *const libc::c_char,
    /// Nullable
    intent_filter: *const CStringArray,
    can_be_enqueued: libc::c_uchar,
    send_intent_not_recognized: libc::c_uchar,
}

impl CActionSessionInit {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(
        text: Option<String>,
        intent_filter: Option<Vec<String>>,
        can_be_enqueued: bool,
        send_intent_not_recognized: bool,
    ) -> Fallible<Self> {
        Ok(Self {
            text: convert_to_nullable_c_string!(text),
            intent_filter: convert_to_nullable_c_string_array!(intent_filter),
            can_be_enqueued: if can_be_enqueued { 1 } else { 0 },
            send_intent_not_recognized: if send_intent_not_recognized { 1 } else { 0 },
        })
    }

    pub fn to_action_session_init(&self) -> Fallible<hermes::SessionInit> {
        Ok(hermes::SessionInit::Action {
            text: create_optional_rust_string_from!(self.text),
            intent_filter: match unsafe { self.intent_filter.as_ref() } {
                Some(it) => Some(it.as_rust()?),
                None => None,
            },
            can_be_enqueued: self.can_be_enqueued == 1,
            send_intent_not_recognized: self.send_intent_not_recognized == 1,
        })
    }
}

impl Drop for CActionSessionInit {
    fn drop(&mut self) {
        take_back_nullable_c_string!(self.text);
        take_back_nullable_c_string_array!(self.intent_filter);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CSessionInit {
    init_type: SNIPS_SESSION_INIT_TYPE,
    /// Points to either a *const char, a *const CActionSessionInit
    value: *const libc::c_void,
}

impl CSessionInit {
    fn from(init: hermes::SessionInit) -> Fallible<Self> {
        let init_type = SNIPS_SESSION_INIT_TYPE::from(&init);
        let value: *const libc::c_void = match init {
            hermes::SessionInit::Action {
                text,
                intent_filter,
                can_be_enqueued,
                send_intent_not_recognized,
            } => Box::into_raw(Box::new(CActionSessionInit::new(
                text,
                intent_filter,
                can_be_enqueued,
                send_intent_not_recognized,
            )?)) as _,
            hermes::SessionInit::Notification { text } => convert_to_c_string!(text) as _,
        };
        Ok(Self { init_type, value })
    }

    fn to_session_init(&self) -> Fallible<hermes::SessionInit> {
        match self.init_type {
            SNIPS_SESSION_INIT_TYPE::SNIPS_SESSION_INIT_TYPE_ACTION => {
                unsafe { (self.value as *const CActionSessionInit).as_ref() }
                    .ok_or_else(|| format_err!("unexpected null pointer in SessionInit value"))?
                    .to_action_session_init()
            }
            SNIPS_SESSION_INIT_TYPE::SNIPS_SESSION_INIT_TYPE_NOTIFICATION => Ok(hermes::SessionInit::Notification {
                text: create_rust_string_from!((self.value as *const libc::c_char)
                    .as_ref()
                    .ok_or_else(|| format_err!("unexpected null pointer in SessionInit value"))?),
            }),
        }
    }
}

impl Drop for CSessionInit {
    fn drop(&mut self) {
        match self.init_type {
            SNIPS_SESSION_INIT_TYPE::SNIPS_SESSION_INIT_TYPE_ACTION => unsafe {
                let _ = CActionSessionInit::from_raw_pointer(self.value as _);
            },
            SNIPS_SESSION_INIT_TYPE::SNIPS_SESSION_INIT_TYPE_NOTIFICATION => {
                take_back_c_string!(self.value as *const libc::c_char);
            }
        };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CStartSessionMessage {
    pub init: CSessionInit,
    pub custom_data: *const libc::c_char,
    pub site_id: *const libc::c_char,
}

unsafe impl Sync for CStartSessionMessage {}

impl CStartSessionMessage {
    pub fn from(input: hermes::StartSessionMessage) -> Fallible<Self> {
        Self::c_repr_of(input)
    }

    pub fn to_start_session_message(&self) -> Fallible<hermes::StartSessionMessage> {
        self.as_rust()
    }
}

impl CReprOf<hermes::StartSessionMessage> for CStartSessionMessage {
    fn c_repr_of(input: hermes::StartSessionMessage) -> Fallible<Self> {
        Ok(Self {
            init: CSessionInit::from(input.init)?,
            custom_data: convert_to_nullable_c_string!(input.custom_data),
            site_id: convert_to_nullable_c_string!(input.site_id),
        })
    }
}

impl AsRust<hermes::StartSessionMessage> for CStartSessionMessage {
    fn as_rust(&self) -> Fallible<hermes::StartSessionMessage> {
        Ok(hermes::StartSessionMessage {
            init: self.init.to_session_init()?,
            custom_data: create_optional_rust_string_from!(self.custom_data),
            site_id: create_optional_rust_string_from!(self.site_id),
        })
    }
}

impl Drop for CStartSessionMessage {
    fn drop(&mut self) {
        take_back_nullable_c_string!(self.custom_data);
        take_back_nullable_c_string!(self.site_id);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CSessionStartedMessage {
    pub session_id: *const libc::c_char,
    /// Nullable
    pub custom_data: *const libc::c_char,
    pub site_id: *const libc::c_char,
    /// Nullable
    pub reactivated_from_session_id: *const libc::c_char,
}

unsafe impl Sync for CSessionStartedMessage {}

impl CSessionStartedMessage {
    pub fn from(input: hermes::SessionStartedMessage) -> Fallible<Self> {
        Self::c_repr_of(input)
    }
}

impl CReprOf<hermes::SessionStartedMessage> for CSessionStartedMessage {
    fn c_repr_of(input: hermes::SessionStartedMessage) -> Fallible<Self> {
        Ok(Self {
            session_id: convert_to_c_string!(input.session_id),
            custom_data: convert_to_nullable_c_string!(input.custom_data),
            site_id: convert_to_c_string!(input.site_id),
            reactivated_from_session_id: convert_to_nullable_c_string!(input.reactivated_from_session_id),
        })
    }
}

impl AsRust<hermes::SessionStartedMessage> for CSessionStartedMessage {
    fn as_rust(&self) -> Fallible<hermes::SessionStartedMessage> {
        Ok(hermes::SessionStartedMessage {
            session_id: create_rust_string_from!(self.session_id),
            custom_data: create_optional_rust_string_from!(self.custom_data),
            site_id: create_rust_string_from!(self.site_id),
            reactivated_from_session_id: create_optional_rust_string_from!(self.reactivated_from_session_id),
        })
    }
}

impl Drop for CSessionStartedMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.session_id);
        take_back_nullable_c_string!(self.custom_data);
        take_back_c_string!(self.site_id);
        take_back_nullable_c_string!(self.reactivated_from_session_id);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CSessionQueuedMessage {
    pub session_id: *const libc::c_char,
    /// Nullable
    pub custom_data: *const libc::c_char,
    pub site_id: *const libc::c_char,
}

unsafe impl Sync for CSessionQueuedMessage {}

impl CSessionQueuedMessage {
    pub fn from(input: hermes::SessionQueuedMessage) -> Fallible<Self> {
        Self::c_repr_of(input)
    }
}

impl CReprOf<hermes::SessionQueuedMessage> for CSessionQueuedMessage {
    fn c_repr_of(input: hermes::SessionQueuedMessage) -> Fallible<Self> {
        Ok(Self {
            session_id: convert_to_c_string!(input.session_id),
            custom_data: convert_to_nullable_c_string!(input.custom_data),
            site_id: convert_to_c_string!(input.site_id),
        })
    }
}

impl AsRust<hermes::SessionQueuedMessage> for CSessionQueuedMessage {
    fn as_rust(&self) -> Fallible<hermes::SessionQueuedMessage> {
        Ok(hermes::SessionQueuedMessage {
            session_id: create_rust_string_from!(self.session_id),
            custom_data: create_optional_rust_string_from!(self.custom_data),
            site_id: create_rust_string_from!(self.site_id),
        })
    }
}

impl Drop for CSessionQueuedMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.session_id);
        take_back_nullable_c_string!(self.custom_data);
        take_back_c_string!(self.site_id);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CContinueSessionMessage {
    pub session_id: *const libc::c_char,
    pub text: *const libc::c_char,
    /// Nullable
    pub intent_filter: *const CStringArray,
    /// Nullable
    pub custom_data: *const libc::c_char,
    /// Nullable
    pub slot: *const libc::c_char,
    pub send_intent_not_recognized: libc::c_uchar,
}

unsafe impl Sync for CContinueSessionMessage {}

impl CContinueSessionMessage {
    pub fn from(input: hermes::ContinueSessionMessage) -> Fallible<Self> {
        Self::c_repr_of(input)
    }

    pub fn to_continue_session_message(&self) -> Fallible<hermes::ContinueSessionMessage> {
        self.as_rust()
    }
}

impl CReprOf<hermes::ContinueSessionMessage> for CContinueSessionMessage {
    fn c_repr_of(input: hermes::ContinueSessionMessage) -> Fallible<Self> {
        Ok(Self {
            session_id: convert_to_c_string!(input.session_id),
            text: convert_to_c_string!(input.text),
            intent_filter: convert_to_nullable_c_string_array!(input.intent_filter),
            custom_data: convert_to_nullable_c_string!(input.custom_data),
            slot: convert_to_nullable_c_string!(input.slot),
            send_intent_not_recognized: if input.send_intent_not_recognized { 1 } else { 0 },
        })
    }
}

impl AsRust<hermes::ContinueSessionMessage> for CContinueSessionMessage {
    fn as_rust(&self) -> Fallible<hermes::ContinueSessionMessage> {
        Ok(hermes::ContinueSessionMessage {
            session_id: create_rust_string_from!(self.session_id),
            text: create_rust_string_from!(self.text),
            intent_filter: match unsafe { self.intent_filter.as_ref() } {
                Some(it) => Some(it.as_rust()?),
                None => None,
            },
            custom_data: create_optional_rust_string_from!(self.custom_data),
            slot: create_optional_rust_string_from!(self.slot),
            send_intent_not_recognized: self.send_intent_not_recognized == 1,
        })
    }
}

impl Drop for CContinueSessionMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.session_id);
        take_back_c_string!(self.text);
        take_back_nullable_c_string_array!(self.intent_filter);
        take_back_nullable_c_string!(self.custom_data);
        take_back_nullable_c_string!(self.slot);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CEndSessionMessage {
    pub session_id: *const libc::c_char,
    /// Nullable
    pub text: *const libc::c_char,
}

unsafe impl Sync for CEndSessionMessage {}

impl CEndSessionMessage {
    pub fn from(input: hermes::EndSessionMessage) -> Fallible<Self> {
        Self::c_repr_of(input)
    }

    pub fn to_end_session_message(&self) -> Fallible<hermes::EndSessionMessage> {
        self.as_rust()
    }
}

impl CReprOf<hermes::EndSessionMessage> for CEndSessionMessage {
    fn c_repr_of(input: hermes::EndSessionMessage) -> Fallible<Self> {
        Ok(Self {
            session_id: convert_to_c_string!(input.session_id),
            text: convert_to_nullable_c_string!(input.text),
        })
    }
}

impl AsRust<hermes::EndSessionMessage> for CEndSessionMessage {
    fn as_rust(&self) -> Fallible<hermes::EndSessionMessage> {
        Ok(hermes::EndSessionMessage {
            session_id: create_rust_string_from!(self.session_id),
            text: create_optional_rust_string_from!(self.text),
        })
    }
}

impl Drop for CEndSessionMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.session_id);
        take_back_nullable_c_string!(self.text);
    }
}

#[repr(C)]
#[derive(Debug)]
pub enum SNIPS_HERMES_COMPONENT {
    SNIPS_HERMES_COMPONENT_NONE = -1,
    SNIPS_HERMES_COMPONENT_AUDIO_SERVER = 1,
    SNIPS_HERMES_COMPONENT_HOTWORD = 2,
    SNIPS_HERMES_COMPONENT_ASR = 3,
    SNIPS_HERMES_COMPONENT_NLU = 4,
    SNIPS_HERMES_COMPONENT_DIALOGUE = 5,
    SNIPS_HERMES_COMPONENT_TTS = 6,
    SNIPS_HERMES_COMPONENT_INJECTION = 7,
    SNIPS_HERMES_COMPONENT_CLIENT_APP = 8,
}

impl From<Option<hermes::HermesComponent>> for SNIPS_HERMES_COMPONENT {
    fn from(component: Option<hermes::HermesComponent>) -> Self {
        match component {
            None => SNIPS_HERMES_COMPONENT::SNIPS_HERMES_COMPONENT_NONE,
            Some(hermes::HermesComponent::AudioServer) => SNIPS_HERMES_COMPONENT::SNIPS_HERMES_COMPONENT_AUDIO_SERVER,
            Some(hermes::HermesComponent::Hotword) => SNIPS_HERMES_COMPONENT::SNIPS_HERMES_COMPONENT_HOTWORD,
            Some(hermes::HermesComponent::Asr) => SNIPS_HERMES_COMPONENT::SNIPS_HERMES_COMPONENT_ASR,
            Some(hermes::HermesComponent::Nlu) => SNIPS_HERMES_COMPONENT::SNIPS_HERMES_COMPONENT_NLU,
            Some(hermes::HermesComponent::Dialogue) => SNIPS_HERMES_COMPONENT::SNIPS_HERMES_COMPONENT_DIALOGUE,
            Some(hermes::HermesComponent::Tts) => SNIPS_HERMES_COMPONENT::SNIPS_HERMES_COMPONENT_TTS,
            Some(hermes::HermesComponent::Injection) => SNIPS_HERMES_COMPONENT::SNIPS_HERMES_COMPONENT_INJECTION,
            Some(hermes::HermesComponent::ClientApp) => SNIPS_HERMES_COMPONENT::SNIPS_HERMES_COMPONENT_CLIENT_APP,
        }
    }
}

impl AsRust<Option<hermes::HermesComponent>> for SNIPS_HERMES_COMPONENT {
    fn as_rust(&self) -> Fallible<Option<hermes::HermesComponent>> {
        Ok(match self {
            SNIPS_HERMES_COMPONENT::SNIPS_HERMES_COMPONENT_NONE => None,
            SNIPS_HERMES_COMPONENT::SNIPS_HERMES_COMPONENT_AUDIO_SERVER => Some(hermes::HermesComponent::AudioServer),
            SNIPS_HERMES_COMPONENT::SNIPS_HERMES_COMPONENT_HOTWORD => Some(hermes::HermesComponent::Hotword),
            SNIPS_HERMES_COMPONENT::SNIPS_HERMES_COMPONENT_ASR => Some(hermes::HermesComponent::Asr),
            SNIPS_HERMES_COMPONENT::SNIPS_HERMES_COMPONENT_NLU => Some(hermes::HermesComponent::Nlu),
            SNIPS_HERMES_COMPONENT::SNIPS_HERMES_COMPONENT_DIALOGUE => Some(hermes::HermesComponent::Dialogue),
            SNIPS_HERMES_COMPONENT::SNIPS_HERMES_COMPONENT_TTS => Some(hermes::HermesComponent::Tts),
            SNIPS_HERMES_COMPONENT::SNIPS_HERMES_COMPONENT_INJECTION => Some(hermes::HermesComponent::Injection),
            SNIPS_HERMES_COMPONENT::SNIPS_HERMES_COMPONENT_CLIENT_APP => Some(hermes::HermesComponent::ClientApp),
        })
    }
}

#[repr(C)]
#[derive(Debug)]
pub enum SNIPS_SESSION_TERMINATION_TYPE {
    SNIPS_SESSION_TERMINATION_TYPE_NOMINAL = 1,
    SNIPS_SESSION_TERMINATION_TYPE_SITE_UNAVAILABLE = 2,
    SNIPS_SESSION_TERMINATION_TYPE_ABORTED_BY_USER = 3,
    SNIPS_SESSION_TERMINATION_TYPE_INTENT_NOT_RECOGNIZED = 4,
    SNIPS_SESSION_TERMINATION_TYPE_TIMEOUT = 5,
    SNIPS_SESSION_TERMINATION_TYPE_ERROR = 6,
}

impl SNIPS_SESSION_TERMINATION_TYPE {
    fn from(termination_type: &hermes::SessionTerminationType) -> SNIPS_SESSION_TERMINATION_TYPE {
        match *termination_type {
            hermes::SessionTerminationType::Nominal => {
                SNIPS_SESSION_TERMINATION_TYPE::SNIPS_SESSION_TERMINATION_TYPE_NOMINAL
            }
            hermes::SessionTerminationType::SiteUnavailable => {
                SNIPS_SESSION_TERMINATION_TYPE::SNIPS_SESSION_TERMINATION_TYPE_SITE_UNAVAILABLE
            }
            hermes::SessionTerminationType::AbortedByUser => {
                SNIPS_SESSION_TERMINATION_TYPE::SNIPS_SESSION_TERMINATION_TYPE_ABORTED_BY_USER
            }
            hermes::SessionTerminationType::IntentNotRecognized => {
                SNIPS_SESSION_TERMINATION_TYPE::SNIPS_SESSION_TERMINATION_TYPE_INTENT_NOT_RECOGNIZED
            }
            hermes::SessionTerminationType::Timeout { .. } => {
                SNIPS_SESSION_TERMINATION_TYPE::SNIPS_SESSION_TERMINATION_TYPE_TIMEOUT
            }
            hermes::SessionTerminationType::Error { .. } => {
                SNIPS_SESSION_TERMINATION_TYPE::SNIPS_SESSION_TERMINATION_TYPE_ERROR
            }
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CSessionTermination {
    termination_type: SNIPS_SESSION_TERMINATION_TYPE,
    /// Nullable,
    data: *const libc::c_char,
    component: SNIPS_HERMES_COMPONENT,
}

impl CSessionTermination {
    fn from(termination: hermes::SessionTerminationType) -> Fallible<Self> {
        let termination_type = SNIPS_SESSION_TERMINATION_TYPE::from(&termination);
        let (data, component): (*const libc::c_char, _) = match termination {
            hermes::SessionTerminationType::Error { error } => (
                convert_to_c_string!(error),
                SNIPS_HERMES_COMPONENT::SNIPS_HERMES_COMPONENT_NONE,
            ),
            hermes::SessionTerminationType::Timeout { component } => (null(), SNIPS_HERMES_COMPONENT::from(component)),
            _ => (null(), SNIPS_HERMES_COMPONENT::SNIPS_HERMES_COMPONENT_NONE),
        };
        Ok(Self {
            termination_type,
            data,
            component,
        })
    }
}

impl AsRust<hermes::SessionTerminationType> for CSessionTermination {
    fn as_rust(&self) -> Fallible<hermes::SessionTerminationType> {
        Ok(match self.termination_type {
            SNIPS_SESSION_TERMINATION_TYPE::SNIPS_SESSION_TERMINATION_TYPE_NOMINAL => {
                hermes::SessionTerminationType::Nominal
            }
            SNIPS_SESSION_TERMINATION_TYPE::SNIPS_SESSION_TERMINATION_TYPE_SITE_UNAVAILABLE => {
                hermes::SessionTerminationType::SiteUnavailable
            }
            SNIPS_SESSION_TERMINATION_TYPE::SNIPS_SESSION_TERMINATION_TYPE_ABORTED_BY_USER => {
                hermes::SessionTerminationType::AbortedByUser
            }
            SNIPS_SESSION_TERMINATION_TYPE::SNIPS_SESSION_TERMINATION_TYPE_INTENT_NOT_RECOGNIZED => {
                hermes::SessionTerminationType::IntentNotRecognized
            }
            SNIPS_SESSION_TERMINATION_TYPE::SNIPS_SESSION_TERMINATION_TYPE_TIMEOUT => {
                hermes::SessionTerminationType::Timeout {
                    component: self.component.as_rust()?,
                }
            }
            SNIPS_SESSION_TERMINATION_TYPE::SNIPS_SESSION_TERMINATION_TYPE_ERROR => {
                hermes::SessionTerminationType::Error {
                    error: create_rust_string_from!(self.data),
                }
            }
        })
    }
}

impl Drop for CSessionTermination {
    fn drop(&mut self) {
        take_back_nullable_c_string!(self.data);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CSessionEndedMessage {
    pub session_id: *const libc::c_char,
    /// Nullable
    pub custom_data: *const libc::c_char,
    pub termination: CSessionTermination,
    pub site_id: *const libc::c_char,
}

unsafe impl Sync for CSessionEndedMessage {}

impl CSessionEndedMessage {
    pub fn from(input: hermes::SessionEndedMessage) -> Fallible<Self> {
        Self::c_repr_of(input)
    }
}

impl CReprOf<hermes::SessionEndedMessage> for CSessionEndedMessage {
    fn c_repr_of(input: hermes::SessionEndedMessage) -> Fallible<Self> {
        Ok(Self {
            session_id: convert_to_c_string!(input.session_id),
            custom_data: convert_to_nullable_c_string!(input.custom_data),
            termination: CSessionTermination::from(input.termination)?,
            site_id: convert_to_c_string!(input.site_id),
        })
    }
}

impl AsRust<hermes::SessionEndedMessage> for CSessionEndedMessage {
    fn as_rust(&self) -> Fallible<hermes::SessionEndedMessage> {
        Ok(hermes::SessionEndedMessage {
            session_id: create_rust_string_from!(self.session_id),
            custom_data: create_optional_rust_string_from!(self.custom_data),
            termination: self.termination.as_rust()?,
            site_id: create_rust_string_from!(self.site_id),
        })
    }
}

impl Drop for CSessionEndedMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.session_id);
        take_back_nullable_c_string!(self.custom_data);
        take_back_c_string!(self.site_id);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CDialogueConfigureIntent {
    pub intent_id: *const libc::c_char,
    /// Optional Boolean 0 => false, 1 => true other values => null
    pub enable: libc::c_uchar,
}

impl CReprOf<hermes::DialogueConfigureIntent> for CDialogueConfigureIntent {
    fn c_repr_of(input: hermes::DialogueConfigureIntent) -> Fallible<Self> {
        Ok(Self {
            intent_id: convert_to_c_string!(input.intent_id),
            enable: match input.enable {
                Some(false) => 0,
                Some(true) => 1,
                None => libc::c_uchar::max_value(),
            },
        })
    }
}

impl AsRust<hermes::DialogueConfigureIntent> for CDialogueConfigureIntent {
    fn as_rust(&self) -> Fallible<hermes::DialogueConfigureIntent> {
        Ok(hermes::DialogueConfigureIntent {
            intent_id: create_rust_string_from!(self.intent_id),
            enable: match self.enable {
                0 => Some(false),
                1 => Some(true),
                _ => None,
            },
        })
    }
}

impl Drop for CDialogueConfigureIntent {
    fn drop(&mut self) {
        take_back_c_string!(self.intent_id)
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CDialogueConfigureIntentArray {
    pub entries: *const *const CDialogueConfigureIntent,
    pub count: libc::c_int,
}

impl CReprOf<Vec<hermes::DialogueConfigureIntent>> for CDialogueConfigureIntentArray {
    fn c_repr_of(input: Vec<hermes::DialogueConfigureIntent>) -> Fallible<Self> {
        let array = Self {
            count: input.len() as _,
            entries: Box::into_raw(
                input
                    .into_iter()
                    .map(|e| CDialogueConfigureIntent::c_repr_of(e).map(RawPointerConverter::into_raw_pointer))
                    .collect::<Fallible<Vec<_>>>()
                    .context("Could not convert map to C Repr")?
                    .into_boxed_slice(),
            ) as *const *const _,
        };
        Ok(array)
    }
}

impl AsRust<Vec<hermes::DialogueConfigureIntent>> for CDialogueConfigureIntentArray {
    fn as_rust(&self) -> Fallible<Vec<hermes::DialogueConfigureIntent>> {
        let mut result = Vec::with_capacity(self.count as usize);

        if self.count > 0 {
            for e in unsafe { slice::from_raw_parts(self.entries, self.count as usize) } {
                result.push(unsafe { CDialogueConfigureIntent::raw_borrow(*e) }?.as_rust()?);
            }
        }
        Ok(result)
    }
}

impl Drop for CDialogueConfigureIntentArray {
    fn drop(&mut self) {
        unsafe {
            let slots = Box::from_raw(std::slice::from_raw_parts_mut(
                self.entries as *mut *mut CDialogueConfigureIntent,
                self.count as usize,
            ));

            for e in slots.iter() {
                let _ = CDialogueConfigureIntent::drop_raw_pointer(*e);
            }
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CDialogueConfigureMessage {
    /// Nullable
    pub site_id: *const libc::c_char,
    /// Nullable
    pub intents: *const CDialogueConfigureIntentArray,
}

unsafe impl Sync for CDialogueConfigureMessage {}

impl CReprOf<hermes::DialogueConfigureMessage> for CDialogueConfigureMessage {
    fn c_repr_of(input: hermes::DialogueConfigureMessage) -> Fallible<Self> {
        Ok(Self {
            site_id: convert_to_nullable_c_string!(input.site_id),
            intents: if let Some(intents) = input.intents {
                CDialogueConfigureIntentArray::c_repr_of(intents)?.into_raw_pointer()
            } else {
                null()
            },
        })
    }
}

impl AsRust<hermes::DialogueConfigureMessage> for CDialogueConfigureMessage {
    fn as_rust(&self) -> Fallible<hermes::DialogueConfigureMessage> {
        Ok(hermes::DialogueConfigureMessage {
            site_id: create_optional_rust_string_from!(self.site_id),
            intents: if self.intents.is_null() {
                None
            } else {
                Some(unsafe { &*self.intents }.as_rust()?)
            },
        })
    }
}

impl Drop for CDialogueConfigureMessage {
    fn drop(&mut self) {
        take_back_nullable_c_string!(self.site_id);
        if !self.intents.is_null() {
            let _ = unsafe { CDialogueConfigureIntentArray::drop_raw_pointer(self.intents) };
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use super::super::tests::round_trip_test;
    use super::*;

    #[test]
    fn round_trip_intent_not_recognized() {
        round_trip_test::<_, CIntentNotRecognizedMessage>(hermes::IntentNotRecognizedMessage {
            site_id: "siteid".into(),
            custom_data: Some("custom".into()),
            session_id: "session id".into(),
            speaker_hypotheses: None,
            input: Some("some text".into()),
            confidence_score: 0.5,
            alternatives: Some(vec![hermes::NluIntentAlternative {
                slots: vec![],
                confidence_score: 0.8,
                intent_name: Some("some intent name".into()),
            }]),
        });

        round_trip_test::<_, CIntentNotRecognizedMessage>(hermes::IntentNotRecognizedMessage {
            site_id: "siteid".into(),
            custom_data: None,
            session_id: "session id".into(),
            speaker_hypotheses: None,
            input: None,
            confidence_score: 0.5,
            alternatives: None,
        });
    }

    #[test]
    fn round_trip_session_started() {
        round_trip_test::<_, CSessionStartedMessage>(hermes::SessionStartedMessage {
            site_id: "siteid".into(),
            custom_data: Some("custom".into()),
            session_id: "session id".into(),
            reactivated_from_session_id: Some("other session id".into()),
        });

        round_trip_test::<_, CSessionStartedMessage>(hermes::SessionStartedMessage {
            site_id: "siteid".into(),
            custom_data: None,
            session_id: "session id".into(),
            reactivated_from_session_id: None,
        })
    }

    #[test]
    fn round_trip_session_ended() {
        round_trip_test::<_, CSessionEndedMessage>(hermes::SessionEndedMessage {
            site_id: "siteid".into(),
            custom_data: Some("custom".into()),
            session_id: "session id".into(),
            termination: hermes::SessionTerminationType::Nominal,
        });

        round_trip_test::<_, CSessionEndedMessage>(hermes::SessionEndedMessage {
            site_id: "siteid".into(),
            custom_data: None,
            session_id: "session_id".into(),
            termination: hermes::SessionTerminationType::Error {
                error: "this is my error".into(),
            },
        });

        round_trip_test::<_, CSessionEndedMessage>(hermes::SessionEndedMessage {
            site_id: "siteid".into(),
            custom_data: None,
            session_id: "session_id".into(),
            termination: hermes::SessionTerminationType::Timeout { component: None },
        });

        round_trip_test::<_, CSessionEndedMessage>(hermes::SessionEndedMessage {
            site_id: "siteid".into(),
            custom_data: None,
            session_id: "session_id".into(),
            termination: hermes::SessionTerminationType::Timeout {
                component: Some(hermes::HermesComponent::Hotword),
            },
        })
    }

    #[test]
    fn round_trip_session_queued() {
        round_trip_test::<_, CSessionQueuedMessage>(hermes::SessionQueuedMessage {
            site_id: "siteid".into(),
            custom_data: Some("custom".into()),
            session_id: "session id".into(),
        });

        round_trip_test::<_, CSessionQueuedMessage>(hermes::SessionQueuedMessage {
            site_id: "siteid".into(),
            custom_data: None,
            session_id: "session_id".into(),
        })
    }

    #[test]
    fn round_trip_start_session() {
        round_trip_test::<_, CStartSessionMessage>(hermes::StartSessionMessage {
            init: hermes::SessionInit::Notification { text: "text".into() },
            custom_data: Some("thing".into()),
            site_id: Some("site".into()),
        });

        round_trip_test::<_, CStartSessionMessage>(hermes::StartSessionMessage {
            init: hermes::SessionInit::Action {
                intent_filter: Some(vec!["filter1".into(), "filter2".into()]),
                text: Some("text".into()),
                can_be_enqueued: true,
                send_intent_not_recognized: false,
            },
            custom_data: Some("thing".into()),
            site_id: Some("site".into()),
        });

        round_trip_test::<_, CStartSessionMessage>(hermes::StartSessionMessage {
            init: hermes::SessionInit::Action {
                intent_filter: None,
                text: None,
                can_be_enqueued: false,
                send_intent_not_recognized: true,
            },
            custom_data: None,
            site_id: None,
        });
    }

    #[test]
    fn round_trip_continue_session() {
        round_trip_test::<_, CContinueSessionMessage>(hermes::ContinueSessionMessage {
            session_id: "my session id".into(),
            text: "some text".into(),
            intent_filter: Some(vec!["filter1".into(), "filter2".into()]),
            custom_data: Some("foo bar".into()),
            slot: Some("some slot".into()),
            send_intent_not_recognized: true,
        });

        round_trip_test::<_, CContinueSessionMessage>(hermes::ContinueSessionMessage {
            session_id: "my session id".into(),
            text: "some text".into(),
            intent_filter: None,
            custom_data: None,
            slot: None,
            send_intent_not_recognized: false,
        });

        round_trip_test::<_, CContinueSessionMessage>(hermes::ContinueSessionMessage {
            session_id: "my session id".into(),
            text: "some text".into(),
            intent_filter: Some(vec![]),
            custom_data: Some("".into()),
            slot: Some("".into()),
            send_intent_not_recognized: true,
        });
    }

    #[test]
    fn round_trip_end_session() {
        round_trip_test::<_, CEndSessionMessage>(hermes::EndSessionMessage {
            session_id: "my session id".into(),
            text: Some("some text".into()),
        });

        round_trip_test::<_, CEndSessionMessage>(hermes::EndSessionMessage {
            session_id: "my session id".into(),
            text: None,
        });
    }

    #[test]
    fn round_trip_dialogue_configure_intent() {
        round_trip_test::<_, CDialogueConfigureIntent>(hermes::DialogueConfigureIntent {
            intent_id: "my intent".into(),
            enable: Some(true),
        });
        round_trip_test::<_, CDialogueConfigureIntent>(hermes::DialogueConfigureIntent {
            intent_id: "an intent".into(),
            enable: Some(false),
        });
        round_trip_test::<_, CDialogueConfigureIntent>(hermes::DialogueConfigureIntent {
            intent_id: "".into(),
            enable: None,
        });
    }

    #[test]
    fn round_trip_dialogue_configure_intent_array() {
        round_trip_test::<_, CDialogueConfigureIntentArray>(vec![
            hermes::DialogueConfigureIntent {
                intent_id: "my intent".into(),
                enable: Some(true),
            },
            hermes::DialogueConfigureIntent {
                intent_id: "an intent".into(),
                enable: Some(false),
            },
            hermes::DialogueConfigureIntent {
                intent_id: "".into(),
                enable: None,
            },
        ]);

        round_trip_test::<_, CDialogueConfigureIntentArray>(vec![]);
    }

    #[test]
    fn round_trip_dialogue_configure() {
        round_trip_test::<_, CDialogueConfigureMessage>(hermes::DialogueConfigureMessage {
            site_id: Some("some site".into()),
            intents: Some(vec![
                hermes::DialogueConfigureIntent {
                    intent_id: "my intent".into(),
                    enable: Some(true),
                },
                hermes::DialogueConfigureIntent {
                    intent_id: "an intent".into(),
                    enable: Some(false),
                },
                hermes::DialogueConfigureIntent {
                    intent_id: "".into(),
                    enable: None,
                },
            ]),
        });

        round_trip_test::<_, CDialogueConfigureMessage>(hermes::DialogueConfigureMessage {
            site_id: None,
            intents: None,
        });
    }

    #[test]
    fn round_trip_intent_message() {
        let slot = hermes::NluSlot {
            nlu_slot: snips_nlu_ontology::Slot {
                raw_value: "Guadeloupe".to_string(),
                value: snips_nlu_ontology::SlotValue::Custom("Guadeloupe".to_string().into()),
                range: Range { start: (22), end: (32) },
                entity: "entity".to_string(),
                slot_name: "forecast_location".to_string(),
                confidence_score: Some(0.8),
                alternatives: vec![
                    snips_nlu_ontology::SlotValue::Custom("Gwadloup".to_string().into()),
                    snips_nlu_ontology::SlotValue::Custom("Point a Pitre".to_string().into()),
                ],
            },
        };

        let asr_token_double_array = vec![
            vec![
                hermes::AsrToken {
                    value: "hello".to_string(),
                    confidence: 0.98,
                    range_start: 1,
                    range_end: 4,
                    time: hermes::AsrDecodingDuration { start: 0.0, end: 5.0 },
                },
                hermes::AsrToken {
                    value: "world".to_string(),
                    confidence: 0.73,
                    range_start: 5,
                    range_end: 9,
                    time: hermes::AsrDecodingDuration { start: 0.0, end: 5.0 },
                },
            ],
            vec![],
            vec![hermes::AsrToken {
                value: "yop".to_string(),
                confidence: 0.97,
                range_start: 5,
                range_end: 1,
                time: hermes::AsrDecodingDuration { start: 1.0, end: 4.5 },
            }],
        ];

        let alternatives = vec![
            hermes::nlu::NluIntentAlternative {
                intent_name: Some("another boring intent".to_string()),
                confidence_score: 0.9,
                slots: vec![
                    hermes::NluSlot {
                        nlu_slot: snips_nlu_ontology::Slot {
                            raw_value: "Martinique".to_string(),
                            value: snips_nlu_ontology::SlotValue::Custom("Martinique".to_string().into()),
                            range: Range { start: (42), end: (66) },
                            entity: "entity2".to_string(),
                            slot_name: "my_slot_name".to_string(),
                            confidence_score: Some(0.6),
                            alternatives: vec![
                                snips_nlu_ontology::SlotValue::Custom("Matnik".to_string().into()),
                                snips_nlu_ontology::SlotValue::Custom("Fort de france".to_string().into()),
                            ],
                        },
                    },
                    hermes::NluSlot {
                        nlu_slot: snips_nlu_ontology::Slot {
                            raw_value: "Marie Galante".to_string(),
                            value: snips_nlu_ontology::SlotValue::Custom("Marie Galante".to_string().into()),
                            range: Range { start: (1), end: (19) },
                            entity: "entity3".to_string(),
                            slot_name: "another_slot_name".to_string(),
                            confidence_score: Some(0.7),
                            alternatives: vec![],
                        },
                    },
                ],
            },
            hermes::nlu::NluIntentAlternative {
                intent_name: Some("yet another boring intent".to_string()),
                confidence_score: 0.8,
                slots: vec![
                    hermes::NluSlot {
                        nlu_slot: snips_nlu_ontology::Slot {
                            raw_value: "Martinique".to_string(),
                            value: snips_nlu_ontology::SlotValue::Custom("Martinique".to_string().into()),
                            range: Range { start: (42), end: (66) },
                            entity: "entity2".to_string(),
                            slot_name: "my_slot_name".to_string(),
                            confidence_score: Some(0.6),
                            alternatives: vec![
                                snips_nlu_ontology::SlotValue::Custom("Matnik".to_string().into()),
                                snips_nlu_ontology::SlotValue::Custom("Fort de france".to_string().into()),
                            ],
                        },
                    },
                    hermes::NluSlot {
                        nlu_slot: snips_nlu_ontology::Slot {
                            raw_value: "Marie Galante".to_string(),
                            value: snips_nlu_ontology::SlotValue::Custom("Marie Galante".to_string().into()),
                            range: Range { start: (1), end: (19) },
                            entity: "entity3".to_string(),
                            slot_name: "another_slot_name".to_string(),
                            confidence_score: Some(0.7),
                            alternatives: vec![
                                snips_nlu_ontology::SlotValue::Custom("Matnik".to_string().into()),
                                snips_nlu_ontology::SlotValue::Custom("Fort de france".to_string().into()),
                            ],
                        },
                    },
                ],
            },
            hermes::nlu::NluIntentAlternative {
                intent_name: None,
                confidence_score: 0.5,
                slots: vec![],
            },
        ];

        round_trip_test::<_, CIntentMessage>(hermes::IntentMessage {
            session_id: "a session id".to_string(),
            custom_data: Some("a custom datum".to_string()),
            site_id: "a site id".to_string(),
            input: "What's the weather in Guadeloupe ?".to_string(),
            speaker_hypotheses: None,
            asr_tokens: Some(asr_token_double_array),
            asr_confidence: Some(0.7),
            intent: hermes::nlu::NluIntentClassifierResult {
                intent_name: "a boring intent".to_string(),
                confidence_score: 1.0,
            },
            slots: vec![slot],
            alternatives: Some(alternatives),
        })
    }
}
