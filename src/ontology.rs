use std::collections::HashMap;

use base64;
use semver;
use serde;
use snips_nlu_ontology::{IntentClassifierResult, Slot};
use std;

pub trait HermesMessage<'de>:
    ::std::fmt::Debug + ::serde::Deserialize<'de> + ::serde::Serialize
{
}

pub type SiteId = String;
pub type SessionId = String;
pub type RequestId = String;

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteMessage {
    /// The site concerned
    pub site_id: SiteId,
    /// An optional session id if there is a related session
    pub session_id: Option<SessionId>,
}

impl Default for SiteMessage {
    fn default() -> Self {
        Self {
            site_id: "default".into(),
            session_id: None,
        }
    }
}

impl<'de> HermesMessage<'de> for SiteMessage {}


#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HotwordModelType {
    Universal, Personal
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HotwordDetectedMessage {
    /// The site where the hotword was triggered
    pub site_id: SiteId,
    /// Which model was triggered
    pub model_id: String,
    /// The version of the model
    pub model_version: Option<String>,
    /// The type of hotword that was triggered
    // TODO make non optional in next major rework of the protocol
    pub model_type: Option<HotwordModelType>,
    /// THe current sensitivity of the detector
    pub current_sensitivity: Option<f32>,
}

impl<'de> HermesMessage<'de> for HotwordDetectedMessage {}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextCapturedMessage {
    /// The text captured
    pub text: String,
    /// The likelihood of the capture
    pub likelihood: f32,
    /// The duration it took to do the processing
    pub seconds: f32,
    /// The site where the text was captured
    pub site_id: SiteId,
    /// An optional session id if there is a related session
    pub session_id: Option<SessionId>,
}

impl<'de> HermesMessage<'de> for TextCapturedMessage {}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NluQueryMessage {
    /// The text to run the NLU on
    pub input: String,
    /// An optional list of intents to restrict the NLU resolution on
    pub intent_filter: Option<Vec<String>>,
    /// An optional id for the request, if provided it will be passed back in the
    /// response `NluIntentMessage` or `NluIntentNotRecognizedMessage`
    pub id: Option<RequestId>,
    /// An optional session id if there is a related session
    pub session_id: Option<SessionId>,
}

impl<'de> HermesMessage<'de> for NluQueryMessage {}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NluSlotQueryMessage {
    /// The text to run the slot detection on
    pub input: String,
    /// The intent to use when doing the slot detection
    pub intent_name: String,
    /// The slot to search
    pub slot_name: String,
    /// An optional id for the request, if provided it will be passed back in the
    /// response `SlotMessage`
    pub id: Option<RequestId>,
    /// An optional session id if there is a related session
    pub session_id: Option<SessionId>,
}

impl<'de> HermesMessage<'de> for NluSlotQueryMessage {}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayBytesMessage {
    /// An id for the request, it will be passed back in the `PlayFinishedMessage`
    pub id: RequestId,
    /// The bytes of the wav to play (should be a regular wav with header)
    /// Note that serde json serialization is provided but in practice most handler impl will want
    /// to avoid the base64 encoding/decoding and give this a special treatment
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub wav_bytes: Vec<u8>,
    /// The site where the bytes should be played
    pub site_id: SiteId,
}

impl<'de> HermesMessage<'de> for PlayBytesMessage {}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioFrameMessage {
    /// The bytes of the wav frame (should be a regular wav with header)
    /// Note that serde json serialization is provided but in practice most handler impl will want
    /// to avoid the base64 encoding/decoding and give this a special treatment
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub wav_frame: Vec<u8>,
    /// The site this frame originates from
    pub site_id: SiteId,
}

impl<'de> HermesMessage<'de> for AudioFrameMessage {}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayFinishedMessage {
    /// The id of the `PlayBytesMessage` which bytes finished playing
    pub id: RequestId,
    /// The site where the sound was played
    pub site_id: SiteId,
}

impl<'de> HermesMessage<'de> for PlayFinishedMessage {}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SayMessage {
    /// The text to say
    pub text: String,
    /// The lang to use when saying the `text`, will use en_GB if not provided
    pub lang: Option<String>,
    /// An optional id for the request, it will be passed back in the `SayFinishedMessage`
    pub id: Option<RequestId>,
    /// The site where the message should be said
    pub site_id: SiteId,
    /// An optional session id if there is a related session
    pub session_id: Option<SessionId>,
}

impl<'de> HermesMessage<'de> for SayMessage {}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SayFinishedMessage {
    /// The id of the `SayMessage` which was has been said
    pub id: Option<RequestId>,
    /// An optional session id if there is a related session
    pub session_id: Option<SessionId>,
}

impl<'de> HermesMessage<'de> for SayFinishedMessage {}

type Value = String;
type Entity = String;
type Prononciation = String;

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum InjectionKind {
    Add, // Add to current assistant
    AddFromVanilla, // Add from the values downloaded
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InjectionRequest {
    /// List of operations to execute in the order of the list on a model
    pub operations: Vec<(InjectionKind, HashMap<Entity, Vec<Value>>)>,
    /// List of pre-computed prononciations to add in a model
    #[serde(default)]
    pub lexicon: HashMap<Value, Vec<Prononciation>>,
    /// Language for cros-language G2P
    pub cross_language: Option<String>,
}

impl<'de> HermesMessage<'de> for InjectionRequest {}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NluSlotMessage {
    /// The id of the `NluSlotQueryMessage` that was processed
    pub id: Option<RequestId>,
    /// The input that was processed
    pub input: String,
    /// The intent used to find the slot
    pub intent_name: String,
    /// The resulting slot, if found
    pub slot: Option<Slot>,
    /// An optional session id if there is a related session
    pub session_id: Option<SessionId>,
}

impl<'de> HermesMessage<'de> for NluSlotMessage {}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NluIntentNotRecognizedMessage {
    /// The id of the `NluQueryMessage` that was processed
    pub id: Option<RequestId>,
    /// The text that didn't match any intent
    pub input: String,
    /// An optional session id if there is a related session
    pub session_id: Option<SessionId>,
}

impl<'de> HermesMessage<'de> for NluIntentNotRecognizedMessage {}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NluIntentMessage {
    /// The id of the `NluQueryMessage` that was processed
    pub id: Option<RequestId>,
    /// The input that was processed
    pub input: String,
    /// The result of the intent classification
    pub intent: IntentClassifierResult,
    /// The detected slots, if any
    pub slots: Option<Vec<Slot>>,
    /// An optional session id if there is a related session
    pub session_id: Option<SessionId>,
}

impl<'de> HermesMessage<'de> for NluIntentMessage {}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IntentMessage {
    /// The session in which this intent was detected
    pub session_id: SessionId,
    /// The custom data that was given at the session creation
    pub custom_data: Option<String>,
    /// The site where the intent was detected.
    pub site_id: SiteId,
    /// The input that generated this intent
    pub input: String,
    /// The result of the intent classification
    pub intent: IntentClassifierResult,
    /// The detected slots, if any
    pub slots: Option<Vec<Slot>>,
}

impl<'de> HermesMessage<'de> for IntentMessage {}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IntentNotRecognizedMessage {
    /// The session in which no intent was recognized
    pub session_id: SessionId,
    /// The custom data that was given at the session creation
    pub custom_data: Option<String>,
    /// The site where the intent was detected.
    pub site_id: SiteId,
    /// The text that didn't match any intent, `None` if no text wa captured
    pub input: Option<String>,
}

impl<'de> HermesMessage<'de> for IntentNotRecognizedMessage {}


#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum SessionInit {
    /// The session expects a response from the user. Users responses will
    /// be provided in the form of `IntentMessage`s.
    #[serde(rename_all = "camelCase")]
    Action {
        /// An optional text to say to the user
        text: Option<String>,
        /// An optional list of intent name to restrict the parsing of the user response to
        intent_filter: Option<Vec<String>>,
        /// An optional boolean to indicate if the session can be enqueued if it can't be started
        /// immediately (ie there is another running session on the site). The default value is true
        #[serde(default="boolean_default_true")]
        can_be_enqueued: bool,
        /// An optional boolean to indicate whether the dialogue manager should handle non
        /// recognized intents by itself or sent them as an `IntentNotRecognizedMessage` for the
        /// client to handle. This setting applies only to the next conversation turn. The default
        /// value is false (and the dialogue manager will handle non recognized intents by itself)
        #[serde(default)]
        send_intent_not_recognized: bool,
    },
    /// The session doesn't expect a response from the user.
    /// If the session cannot be started, it will enqueued.
    Notification { text: String },
}

fn boolean_default_true() -> bool { true }

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartSessionMessage {
    /// The way this session was created
    pub init: SessionInit,
    /// An optional piece of data that will be given back in `IntentMessage`,
    /// `IntentNotRecognizedMessage`, `SessionQueuedMessage`, `SessionStartedMessage` and
    /// `SessionEndedMessage` that are related to this session
    pub custom_data: Option<String>,
    /// The site where the session should be started, a value of `None` will be interpreted as the
    /// default one
    pub site_id: Option<SiteId>,
}

impl<'de> HermesMessage<'de> for StartSessionMessage {}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionStartedMessage {
    /// The id of the session that was started
    pub session_id: String,
    /// The custom data that was given at the session creation
    pub custom_data: Option<String>,
    /// The site on which this session was started
    pub site_id: SiteId,
    /// This optional field indicates this session is a reactivation of a previously ended session.
    /// This is for example provided when the user continues talking to the platform without saying
    /// the hotword again after a session was ended.
    pub reactivated_from_session_id: Option<String>,
}

impl<'de> HermesMessage<'de> for SessionStartedMessage {}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionQueuedMessage {
    /// The id of the session that was started
    pub session_id: String,
    /// The custom data that was given at the session creation
    pub custom_data: Option<String>,
    /// The site on which this session was started
    pub site_id: SiteId,
}

impl<'de> HermesMessage<'de> for SessionQueuedMessage {}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContinueSessionMessage {
    /// The id of the session this action applies to
    pub session_id: String,
    /// The text to say to the user
    pub text: String,
    /// An optional list of intent name to restrict the parsing of the user response to
    pub intent_filter: Option<Vec<String>>,
    /// An optional piece of data that will be given back in `IntentMessage` and
    /// `IntentNotRecognizedMessage` and `SessionEndedMessage` that are related
    /// to this session. If set it will replace any existing custom data previously set on this
    /// session
    pub custom_data: Option<String>,
    /// An optional boolean to indicate whether the dialogue manager should handle not recognized
    /// intents by itself or sent them as an `IntentNotRecognizedMessage` for the client to handle.
    /// This setting applies only to the next conversation turn. The default value is false (and
    /// the dialogue manager will handle non recognized intents by itself)
    #[serde(default)]
    pub send_intent_not_recognized: bool,
}

impl<'de> HermesMessage<'de> for ContinueSessionMessage {}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EndSessionMessage {
    /// The id of the session to end
    pub session_id: String,
    /// An optional text to say to the user before ending the session
    pub text: Option<String>,
}

impl<'de> HermesMessage<'de> for EndSessionMessage {}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(tag = "reason", rename_all = "camelCase")]
pub enum SessionTerminationType {
    /// The session ended as expected
    Nominal,
    /// Dialogue was deactivated on the site the session requested
    SiteUnavailable,
    /// The user aborted the session
    AbortedByUser,
    /// The platform didn't understand was the user said
    IntentNotRecognized,
    /// No response was received from one of the components in a timely manner
    Timeout,
    /// A generic error occurred
    Error { error: String },
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionEndedMessage {
    /// The id of the session that was terminated
    pub session_id: String,
    /// The custom data that was given at the session creation
    pub custom_data: Option<String>,
    /// How the session was ended
    pub termination: SessionTerminationType,
    /// The site on which this session was ended.
    pub site_id: SiteId,
}

impl<'de> HermesMessage<'de> for SessionEndedMessage {}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionMessage {
    /// The version of the component
    pub version: semver::Version,
}

impl<'de> HermesMessage<'de> for VersionMessage {}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorMessage {
    /// An optional session id if there is a related session
    pub session_id: Option<SessionId>,
    /// The error that occurred
    pub error: String,
    /// Optional additional information on the context in which the error occurred
    pub context: Option<String>,
}

impl<'de> HermesMessage<'de> for ErrorMessage {}

fn as_base64<S>(bytes: &[u8], serializer: S) -> std::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&base64::encode(bytes))
}

fn from_base64<'de, D>(deserializer: D) -> std::result::Result<Vec<u8>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    use serde::Deserialize;
    String::deserialize(deserializer)
        .and_then(|string| base64::decode(&string).map_err(|err| Error::custom(err.to_string())))
}
