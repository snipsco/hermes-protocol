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
    fn from_path<P: AsRef<path::Path>>(path: P) -> Option<T>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum HermesTopic {
    Feedback(FeedbackCommand),
    DialogueManager(DialogueManagerCommand),
    Hotword(Option<String>, HotwordCommand),
    Asr(AsrCommand),
    Tts(TtsCommand),
    Nlu(NluCommand),
    Intent(String),
    AudioServer(Option<SiteId>, AudioServerCommand),
    Component(Option<String>, Component, ComponentCommand),
}

impl ToPath for HermesTopic {}

impl FromPath<Self> for HermesTopic {
    fn from_path<P: AsRef<path::Path>>(path: P) -> Option<Self> {
        let feedback = SoundCommand::iter().map(|cmd| HermesTopic::Feedback(FeedbackCommand::Sound(cmd)));
        let asr = AsrCommand::iter().map(HermesTopic::Asr);
        let tts = TtsCommand::iter().map(HermesTopic::Tts);
        let nlu = NluCommand::iter().map(HermesTopic::Nlu);
        let component = ComponentCommand::iter().flat_map(|cmd| {
            Component::iter()
                .map(|component| HermesTopic::Component(None, component, cmd))
                .collect::<Vec<HermesTopic>>()
        });
        let dialogue_manager = DialogueManagerCommand::iter().map(HermesTopic::DialogueManager);

        let path_components = path.as_ref().components()
            .collect::<Vec<::std::path::Component>>();

        let parametric1 = if path_components.len() >= 1 {
            let p = path_components[path_components.len() - 1].as_os_str().to_string_lossy();
            let audio_server = AudioServerCommand::iter().map(|cmd| HermesTopic::AudioServer(None, cmd));
            let hotword = HotwordCommand::iter().map(|cmd| HermesTopic::Hotword(None, cmd));

            let mut res: Vec<HermesTopic> = audio_server.chain(hotword).collect();
            res.extend(vec![HermesTopic::Intent(p.to_string())]);
            res
        } else {
            vec![]
        };
        let parametric2 = if path_components.len() >= 2 {
            let p1 = path_components[path_components.len() - 2].as_os_str().to_string_lossy();
            let p2 = path_components[path_components.len() - 1].as_os_str().to_string_lossy();
            let audio_server = AudioServerCommand::iter().map(|cmd| HermesTopic::AudioServer(Some(p1.to_string()), cmd));
            let hotword = HotwordCommand::iter().map(|cmd| HermesTopic::Hotword(Some(p1.to_string()), cmd));
            let component = ComponentCommand::iter().flat_map(|cmd| {
                Component::iter()
                    .map(|component| HermesTopic::Component(Some(p1.to_string()), component, cmd))
                    .collect::<Vec<HermesTopic>>()
            });
            audio_server.chain(hotword).chain(component).collect()
        } else {
            vec![]
        };
        let parametric3 = if path_components.len() >= 3 {
            let p1 = path_components[path_components.len() - 3].as_os_str().to_string_lossy();
            let _ = path_components[path_components.len() - 2].as_os_str().to_string_lossy();
            let p3 = path_components[path_components.len() - 1].as_os_str().to_string_lossy();
            vec![HermesTopic::AudioServer(Some(p1.to_string()), AudioServerCommand::PlayBytes(p3.into()))]
        } else {
            vec![]
        };

        feedback
            .chain(asr)
            .chain(tts)
            .chain(nlu)
            .chain(dialogue_manager)
            .chain(component)
            .chain(parametric1)
            .chain(parametric2)
            .chain(parametric3)
            .into_iter()
            .find(|p| path::Path::new(&p.as_path()) == path.as_ref())
    }
}

impl fmt::Display for HermesTopic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let subpath = match *self {
            HermesTopic::Feedback(ref cmd) => format!("feedback/{}", cmd.as_path()),
            HermesTopic::Hotword(ref opt_id, ref cmd) => {
                if let Some(ref id) = opt_id.as_ref() {
                    format!("{}/{}/{}", Component::Hotword.as_path(), id, cmd.as_path())
                } else {
                    format!("{}/{}", Component::Hotword.as_path(), cmd.as_path())
                }
            }
            HermesTopic::Asr(ref cmd) => format!("{}/{}", Component::Asr.as_path(), cmd.as_path()),
            HermesTopic::Tts(ref cmd) => format!("{}/{}", Component::Tts.as_path(), cmd.as_path()),
            HermesTopic::Nlu(ref cmd) => format!("{}/{}", Component::Nlu.as_path(), cmd.as_path()),
            HermesTopic::DialogueManager(ref cmd) => format!("{}/{}", Component::DialogueManager.as_path(), cmd.as_path()),
            HermesTopic::Intent(ref intent_name) => format!("intent/{}", intent_name),
            HermesTopic::Component(ref opt_id, ref component, ref cmd) => {
                if let Some(ref id) = opt_id.as_ref() {
                    format!("{}/{}/{}", component.as_path(), id, cmd.as_path())
                } else {
                    format!("{}/{}", component.as_path(), cmd.as_path())
                }
            }
            HermesTopic::AudioServer(ref opt_site_id, ref cmd) => {
                if let Some(ref site_id) = opt_site_id.as_ref() {
                    format!("{}/{}/{}", Component::AudioServer.as_path(), site_id, cmd.as_path())
                } else {
                    format!("{}/{}", Component::AudioServer.as_path(), cmd.as_path())
                }
            }
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

#[derive(Debug, Clone, PartialEq, EnumIter)]
pub enum AudioServerCommand {
    AudioFrame,
    PlayBytes(String),
    PlayFinished,
    ToggleOn,
    ToggleOff,
}

impl fmt::Display for AudioServerCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let subpath = match *self {
            AudioServerCommand::AudioFrame => "audioFrame".to_owned(),
            AudioServerCommand::PlayBytes(ref id) => format!("playBytes/{}", id),
            AudioServerCommand::PlayFinished => "playFinished".to_owned(),
            AudioServerCommand::ToggleOn => "toggleOn".to_owned(),
            AudioServerCommand::ToggleOff => "toggleOff".to_owned(),
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

#[cfg(test)]
mod tests {
    use super::*;

    fn routes() -> Vec<(HermesTopic, &'static str)> {
        vec![
            (HermesTopic::DialogueManager(DialogueManagerCommand::ToggleOn), "hermes/dialogueManager/toggleOn"),
            (HermesTopic::DialogueManager(DialogueManagerCommand::ToggleOff), "hermes/dialogueManager/toggleOff"),
            (HermesTopic::DialogueManager(DialogueManagerCommand::StartSession), "hermes/dialogueManager/startSession"),
            (HermesTopic::DialogueManager(DialogueManagerCommand::ContinueSession), "hermes/dialogueManager/continueSession"),
            (HermesTopic::DialogueManager(DialogueManagerCommand::EndSession), "hermes/dialogueManager/endSession"),
            (HermesTopic::DialogueManager(DialogueManagerCommand::SessionQueued), "hermes/dialogueManager/sessionQueued"),
            (HermesTopic::DialogueManager(DialogueManagerCommand::SessionStarted), "hermes/dialogueManager/sessionStarted"),
            (HermesTopic::DialogueManager(DialogueManagerCommand::SessionEnded), "hermes/dialogueManager/sessionEnded"),
            (HermesTopic::Component(None, Component::DialogueManager, ComponentCommand::VersionRequest), "hermes/dialogueManager/versionRequest"),
            (HermesTopic::Component(None, Component::DialogueManager, ComponentCommand::Version), "hermes/dialogueManager/version"),
            (HermesTopic::Component(None, Component::DialogueManager, ComponentCommand::Error), "hermes/dialogueManager/error"),

            (HermesTopic::Feedback(FeedbackCommand::Sound(SoundCommand::ToggleOn)), "hermes/feedback/sound/toggleOn"),
            (HermesTopic::Feedback(FeedbackCommand::Sound(SoundCommand::ToggleOff)), "hermes/feedback/sound/toggleOff"),

            (HermesTopic::Hotword(None, HotwordCommand::ToggleOn), "hermes/hotword/toggleOn"),
            (HermesTopic::Hotword(None, HotwordCommand::ToggleOff), "hermes/hotword/toggleOff"),
            (HermesTopic::Hotword(Some("default".into()), HotwordCommand::Detected), "hermes/hotword/default/detected"),
            (HermesTopic::Component(Some("default".into()), Component::Hotword, ComponentCommand::VersionRequest), "hermes/hotword/default/versionRequest"),
            (HermesTopic::Component(Some("default".into()), Component::Hotword, ComponentCommand::Version), "hermes/hotword/default/version"),
            (HermesTopic::Component(Some("default".into()), Component::Hotword, ComponentCommand::Error), "hermes/hotword/default/error"),

            (HermesTopic::Asr(AsrCommand::ToggleOn), "hermes/asr/toggleOn"),
            (HermesTopic::Asr(AsrCommand::ToggleOff), "hermes/asr/toggleOff"),
            (HermesTopic::Asr(AsrCommand::TextCaptured), "hermes/asr/textCaptured"),
            (HermesTopic::Asr(AsrCommand::PartialTextCaptured), "hermes/asr/partialTextCaptured"),
            (HermesTopic::Component(None, Component::Asr, ComponentCommand::VersionRequest), "hermes/asr/versionRequest"),
            (HermesTopic::Component(None, Component::Asr, ComponentCommand::Version), "hermes/asr/version"),
            (HermesTopic::Component(None, Component::Asr, ComponentCommand::Error), "hermes/asr/error"),

            (HermesTopic::AudioServer(None, AudioServerCommand::ToggleOn), "hermes/audioServer/toggleOn"),
            (HermesTopic::AudioServer(None, AudioServerCommand::ToggleOff), "hermes/audioServer/toggleOff"),
            (HermesTopic::AudioServer(Some("default".into()), AudioServerCommand::AudioFrame), "hermes/audioServer/default/audioFrame"),
            (HermesTopic::AudioServer(Some("default".into()), AudioServerCommand::PlayBytes("kikoo".into())), "hermes/audioServer/default/playBytes/kikoo"),
            (HermesTopic::AudioServer(Some("default".into()), AudioServerCommand::PlayFinished), "hermes/audioServer/default/playFinished"),
            (HermesTopic::Component(Some("default".into()), Component::AudioServer, ComponentCommand::VersionRequest), "hermes/audioServer/default/versionRequest"),
            (HermesTopic::Component(Some("default".into()), Component::AudioServer, ComponentCommand::Version), "hermes/audioServer/default/version"),
            (HermesTopic::Component(Some("default".into()), Component::AudioServer, ComponentCommand::Error), "hermes/audioServer/default/error"),

            (HermesTopic::Tts(TtsCommand::Say), "hermes/tts/say"),
            (HermesTopic::Tts(TtsCommand::SayFinished), "hermes/tts/sayFinished"),
            (HermesTopic::Component(None, Component::Tts, ComponentCommand::VersionRequest), "hermes/tts/versionRequest"),
            (HermesTopic::Component(None, Component::Tts, ComponentCommand::Version), "hermes/tts/version"),
            (HermesTopic::Component(None, Component::Tts, ComponentCommand::Error), "hermes/tts/error"),

            (HermesTopic::Intent("harakiri_intent".into()), "hermes/intent/harakiri_intent"),

            (HermesTopic::Nlu(NluCommand::Query), "hermes/nlu/query"),
            (HermesTopic::Nlu(NluCommand::PartialQuery), "hermes/nlu/partialQuery"),
            (HermesTopic::Nlu(NluCommand::SlotParsed), "hermes/nlu/slotParsed"),
            (HermesTopic::Nlu(NluCommand::IntentParsed), "hermes/nlu/intentParsed"),
            (HermesTopic::Nlu(NluCommand::IntentNotRecognized), "hermes/nlu/intentNotRecognized"),
            (HermesTopic::Component(None, Component::Nlu, ComponentCommand::VersionRequest), "hermes/nlu/versionRequest"),
            (HermesTopic::Component(None, Component::Nlu, ComponentCommand::Version), "hermes/nlu/version"),
            (HermesTopic::Component(None, Component::Nlu, ComponentCommand::Error), "hermes/nlu/error"),
        ]
    }

    #[test]
    fn string_to_enum_conversion_works() {
        for (route, expected_path) in routes() {
            assert_eq!(route.as_path(), expected_path);
        }
    }

    #[test]
    fn enum_to_string_conversion_works() {
        for (expected_route, path) in routes() {
            assert_eq!(HermesTopic::from_path(path), Some(expected_route));
        }
    }
}
