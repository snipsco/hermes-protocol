use super::asr::AsrToken;
use super::HermesMessage;

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
    /// Expresses the confidence that no intent was found
    pub confidence_score: f32,
    /// An optional session id if there is a related session
    pub session_id: Option<String>,
}

impl<'de> HermesMessage<'de> for NluIntentNotRecognizedMessage {}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NluSlot {
    //FIXME V2.0: Remove this redundant struct
    #[serde(flatten)]
    pub nlu_slot: snips_nlu_ontology::Slot,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NluIntentClassifierResult {
    /// Name of the intent that was found
    pub intent_name: String,
    /// The confidence score
    pub confidence_score: f32,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NluIntentMessage {
    /// The id of the `NluQueryMessage` that was processed
    pub id: Option<String>,
    /// The input that was processed
    pub input: String,
    /// The result of the intent classification
    pub intent: NluIntentClassifierResult,
    /// The detected slots, if any
    pub slots: Vec<NluSlot>,
    /// An optional session id if there is a related session
    pub session_id: Option<String>,
    /// Alternatives intent resolutions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternatives: Option<Vec<NluIntentAlternative>>,
}

impl<'de> HermesMessage<'de> for NluIntentMessage {}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NluIntentAlternative {
    /// Name of the intent that was found, or None is not intent was recognized
    pub intent_name: Option<String>,
    /// The confidence score of this alternative
    pub confidence_score: f32,
    /// The detected slots, if any
    pub slots: Vec<NluSlot>,
}

pub mod nlu_ontology {
    pub use snips_nlu_ontology::*;
}
