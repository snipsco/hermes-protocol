#[macro_use]
extern crate error_chain;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod errors;

use std::collections::HashMap;
use std::ops::Range;
//use errors::*;

pub enum HermesTopic {
    Hotword,
    Intent,
    Speech,
    SpeechToText,
    TextToSpeech,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct SpeechToTextMessage {
    pub text: String,
    pub likelihood: f32,
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

impl HermesTopic {
    fn all() -> Vec<Self> {
        vec![
            HermesTopic::Hotword,
            HermesTopic::Intent,
            HermesTopic::Speech,
            HermesTopic::SpeechToText,
            HermesTopic::TextToSpeech,
        ]
    }

    pub fn from_path(path: &str) -> Option<Self> {
        HermesTopic::all().into_iter().find(|m| m.as_path() == path)
    }

    pub fn as_path(&self) -> &str {
        match *self {
            HermesTopic::Hotword => "hermes/hotword",
            HermesTopic::Intent => "hermes/intent",
            HermesTopic::Speech => "hermes/speech",
            HermesTopic::SpeechToText => "hermes/stt",
            HermesTopic::TextToSpeech => "hermes/tts",
        }
    }
}
