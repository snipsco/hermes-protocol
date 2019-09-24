use super::HermesMessage;

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HotwordModelType {
    Universal,
    Personal,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize, Example)]
#[serde(rename_all = "camelCase")]
pub struct HotwordDetectedMessage {
    /// The site where the hotword was triggered
    pub site_id: String,
    /// Which model was triggered
    pub model_id: String,
    /// The version of the model
    pub model_version: Option<String>,
    /// The type of hotword that was triggered
    // TODO make non optional in next major rework of the protocol
    #[example_value(Some(HotwordModelType::Universal))]
    pub model_type: Option<HotwordModelType>,
    /// The current sensitivity of the detector
    pub current_sensitivity: Option<f32>,
    /// Timestamp of the audio frame that triggered the hotword
    pub detection_signal_ms: Option<i64>,
    /// Timestamp of the audio frame where the hotword is likely to end
    pub end_signal_ms: Option<i64>,
}

impl<'de> HermesMessage<'de> for HotwordDetectedMessage {}
