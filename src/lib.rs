extern crate base64;
#[macro_use]
extern crate error_chain;
extern crate nlu_rust_ontology;
#[cfg(feature = "mqtt")]
#[macro_use]
extern crate log;
#[cfg(feature = "mqtt")]
extern crate rumqtt;
extern crate semver;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[cfg(feature = "mqtt")]
extern crate strum;
#[cfg(feature = "mqtt")]
#[macro_use]
extern crate strum_macros;

mod errors;

#[cfg(feature = "mqtt")]
mod mqtt;

pub use errors::{Error, ErrorKind, Result};
#[cfg(feature = "mqtt")]
pub use mqtt::MqttHermesProtocolHandler;
pub use nlu_rust_ontology::*;

pub struct Callback<T> {
    callback: Box<Fn(&T) -> () + Send + Sync>
}

impl<T> Callback<T> {
    pub fn new<F: 'static>(handler: F) -> Callback<T> where F: Fn(&T) -> () + Send + Sync {
        Callback { callback: Box::new(handler) }
    }

    pub fn call(&self, arg: &T) { (self.callback)(arg) }
}

pub struct Callback0 {
     callback: Box<Fn() -> () + Send + Sync>
}

impl Callback0 {
    pub fn new<F: 'static>(handler: F) -> Callback0 where F: Fn() -> () + Send + Sync {
        Callback0 { callback: Box::new(handler) }
    }

    pub fn call(&self) { (self.callback)() }
}

pub trait ToggleableFacade : Send + Sync {
    fn publish_toggle_on(&self) -> Result<()>;
    fn publish_toggle_off(&self) -> Result<()>;
}

pub trait ToggleableBackendFacade : Send + Sync {
    fn subscribe_toggle_on(&self, handler: Callback0) -> Result<()>;
    fn subscribe_toggle_off(&self, handler: Callback0) -> Result<()>;
}

pub trait HotwordFacade: ComponentFacade + ToggleableFacade {
    fn publish_wait(&self) -> Result<()>;
    fn subscribe_detected(&self, handler: Callback0) -> Result<()>;
}

pub trait HotwordBackendFacade: ComponentBackendFacade + ToggleableBackendFacade {
    fn publish_detected(&self) -> Result<()>;
    fn subscribe_wait(&self, handler: Callback0) -> Result<()>;
}

pub trait SoundFeedbackFacade: ToggleableFacade {}

pub trait SoundFeedbackBackendFacade: ToggleableBackendFacade {}

pub trait AsrFacade: ComponentFacade + ToggleableFacade {
    fn subscribe_text_captured(&self, handler: Callback<TextCapturedMessage>) -> Result<()>;
    fn subscribe_partial_text_captured(&self, handler: Callback<TextCapturedMessage>) -> Result<()>;
}

pub trait AsrBackendFacade: ComponentBackendFacade + ToggleableBackendFacade {
    fn publish_text_captured(&self, text_captured: TextCapturedMessage) -> Result<()>;
    fn publish_partial_text_captured(&self, text_captured: TextCapturedMessage) -> Result<()>;
}

pub trait TtsFacade: ComponentFacade {
    fn publish_say(&self, to_say: SayMessage) -> Result<()>;
    fn subscribe_say_finished(&self, handler: Callback0) -> Result<()>;
}

pub trait TtsBackendFacade: ComponentBackendFacade {
    fn publish_say_finished(&self) -> Result<()>;
    fn subscribe_say(&self, handler: Callback<SayMessage>) -> Result<()>;
}

pub trait NluFacade: ComponentFacade {
    fn publish_query(&self, query: NluQueryMessage) -> Result<()>;
    fn publish_partial_query(&self, query: NluSlotQueryMessage) -> Result<()>;
    fn subscribe_slot_parsed(&self, handler: Callback<SlotMessage>) -> Result<()>;
    fn subscribe_intent_parsed(&self, handler: Callback<IntentMessage>) -> Result<()>;
    fn subscribe_intent_not_recognized(&self, handler: Callback<IntentNotRecognizedMessage>) -> Result<()>;
}

pub trait NluBackendFacade: ComponentBackendFacade {
    fn subscribe_query(&self, handler: Callback<NluQueryMessage>) -> Result<()>;
    fn subscribe_partial_query(&self, handler: Callback<NluSlotQueryMessage>) -> Result<()>;
    fn publish_slot_parsed(&self, slot: SlotMessage) -> Result<()>;
    fn publish_intent_parsed(&self, intent: IntentMessage) -> Result<()>;
    fn publish_intent_not_recognized(&self, status: IntentNotRecognizedMessage) -> Result<()>;
}

pub trait AudioServerFacade: ComponentFacade {
    fn publish_play_file(&self, file: PlayFileMessage) -> Result<()>;
    fn publish_play_bytes(&self, bytes: PlayBytesMessage) -> Result<()>;
    fn subscribe_play_finished(&self, handler: Callback<PlayFinishedMessage>) -> Result<()>;
}

pub trait AudioServerBackendFacade: ComponentBackendFacade {
    fn subscribe_play_bytes(&self, handler: Callback<PlayBytesMessage>) -> Result<()>;
    fn subscribe_play_file(&self, handler: Callback<PlayFileMessage>) -> Result<()>;
    fn publish_play_finished(&self, status: PlayFinishedMessage) -> Result<()>;
}

pub trait ComponentFacade : Send + Sync {
    fn publish_version_request(&self) -> Result<()>;
    fn subscribe_version(&self, handler: Callback<VersionMessage>) -> Result<()>;
    fn subscribe_error(&self, handler: Callback<ErrorMessage>) -> Result<()>;
}

pub trait ComponentBackendFacade : Send + Sync{
    fn subscribe_version_request(&self, handler: Callback0) -> Result<()>;
    fn publish_version(&self, version: VersionMessage) -> Result<()>;
    fn publish_error(&self, error: ErrorMessage) -> Result<()>;
}

pub trait IntentFacade : Send + Sync {
    fn subscribe_intent(&self, intent_name: String, handler: Callback<IntentMessage>) -> Result<()>;
}

pub trait IntentBackendFacade : Send + Sync {
    fn publish_intent(&self, intent: IntentMessage) -> Result<()>;
}

pub trait HermesProtocolHandler : Send + Sync{
    fn hotword(&self) -> Box<HotwordFacade>;
    fn sound_feedback(&self) -> Box<SoundFeedbackFacade>;
    fn asr(&self) -> Box<AsrFacade>;
    fn tts(&self) -> Box<TtsFacade>;
    fn nlu(&self) -> Box<NluFacade>;
    fn audio_server(&self) -> Box<AudioServerFacade>;
    fn hotword_backend(&self) -> Box<HotwordBackendFacade>;
    fn sound_feedback_backend(&self) -> Box<SoundFeedbackBackendFacade>;
    fn asr_backend(&self) -> Box<AsrBackendFacade>;
    fn tts_backend(&self) -> Box<TtsBackendFacade>;
    fn nlu_backend(&self) -> Box<NluBackendFacade>;
    fn audio_server_backend(&self) -> Box<AudioServerBackendFacade>;
    fn intent(&self) -> Box<IntentFacade>;
    fn intent_backend(&self) -> Box<IntentBackendFacade>;
}

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
    #[serde(rename = "wavBytes", serialize_with = "as_base64", deserialize_with = "from_base64")]
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
    pub version: semver::Version,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ErrorMessage {
    pub error: String,
    pub context: Option<String>,
}

fn as_base64<S>(bytes: &[u8], serializer: S) -> std::result::Result<S::Ok, S::Error>
    where S: serde::Serializer {
    serializer.serialize_str(&base64::encode(bytes))
}

fn from_base64<'de, D>(deserializer: D) -> std::result::Result<Vec<u8>, D::Error>
    where D: serde::Deserializer<'de> {
    use serde::de::Error;
    use serde::Deserialize;
    String::deserialize(deserializer)
        .and_then(|string| base64::decode(&string).map_err(|err| Error::custom(err.to_string())))
}
