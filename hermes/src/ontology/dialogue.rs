use super::asr::{AsrToken, SpeakerId};
use super::nlu::{NluIntentClassifierResult, NluSlot};
use super::HermesMessage;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IntentMessage {
    /// The session in which this intent was detected
    pub session_id: String,
    /// The custom data that was given at the session creation
    pub custom_data: Option<String>,
    /// The site where the intent was detected.
    pub site_id: String,
    /// The input that generated this intent
    pub input: String,
    /// The tokens detected by the ASR. The first vec represents the different ASR invocations
    pub asr_tokens: Option<Vec<Vec<AsrToken>>>,
    /// Confidence of the asr capture
    pub asr_confidence: Option<f32>,
    /// Optional list of the most probable speaker detected
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speaker_hypotheses: Option<Vec<SpeakerId>>,
    /// The result of the intent classification
    pub intent: NluIntentClassifierResult,
    /// The detected slots, if any
    pub slots: Vec<NluSlot>,
}

impl<'de> HermesMessage<'de> for IntentMessage {}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IntentNotRecognizedMessage {
    /// The session in which no intent was recognized
    pub session_id: String,
    /// The custom data that was given at the session creation
    pub custom_data: Option<String>,
    /// The site where the intent was detected.
    pub site_id: String,
    /// The text that didn't match any intent, `None` if no text wa captured
    pub input: Option<String>,
    /// Optional list of the most probable speaker detected
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speaker_hypotheses: Option<Vec<SpeakerId>>,
    /// Expresses the confidence that no intent was found
    pub confidence_score: f32,
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
        #[serde(default = "boolean_default_true")]
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

fn boolean_default_true() -> bool {
    true
}

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
    pub site_id: Option<String>,
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
    pub site_id: String,
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
    pub site_id: String,
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
    /// An optional string, requires `intent_filter` to contain a single value. If set, the dialogue
    /// engine will not run the the intent classification on the user response and go straight to
    /// slot filling, assuming the intent is the one passed in the `intent_filter`, and searching
    /// the value of the given slot
    pub slot: Option<String>,
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
    pub site_id: String,
}

impl<'de> HermesMessage<'de> for SessionEndedMessage {}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DialogueConfigureMessage {
    /// The site on which this configuration applies, if None the configuration will be applied to
    /// all sites.
    pub site_id: Option<String>,
    /// Intent configurations to apply.
    pub intents: Option<Vec<DialogueConfigureIntent>>,
}

impl<'de> HermesMessage<'de> for DialogueConfigureMessage {}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DialogueConfigureIntent {
    /// The name of the intent that should be configured.
    pub intent_id: String,
    /// Whether this intent should be activated on not.
    pub enable: Option<bool>,
}
