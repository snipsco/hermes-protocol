use super::HermesMessage;

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VadUpMessage {
    /// The site concerned
    pub site_id: String,
    /// Timestamp of the audio frame where voice started to be detected
    pub signal_ms: Option<i64>,
}

impl<'de> HermesMessage<'de> for VadUpMessage {}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VadDownMessage {
    /// The site concerned
    pub site_id: String,
    /// Timestamp of the audio frame where voice started to be detected
    pub signal_ms: Option<i64>,
}

impl<'de> HermesMessage<'de> for VadDownMessage {}
