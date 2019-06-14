use std::fmt;

use semver;
use serde::{Deserialize, Serialize};

pub use self::asr::*;
pub use self::audio_server::*;
pub use self::dialogue::*;
pub use self::hotword::*;
pub use self::injection::*;
pub use self::nlu::*;
pub use self::tts::*;
pub use self::vad::*;

pub mod asr;
pub mod audio_server;
pub mod dialogue;
pub mod hotword;
pub mod injection;
pub mod nlu;
pub mod tts;
pub mod vad;

pub trait HermesMessage<'de>: fmt::Debug + Deserialize<'de> + Serialize {}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteMessage {
    /// The site concerned
    pub site_id: String,
    /// An optional session id if there is a related session
    pub session_id: Option<String>,
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
    pub session_id: Option<String>,
    /// The error that occurred
    pub error: String,
    /// Optional additional information on the context in which the error occurred
    pub context: Option<String>,
}

impl<'de> HermesMessage<'de> for ErrorMessage {}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(tag = "reason", rename_all = "camelCase")]
pub enum HermesComponent {
    AudioServer,
    Hotword,
    Asr,
    Nlu,
    Dialogue,
    Tts,
    Injection,
    ClientApp,
}

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
    String::deserialize(deserializer)
        .and_then(|string| base64::decode(&string).map_err(|err| Error::custom(err.to_string())))
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentLoadedOnSiteMessage {
    /// Optional id associated to a load/reload operation for a component
    pub id: Option<String>,
    /// boolean that indicates if the component was reloaded or if it's its initial load.
    pub reloaded: bool,
    /// The site concerned
    pub site_id: String,
}

impl<'de> HermesMessage<'de> for ComponentLoadedOnSiteMessage {}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestComponentReloadMessage {
    /// Id associated to a reload request operation of a component
    pub id: String,
}

impl<'de> HermesMessage<'de> for RequestComponentReloadMessage {}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentLoadedMessage {
    /// Optional id associated to a load/reload operation for a component
    pub id: Option<String>,
    /// boolean that indicates if the component was reloaded or if it's its initial load.
    pub reloaded: bool,
}

impl<'de> HermesMessage<'de> for ComponentLoadedMessage {}

impl Default for ComponentLoadedMessage {
    fn default() -> Self {
        Self {
            id: None,
            reloaded: false,
        }
    }
}

impl From<RequestComponentReloadMessage> for ComponentLoadedMessage {
    fn from(req: RequestComponentReloadMessage) -> Self {
        Self {
            id: Some(req.id),
            reloaded: true,
        }
    }
}

impl From<&RequestComponentReloadMessage> for ComponentLoadedMessage {
    fn from(req: &RequestComponentReloadMessage) -> Self {
        Self {
            id: Some(req.id.clone()),
            reloaded: true,
        }
    }
}
