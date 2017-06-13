#[macro_use]
extern crate error_chain;
extern crate semver;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod errors;
mod builtin_entities_ontology;

use std::path;
use std::ops::Range;

pub use builtin_entities_ontology::*;


pub trait ToPath {
    fn as_path(&self) -> String;
}

pub trait FromPath<T: Sized> {
    fn from_path(&str) -> Option<T>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum HermesTopic {
    Feedback(FeedbackCommand),
    Hotword(HotwordCommand),
    ASR(ASRCommand),
    TTS(TTSCommand),
    NLU(NLUCommand),
    Intent(String),
    AudioServer(AudioServerCommand),
    Component(Component, ComponentCommand),
}

impl ToPath for HermesTopic {
    fn as_path(&self) -> String {
        let subpath = match *self {
            HermesTopic::Feedback(ref cmd) => format!("feedback/{}", cmd.as_path()),
            HermesTopic::Hotword(ref cmd) => format!("{}/{}", Component::Hotword.as_path(), cmd.as_path()),
            HermesTopic::ASR(ref cmd) => format!("{}/{}", Component::ASR.as_path(), cmd.as_path()),
            HermesTopic::TTS(ref cmd) => format!("{}/{}", Component::TTS.as_path(), cmd.as_path()),
            HermesTopic::NLU(ref cmd) => format!("{}/{}", Component::NLU.as_path(), cmd.as_path()),
            HermesTopic::Intent(ref intent_name) => format!("intent/{}", intent_name),
            HermesTopic::AudioServer(ref cmd) => format!("{}/{}", Component::AudioServer.as_path(), cmd.as_path()),
            HermesTopic::Component(ref component, ref cmd) => format!("component/{}/{}", component.as_path(), cmd.as_path()),
        };
        format!("hermes/{}", subpath)
    }
}

impl FromPath<Self> for HermesTopic {
    fn from_path(path: &str) -> Option<Self> {
        let mut all = vec![
            HermesTopic::Feedback(FeedbackCommand::Sound(SoundCommand::ToggleOn)),
            HermesTopic::Feedback(FeedbackCommand::Sound(SoundCommand::ToggleOff)),
            HermesTopic::Hotword(HotwordCommand::ToggleOn),
            HermesTopic::Hotword(HotwordCommand::ToggleOff),
            HermesTopic::Hotword(HotwordCommand::Wait),
            HermesTopic::Hotword(HotwordCommand::Detected),
            HermesTopic::ASR(ASRCommand::ToggleOn),
            HermesTopic::ASR(ASRCommand::ToggleOff),
            HermesTopic::ASR(ASRCommand::TextCaptured),
            HermesTopic::ASR(ASRCommand::PartialTextCaptured),
            HermesTopic::TTS(TTSCommand::Say),
            HermesTopic::TTS(TTSCommand::SayFinished),
            HermesTopic::NLU(NLUCommand::Query),
            HermesTopic::NLU(NLUCommand::PartialQuery),
            HermesTopic::NLU(NLUCommand::IntentParsed),
            HermesTopic::NLU(NLUCommand::SlotParsed),
            HermesTopic::NLU(NLUCommand::IntentNotRecognized),
            HermesTopic::AudioServer(AudioServerCommand::PlayFile),
            HermesTopic::Component(Component::AudioServer, ComponentCommand::VersionRequest),
            HermesTopic::Component(Component::AudioServer, ComponentCommand::Version),
            HermesTopic::Component(Component::Hotword, ComponentCommand::VersionRequest),
            HermesTopic::Component(Component::Hotword, ComponentCommand::Version),
            HermesTopic::Component(Component::Hotword, ComponentCommand::Error),
            HermesTopic::Component(Component::ASR, ComponentCommand::VersionRequest),
            HermesTopic::Component(Component::ASR, ComponentCommand::Version),
            HermesTopic::Component(Component::ASR, ComponentCommand::Error),
            HermesTopic::Component(Component::TTS, ComponentCommand::VersionRequest),
            HermesTopic::Component(Component::TTS, ComponentCommand::Version),
            HermesTopic::Component(Component::TTS, ComponentCommand::Error),
            HermesTopic::Component(Component::NLU, ComponentCommand::VersionRequest),
            HermesTopic::Component(Component::NLU, ComponentCommand::Version),
            HermesTopic::Component(Component::NLU, ComponentCommand::Error),
            HermesTopic::Component(Component::DialogManager, ComponentCommand::VersionRequest),
            HermesTopic::Component(Component::DialogManager, ComponentCommand::Version),
            HermesTopic::Component(Component::DialogManager, ComponentCommand::Error),
            HermesTopic::Component(Component::IntentParserManager, ComponentCommand::VersionRequest),
            HermesTopic::Component(Component::IntentParserManager, ComponentCommand::Version),
            HermesTopic::Component(Component::IntentParserManager, ComponentCommand::Error),
            HermesTopic::Component(Component::SkillManager, ComponentCommand::VersionRequest),
            HermesTopic::Component(Component::SkillManager, ComponentCommand::Version),
            HermesTopic::Component(Component::SkillManager, ComponentCommand::Error),
        ];

        let path_buf = path::PathBuf::from(path);
        if let Some(last_component) = path_buf.components().last() {
            if let Some(intent_name) = last_component.as_os_str().to_str() {
                all.push(HermesTopic::Intent(intent_name.to_string()));
            }
        }

        all.into_iter().find(|p| p.as_path() == path)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Component {
    Hotword,
    ASR,
    TTS,
    NLU,
    DialogManager,
    IntentParserManager,
    SkillManager,
    AudioServer,
}

impl ToPath for Component {
    fn as_path(&self) -> String {
        match *self {
            Component::Hotword => "hotword",
            Component::ASR => "asr",
            Component::TTS => "tts",
            Component::NLU => "nlu",
            Component::DialogManager => "dialogManager",
            Component::IntentParserManager => "intentParserManager",
            Component::SkillManager => "skillManager",
            Component::AudioServer => "audioServer"
        }.into()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FeedbackCommand {
    Sound(SoundCommand),
}

impl ToPath for FeedbackCommand {
    fn as_path(&self) -> String {
        match *self {
            FeedbackCommand::Sound(ref cmd) => format!("sound/{}", cmd.as_path()),
        }.into()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SoundCommand {
    ToggleOn,
    ToggleOff,
}

impl ToPath for SoundCommand {
    fn as_path(&self) -> String {
        match *self {
            SoundCommand::ToggleOn => "toggleOn",
            SoundCommand::ToggleOff => "toggleOff",
        }.into()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum HotwordCommand {
    ToggleOn,
    ToggleOff,
    Wait,
    Detected
}

impl ToPath for HotwordCommand {
    fn as_path(&self) -> String {
        match *self {
            HotwordCommand::ToggleOn => "toggleOn",
            HotwordCommand::ToggleOff => "toggleOff",
            HotwordCommand::Wait => "wait",
            HotwordCommand::Detected => "detected",
        }.into()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ASRCommand {
    ToggleOn,
    ToggleOff,
    TextCaptured,
    PartialTextCaptured,
}

impl ToPath for ASRCommand {
    fn as_path(&self) -> String {
        match *self {
            ASRCommand::ToggleOn => "toggleOn",
            ASRCommand::ToggleOff => "toggleOff",
            ASRCommand::TextCaptured => "textCaptured",
            ASRCommand::PartialTextCaptured => "partialTextCaptured",
        }.into()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TTSCommand {
    Say,
    SayFinished,
}

impl ToPath for TTSCommand {
    fn as_path(&self) -> String {
        match *self {
            TTSCommand::Say => "say",
            TTSCommand::SayFinished => "sayFinished",
        }.into()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum NLUCommand {
    Query,
    PartialQuery,
    SlotParsed,
    IntentParsed,
    IntentNotRecognized,
}

impl ToPath for NLUCommand {
    fn as_path(&self) -> String {
        match *self {
            NLUCommand::Query => "query",
            NLUCommand::PartialQuery => "partialQuery",
            NLUCommand::SlotParsed => "slotParsed",
            NLUCommand::IntentParsed => "intentParsed",
            NLUCommand::IntentNotRecognized => "IntentNotRecognized",
        }.into()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AudioServerCommand {
    PlayFile
}

impl ToPath for AudioServerCommand {
    fn as_path(&self) -> String {
        match *self {
            AudioServerCommand::PlayFile => "playFile",
        }.into()
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum ComponentCommand {
    VersionRequest,
    Version,
    Error,
}

impl ToPath for ComponentCommand {
    fn as_path(&self) -> String {
        match *self {
            ComponentCommand::VersionRequest => "versionRequest",
            ComponentCommand::Version => "version",
            ComponentCommand::Error => "error",
        }.into()
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct TextCapturedMessage {
    pub text: String,
    pub likelihood: f32,
    pub seconds: f32,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct NLUQueryMessage {
    pub text: String,
    pub likelihood: Option<f32>,
    pub seconds: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct NLUSlotQueryMessage {
    pub text: String,
    pub likelihood: f32,
    pub seconds: f32,
    #[serde(rename="intentName")]
    pub intent_name: String,
    #[serde(rename="slotName")]
    pub slot_name: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct PlayFileMessage {
    #[serde(rename="filePath")]
    pub file_path: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct SayMessage {
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SlotMessage {
    pub slot: Option<Slot>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct IntentNotRecognizedMessage {
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct IntentMessage {
    pub input: String,
    pub intent: IntentClassifierResult,
    pub slots: Option<Vec<Slot>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct IntentClassifierResult {
    #[serde(rename="intentName")]
    pub intent_name: String,
    pub probability: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Slot {
    pub value: SlotValue,
    pub raw_value: String,
    pub range: Option<Range<usize>>,
    pub entity: String,
    #[serde(rename="slotName")]
    pub slot_name: String
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content="value")]
pub enum SlotValue {
    Custom(String),
    Builtin(BuiltinEntity),
}


#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct VersionMessage {
    pub component: String,
    pub version: semver::Version,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ErrorMessage {
    pub error: String,
    pub context: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_custom_slot() {
        let slot = Slot {
            value: SlotValue::Custom("value".into()),
            range: None,
            entity: "toto".into(),
            slot_name: "toto".into(),
        };

        assert!(serde_json::to_string(&slot).is_ok());
        assert!(serde_json::from_str::<Slot>(&serde_json::to_string(&slot).unwrap()).is_ok());
    }

    #[test]
    fn test_builtin_slot_1() {
        let slot = Slot {
            value: SlotValue::Builtin(BuiltinEntity::Ordinal(OrdinalValue(5))),
            range: None,
            entity: "toto".into(),
            slot_name: "toto".into(),
        };
        assert!(serde_json::to_string(&slot).is_ok());
        assert!(serde_json::from_str::<Slot>(&serde_json::to_string(&slot).unwrap()).is_ok());
    }

    #[test]
    fn test_builtin_slot_2() {
        let slot = Slot {
            value: SlotValue::Builtin(
                    BuiltinEntity::Time(
                        TimeValue::InstantTime(
                            InstantTimeValue { 
                                value: "some_value".into(), 
                                grain: Grain::Year, 
                                precision: Precision::Exact 
                            }
                        )
                    )
                ),
            range: None,
            entity: "toto".into(),
            slot_name: "toto".into(),
        };
        assert!(serde_json::to_string(&slot).is_ok());
        assert!(serde_json::from_str::<Slot>(&serde_json::to_string(&slot).unwrap()).is_ok());
    }
}

