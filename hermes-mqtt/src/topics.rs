use hermes::SiteId;

use std::{fmt, path};
use strum::IntoEnumIterator;


pub trait ToPath: ToString {
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
    fn from_path(path : &str) -> Option<T>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum HermesTopic {
    Feedback(FeedbackCommand),
    DialogueManager(DialogueManagerCommand),
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
        let audio_server = vec![HermesTopic::AudioServer(AudioServerCommand::PlayFinished)];
        let component = ComponentCommand::iter().flat_map(|cmd| {
            Component::iter()
                .map(|component| HermesTopic::Component(component, cmd))
                .collect::<Vec<HermesTopic>>()
        });
        let dialogue_manager = DialogueManagerCommand::iter().map(HermesTopic::DialogueManager);
        let path_buf = path::PathBuf::from(path);
        let path_components = path_buf.components()
            .collect::<Vec<::std::path::Component>>();
        let parametric1 = if path_components.len() >= 1 {
            let p = path_components[path_components.len() - 1].as_os_str().to_string_lossy();
            vec![HermesTopic::Intent(p.to_string()),
                 HermesTopic::AudioServer(AudioServerCommand::AudioFrame(p.into()))]
        } else {
            vec![]
        };
        let parametric2 = if path_components.len() >= 2 {
            let p1 = path_components[path_components.len() - 2].as_os_str().to_string_lossy();
            let p2 = path_components[path_components.len() - 1].as_os_str().to_string_lossy();
            vec![HermesTopic::AudioServer(AudioServerCommand::PlayBytes(p1.into(), p2.into()))]
        } else {
            vec![]
        };
        feedback
            .chain(hotword)
            .chain(asr)
            .chain(tts)
            .chain(nlu)
            .chain(audio_server)
            .chain(dialogue_manager)
            .chain(component)
            .chain(parametric1)
            .chain(parametric2)
            .into_iter()
            .find(|p| p.as_path() == path)
    }
}

impl fmt::Display for HermesTopic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let subpath = match *self {
            HermesTopic::Feedback(ref cmd) => format!("feedback/{}", cmd.as_path()),
            HermesTopic::Hotword(ref cmd) => format!("{}/{}", Component::Hotword.as_path(), cmd.as_path()),
            HermesTopic::Asr(ref cmd) => format!("{}/{}", Component::Asr.as_path(), cmd.as_path()),
            HermesTopic::Tts(ref cmd) => format!("{}/{}", Component::Tts.as_path(), cmd.as_path()),
            HermesTopic::Nlu(ref cmd) => format!("{}/{}", Component::Nlu.as_path(), cmd.as_path()),
            HermesTopic::DialogueManager(ref cmd) => format!("{}/{}", Component::DialogueManager.as_path(), cmd.as_path()),
            HermesTopic::Intent(ref intent_name) => format!("intent/{}", intent_name),
            HermesTopic::AudioServer(ref cmd) => format!("{}/{}", Component::AudioServer.as_path(), cmd.as_path()),
            HermesTopic::Component(ref component, ref cmd) => format!("component/{}/{}", component.as_path(), cmd.as_path()),
        };
        write!(f, "hermes/{}", subpath)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, ToString, EnumIter)]
pub enum Component {
    Hotword,
    Asr,
    Tts,
    Nlu,
    DialogueManager,
    IntentParserManager,
    SkillManager,
    AudioServer,
}

impl ToPath for Component {}


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FeedbackCommand {
    Sound(SoundCommand),
}

impl ToPath for FeedbackCommand {}

impl fmt::Display for FeedbackCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
pub enum DialogueManagerCommand {
    ToggleOn,
    ToggleOff,
    StartSession,
    ContinueSession,
    EndSession,
    SessionQueued,
    SessionStarted,
    SessionEnded,
}


impl ToPath for DialogueManagerCommand {}

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

#[derive(Debug, Clone, PartialEq)]
pub enum AudioServerCommand {
    AudioFrame(SiteId),
    PlayBytes(SiteId, String),
    PlayFinished,
}

impl fmt::Display for AudioServerCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let subpath = match *self {
            AudioServerCommand::AudioFrame(ref site_id) => format!("audioFrame/{}", site_id),
            AudioServerCommand::PlayBytes(ref site_id, ref id) => format!("playBytes/{}/{}", site_id, id),
            AudioServerCommand::PlayFinished => "playFinished".into(),
        };
        write!(f, "{}", subpath)
    }
}

impl ToPath for AudioServerCommand {}

#[derive(Debug, Clone, Copy, PartialEq, ToString, EnumIter)]
pub enum ComponentCommand {
    VersionRequest,
    Version,
    Error,
}

impl ToPath for ComponentCommand {}

