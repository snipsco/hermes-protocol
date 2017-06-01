#[macro_use]
extern crate error_chain;
extern crate semver;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod errors;

use std::path;
use std::ops::Range;
//use errors::*;

pub trait ToPath {
    fn as_path(&self) -> String;
}

pub trait FromPath<T: Sized> {
    fn from_path(&str) -> Option<T>;
}

#[derive(Debug)]
pub enum HermesTopic {
    Hotword(HotwordCommand),
    ASR(ASRCommand),
    TTS(TTSCommand),
    NLU(NLUCommand),
    Intent(String),
    Component(Component, ComponentCommand),
}

impl ToPath for HermesTopic {
    fn as_path(&self) -> String {
        let subpath = match *self {
            HermesTopic::Hotword(ref cmd) => format!("{}/{}", Component::Hotword.as_path(), cmd.as_path()),
            HermesTopic::ASR(ref cmd) => format!("{}/{}", Component::ASR.as_path(), cmd.as_path()),
            HermesTopic::TTS(ref cmd) => format!("{}/{}", Component::TTS.as_path(), cmd.as_path()),
            HermesTopic::NLU(ref cmd) => format!("{}/{}", Component::NLU.as_path(), cmd.as_path()),
            HermesTopic::Intent(ref intent_name) => format!("intent/{}", intent_name),
            HermesTopic::Component(ref component, ref cmd) => format!("component/{}/{}", component.as_path(), cmd.as_path()),
        };
        format!("hermes/{}", subpath)
    }
}

impl FromPath<Self> for HermesTopic {
    fn from_path(path: &str) -> Option<Self> {
        let mut all = vec![
            HermesTopic::Hotword(HotwordCommand::ToggleOn),
            HermesTopic::Hotword(HotwordCommand::ToggleOff),
            HermesTopic::Hotword(HotwordCommand::Wait),
            HermesTopic::Hotword(HotwordCommand::Detected),
            HermesTopic::ASR(ASRCommand::StartListening),
            HermesTopic::ASR(ASRCommand::StopListening),
            HermesTopic::ASR(ASRCommand::TextCaptured),
            HermesTopic::ASR(ASRCommand::PartialTextCaptured),
            HermesTopic::TTS(TTSCommand::Say),
            HermesTopic::TTS(TTSCommand::SayFinished),
            HermesTopic::NLU(NLUCommand::Query),
            HermesTopic::NLU(NLUCommand::IntentParsed),
            HermesTopic::NLU(NLUCommand::Error),
            HermesTopic::Component(Component::Hotword, ComponentCommand::VersionRequest),
            HermesTopic::Component(Component::Hotword, ComponentCommand::Version),
            HermesTopic::Component(Component::ASR, ComponentCommand::VersionRequest),
            HermesTopic::Component(Component::ASR, ComponentCommand::Version),
            HermesTopic::Component(Component::TTS, ComponentCommand::VersionRequest),
            HermesTopic::Component(Component::TTS, ComponentCommand::Version),
            HermesTopic::Component(Component::NLU, ComponentCommand::VersionRequest),
            HermesTopic::Component(Component::NLU, ComponentCommand::Version),
            HermesTopic::Component(Component::DialogManager, ComponentCommand::VersionRequest),
            HermesTopic::Component(Component::DialogManager, ComponentCommand::Version),
            HermesTopic::Component(Component::IntentParserManager, ComponentCommand::VersionRequest),
            HermesTopic::Component(Component::IntentParserManager, ComponentCommand::Version),
            HermesTopic::Component(Component::SkillManager, ComponentCommand::VersionRequest),
            HermesTopic::Component(Component::SkillManager, ComponentCommand::Version),
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

#[derive(Debug)]
pub enum Component {
    Hotword,
    ASR,
    TTS,
    NLU,
    DialogManager,
    IntentParserManager,
    SkillManager,
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
        }.into()
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum ASRCommand {
    StartListening,
    StopListening,
    TextCaptured,
    PartialTextCaptured,
}

impl ToPath for ASRCommand {
    fn as_path(&self) -> String {
        match *self {
            ASRCommand::StartListening => "startListening",
            ASRCommand::StopListening => "stopListening",
            ASRCommand::TextCaptured => "textCaptured",
            ASRCommand::PartialTextCaptured => "partialTextCaptured",
        }.into()
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum NLUCommand {
    Query,
    IntentParsed,
    Error
}

impl ToPath for NLUCommand {
    fn as_path(&self) -> String {
        match *self {
            NLUCommand::Query => "query",
            NLUCommand::IntentParsed => "intentParsed",
            NLUCommand::Error => "error",
        }.into()
    }
}

#[derive(Debug)]
pub enum ComponentCommand {
    VersionRequest,
    Version,
}

impl ToPath for ComponentCommand {
    fn as_path(&self) -> String {
        match *self {
            ComponentCommand::VersionRequest => "versionRequest",
            ComponentCommand::Version => "version",
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
pub struct SayMessage {
    pub text: String,
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
    pub component: String,
    pub version: semver::Version,
}
