use super::{RequestId, SiteId, SessionId, HermesMessage};

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
