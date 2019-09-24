use super::HermesMessage;

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize, Example)]
#[serde(rename_all = "camelCase")]
pub struct SayMessage {
    /// The text to say
    #[example_value("Hello, world!")]
    pub text: String,
    /// The lang to use when saying the `text`, will use en_GB if not provided
    pub lang: Option<String>,
    /// An optional id for the request, it will be passed back in the `SayFinishedMessage`
    pub id: Option<String>,
    /// The site where the message should be said
    pub site_id: String,
    /// An optional session id if there is a related session
    pub session_id: Option<String>,
}

impl<'de> HermesMessage<'de> for SayMessage {}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize, Example)]
#[serde(rename_all = "camelCase")]
pub struct SayFinishedMessage {
    /// The id of the `SayMessage` which was has been said
    pub id: Option<String>,
    /// An optional session id if there is a related session
    pub session_id: Option<String>,
}

impl<'de> HermesMessage<'de> for SayFinishedMessage {}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize, Example)]
#[serde(rename_all = "camelCase")]
pub struct RegisterSoundMessage {
    /// The sound to register encoded as a wav.
    #[serde(serialize_with = "super::as_base64", deserialize_with = "super::from_base64")]
    pub wav_sound: Vec<u8>,
    /// The id this sound should be registered under
    pub sound_id: String,
}

impl<'de> HermesMessage<'de> for RegisterSoundMessage {}
