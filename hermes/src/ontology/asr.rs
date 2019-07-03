use super::HermesMessage;

#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AsrStartListeningMessage {
    /// The site that must be listened too
    pub site_id: String,
    /// An optional session id if there is a related session
    pub session_id: Option<String>,
    /// Signal instant to start listening from
    pub start_signal_ms: Option<i64>,
}

impl<'de> HermesMessage<'de> for AsrStartListeningMessage {}

#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AsrDecodingDuration {
    pub start: f32,
    pub end: f32,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
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
    /// The tokens captures (with confidence, range and timing)
    pub tokens: Option<Vec<AsrToken>>,
    /// The duration it took to do the processing
    pub seconds: f32,
    /// The site where the text was captured
    pub site_id: String,
    /// An optional session id if there is a related session
    pub session_id: Option<String>,
    /// Optional list of the most probable speaker detected
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speaker_hypotheses: Option<Vec<SpeakerId>>,
}

impl<'de> HermesMessage<'de> for TextCapturedMessage {}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeakerId {
    /// The name of the detected speaker, `None` represents unknown speakers
    pub name: Option<String>,
    /// The confidence of the detection
    pub confidence: f32,
}

impl<'de> HermesMessage<'de> for SpeakerId {}
