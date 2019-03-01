use std::{fmt, path};

use strum_macros::ToString;

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
    VoiceActivity(String, VoiceActivityCommand),
    Hotword(Option<String>, HotwordCommand),
    Asr(AsrCommand),
    Tts(TtsCommand),
    Nlu(NluCommand),
    Intent(String),
    AudioServer(Option<String>, AudioServerCommand),
    Injection(InjectionCommand),
    Component(Option<String>, Component, ComponentCommand),
}

impl ToPath for HermesTopic {}

impl HermesTopic {
    fn parse_asr<'a, It: Iterator<Item = &'a str>>(mut comps: It) -> Option<HermesTopic> {
        use self::AsrCommand::*;
        use self::HermesTopic::Asr;
        match comps.next() {
            Some("toggleOn") => Some(Asr(ToggleOn)),
            Some("toggleOff") => Some(Asr(ToggleOff)),
            Some("textCaptured") => Some(Asr(TextCaptured)),
            Some("partialTextCaptured") => Some(Asr(PartialTextCaptured)),
            Some("reload") => Some(Asr(Reload)),
            Some("versionRequest") => Some(HermesTopic::Component(
                None,
                Component::Asr,
                ComponentCommand::VersionRequest,
            )),
            Some("version") => Some(HermesTopic::Component(None, Component::Asr, ComponentCommand::Version)),
            Some("error") => Some(HermesTopic::Component(None, Component::Asr, ComponentCommand::Error)),
            _ => None,
        }
    }

    fn parse_audio_server<'a, It: Iterator<Item = &'a str>>(mut comps: It) -> Option<HermesTopic> {
        use self::AudioServerCommand::*;
        use self::HermesTopic::AudioServer;
        match (comps.next(), comps.next(), comps.next()) {
            (Some("toggleOn"), None, None) => Some(AudioServer(None, ToggleOn)),
            (Some("toggleOff"), None, None) => Some(AudioServer(None, ToggleOff)),
            (Some(site_id), Some("audioFrame"), None) => Some(AudioServer(Some(site_id.into()), AudioFrame)),
            (Some(site_id), Some("replayRequest"), None) => Some(AudioServer(Some(site_id.into()), ReplayRequest)),
            (Some(site_id), Some("replayResponse"), None) => Some(AudioServer(Some(site_id.into()), ReplayResponse)),
            (Some(site_id), Some("playBytes"), Some(file)) => {
                Some(AudioServer(Some(site_id.into()), PlayBytes(file.into())))
            }
            (Some(site_id), Some("playFinished"), None) => Some(AudioServer(Some(site_id.into()), PlayFinished)),
            (Some(site_id), Some("versionRequest"), None) => Some(HermesTopic::Component(
                Some(site_id.to_string()),
                Component::AudioServer,
                ComponentCommand::VersionRequest,
            )),
            (Some(site_id), Some("version"), None) => Some(HermesTopic::Component(
                Some(site_id.to_string()),
                Component::AudioServer,
                ComponentCommand::Version,
            )),
            (Some(site_id), Some("error"), None) => Some(HermesTopic::Component(
                Some(site_id.to_string()),
                Component::AudioServer,
                ComponentCommand::Error,
            )),
            _ => None,
        }
    }

    fn parse_dialogue_manager<'a, It: Iterator<Item = &'a str>>(mut comps: It) -> Option<HermesTopic> {
        use self::DialogueManagerCommand::*;
        use self::HermesTopic::DialogueManager;
        let command = comps.next();
        match command {
            Some("toggleOn") => Some(DialogueManager(ToggleOn)),
            Some("toggleOff") => Some(DialogueManager(ToggleOff)),
            Some("startSession") => Some(DialogueManager(StartSession)),
            Some("continueSession") => Some(DialogueManager(ContinueSession)),
            Some("endSession") => Some(DialogueManager(EndSession)),
            Some("sessionQueued") => Some(DialogueManager(SessionQueued)),
            Some("sessionStarted") => Some(DialogueManager(SessionStarted)),
            Some("sessionEnded") => Some(DialogueManager(SessionEnded)),
            Some("intentNotRecognized") => Some(DialogueManager(IntentNotRecognized)),
            Some("configure") => Some(DialogueManager(Configure)),
            Some("versionRequest") => Some(HermesTopic::Component(
                None,
                Component::DialogueManager,
                ComponentCommand::VersionRequest,
            )),
            Some("version") => Some(HermesTopic::Component(
                None,
                Component::DialogueManager,
                ComponentCommand::Version,
            )),
            Some("error") => Some(HermesTopic::Component(
                None,
                Component::DialogueManager,
                ComponentCommand::Error,
            )),
            _ => None,
        }
    }

    fn parse_feedback<'a, It: Iterator<Item = &'a str>>(mut comps: It) -> Option<HermesTopic> {
        use self::HermesTopic::Feedback;
        let medium = comps.next();
        let command = comps.next();
        match (medium, command) {
            (Some("sound"), Some("toggleOn")) => Some(Feedback(FeedbackCommand::Sound(SoundCommand::ToggleOn))),
            (Some("sound"), Some("toggleOff")) => Some(Feedback(FeedbackCommand::Sound(SoundCommand::ToggleOff))),
            _ => None,
        }
    }

    fn parse_voice_activity<'a, It: Iterator<Item = &'a str>>(mut comps: It) -> Option<HermesTopic> {
        use self::HermesTopic::VoiceActivity;
        use self::VoiceActivityCommand::*;
        let one = comps.next();
        let two = comps.next();
        match (one, two) {
            (Some(site_id), Some("vadUp")) => Some(VoiceActivity(site_id.to_string(), VadUp)),
            (Some(site_id), Some("vadDown")) => Some(VoiceActivity(site_id.to_string(), VadDown)),
            _ => None,
        }
    }

    fn parse_hotword<'a, It: Iterator<Item = &'a str>>(mut comps: It) -> Option<HermesTopic> {
        use self::HermesTopic::Hotword;
        use self::HotwordCommand::*;
        let one = comps.next();
        let two = comps.next();
        match (one, two) {
            (Some("toggleOn"), None) => Some(Hotword(None, ToggleOn)),
            (Some("toggleOff"), None) => Some(Hotword(None, ToggleOff)),
            (Some(site_id), Some("detected")) => Some(Hotword(Some(site_id.to_string()), Detected)),
            (Some(site_id), Some("versionRequest")) => Some(HermesTopic::Component(
                Some(site_id.to_string()),
                Component::Hotword,
                ComponentCommand::VersionRequest,
            )),
            (Some(site_id), Some("version")) => Some(HermesTopic::Component(
                Some(site_id.to_string()),
                Component::Hotword,
                ComponentCommand::Version,
            )),
            (Some(site_id), Some("error")) => Some(HermesTopic::Component(
                Some(site_id.to_string()),
                Component::Hotword,
                ComponentCommand::Error,
            )),
            _ => None,
        }
    }

    fn parse_intent<'a, It: Iterator<Item = &'a str>>(mut comps: It) -> Option<HermesTopic> {
        use self::HermesTopic::Intent;
        match comps.next() {
            Some(name) => Some(Intent(name.into())),
            _ => None,
        }
    }

    fn parse_nlu<'a, It: Iterator<Item = &'a str>>(mut comps: It) -> Option<HermesTopic> {
        use self::HermesTopic::Nlu;
        use self::NluCommand::*;
        let command = comps.next();
        match command {
            Some("query") => Some(Nlu(Query)),
            Some("partialQuery") => Some(Nlu(PartialQuery)),
            Some("slotParsed") => Some(Nlu(SlotParsed)),
            Some("intentParsed") => Some(Nlu(IntentParsed)),
            Some("intentNotRecognized") => Some(Nlu(IntentNotRecognized)),
            Some("reload") => Some(Nlu(Reload)),
            Some("versionRequest") => Some(HermesTopic::Component(
                None,
                Component::Nlu,
                ComponentCommand::VersionRequest,
            )),
            Some("version") => Some(HermesTopic::Component(None, Component::Nlu, ComponentCommand::Version)),
            Some("error") => Some(HermesTopic::Component(None, Component::Nlu, ComponentCommand::Error)),
            _ => None,
        }
    }

    fn parse_tts<'a, It: Iterator<Item = &'a str>>(mut comps: It) -> Option<HermesTopic> {
        use self::HermesTopic::Tts;
        use self::TtsCommand::*;
        match comps.next() {
            Some("say") => Some(Tts(Say)),
            Some("sayFinished") => Some(Tts(SayFinished)),
            Some("registerSound") => match comps.next() {
                Some(id) => Some(Tts(RegisterSound(id.into()))),
                _ => None,
            },
            Some("versionRequest") => Some(HermesTopic::Component(
                None,
                Component::Tts,
                ComponentCommand::VersionRequest,
            )),
            Some("version") => Some(HermesTopic::Component(None, Component::Tts, ComponentCommand::Version)),
            Some("error") => Some(HermesTopic::Component(None, Component::Tts, ComponentCommand::Error)),
            _ => None,
        }
    }

    fn parse_injection<'a, It: Iterator<Item = &'a str>>(mut comps: It) -> Option<HermesTopic> {
        use self::HermesTopic::Injection;
        use self::InjectionCommand::*;
        match comps.next() {
            Some("perform") => Some(Injection(Perform)),
            Some("status") => Some(Injection(Status)),
            Some("statusRequest") => Some(Injection(StatusRequest)),
            Some("versionRequest") => Some(HermesTopic::Component(
                None,
                Component::Injection,
                ComponentCommand::VersionRequest,
            )),
            Some("version") => Some(HermesTopic::Component(
                None,
                Component::Injection,
                ComponentCommand::Version,
            )),
            Some("error") => Some(HermesTopic::Component(
                None,
                Component::Injection,
                ComponentCommand::Error,
            )),
            _ => None,
        }
    }
}

impl FromPath<Self> for HermesTopic {
    fn from_path<P: AsRef<path::Path>>(path: P) -> Option<Self> {
        let comps: Vec<Option<&str>> = path.as_ref().components().map(|s| s.as_os_str().to_str()).collect();
        // sanity checks
        if comps.iter().any(Option::is_none) || comps.len() < 2 || comps[0] != Some("hermes") {
            return None;
        }
        let mut comps = comps.iter().skip(1).map(|c| c.unwrap()); // checked
        match comps.next() {
            // keep audio server first, despite alphabetical order (high
            // traffic)
            Some("audioServer") => HermesTopic::parse_audio_server(comps),
            Some("voiceActivity") => HermesTopic::parse_voice_activity(comps),
            Some("asr") => HermesTopic::parse_asr(comps),
            Some("dialogueManager") => HermesTopic::parse_dialogue_manager(comps),
            Some("feedback") => HermesTopic::parse_feedback(comps),
            Some("intent") => HermesTopic::parse_intent(comps),
            Some("hotword") => HermesTopic::parse_hotword(comps),
            Some("nlu") => HermesTopic::parse_nlu(comps),
            Some("tts") => HermesTopic::parse_tts(comps),
            Some("injection") => HermesTopic::parse_injection(comps),
            _ => None,
        }
    }
}

impl fmt::Display for HermesTopic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let subpath = match *self {
            HermesTopic::VoiceActivity(ref site_id, ref cmd) => format!("voiceActivity/{}/{}", site_id, cmd.as_path()),
            HermesTopic::Feedback(ref cmd) => format!("feedback/{}", cmd.as_path()),
            HermesTopic::Hotword(ref opt_id, ref cmd) => {
                if let Some(id) = opt_id.as_ref() {
                    format!("{}/{}/{}", Component::Hotword.as_path(), id, cmd.as_path())
                } else {
                    format!("{}/{}", Component::Hotword.as_path(), cmd.as_path())
                }
            }
            HermesTopic::Asr(ref cmd) => format!("{}/{}", Component::Asr.as_path(), cmd.as_path()),
            HermesTopic::Tts(ref cmd) => format!("{}/{}", Component::Tts.as_path(), cmd.as_path()),
            HermesTopic::Nlu(ref cmd) => format!("{}/{}", Component::Nlu.as_path(), cmd.as_path()),
            HermesTopic::DialogueManager(ref cmd) => {
                format!("{}/{}", Component::DialogueManager.as_path(), cmd.as_path())
            }
            HermesTopic::Intent(ref intent_name) => format!("intent/{}", intent_name),
            HermesTopic::Component(ref opt_id, ref component, ref cmd) => {
                if let Some(id) = opt_id.as_ref() {
                    format!("{}/{}/{}", component.as_path(), id, cmd.as_path())
                } else {
                    format!("{}/{}", component.as_path(), cmd.as_path())
                }
            }
            HermesTopic::AudioServer(ref opt_site_id, ref cmd) => {
                if let Some(site_id) = opt_site_id.as_ref() {
                    format!("{}/{}/{}", Component::AudioServer.as_path(), site_id, cmd.as_path())
                } else {
                    format!("{}/{}", Component::AudioServer.as_path(), cmd.as_path())
                }
            }
            HermesTopic::Injection(ref cmd) => format!("{}/{}", Component::Injection.as_path(), cmd.as_path()),
        };
        write!(f, "hermes/{}", subpath)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, ToString)]
pub enum Component {
    VoiceActivity,
    Hotword,
    Asr,
    Tts,
    Nlu,
    DialogueManager,
    AudioServer,
    Injection,
}

impl ToPath for Component {}

#[derive(Debug, Clone, Copy, PartialEq, ToString)]
pub enum VoiceActivityCommand {
    VadUp,
    VadDown,
}
impl ToPath for VoiceActivityCommand {}

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

#[derive(Debug, Clone, Copy, PartialEq, ToString)]
pub enum SoundCommand {
    ToggleOn,
    ToggleOff,
}

impl ToPath for SoundCommand {}

#[derive(Debug, Clone, Copy, PartialEq, ToString)]
pub enum DialogueManagerCommand {
    ToggleOn,
    ToggleOff,
    StartSession,
    ContinueSession,
    EndSession,
    SessionQueued,
    SessionStarted,
    SessionEnded,
    IntentNotRecognized,
    Configure,
}

impl ToPath for DialogueManagerCommand {}

#[derive(Debug, Clone, Copy, PartialEq, ToString)]
pub enum HotwordCommand {
    ToggleOn,
    ToggleOff,
    Detected,
}

impl ToPath for HotwordCommand {}

#[derive(Debug, Clone, Copy, PartialEq, ToString)]
pub enum AsrCommand {
    ToggleOn,
    ToggleOff,
    StartListening,
    StopListening,
    TextCaptured,
    PartialTextCaptured,
    Reload,
}

impl ToPath for AsrCommand {}

#[derive(Debug, Clone, PartialEq)]
pub enum TtsCommand {
    Say,
    SayFinished,
    RegisterSound(String),
}

impl fmt::Display for TtsCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let subpath = match *self {
            TtsCommand::Say => "say".to_owned(),
            TtsCommand::SayFinished => "sayFinished".to_owned(),
            TtsCommand::RegisterSound(ref sound) => format!("registerSound/{}", sound),
        };
        write!(f, "{}", subpath)
    }
}

impl ToPath for TtsCommand {}

#[derive(Debug, Clone, Copy, PartialEq, ToString)]
pub enum NluCommand {
    Query,
    PartialQuery,
    SlotParsed,
    IntentParsed,
    IntentNotRecognized,
    Reload,
}

impl ToPath for NluCommand {}

#[derive(Debug, Clone, PartialEq)]
pub enum AudioServerCommand {
    AudioFrame,
    ReplayRequest,
    ReplayResponse,
    PlayBytes(String),
    PlayFinished,
    ToggleOn,
    ToggleOff,
}

impl fmt::Display for AudioServerCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let subpath = match *self {
            AudioServerCommand::AudioFrame => "audioFrame".to_owned(),
            AudioServerCommand::ReplayRequest => "replayRequest".to_owned(),
            AudioServerCommand::ReplayResponse => "replayResponse".to_owned(),
            AudioServerCommand::PlayBytes(ref id) => format!("playBytes/{}", id),
            AudioServerCommand::PlayFinished => "playFinished".to_owned(),
            AudioServerCommand::ToggleOn => "toggleOn".to_owned(),
            AudioServerCommand::ToggleOff => "toggleOff".to_owned(),
        };
        write!(f, "{}", subpath)
    }
}

impl ToPath for AudioServerCommand {}

#[derive(Debug, Clone, PartialEq, ToString)]
pub enum InjectionCommand {
    Perform,
    Status,
    StatusRequest,
}

impl ToPath for InjectionCommand {}

#[derive(Debug, Clone, Copy, PartialEq, ToString)]
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
            (
                HermesTopic::DialogueManager(DialogueManagerCommand::ToggleOn),
                "hermes/dialogueManager/toggleOn",
            ),
            (
                HermesTopic::DialogueManager(DialogueManagerCommand::ToggleOff),
                "hermes/dialogueManager/toggleOff",
            ),
            (
                HermesTopic::DialogueManager(DialogueManagerCommand::StartSession),
                "hermes/dialogueManager/startSession",
            ),
            (
                HermesTopic::DialogueManager(DialogueManagerCommand::ContinueSession),
                "hermes/dialogueManager/continueSession",
            ),
            (
                HermesTopic::DialogueManager(DialogueManagerCommand::EndSession),
                "hermes/dialogueManager/endSession",
            ),
            (
                HermesTopic::DialogueManager(DialogueManagerCommand::SessionQueued),
                "hermes/dialogueManager/sessionQueued",
            ),
            (
                HermesTopic::DialogueManager(DialogueManagerCommand::SessionStarted),
                "hermes/dialogueManager/sessionStarted",
            ),
            (
                HermesTopic::DialogueManager(DialogueManagerCommand::SessionEnded),
                "hermes/dialogueManager/sessionEnded",
            ),
            (
                HermesTopic::DialogueManager(DialogueManagerCommand::IntentNotRecognized),
                "hermes/dialogueManager/intentNotRecognized",
            ),
            (
                HermesTopic::DialogueManager(DialogueManagerCommand::Configure),
                "hermes/dialogueManager/configure",
            ),
            (
                HermesTopic::Component(None, Component::DialogueManager, ComponentCommand::VersionRequest),
                "hermes/dialogueManager/versionRequest",
            ),
            (
                HermesTopic::Component(None, Component::DialogueManager, ComponentCommand::Version),
                "hermes/dialogueManager/version",
            ),
            (
                HermesTopic::Component(None, Component::DialogueManager, ComponentCommand::Error),
                "hermes/dialogueManager/error",
            ),
            (
                HermesTopic::Feedback(FeedbackCommand::Sound(SoundCommand::ToggleOn)),
                "hermes/feedback/sound/toggleOn",
            ),
            (
                HermesTopic::Feedback(FeedbackCommand::Sound(SoundCommand::ToggleOff)),
                "hermes/feedback/sound/toggleOff",
            ),
            (
                HermesTopic::VoiceActivity("mysite".into(), VoiceActivityCommand::VadUp),
                "hermes/voiceActivity/mysite/vadUp",
            ),
            (
                HermesTopic::VoiceActivity("mysite".into(), VoiceActivityCommand::VadDown),
                "hermes/voiceActivity/mysite/vadDown",
            ),
            (
                HermesTopic::Hotword(None, HotwordCommand::ToggleOn),
                "hermes/hotword/toggleOn",
            ),
            (
                HermesTopic::Hotword(None, HotwordCommand::ToggleOff),
                "hermes/hotword/toggleOff",
            ),
            (
                HermesTopic::Hotword(Some("default".into()), HotwordCommand::Detected),
                "hermes/hotword/default/detected",
            ),
            (
                HermesTopic::Component(
                    Some("default".into()),
                    Component::Hotword,
                    ComponentCommand::VersionRequest,
                ),
                "hermes/hotword/default/versionRequest",
            ),
            (
                HermesTopic::Component(Some("default".into()), Component::Hotword, ComponentCommand::Version),
                "hermes/hotword/default/version",
            ),
            (
                HermesTopic::Component(Some("default".into()), Component::Hotword, ComponentCommand::Error),
                "hermes/hotword/default/error",
            ),
            (HermesTopic::Asr(AsrCommand::ToggleOn), "hermes/asr/toggleOn"),
            (HermesTopic::Asr(AsrCommand::ToggleOff), "hermes/asr/toggleOff"),
            (HermesTopic::Asr(AsrCommand::TextCaptured), "hermes/asr/textCaptured"),
            (
                HermesTopic::Asr(AsrCommand::PartialTextCaptured),
                "hermes/asr/partialTextCaptured",
            ),
            (HermesTopic::Asr(AsrCommand::Reload), "hermes/asr/reload"),
            (
                HermesTopic::Component(None, Component::Asr, ComponentCommand::VersionRequest),
                "hermes/asr/versionRequest",
            ),
            (
                HermesTopic::Component(None, Component::Asr, ComponentCommand::Version),
                "hermes/asr/version",
            ),
            (
                HermesTopic::Component(None, Component::Asr, ComponentCommand::Error),
                "hermes/asr/error",
            ),
            (
                HermesTopic::AudioServer(None, AudioServerCommand::ToggleOn),
                "hermes/audioServer/toggleOn",
            ),
            (
                HermesTopic::AudioServer(None, AudioServerCommand::ToggleOff),
                "hermes/audioServer/toggleOff",
            ),
            (
                HermesTopic::AudioServer(Some("default".into()), AudioServerCommand::AudioFrame),
                "hermes/audioServer/default/audioFrame",
            ),
            (
                HermesTopic::AudioServer(Some("default".into()), AudioServerCommand::ReplayRequest),
                "hermes/audioServer/default/replayRequest",
            ),
            (
                HermesTopic::AudioServer(Some("default".into()), AudioServerCommand::ReplayResponse),
                "hermes/audioServer/default/replayResponse",
            ),
            (
                HermesTopic::AudioServer(Some("default".into()), AudioServerCommand::PlayBytes("kikoo".into())),
                "hermes/audioServer/default/playBytes/kikoo",
            ),
            (
                HermesTopic::AudioServer(Some("default".into()), AudioServerCommand::PlayFinished),
                "hermes/audioServer/default/playFinished",
            ),
            (
                HermesTopic::Component(
                    Some("default".into()),
                    Component::AudioServer,
                    ComponentCommand::VersionRequest,
                ),
                "hermes/audioServer/default/versionRequest",
            ),
            (
                HermesTopic::Component(
                    Some("default".into()),
                    Component::AudioServer,
                    ComponentCommand::Version,
                ),
                "hermes/audioServer/default/version",
            ),
            (
                HermesTopic::Component(Some("default".into()), Component::AudioServer, ComponentCommand::Error),
                "hermes/audioServer/default/error",
            ),
            (HermesTopic::Tts(TtsCommand::Say), "hermes/tts/say"),
            (HermesTopic::Tts(TtsCommand::SayFinished), "hermes/tts/sayFinished"),
            (
                HermesTopic::Tts(TtsCommand::RegisterSound("foobar".into())),
                "hermes/tts/registerSound/foobar",
            ),
            (
                HermesTopic::Component(None, Component::Tts, ComponentCommand::VersionRequest),
                "hermes/tts/versionRequest",
            ),
            (
                HermesTopic::Component(None, Component::Tts, ComponentCommand::Version),
                "hermes/tts/version",
            ),
            (
                HermesTopic::Component(None, Component::Tts, ComponentCommand::Error),
                "hermes/tts/error",
            ),
            (
                HermesTopic::Intent("harakiri_intent".into()),
                "hermes/intent/harakiri_intent",
            ),
            (HermesTopic::Nlu(NluCommand::Query), "hermes/nlu/query"),
            (HermesTopic::Nlu(NluCommand::PartialQuery), "hermes/nlu/partialQuery"),
            (HermesTopic::Nlu(NluCommand::SlotParsed), "hermes/nlu/slotParsed"),
            (HermesTopic::Nlu(NluCommand::IntentParsed), "hermes/nlu/intentParsed"),
            (
                HermesTopic::Nlu(NluCommand::IntentNotRecognized),
                "hermes/nlu/intentNotRecognized",
            ),
            (HermesTopic::Nlu(NluCommand::Reload), "hermes/nlu/reload"),
            (
                HermesTopic::Component(None, Component::Nlu, ComponentCommand::VersionRequest),
                "hermes/nlu/versionRequest",
            ),
            (
                HermesTopic::Component(None, Component::Nlu, ComponentCommand::Version),
                "hermes/nlu/version",
            ),
            (
                HermesTopic::Component(None, Component::Nlu, ComponentCommand::Error),
                "hermes/nlu/error",
            ),
            (
                HermesTopic::Injection(InjectionCommand::Perform),
                "hermes/injection/perform",
            ),
            (
                HermesTopic::Injection(InjectionCommand::Status),
                "hermes/injection/status",
            ),
            (
                HermesTopic::Injection(InjectionCommand::StatusRequest),
                "hermes/injection/statusRequest",
            ),
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
            assert_eq!(
                HermesTopic::from_path(path),
                Some(expected_route),
                "failed parsing {}",
                path
            );
        }
    }
}
