use base64;
use serde;
use semver;
use snips_queries_ontology::{IntentClassifierResult, Slot};
use std;

pub trait HermesMessage: ::std::fmt::Debug {}

pub type SiteId = String;

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct SiteMessage {
    /// The site concerned
    #[serde(rename = "siteId")]
    pub site_id: SiteId,
}

impl HermesMessage for SiteMessage {}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct TextCapturedMessage {
    /// The text captured
    pub text: String,
    /// The likelihood of the capture
    pub likelihood: f32,
    /// The duration it took to do the processing
    pub seconds: f32,
    /// The site where the text was captured
    #[serde(rename = "siteId")]
    pub site_id: SiteId,
}

impl HermesMessage for TextCapturedMessage {}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct NluQueryMessage {
    /// The text to run the NLU on
    pub text: String,
    /// An optional list of intents to restrict the NLU resolution on
    #[serde(rename = "intentFilter")]
    pub intent_filter: Option<Vec<String>>,
    /// An optional id for the request, if provided it will be passed back in the
    /// response `NluIntentMessage` or `NluIntentNotRecognizedMessage`
    pub id: Option<String>
}

impl HermesMessage for NluQueryMessage {}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct NluSlotQueryMessage {
    /// The text to run the slot detection on
    pub text: String,
    #[serde(rename = "intentName")]
    /// The intent to use when doing the slot detection
    pub intent_name: String,
    /// The slot to search
    #[serde(rename = "slotName")]
    pub slot_name: String,
    /// An optional id for the request, if provided it will be passed back in the
    /// response `SlotMessage`
    pub id: Option<String>,
}

impl HermesMessage for NluSlotQueryMessage {}


#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct PlayBytesMessage {
    /// An id for the request, it will be passed back in the `PlayFinishedMessage`
    pub id: String,
    /// The bytes of the wav to play (should be a regular wav with header)
    /// Note that serde json serialization is provided but in practice most handler impl will want
    /// to avoid the base64 encoding/decoding and give this a special treatment
    #[serde(rename = "wavBytes", serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub wav_bytes: Vec<u8>,
    /// The site where the bytes should be played
    #[serde(rename = "siteId")]
    pub site_id: SiteId,
}

impl HermesMessage for PlayBytesMessage {}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct AudioFrameMessage {
    /// The bytes of the wav frame (should be a regular wav with header)
    /// Note that serde json serialization is provided but in practice most handler impl will want
    /// to avoid the base64 encoding/decoding and give this a special treatment
    #[serde(rename = "wavFrame", serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub wav_frame: Vec<u8>,
    /// The site this frame originates from
    #[serde(rename = "siteId")]
    pub site_id: SiteId,
}

impl HermesMessage for AudioFrameMessage {}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct PlayFinishedMessage {
    /// The id of the `PlayBytesMessage` which bytes finished playing
    pub id: String
}

impl HermesMessage for PlayFinishedMessage {}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct SayMessage {
    /// The text to say
    pub text: String,
    /// The lang to use when saying the `text`, will use en_GB if not provided
    pub lang: Option<String>,
    /// An optional id for the request, it will be passed back in the `SayFinishedMessage`
    pub id: Option<String>,
    /// The site where the message should be said
    #[serde(rename = "siteId")]
    pub site_id: SiteId,
}

impl HermesMessage for SayMessage {}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct SayFinishedMessage {
    /// The id of the `SayMessage` which was has been said
    pub id: Option<String>
}

impl HermesMessage for SayFinishedMessage {}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct NluSlotMessage {
    /// The id of the `NluSlotQueryMessage` that was processed
    pub id: Option<String>,
    /// The input that was processed
    pub input: String,
    /// The intent used to find the slot
    pub intent: String,
    /// The resulting slot, if found
    pub slot: Option<Slot>,
}

impl HermesMessage for NluSlotMessage {}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct NluIntentNotRecognizedMessage {
    /// The id of the `NluQueryMessage` that was processed
    pub id: Option<String>,
    /// The text that didn't match any intent
    pub input: String,
}

impl HermesMessage for NluIntentNotRecognizedMessage {}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct NluIntentMessage {
    /// The id of the `NluQueryMessage` that was processed
    pub id: Option<String>,
    /// The input that was processed
    pub input: String,
    /// The result of the intent classification
    pub intent: IntentClassifierResult,
    /// The detected slots, if any
    pub slots: Option<Vec<Slot>>,
}

impl HermesMessage for NluIntentMessage {}


#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct IntentMessage {
    /// The session in with this intent was detected
    #[serde(rename = "sessionId")]
    pub session_id: String,
    /// The custom data that was given at the session creation
    #[serde(rename = "customData")]
    pub custom_data: Option<String>,
    /// The input that generated this intent
    pub input: String,
    /// The result of the intent classification
    pub intent: IntentClassifierResult,
    /// The detected slots, if any
    pub slots: Option<Vec<Slot>>,
}

impl HermesMessage for IntentMessage {}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(tag = "from")]
pub enum SessionInit {
    /// Interaction was initiated by the user
    User,
    /// Interaction was initiated by a lambda
    Lambda { action: SessionAction }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SessionAction {
    /// The text to say to the user
    pub text: String,
    /// Whether of not the lambda expects the user to respond, if set to `false` the session will
    /// be closed once the text has been said
    #[serde(rename = "expectResponse")]
    pub expect_response: bool,
    /// An optional list of intent name to restrict the parsing of the user response to. This will
    /// be ignored if `expect_response` is set to `false`
    #[serde(rename = "intentFilter")]
    pub intent_filter: Option<Vec<String>>
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct StartSessionMessage {
    /// The way this session was created
    pub init: SessionInit,
    /// The custom data that was given at the session creation
    #[serde(rename = "customData")]
    pub custom_data: Option<String>,
    /// The site where the session should be started, a value of `None` will be interpreted as the
    /// default one
    #[serde(rename = "siteId")]
    pub site_id: Option<SiteId>,
}

impl HermesMessage for StartSessionMessage {}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SessionStartedMessage {
    /// The id of the session that was started
    pub session_id: String,
    /// An optional piece of data that will be given back in `IntentMessage` and
    /// `SessionAbortedMessage` that are related to this session
    #[serde(rename = "customData")]
    pub custom_data: Option<String>,
    /// The site on which this session was started
    pub site_id: SiteId
}

impl HermesMessage for SessionStartedMessage {}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ContinueSessionMessage {
    /// The id of the session this action applies to
    #[serde(rename = "sessionId")]
    pub session_id: String,
    /// The action to perform
    pub action: SessionAction,
}

impl HermesMessage for ContinueSessionMessage {}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EndSessionMessage {
    /// The id of the session to end
    #[serde(rename = "sessionId")]
    pub session_id: String,
}

impl HermesMessage for EndSessionMessage {}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type")]
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
    Error{ error : String },
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SessionEndedMessage {
    /// The id of the session that was terminated
    #[serde(rename = "sessionId")]
    pub session_id: String,
    /// The custom data that was given at the session creation
    #[serde(rename = "customData")]
    pub custom_data: Option<String>,
    /// How the session was ended
    pub termination: SessionTerminationType,
}

impl HermesMessage for SessionEndedMessage {}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct VersionMessage {
    /// The version of the component
    pub version: semver::Version,
}

impl HermesMessage for VersionMessage {}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ErrorMessage {
    /// The error that occurred
    pub error: String,
    /// Optional additional information on the context in which the error occurred
    pub context: Option<String>,
}

impl HermesMessage for ErrorMessage {}

fn as_base64<S>(bytes: &[u8], serializer: S) -> std::result::Result<S::Ok, S::Error>
    where S: serde::Serializer {
    serializer.serialize_str(&base64::encode(bytes))
}

fn from_base64<'de, D>(deserializer: D) -> std::result::Result<Vec<u8>, D::Error>
    where D: serde::Deserializer<'de> {
    use serde::de::Error;
    use serde::Deserialize;
    String::deserialize(deserializer)
        .and_then(|string| base64::decode(&string).map_err(|err| Error::custom(err.to_string())))
}