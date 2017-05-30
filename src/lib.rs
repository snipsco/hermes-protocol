#[macro_use]
extern crate error_chain;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod errors;

use std::ops::Range;
//use errors::*;

pub enum HermesTopic<'a> {
    Hotword,
    Intent(&'a str),
    Speech,
    SpeechToText,
    TextToSpeech,
    Version(&'a str),
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct SpeechToTextMessage {
    pub text: String,
    pub likelihood: f32,
    pub seconds: f32,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct IntentMessage {
    pub input: String,
    pub intent: Option<IntentClassifierResult>,
    pub slots: Option<Vec<Slot>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct IntentClassifierResult {
    pub intent_name: String,
    pub probability: f32,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Slot {
    pub value: String,
    pub range: Range<usize>,
    pub entity: String,
    pub slot_name: String
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct VersionMessage {
    pub minor: usize,
    pub major: usize,
    pub patch: usize,
}

impl<'a> HermesTopic<'a> {
    fn all() -> Vec<Self> {
        vec![
            HermesTopic::Hotword,
            HermesTopic::Intent(""),
            HermesTopic::Speech,
            HermesTopic::SpeechToText,
            HermesTopic::TextToSpeech,
            HermesTopic::Version(""),
        ]
    }

    pub fn from_path(path: &str) -> Option<Self> {
        HermesTopic::all().into_iter().find(|m| m.as_path() == path)
    }

    pub fn as_path(&self) -> String {
        match *self {
            HermesTopic::Hotword => "hermes/hotword".into(),
            HermesTopic::Intent(name) => format!("hermes/intent/{}", name),
            HermesTopic::Speech => "hermes/speech".into(),
            HermesTopic::SpeechToText => "hermes/stt".into(),
            HermesTopic::TextToSpeech => "hermes/tts".into(),
            HermesTopic::Version(component) => format!("hermes/version/{}", component),
        }
    }
}
