use std::fmt;
use std::collections::HashMap;

use base64;
use chrono::prelude::*;
use semver;
use serde;

use serde::{Serialize, Serializer, Deserialize, Deserializer};

pub trait HermesMessage<'de>: fmt::Debug + Deserialize<'de> + Serialize {}

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

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VadUpMessage {
    /// The site concerned
    pub site_id: SiteId,
    /// Timestamp of the audio frame where voice started to be detected
    pub signal_ms: Option<i64>,
}

impl<'de> HermesMessage<'de> for VadUpMessage {}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VadDownMessage {
    /// The site concerned
    pub site_id: SiteId,
    /// Timestamp of the audio frame where voice started to be detected
    pub signal_ms: Option<i64>,
}

impl<'de> HermesMessage<'de> for VadDownMessage {}

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
    /// The current sensitivity of the detector
    pub current_sensitivity: Option<f32>,
    /// Timestamp of the audio frame that generated the hotword
    pub detection_signal_ms: Option<i64>,
}

impl<'de> HermesMessage<'de> for HotwordDetectedMessage {}

#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct AsrDecodingDuration {
    pub start: f32,
    pub end: f32,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct AsrToken {
    /// The value of the token
    pub value: String,
    /// The confidence of the token
    pub confidence: f32,
    // TODO: change this range_start/stop when Range will be PartialOrd (only in nightly now. see issue #32311)
    /// The start range in which the token is in the original input
    pub range_start: usize,
    /// The end range in which the token is in the original input
    pub range_end: usize,
    /// TODO: Put doc
    pub time: AsrDecodingDuration,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextCapturedMessage {
    /// The text captured
    pub text: String,
    /// The likelihood of the capture
    pub likelihood: f32,
    /// The confidence by tokens
    pub tokens: Option<Vec<AsrToken>>,
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
    /// The confidence by tokens
    pub asr_tokens: Option<Vec<AsrToken>>,
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
    /// The confidence by tokens
    pub asr_tokens: Option<Vec<AsrToken>>,
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
pub struct ReplayRequestMessage {
    /// An id for the request, it will be passed back in the replayed frames headers.
    pub request_id: RequestId,
    /// When to start replay from
    pub start_at_ms: i64,
    /// The site this frame originates from
    pub site_id: SiteId,
}

impl<'de> HermesMessage<'de> for ReplayRequestMessage {}

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
    /// Add to current assistant
    Add,
    /// Add from the values downloaded
    AddFromVanilla,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct EntityValue {
    pub value: String,
    pub weight: u32,
}

impl<'de> Deserialize<'de> for EntityValue {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum DeEntityValue {
            Value(String),
            WeightedValue((String, u32)),
        }

        let (value, weight) = match DeEntityValue::deserialize(deserializer)? {
            DeEntityValue::Value(value) => (value, 1),
            DeEntityValue::WeightedValue(weighted_value) => weighted_value,
        };

        Ok(Self { value, weight })
    }
}

impl Serialize for EntityValue {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(untagged)]
        enum SerEntityValue<'a> {
            WeightedValue((&'a str, u32)),
        }

        SerEntityValue::WeightedValue((&*self.value, self.weight)).serialize(serializer)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InjectionRequestMessage {
    /// List of operations to execute in the order of the list on a model
    pub operations: Vec<(InjectionKind, HashMap<Entity, Vec<EntityValue>>)>,
    /// List of pre-computed prononciations to add in a model
    #[serde(default)]
    pub lexicon: HashMap<Value, Vec<Prononciation>>,
    /// Language for cross-language G2P
    pub cross_language: Option<String>,
    /// The id of the `InjectionRequest` that was processed
    pub id: Option<RequestId>,
}

impl<'de> HermesMessage<'de> for InjectionRequestMessage {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InjectionStatusMessage {
    /// Date of the latest injection
    pub last_injection_date: Option<DateTime<Utc>>,
}

impl<'de> HermesMessage<'de> for InjectionStatusMessage {}

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
    pub slot: Option<NluSlot>,
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
pub struct NluSlot {
    /// The slot confidence
    pub confidence: Option<f32>,
    #[serde(flatten)]
    pub nlu_slot: ::snips_nlu_ontology::Slot,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NluIntentMessage {
    /// The id of the `NluQueryMessage` that was processed
    pub id: Option<RequestId>,
    /// The input that was processed
    pub input: String,
    /// The result of the intent classification
    pub intent: ::snips_nlu_ontology::IntentClassifierResult,
    /// The detected slots, if any
    pub slots: Option<Vec<NluSlot>>,
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
    /// The tokens detected by the ASR. The first vec represents the different ASR invocations
    pub asr_tokens: Option<Vec<Vec<AsrToken>>>,
    /// The result of the intent classification
    pub intent: ::snips_nlu_ontology::IntentClassifierResult,
    /// The detected slots, if any
    pub slots: Option<Vec<NluSlot>>,
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

fn as_base64<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&base64::encode(bytes))
}

fn from_base64<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    use serde::Deserialize;
    String::deserialize(deserializer)
        .and_then(|string| base64::decode(&string).map_err(|err| Error::custom(err.to_string())))
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json;

    #[test]
    fn custom_deserialization_entityvalue_works() {
        let json = r#" "a" "#;
        let entity_value: EntityValue = serde_json::from_str(&json).unwrap();
        assert_eq!(entity_value, EntityValue { value: "a".to_string(), weight: 1 });

        let json = r#"["a", 42]"#;
        let entity_value: EntityValue = serde_json::from_str(&json).unwrap();
        assert_eq!(entity_value, EntityValue { value: "a".to_string(), weight: 42 });
    }

    #[test]
    fn custom_serialization_entityvalue_works() {
        let entity_value = EntityValue { value: "hello".to_string(), weight: 42 };
        let string = serde_json::to_string(&entity_value).unwrap();
        assert_eq!(string, r#"["hello",42]"#);
    }

    #[test]
    fn without_weights_works() {
        let json = r#"{
            "operations": [["add", {"e_0": ["a", ["b", 42]]}]]
        }"#;

        let my_struct: InjectionRequestMessage = serde_json::from_str(&json).unwrap();
        let (operation, values_per_entity) = &my_struct.operations[0];

        assert_eq!(operation, &InjectionKind::Add);
        assert_eq!(values_per_entity["e_0"][0], EntityValue { value: "a".to_string(), weight: 1 });
        assert_eq!(values_per_entity["e_0"][1], EntityValue { value: "b".to_string(), weight: 42 });
    }

    #[test]
    fn with_weights_works() {
        let json = r#"{
            "operations": [["add", {"e_0": [["a", 22], ["b", 31]]}]]
        }"#;

        let my_struct: InjectionRequestMessage = serde_json::from_str(&json).unwrap();
        let (operation, values_per_entity) = &my_struct.operations[0];

        assert_eq!(operation, &InjectionKind::Add);
        assert_eq!(values_per_entity["e_0"][0], EntityValue { value: "a".to_string(), weight: 22 });
        assert_eq!(values_per_entity["e_0"][1], EntityValue { value: "b".to_string(), weight: 31 });
    }
}
