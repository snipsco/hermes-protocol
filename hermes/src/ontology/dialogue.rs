use super::{SessionId, SiteId, AsrToken, NluSlot, HermesMessage};


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

