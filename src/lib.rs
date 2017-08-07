#[macro_use]
extern crate error_chain;
extern crate semver;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate nlu_rust_ontology;
extern crate strum;
#[macro_use]
extern crate strum_macros;

mod errors;

use std::path;
use std::string::ToString;

use strum::IntoEnumIterator;

pub use nlu_rust_ontology::*;

pub trait ToPath: ToString{
    fn as_path(&self) -> String {
        let raw_path = self.to_string();
        let mut c = raw_path.chars();

        match c.next() {
            None => String::new(),
            Some(f) => f.to_lowercase().chain(c).collect(),
        }
    }
}

pub trait FromPath<T: Sized> {
    fn from_path(&str) -> Option<T>;
}

// - Topics

#[derive(Debug, Clone, PartialEq)]
pub enum HermesTopic {
    Feedback(FeedbackCommand),
    Hotword(HotwordCommand),
    Asr(AsrCommand),
    Tts(TtsCommand),
    Nlu(NluCommand),
    Intent(String),
    AudioServer(AudioServerCommand),
    Component(Component, ComponentCommand),
}

impl ToPath for HermesTopic {}

impl FromPath<Self> for HermesTopic {
    fn from_path(path: &str) -> Option<Self> {
        let feedback = SoundCommand::iter().map(|cmd| HermesTopic::Feedback(FeedbackCommand::Sound(cmd)));
        let hotword = HotwordCommand::iter().map(HermesTopic::Hotword);
        let asr = AsrCommand::iter().map(HermesTopic::Asr);
        let tts = TtsCommand::iter().map(HermesTopic::Tts);
        let nlu = NluCommand::iter().map(HermesTopic::Nlu);
        let audio_server = AudioServerCommand::iter().map(HermesTopic::AudioServer);
        let component = ComponentCommand::iter().flat_map(|cmd| {
            Component::iter()
                .map(|component| HermesTopic::Component(component, cmd))
                .collect::<Vec<HermesTopic>>()
        });
        let intent = if let Some(last_component) = path::PathBuf::from(path).components().last() {
            last_component.as_os_str().to_str()
                .map(|intent_name| vec![HermesTopic::Intent(intent_name.to_string())])
                .unwrap_or(vec![])
        } else {
            vec![]
        };

        feedback
            .chain(hotword)
            .chain(asr)
            .chain(tts)
            .chain(nlu)
            .chain(audio_server)
            .chain(component)
            .chain(intent)
            .into_iter()
            .find(|p| p.as_path() == path)
    }
}

impl std::fmt::Display for HermesTopic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let subpath = match *self {
            HermesTopic::Feedback(ref cmd) => format!("feedback/{}", cmd.as_path()),
            HermesTopic::Hotword(ref cmd) => format!("{}/{}", Component::Hotword.as_path(), cmd.as_path()),
            HermesTopic::Asr(ref cmd) => format!("{}/{}", Component::Asr.as_path(), cmd.as_path()),
            HermesTopic::Tts(ref cmd) => format!("{}/{}", Component::Tts.as_path(), cmd.as_path()),
            HermesTopic::Nlu(ref cmd) => format!("{}/{}", Component::Nlu.as_path(), cmd.as_path()),
            HermesTopic::Intent(ref intent_name) => format!("intent/{}", intent_name),
            HermesTopic::AudioServer(ref cmd) => format!("{}/{}", Component::AudioServer.as_path(), cmd.as_path()),
            HermesTopic::Component(ref component, ref cmd) => format!("component/{}/{}", component.as_path(), cmd.as_path()),
        };
        write!(f, "hermes/{}", subpath)
    }
}

// - Components

#[derive(Debug, Clone, Copy, PartialEq, ToString, EnumIter)]
pub enum Component {
    Hotword,
    Asr,
    Tts,
    Nlu,
    DialogManager,
    IntentParserManager,
    SkillManager,
    AudioServer,
}

impl ToPath for Component {}

// - Commands

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FeedbackCommand {
    Sound(SoundCommand),
}

impl ToPath for FeedbackCommand {}

impl std::fmt::Display for FeedbackCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let subpath = match *self {
            FeedbackCommand::Sound(ref cmd) => format!("sound/{}", cmd.as_path()),
        };
        write!(f, "{}", subpath)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, ToString, EnumIter)]
pub enum SoundCommand {
    ToggleOn,
    ToggleOff,
}

impl ToPath for SoundCommand {}

#[derive(Debug, Clone, Copy, PartialEq, ToString, EnumIter)]
pub enum HotwordCommand {
    ToggleOn,
    ToggleOff,
    Wait,
    Detected
}

impl ToPath for HotwordCommand {}

#[derive(Debug, Clone, Copy, PartialEq, ToString, EnumIter)]
pub enum AsrCommand {
    ToggleOn,
    ToggleOff,
    TextCaptured,
    PartialTextCaptured,
}

impl ToPath for AsrCommand {}

#[derive(Debug, Clone, Copy, PartialEq, ToString, EnumIter)]
pub enum TtsCommand {
    Say,
    SayFinished,
}

impl ToPath for TtsCommand {}

#[derive(Debug, Clone, Copy, PartialEq, ToString, EnumIter)]
pub enum NluCommand {
    Query,
    PartialQuery,
    SlotParsed,
    IntentParsed,
    IntentNotRecognized,
}

impl ToPath for NluCommand {}

#[derive(Debug, Clone, Copy, PartialEq, ToString, EnumIter)]
pub enum AudioServerCommand {
    PlayFile,
    PlayBytes,
    PlayFinished,
}

impl ToPath for AudioServerCommand {}

#[derive(Debug, Clone, Copy, PartialEq, ToString, EnumIter)]
pub enum ComponentCommand {
    VersionRequest,
    Version,
    Error,
}

impl ToPath for ComponentCommand {}

// - Messages

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct TextCapturedMessage {
    pub text: String,
    pub likelihood: f32,
    pub seconds: f32,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct NluQueryMessage {
    pub text: String,
    pub likelihood: Option<f32>,
    pub seconds: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct NluSlotQueryMessage {
    pub text: String,
    pub likelihood: f32,
    pub seconds: f32,
    #[serde(rename = "intentName")]
    pub intent_name: String,
    #[serde(rename = "slotName")]
    pub slot_name: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct PlayFileMessage {
    #[serde(rename = "filePath")]
    pub file_path: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct PlayBytesMessage {
    pub id: String,
    #[serde(rename = "wavBytes")]
    pub wav_bytes: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct PlayFinishedMessage {
    pub id: String
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct SayMessage {
    pub text: String,
    pub lang: Option<String>
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
pub struct VersionMessage {
    pub component: String,
    pub version: semver::Version,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ErrorMessage {
    pub error: String,
    pub context: Option<String>,
}
