extern crate base64;
extern crate chrono;
#[macro_use]
extern crate failure;
extern crate semver;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate snips_nlu_ontology;

pub mod errors;

mod ontology;

pub use errors::*;
pub use ontology::*;

/// A struct wrapping a callback with one argument, create one with the `new` method
pub struct Callback<T> {
    callback: Box<Fn(&T) -> () + Send + Sync>,
}

impl<T> Callback<T> {
    pub fn new<F: 'static>(handler: F) -> Callback<T>
    where
        F: Fn(&T) -> () + Send + Sync,
    {
        Callback {
            callback: Box::new(handler),
        }
    }

    pub fn call(&self, arg: &T) {
        (self.callback)(arg)
    }
}

/// A struct wrapping a callback with no argument, create one with the `new` method
pub struct Callback0 {
    callback: Box<Fn() -> () + Send + Sync>,
}

impl Callback0 {
    pub fn new<F: 'static>(handler: F) -> Callback0
    where
        F: Fn() -> () + Send + Sync,
    {
        Callback0 {
            callback: Box::new(handler),
        }
    }

    pub fn call(&self) {
        (self.callback)()
    }
}

/// A generic facade used to interact with a component
pub trait ComponentFacade: Send + Sync {
    fn publish_version_request(&self) -> Result<()>;
    fn subscribe_version(&self, handler: Callback<VersionMessage>) -> Result<()>;
    fn subscribe_error(&self, handler: Callback<ErrorMessage>) -> Result<()>;
}

/// A generic facade used to interact with a component
pub trait IdentifiableComponentFacade: Send + Sync {
    fn publish_version_request(&self, id: String) -> Result<()>;
    fn subscribe_version(&self, id: String, handler: Callback<VersionMessage>) -> Result<()>;
    fn subscribe_error(&self, id: String, handler: Callback<ErrorMessage>) -> Result<()>;
}

/// A generic facade all components must use to publish their errors and versions (when requested)
pub trait ComponentBackendFacade: Send + Sync {
    fn subscribe_version_request(&self, handler: Callback0) -> Result<()>;
    fn publish_version(&self, version: VersionMessage) -> Result<()>;
    fn publish_error(&self, error: ErrorMessage) -> Result<()>;
}

/// A generic facade all components must use to publish their errors and versions (when requested)
pub trait IdentifiableComponentBackendFacade: Send + Sync {
    fn subscribe_version_request(&self, id: String, handler: Callback0) -> Result<()>;
    fn publish_version(&self, id: String, version: VersionMessage) -> Result<()>;
    fn publish_error(&self, id: String, error: ErrorMessage) -> Result<()>;
}

/// A facade to interact with a component that can be toggled on an off at a specific site
pub trait ToggleableFacade: Send + Sync {
    fn publish_toggle_on(&self) -> Result<()>;
    fn publish_toggle_off(&self) -> Result<()>;
}

/// The facade a component that can be toggled on an off at a specific site must use to receive
/// its orders
pub trait ToggleableBackendFacade: Send + Sync {
    fn subscribe_toggle_on(&self, handler: Callback0) -> Result<()>;
    fn subscribe_toggle_off(&self, handler: Callback0) -> Result<()>;
}

/// A facade to interact with a component that can be toggled on an off at a specific site
pub trait IdentifiableToggleableFacade: Send + Sync {
    fn publish_toggle_on(&self, site: SiteMessage) -> Result<()>;
    fn publish_toggle_off(&self, site: SiteMessage) -> Result<()>;
}

/// The facade a component that can be toggled on an off at a specific site must use to receive
/// its orders
pub trait IdentifiableToggleableBackendFacade: Send + Sync {
    fn subscribe_toggle_on(&self, handler: Callback<SiteMessage>) -> Result<()>;
    fn subscribe_toggle_off(&self, handler: Callback<SiteMessage>) -> Result<()>;
}

//
// COMPONENTS
//

/// Facade used to interact with the voice activity component
pub trait VoiceActivityFacade: IdentifiableComponentFacade {
    fn subscribe_vad_up(&self, site_id: String, handler: Callback<VadUpMessage>) -> Result<()>;
    fn subscribe_vad_down(&self, site_id: String, handler: Callback<VadDownMessage>) -> Result<()>;
    fn subscribe_all_vad_up(&self, handler: Callback<VadUpMessage>) -> Result<()>;
    fn subscribe_all_vad_down(&self, handler: Callback<VadDownMessage>) -> Result<()>;
}

/// Facade the voice activity component must use to publish its results
pub trait VoiceActivityBackendFacade: IdentifiableComponentBackendFacade {
    fn publish_vad_up(&self, vad_up: VadUpMessage) -> Result<()>;
    fn publish_vad_down(&self, vad_down: VadDownMessage) -> Result<()>;
}

/// The facade to interact with the hotword component
pub trait HotwordFacade: IdentifiableComponentFacade + IdentifiableToggleableFacade {
    fn subscribe_detected(
        &self,
        site_id: String,
        handler: Callback<HotwordDetectedMessage>,
    ) -> Result<()>;
    fn subscribe_all_detected(&self, handler: Callback<HotwordDetectedMessage>) -> Result<()>;
}

/// The facade the hotword feature must use receive its orders and publish detected hotwords
pub trait HotwordBackendFacade:
    IdentifiableComponentBackendFacade + IdentifiableToggleableBackendFacade
{
    fn publish_detected(&self, site_id: String, message: HotwordDetectedMessage) -> Result<()>;
}

/// The facade used to toggle on and of the sound feedback at a specific site
pub trait SoundFeedbackFacade: IdentifiableToggleableFacade {}

/// The facade a component that manages sound feedback must use to receive its orders
pub trait SoundFeedbackBackendFacade: IdentifiableToggleableBackendFacade {}

/// The facade to interact with the automatic speech recognition component
pub trait AsrFacade: ComponentFacade + ToggleableFacade {
    fn publish_start_listening(&self, start: AsrStartListeningMessage) -> Result<()>;
    fn publish_stop_listening(&self, site: SiteMessage) -> Result<()>;
    fn publish_reload(&self) -> Result<()>;
    fn subscribe_text_captured(&self, handler: Callback<TextCapturedMessage>) -> Result<()>;
    fn subscribe_partial_text_captured(&self, handler: Callback<TextCapturedMessage>) -> Result<()>;
}

/// The facade the automatic speech recognition must use to receive its orders and publish
/// recognized text
pub trait AsrBackendFacade: ComponentBackendFacade + ToggleableBackendFacade {
    fn subscribe_start_listening(&self, handler: Callback<AsrStartListeningMessage>) -> Result<()>;
    fn subscribe_stop_listening(&self, handler: Callback<SiteMessage>) -> Result<()>;
    fn subscribe_reload(&self, handler: Callback0) -> Result<()>;
    fn publish_text_captured(&self, text_captured: TextCapturedMessage) -> Result<()>;
    fn publish_partial_text_captured(&self, text_captured: TextCapturedMessage) -> Result<()>;
}

/// The facade to interact with the text to speech component
pub trait TtsFacade: ComponentFacade {
    fn publish_say(&self, to_say: SayMessage) -> Result<()>;
    fn subscribe_say_finished(&self, handler: Callback<SayFinishedMessage>) -> Result<()>;
}

/// The facade the text to speech must use to receive its orders and advertise when it has finished
pub trait TtsBackendFacade: ComponentBackendFacade {
    fn publish_say_finished(&self, status: SayFinishedMessage) -> Result<()>;
    fn subscribe_say(&self, handler: Callback<SayMessage>) -> Result<()>;
}

/// The facade to interact with the natural language understanding component
pub trait NluFacade: ComponentFacade {
    fn publish_query(&self, query: NluQueryMessage) -> Result<()>;
    fn publish_partial_query(&self, query: NluSlotQueryMessage) -> Result<()>;
    fn publish_reload(&self) -> Result<()>;
    fn subscribe_slot_parsed(&self, handler: Callback<NluSlotMessage>) -> Result<()>;
    fn subscribe_intent_parsed(&self, handler: Callback<NluIntentMessage>) -> Result<()>;
    fn subscribe_intent_not_recognized(
        &self,
        handler: Callback<NluIntentNotRecognizedMessage>,
    ) -> Result<()>;
}

/// The facade the natural language understanding must use to receive its orders and publish
/// its results
pub trait NluBackendFacade: ComponentBackendFacade {
    fn subscribe_query(&self, handler: Callback<NluQueryMessage>) -> Result<()>;
    fn subscribe_partial_query(&self, handler: Callback<NluSlotQueryMessage>) -> Result<()>;
    fn subscribe_reload(&self, handler: Callback0) -> Result<()>;
    fn publish_slot_parsed(&self, slot: NluSlotMessage) -> Result<()>;
    fn publish_intent_parsed(&self, intent: NluIntentMessage) -> Result<()>;
    fn publish_intent_not_recognized(&self, status: NluIntentNotRecognizedMessage) -> Result<()>;
}

/// The facade to interact with the audio server
pub trait AudioServerFacade: IdentifiableComponentFacade + IdentifiableToggleableFacade {
    fn publish_play_bytes(&self, bytes: PlayBytesMessage) -> Result<()>;
    fn subscribe_play_finished(
        &self,
        site_id: SiteId,
        handler: Callback<PlayFinishedMessage>,
    ) -> Result<()>;
    fn subscribe_all_play_finished(&self, handler: Callback<PlayFinishedMessage>) -> Result<()>;
    fn subscribe_audio_frame(
        &self,
        site_id: SiteId,
        handler: Callback<AudioFrameMessage>,
    ) -> Result<()>;
    fn publish_replay_request(&self, request: ReplayRequestMessage) -> Result<()>;
    fn subscribe_replay_response(&self, site_id: SiteId, handler:Callback<AudioFrameMessage>) -> Result<()>;
}

/// The facade the audio server must use to receive its orders and advertise when it has finished
pub trait AudioServerBackendFacade:
    IdentifiableComponentBackendFacade + IdentifiableToggleableBackendFacade
{
    fn subscribe_play_bytes(
        &self,
        site_id: SiteId,
        handler: Callback<PlayBytesMessage>,
    ) -> Result<()>;
    fn subscribe_all_play_bytes(&self, handler: Callback<PlayBytesMessage>) -> Result<()>;
    fn publish_play_finished(&self, status: PlayFinishedMessage) -> Result<()>;
    fn publish_audio_frame(&self, frame: AudioFrameMessage) -> Result<()>;
    fn subscribe_replay_request(&self, site_id: SiteId, handler: Callback<ReplayRequestMessage>) -> Result<()>;
    fn publish_replay_response(&self, frame: AudioFrameMessage) -> Result<()>;
}

/// The facade to use to interact with the dialogue manager, this is the principal interface that a
/// lambda should use
pub trait DialogueFacade: ComponentFacade + ToggleableFacade {
    fn subscribe_session_queued(&self, handler: Callback<SessionQueuedMessage>) -> Result<()>;
    fn subscribe_session_started(&self, handler: Callback<SessionStartedMessage>) -> Result<()>;
    fn subscribe_intent(&self, intent_name: String, handler: Callback<IntentMessage>)
        -> Result<()>;
    fn subscribe_intents(&self, handler: Callback<IntentMessage>) -> Result<()>;
    fn subscribe_intent_not_recognized(&self, handler: Callback<IntentNotRecognizedMessage>) -> Result<()>;
    fn subscribe_session_ended(&self, handler: Callback<SessionEndedMessage>) -> Result<()>;
    fn publish_start_session(&self, start_session: StartSessionMessage) -> Result<()>;
    fn publish_continue_session(&self, continue_session: ContinueSessionMessage) -> Result<()>;
    fn publish_end_session(&self, end_session: EndSessionMessage) -> Result<()>;
}

/// The facade the dialogue manager must use to interact with the lambdas
pub trait DialogueBackendFacade: ComponentBackendFacade + ToggleableBackendFacade {
    fn publish_session_queued(&self, status: SessionQueuedMessage) -> Result<()>;
    fn publish_session_started(&self, status: SessionStartedMessage) -> Result<()>;
    fn publish_intent(&self, intent: IntentMessage) -> Result<()>;
    fn publish_intent_not_recognized(&self, intent_not_recognized: IntentNotRecognizedMessage) -> Result<()>;
    fn publish_session_ended(&self, status: SessionEndedMessage) -> Result<()>;
    fn subscribe_start_session(&self, handler: Callback<StartSessionMessage>) -> Result<()>;
    fn subscribe_continue_session(&self, handler: Callback<ContinueSessionMessage>) -> Result<()>;
    fn subscribe_end_session(&self, handler: Callback<EndSessionMessage>) -> Result<()>;
}

/// The facade to interact with the injection component
pub trait InjectionFacade: ComponentFacade {
    fn publish_injection_request(&self, request: InjectionRequestMessage) -> Result<()>;
    fn publish_injection_status_request(&self) -> Result<()>;
    fn subscribe_injection_status(&self, handler: Callback<InjectionStatusMessage>) -> Result<()>;
}

/// The facade the injecter must use to receive its orders and advertise when it has finished
pub trait InjectionBackendFacade: ComponentBackendFacade {
    fn subscribe_injection_request(&self, handler: Callback<InjectionRequestMessage>) -> Result<()>;
    fn subscribe_injection_status_request(&self, handler: Callback0) -> Result<()>;
    fn publish_injection_status(&self, status: InjectionStatusMessage) -> Result<()>;
}

pub trait HermesProtocolHandler: Send + Sync + std::fmt::Display {
    fn voice_activity(&self) -> Box<VoiceActivityFacade>;
    fn hotword(&self) -> Box<HotwordFacade>;
    fn sound_feedback(&self) -> Box<SoundFeedbackFacade>;
    fn asr(&self) -> Box<AsrFacade>;
    fn tts(&self) -> Box<TtsFacade>;
    fn nlu(&self) -> Box<NluFacade>;
    fn audio_server(&self) -> Box<AudioServerFacade>;
    fn dialogue(&self) -> Box<DialogueFacade>;
    fn injection(&self) -> Box<InjectionFacade>;
    fn voice_activity_backend(&self) -> Box<VoiceActivityBackendFacade>;
    fn hotword_backend(&self) -> Box<HotwordBackendFacade>;
    fn sound_feedback_backend(&self) -> Box<SoundFeedbackBackendFacade>;
    fn asr_backend(&self) -> Box<AsrBackendFacade>;
    fn tts_backend(&self) -> Box<TtsBackendFacade>;
    fn nlu_backend(&self) -> Box<NluBackendFacade>;
    fn audio_server_backend(&self) -> Box<AudioServerBackendFacade>;
    fn dialogue_backend(&self) -> Box<DialogueBackendFacade>;
    fn injection_backend(&self) -> Box<InjectionBackendFacade>;
}
