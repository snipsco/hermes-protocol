use std::fmt;

use semver;

use serde::{Deserialize, Serialize};

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

pub mod vad;
pub use vad::*;

pub mod hotword;
pub use hotword::*;

pub mod asr;
pub use asr::*;

pub mod nlu;
pub use nlu::*;

pub mod audio_server;
pub use audio_server::*;

pub mod tts;
pub use tts::*;

pub mod injection;
pub use injection::*;

pub mod dialogue;
pub use dialogue::*;

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
