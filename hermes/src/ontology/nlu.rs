use super::{AsrToken, HermesMessage};

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
    pub id: Option<String>,
    /// An optional session id if there is a related session
    pub session_id: Option<String>,
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
    pub id: Option<String>,
    /// An optional session id if there is a related session
    pub session_id: Option<String>,
}

impl<'de> HermesMessage<'de> for NluSlotQueryMessage {}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NluSlotMessage {
    /// The id of the `NluSlotQueryMessage` that was processed
    pub id: Option<String>,
    /// The input that was processed
    pub input: String,
    /// The intent used to find the slot
    pub intent_name: String,
    /// The resulting slot, if found
    pub slot: Option<NluSlot>,
    /// An optional session id if there is a related session
    pub session_id: Option<String>,
}

impl<'de> HermesMessage<'de> for NluSlotMessage {}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NluIntentNotRecognizedMessage {
    /// The id of the `NluQueryMessage` that was processed
    pub id: Option<String>,
    /// The text that didn't match any intent
    pub input: String,
    /// An optional session id if there is a related session
    pub session_id: Option<String>,
}

impl<'de> HermesMessage<'de> for NluIntentNotRecognizedMessage {}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NluSlot {
    /// The slot confidence
    pub confidence: Option<f32>,
    #[serde(flatten)]
    pub nlu_slot: snips_nlu_ontology::Slot,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NluIntentMessage {
    /// The id of the `NluQueryMessage` that was processed
    pub id: Option<String>,
    /// The input that was processed
    pub input: String,
    /// The result of the intent classification
    pub intent: snips_nlu_ontology::IntentClassifierResult,
    /// The detected slots, if any
    pub slots: Option<Vec<NluSlot>>,
    /// An optional session id if there is a related session
    pub session_id: Option<String>,
}

impl<'de> HermesMessage<'de> for NluIntentMessage {}
