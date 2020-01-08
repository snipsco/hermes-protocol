use std::ptr::null;

use failure::format_err;
use failure::Fallible;
use failure::ResultExt;
use ffi_utils::*;
use ffi_utils_derive::{CReprOf, AsRust};

use hermes::*;
use crate::ontology::nlu::{CNluIntentClassifierResult};
use crate::{CAsrTokenDoubleArray, CNluSlot, CNluIntentAlternative};

#[repr(C)]
#[derive(Debug, CReprOf, AsRust)]
#[target_type(IntentMessage)]
pub struct CIntentMessage {
    /// The session identifier in which this intent was detected
    pub session_id: *const libc::c_char,
    /// Nullable, the custom data that was given at the session creation
    #[nullable]
    pub custom_data: *const libc::c_char,
    /// The site where the intent was detected.
    pub site_id: *const libc::c_char,
    /// The input that generated this intent
    pub input: *const libc::c_char,
    /// The result of the intent classification
    pub intent: *const CNluIntentClassifierResult,
    /// Nullable, the detected slots, if any
    pub slots: *const CArray<CNluSlot>,
    /// Nullable, alternatives intent resolutions
    #[nullable]
    pub alternatives: *const CArray<CNluIntentAlternative>,
//    /// Nullable
//    #[nullable]
//    pub speaker_hypotheses: *const CArray<CSpeakerId>,
    /// Nullable, the tokens detected by the ASR, the first array level represents the asr
    /// invocation, the second one the tokens
    #[nullable]
    pub asr_tokens: *const CAsrTokenDoubleArray,
    /// Confidence of the asr capture, this value is optional. Any value not in [0,1] should be ignored.
    #[nullable]
    pub asr_confidence: *const f32,
}

unsafe impl Sync for CIntentMessage {}

impl CIntentMessage {
    pub fn from(input: hermes::IntentMessage) -> Fallible<Self> {
        Self::c_repr_of(input)
    }
}

impl Drop for CIntentMessage {
    fn drop(&mut self) {
        take_back_c_string!(self.session_id);
        take_back_nullable_c_string!(self.custom_data);
        take_back_c_string!(self.site_id);
        take_back_c_string!(self.input);
        /*if !self.speaker_hypotheses.is_null() {
            let _ = unsafe { CSpeakerIdArray::drop_raw_pointer(self.speaker_hypotheses) };
        }*/
    }
}

#[repr(C)]
#[derive(Debug, CReprOf, AsRust)]
#[target_type(IntentNotRecognizedMessage)]
pub struct CIntentNotRecognizedMessage {
    /// The site where no intent was recognized
    pub site_id: *const libc::c_char,
    /// The session in which no intent was recognized
    pub session_id: *const libc::c_char,
    /// Nullable, the text that didn't match any intent
    #[nullable]
    pub input: *const libc::c_char,
    /// Nullable, the custom data that was given at the session creation
    #[nullable]
    pub custom_data: *const libc::c_char,
    /// Nullable, alternatives intent resolutions
    #[nullable]
    pub alternatives: *const CArray<CNluIntentAlternative>,
    //pub speaker_hypotheses: *const CSpeakerIdArray,
    /// Expresses the confidence that no intent was found
    pub confidence_score: f32,
}

unsafe impl Sync for CIntentNotRecognizedMessage {}

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
    /// The session expects a response from the user. Users responses will be provided in the form
    /// of `CIntentMessage`s.
    SNIPS_SESSION_INIT_TYPE_ACTION = 1,
    /// The session doesn't expect a response from the user. If the session cannot be started, it
    /// will be enqueued.
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
    /// Nullable, an optional text to be told to the user
    text: *const libc::c_char,
    /// Nullable, an optional list of intent name to restrict the parsing of the user response to
    intent_filter: *const CStringArray,
    /// A boolean to indicate if the session can be enqueued if it can't be started immediately (ie
    /// there is another running session on the site). 1 = true, 0 = false
    can_be_enqueued: libc::c_uchar,
    /// A boolean to indicate whether the dialogue manager should handle non recognized intents by
    /// itself or sent them as an `CIntentNotRecognizedMessage` for the client to handle. This
    /// setting applies only to the next conversation turn. 1 = true, 0 = false
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
    /// The type of session to start
    init_type: SNIPS_SESSION_INIT_TYPE,
    /// Points to either a *const char if the type is `SNIPS_SESSION_INIT_TYPE_NOTIFICATION`, or a
    /// *const CActionSessionInit if the type is `SNIPS_SESSION_INIT_TYPE_ACTION`
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

    fn c_repr_of(init: hermes::SessionInit) -> Fallible<Self> {
        Self::from(init)
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

    fn as_rust(&self) -> Fallible<hermes::SessionInit> {
        self.to_session_init()
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
#[derive(Debug, CReprOf, AsRust)]
#[target_type(StartSessionMessage)]
pub struct CStartSessionMessage {
    /// The way this session should be created
    pub init: CSessionInit,
    /// Nullable
    /// An optional string that will be given back in `CIntentMessage`,
    /// `CIntentNotRecognizedMessage`, `CSessionQueuedMessage`, `CSessionStartedMessage` and
    /// `CSessionEndedMessage` that are related to this session
    #[nullable]
    pub custom_data: *const libc::c_char,
    /// Nullable
    /// The site where the session should be started, a null value will be interpreted as the
    /// default one
    #[nullable]
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

impl Drop for CStartSessionMessage {
    fn drop(&mut self) {
        take_back_nullable_c_string!(self.custom_data);
        take_back_nullable_c_string!(self.site_id);
    }
}

#[repr(C)]
#[derive(Debug, CReprOf, AsRust)]
#[target_type(SessionStartedMessage)]
pub struct CSessionStartedMessage {
    /// The id of the session that was started
    pub session_id: *const libc::c_char,
    /// Nullable, the custom data that was given at the creation of the session
    #[nullable]
    pub custom_data: *const libc::c_char,
    /// The site on which this session was started
    pub site_id: *const libc::c_char,
    /// Nullable, this field indicates this session is a reactivation of a previously ended session.
    /// This is for example provided when the user continues talking to the platform without saying
    /// the hotword again after a session was ended.
    #[nullable]
    pub reactivated_from_session_id: *const libc::c_char,
}

unsafe impl Sync for CSessionStartedMessage {}

impl CSessionStartedMessage {
    pub fn from(input: hermes::SessionStartedMessage) -> Fallible<Self> {
        Self::c_repr_of(input)
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
#[derive(Debug, CReprOf, AsRust)]
#[target_type(SessionQueuedMessage)]
pub struct CSessionQueuedMessage {
    /// The id of the session that was queued
    pub session_id: *const libc::c_char,
    /// Nullable, the custom data that was given at the creation of the session
    #[nullable]
    pub custom_data: *const libc::c_char,
    /// The site on which this session was queued
    pub site_id: *const libc::c_char,
}

unsafe impl Sync for CSessionQueuedMessage {}

impl CSessionQueuedMessage {
    pub fn from(input: hermes::SessionQueuedMessage) -> Fallible<Self> {
        Self::c_repr_of(input)
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
#[derive(Debug, CReprOf, AsRust)]
#[target_type(ContinueSessionMessage)]
//#[derive(Debug)]
pub struct CContinueSessionMessage {
    /// The id of the session this action applies to
    pub session_id: *const libc::c_char,
    /// The text to say to the user
    pub text: *const libc::c_char,
    /// Nullable, an optional list of intent name to restrict the parsing of the user response to
    #[nullable]
    pub intent_filter: *const CStringArray,
    /// Nullable, an optional piece of data that will be given back in `CIntentMessage`,
    /// `CIntentNotRecognizedMessage` and `CSessionEndedMessage` that are related
    /// to this session. If set it will replace any existing custom data previously set on this
    /// session
    #[nullable]
    pub custom_data: *const libc::c_char,
    /// Nullable,  An optional string, requires `intent_filter` to contain a single value. If set,
    /// the dialogue engine will not run the the intent classification on the user response and go
    /// straight to slot filling, assuming the intent is the one passed in the `intent_filter`, and
    /// searching the value of the given slot
    #[nullable]
    pub slot: *const libc::c_char,
    /// A boolean to indicate whether the dialogue manager should handle not recognized
    /// intents by itself or sent them as a `CIntentNotRecognizedMessage` for the client to handle.
    /// This setting applies only to the next conversation turn. The default value is false (and
    /// the dialogue manager will handle non recognized intents by itself) true = 1, false = 0
    pub send_intent_not_recognized: u8,
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
#[derive(Debug, CReprOf, AsRust)]
#[target_type(EndSessionMessage)]
pub struct CEndSessionMessage {
    /// The id of the session to end
    pub session_id: *const libc::c_char,
    /// Nullable, an optional text to be told to the user before ending the session
    #[nullable]
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
    /// The session ended as expected
    SNIPS_SESSION_TERMINATION_TYPE_NOMINAL = 1,
    /// Dialogue was deactivated on the site the session requested
    SNIPS_SESSION_TERMINATION_TYPE_SITE_UNAVAILABLE = 2,
    /// The user aborted the session
    SNIPS_SESSION_TERMINATION_TYPE_ABORTED_BY_USER = 3,
    /// The platform didn't understand was the user said
    SNIPS_SESSION_TERMINATION_TYPE_INTENT_NOT_RECOGNIZED = 4,
    /// No response was received from one of the components in a timely manner
    SNIPS_SESSION_TERMINATION_TYPE_TIMEOUT = 5,
    /// A generic error occurred
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
    /// The type of the termination
    termination_type: SNIPS_SESSION_TERMINATION_TYPE,
    /// Nullable, set id the type is `SNIPS_SESSION_TERMINATION_TYPE_ERROR` and gives more info on
    /// the error that happen
    data: *const libc::c_char,
    /// If the type is `SNIPS_SESSION_TERMINATION_TYPE_TIMEOUT`, this gives the component that
    /// generated the timeout
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

    fn c_repr_of(termination: hermes::SessionTerminationType) -> Fallible<Self> {
        Self::from(termination)
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
#[derive(Debug, CReprOf, AsRust)]
#[target_type(SessionEndedMessage)]
pub struct CSessionEndedMessage {
    /// The id of the session that was terminated
    pub session_id: *const libc::c_char,
    /// Nullable, the custom data associated to this session
    #[nullable]
    pub custom_data: *const libc::c_char,
    /// How the session was ended
    pub termination: CSessionTermination,
    /// The site on which this session took place
    pub site_id: *const libc::c_char,
}

unsafe impl Sync for CSessionEndedMessage {}

impl CSessionEndedMessage {
    pub fn from(input: hermes::SessionEndedMessage) -> Fallible<Self> {
        Self::c_repr_of(input)
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
#[derive(Debug, CReprOf, AsRust)]
#[target_type(DialogueConfigureIntent)]
pub struct CDialogueConfigureIntent {
    /// The name of the intent that should be configured.
    pub intent_id: *const libc::c_char,
    /// Optional Boolean 0 => false, 1 => true other values => null,
    /// Whether this intent should be activated on not.
    #[nullable]
    pub enable: *const u8,
}

impl Drop for CDialogueConfigureIntent {
    fn drop(&mut self) {
        take_back_c_string!(self.intent_id)
    }
}

#[repr(C)]
#[derive(Debug, CReprOf, AsRust)]
#[target_type(DialogueConfigureMessage)]
pub struct CDialogueConfigureMessage {
    /// Nullable, the site on which this configuration applies, if `null` the configuration will
    /// be applied to all sites
    #[nullable]
    pub site_id: *const libc::c_char,
    /// Nullable, Intent configurations to apply
    #[nullable]
    pub intents: *const CArray<CDialogueConfigureIntent>,
}

unsafe impl Sync for CDialogueConfigureMessage {}

impl Drop for CDialogueConfigureMessage {
    fn drop(&mut self) {
        take_back_nullable_c_string!(self.site_id);
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use hermes::hermes_utils::Example;

    use super::super::tests::round_trip_test;
    use super::*;

    #[test]
    fn round_trip_intent_not_recognized() {
        round_trip_test::<_, CIntentNotRecognizedMessage>(hermes::IntentNotRecognizedMessage::minimal_example());

        round_trip_test::<_, CIntentNotRecognizedMessage>(hermes::IntentNotRecognizedMessage {
            //speaker_hypotheses: None, // TODO these are not supported by the ffi just yet
            ..hermes::IntentNotRecognizedMessage::full_example()
        });
    }

    #[test]
    fn round_trip_session_started() {
        round_trip_test::<_, CSessionStartedMessage>(hermes::SessionStartedMessage::minimal_example());

        round_trip_test::<_, CSessionStartedMessage>(hermes::SessionStartedMessage::full_example())
    }

    #[test]
    fn round_trip_session_ended() {
        round_trip_test::<_, CSessionEndedMessage>(hermes::SessionEndedMessage::minimal_example());
        round_trip_test::<_, CSessionEndedMessage>(hermes::SessionEndedMessage::full_example());

        round_trip_test::<_, CSessionEndedMessage>(hermes::SessionEndedMessage {
            termination: hermes::SessionTerminationType::Error {
                error: "this is my error".into(),
            },
            ..hermes::SessionEndedMessage::full_example()
        });

        round_trip_test::<_, CSessionEndedMessage>(hermes::SessionEndedMessage {
            termination: hermes::SessionTerminationType::Timeout { component: None },
            ..hermes::SessionEndedMessage::full_example()
        });

        round_trip_test::<_, CSessionEndedMessage>(hermes::SessionEndedMessage {
            termination: hermes::SessionTerminationType::Timeout {
                component: Some(hermes::HermesComponent::Hotword),
            },
            ..hermes::SessionEndedMessage::full_example()
        })
    }

    #[test]
    fn round_trip_session_queued() {
        round_trip_test::<_, CSessionQueuedMessage>(hermes::SessionQueuedMessage::minimal_example());

        round_trip_test::<_, CSessionQueuedMessage>(hermes::SessionQueuedMessage::full_example())
    }

    #[test]
    fn round_trip_start_session() {
        round_trip_test::<_, CStartSessionMessage>(hermes::StartSessionMessage::minimal_example());
        round_trip_test::<_, CStartSessionMessage>(hermes::StartSessionMessage::full_example());

        round_trip_test::<_, CStartSessionMessage>(hermes::StartSessionMessage {
            init: hermes::SessionInit::Notification { text: "text".into() },
            ..hermes::StartSessionMessage::full_example()
        });

        round_trip_test::<_, CStartSessionMessage>(hermes::StartSessionMessage {
            init: hermes::SessionInit::Action {
                intent_filter: Some(vec!["filter1".into(), "filter2".into()]),
                text: Some("text".into()),
                can_be_enqueued: true,
                send_intent_not_recognized: false,
            },
            ..hermes::StartSessionMessage::full_example()
        });

        round_trip_test::<_, CStartSessionMessage>(hermes::StartSessionMessage {
            init: hermes::SessionInit::Action {
                intent_filter: None,
                text: None,
                can_be_enqueued: false,
                send_intent_not_recognized: true,
            },
            ..hermes::StartSessionMessage::minimal_example()
        });
    }

    #[test]
    fn round_trip_continue_session() {
        round_trip_test::<_, CContinueSessionMessage>(hermes::ContinueSessionMessage::minimal_example());
        // TODO: Investigate when optional fields with empty vec, test will crash
        //round_trip_test::<_, CContinueSessionMessage>(hermes::ContinueSessionMessage::full_example());

        round_trip_test::<_, CContinueSessionMessage>(hermes::ContinueSessionMessage {
            intent_filter: None,
            custom_data: Some("a".into()),
            slot: Some("a".into()),
            send_intent_not_recognized: true,
            ..hermes::ContinueSessionMessage::full_example()
        });
    }

    #[test]
    fn round_trip_end_session() {
        round_trip_test::<_, CEndSessionMessage>(hermes::EndSessionMessage::minimal_example());

        round_trip_test::<_, CEndSessionMessage>(hermes::EndSessionMessage::full_example());
    }

    #[test]
    fn round_trip_dialogue_configure_intent() {
        round_trip_test::<_, CDialogueConfigureIntent>(hermes::DialogueConfigureIntent::minimal_example());
        round_trip_test::<_, CDialogueConfigureIntent>(hermes::DialogueConfigureIntent::full_example());
        round_trip_test::<_, CDialogueConfigureIntent>(hermes::DialogueConfigureIntent {
            enable: Some(true),
            ..hermes::DialogueConfigureIntent::full_example()
        });
    }

    #[test]
    fn round_trip_dialogue_configure() {
        round_trip_test::<_, CDialogueConfigureMessage>(hermes::DialogueConfigureMessage::minimal_example());

        round_trip_test::<_, CDialogueConfigureMessage>(hermes::DialogueConfigureMessage::full_example());
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
            //speaker_hypotheses: None,
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
