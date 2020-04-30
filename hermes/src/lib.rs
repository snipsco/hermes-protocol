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

#[macro_use]
pub extern crate hermes_utils;

pub mod errors;
pub mod ontology;

pub use crate::errors::*;
pub use crate::ontology::*;

use failure::Fallible;

/// A struct wrapping a callback with one argument, create one with the `new` method
pub struct Callback<T> {
    callback: Box<dyn Fn(&T) -> () + Send + Sync>,
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
    callback: Box<dyn Fn() -> () + Send + Sync>,
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
    fn publish_version_request(&self) -> Fallible<()>;
    fn subscribe_version(&self, handler: Callback<VersionMessage>) -> Fallible<()>;
    fn subscribe_error(&self, handler: Callback<ErrorMessage>) -> Fallible<()>;
    fn subscribe_component_loaded(&self, handler: Callback<ComponentLoadedMessage>) -> Fallible<()>;
}

/// A generic facade used to interact with a component
pub trait IdentifiableComponentFacade: Send + Sync {
    fn publish_version_request(&self, id: String) -> Fallible<()>;
    fn subscribe_version(&self, id: String, handler: Callback<VersionMessage>) -> Fallible<()>;
    fn subscribe_error(&self, id: String, handler: Callback<SiteErrorMessage>) -> Fallible<()>;
    fn subscribe_all_error(&self, handler: Callback<SiteErrorMessage>) -> Fallible<()>;
    fn subscribe_component_loaded(&self, id: String, handler: Callback<ComponentLoadedOnSiteMessage>) -> Fallible<()>;
    fn subscribe_all_component_loaded(&self, handler: Callback<ComponentLoadedOnSiteMessage>) -> Fallible<()>;
}

/// A generic facade all components must use to publish their errors and versions (when requested)
pub trait ComponentBackendFacade: Send + Sync {
    fn subscribe_version_request(&self, handler: Callback0) -> Fallible<()>;
    fn publish_version(&self, version: VersionMessage) -> Fallible<()>;
    fn publish_error(&self, error: ErrorMessage) -> Fallible<()>;
    fn publish_component_loaded(&self, component_loaded: ComponentLoadedMessage) -> Fallible<()>;
}

/// A generic facade all components must use to publish their errors and versions (when requested)
pub trait IdentifiableComponentBackendFacade: Send + Sync {
    fn subscribe_version_request(&self, id: String, handler: Callback0) -> Fallible<()>;
    fn publish_version(&self, id: String, version: VersionMessage) -> Fallible<()>;
    fn publish_error(&self, id: String, error: SiteErrorMessage) -> Fallible<()>;
    fn publish_component_loaded(&self, id: String, component_loaded: ComponentLoadedOnSiteMessage) -> Fallible<()>;
}

/// A facade to interact with a component that can be toggled on an off at a specific site
pub trait ToggleableFacade: Send + Sync {
    fn publish_toggle_on(&self) -> Fallible<()>;
    fn publish_toggle_off(&self) -> Fallible<()>;
}

/// The facade a component that can be toggled on an off at a specific site must use to receive
/// its orders
pub trait ToggleableBackendFacade: Send + Sync {
    fn subscribe_toggle_on(&self, handler: Callback0) -> Fallible<()>;
    fn subscribe_toggle_off(&self, handler: Callback0) -> Fallible<()>;
}

/// A facade to interact with a component that can be toggled on an off at a specific site
pub trait IdentifiableToggleableFacade: Send + Sync {
    fn publish_toggle_on(&self, site: SiteMessage) -> Fallible<()>;
    fn publish_toggle_off(&self, site: SiteMessage) -> Fallible<()>;
}

/// The facade a component that can be toggled on an off at a specific site must use to receive
/// its orders
pub trait IdentifiableToggleableBackendFacade: Send + Sync {
    fn subscribe_toggle_on(&self, handler: Callback<SiteMessage>) -> Fallible<()>;
    fn subscribe_toggle_off(&self, handler: Callback<SiteMessage>) -> Fallible<()>;
}

//
// COMPONENTS
//

/// Facade used to interact with the voice activity component
pub trait VoiceActivityFacade: IdentifiableComponentFacade {
    fn subscribe_vad_up(&self, site_id: String, handler: Callback<VadUpMessage>) -> Fallible<()>;
    fn subscribe_vad_down(&self, site_id: String, handler: Callback<VadDownMessage>) -> Fallible<()>;
    fn subscribe_all_vad_up(&self, handler: Callback<VadUpMessage>) -> Fallible<()>;
    fn subscribe_all_vad_down(&self, handler: Callback<VadDownMessage>) -> Fallible<()>;
}

/// Facade the voice activity component must use to publish its results
pub trait VoiceActivityBackendFacade: IdentifiableComponentBackendFacade {
    fn publish_vad_up(&self, vad_up: VadUpMessage) -> Fallible<()>;
    fn publish_vad_down(&self, vad_down: VadDownMessage) -> Fallible<()>;
}

/// The facade to interact with the hotword component
pub trait HotwordFacade: IdentifiableComponentFacade + IdentifiableToggleableFacade {
    fn subscribe_detected(&self, site_id: String, handler: Callback<HotwordDetectedMessage>) -> Fallible<()>;
    fn subscribe_all_detected(&self, handler: Callback<HotwordDetectedMessage>) -> Fallible<()>;
}

/// The facade the hotword feature must use receive its orders and publish detected hotwords
pub trait HotwordBackendFacade: IdentifiableComponentBackendFacade + IdentifiableToggleableBackendFacade {
    fn publish_detected(&self, site_id: String, message: HotwordDetectedMessage) -> Fallible<()>;
}

/// The facade used to toggle on and of the sound feedback at a specific site
pub trait SoundFeedbackFacade: IdentifiableToggleableFacade {}

/// The facade a component that manages sound feedback must use to receive its orders
pub trait SoundFeedbackBackendFacade: IdentifiableToggleableBackendFacade {}

/// The facade to interact with the automatic speech recognition component
pub trait AsrFacade: ComponentFacade + ToggleableFacade {
    fn publish_start_listening(&self, start: AsrStartListeningMessage) -> Fallible<()>;
    fn publish_stop_listening(&self, site: SiteMessage) -> Fallible<()>;
    fn publish_component_reload(&self, component_reload: RequestComponentReloadMessage) -> Fallible<()>;
    fn subscribe_text_captured(&self, handler: Callback<TextCapturedMessage>) -> Fallible<()>;
    fn subscribe_partial_text_captured(&self, handler: Callback<TextCapturedMessage>) -> Fallible<()>;
}

/// The facade the automatic speech recognition must use to receive its orders and publish
/// recognized text
pub trait AsrBackendFacade: ComponentBackendFacade + ToggleableBackendFacade {
    fn subscribe_start_listening(&self, handler: Callback<AsrStartListeningMessage>) -> Fallible<()>;
    fn subscribe_stop_listening(&self, handler: Callback<SiteMessage>) -> Fallible<()>;
    fn subscribe_component_reload(&self, handler: Callback<RequestComponentReloadMessage>) -> Fallible<()>;
    fn publish_text_captured(&self, text_captured: TextCapturedMessage) -> Fallible<()>;
    fn publish_partial_text_captured(&self, text_captured: TextCapturedMessage) -> Fallible<()>;
}

/// The facade to interact with the text to speech component
pub trait TtsFacade: ComponentFacade {
    fn publish_say(&self, to_say: SayMessage) -> Fallible<()>;
    fn subscribe_say_finished(&self, handler: Callback<SayFinishedMessage>) -> Fallible<()>;
    fn publish_register_sound(&self, sound: RegisterSoundMessage) -> Fallible<()>;
}

/// The facade the text to speech must use to receive its orders and advertise when it has finished
pub trait TtsBackendFacade: ComponentBackendFacade {
    fn publish_say_finished(&self, status: SayFinishedMessage) -> Fallible<()>;
    fn subscribe_say(&self, handler: Callback<SayMessage>) -> Fallible<()>;
    fn subscribe_register_sound(&self, handler: Callback<RegisterSoundMessage>) -> Fallible<()>;
}

/// The facade to interact with the natural language understanding component
pub trait NluFacade: ComponentFacade {
    fn publish_query(&self, query: NluQueryMessage) -> Fallible<()>;
    fn publish_partial_query(&self, query: NluSlotQueryMessage) -> Fallible<()>;
    fn publish_component_reload(&self, component_reload: RequestComponentReloadMessage) -> Fallible<()>;
    fn subscribe_slot_parsed(&self, handler: Callback<NluSlotMessage>) -> Fallible<()>;
    fn subscribe_intent_parsed(&self, handler: Callback<NluIntentMessage>) -> Fallible<()>;
    fn subscribe_intent_not_recognized(&self, handler: Callback<NluIntentNotRecognizedMessage>) -> Fallible<()>;
}

/// The facade the natural language understanding must use to receive its orders and publish
/// its results
pub trait NluBackendFacade: ComponentBackendFacade {
    fn subscribe_query(&self, handler: Callback<NluQueryMessage>) -> Fallible<()>;
    fn subscribe_partial_query(&self, handler: Callback<NluSlotQueryMessage>) -> Fallible<()>;
    fn subscribe_component_reload(&self, handler: Callback<RequestComponentReloadMessage>) -> Fallible<()>;
    fn publish_slot_parsed(&self, slot: NluSlotMessage) -> Fallible<()>;
    fn publish_intent_parsed(&self, intent: NluIntentMessage) -> Fallible<()>;
    fn publish_intent_not_recognized(&self, status: NluIntentNotRecognizedMessage) -> Fallible<()>;
}

/// The facade to interact with the audio server
pub trait AudioServerFacade: IdentifiableComponentFacade + IdentifiableToggleableFacade {
    fn publish_play_bytes(&self, bytes: PlayBytesMessage) -> Fallible<()>;
    fn subscribe_play_finished(&self, site_id: String, handler: Callback<PlayFinishedMessage>) -> Fallible<()>;
    fn subscribe_all_play_finished(&self, handler: Callback<PlayFinishedMessage>) -> Fallible<()>;
    fn subscribe_audio_frame(&self, site_id: String, handler: Callback<AudioFrameMessage>) -> Fallible<()>;
    fn publish_replay_request(&self, request: ReplayRequestMessage) -> Fallible<()>;
    fn subscribe_replay_response(&self, site_id: String, handler: Callback<AudioFrameMessage>) -> Fallible<()>;
    fn publish_stream_bytes(&self, play_bytes_streaming_message: StreamBytesMessage) -> Fallible<()>;
    fn subscribe_stream_finished(&self, site_id: String, handler: Callback<StreamFinishedMessage>) -> Fallible<()>;
    fn subscribe_all_stream_finished(&self, handler: Callback<StreamFinishedMessage>) -> Fallible<()>;
}

/// The facade the audio server must use to receive its orders and advertise when it has finished
pub trait AudioServerBackendFacade: IdentifiableComponentBackendFacade + IdentifiableToggleableBackendFacade {
    fn subscribe_play_bytes(&self, site_id: String, handler: Callback<PlayBytesMessage>) -> Fallible<()>;
    fn subscribe_all_play_bytes(&self, handler: Callback<PlayBytesMessage>) -> Fallible<()>;
    fn publish_play_finished(&self, status: PlayFinishedMessage) -> Fallible<()>;
    fn publish_audio_frame(&self, frame: AudioFrameMessage) -> Fallible<()>;
    fn subscribe_replay_request(&self, site_id: String, handler: Callback<ReplayRequestMessage>) -> Fallible<()>;
    fn publish_replay_response(&self, frame: AudioFrameMessage) -> Fallible<()>;
    fn subscribe_stream_bytes(&self, site_id: String, handler: Callback<StreamBytesMessage>) -> Fallible<()>;
    fn subscribe_all_stream_bytes(&self, handler: Callback<StreamBytesMessage>) -> Fallible<()>;
    fn publish_stream_finished(&self, status: StreamFinishedMessage) -> Fallible<()>;
}

/// The facade to use to interact with the dialogue manager, this is the principal interface that a
/// lambda should use
pub trait DialogueFacade: ComponentFacade + ToggleableFacade {
    fn subscribe_session_queued(&self, handler: Callback<SessionQueuedMessage>) -> Fallible<()>;
    fn subscribe_session_started(&self, handler: Callback<SessionStartedMessage>) -> Fallible<()>;
    fn subscribe_intent(&self, intent_name: String, handler: Callback<IntentMessage>) -> Fallible<()>;
    fn subscribe_intents(&self, handler: Callback<IntentMessage>) -> Fallible<()>;
    fn subscribe_intent_not_recognized(&self, handler: Callback<IntentNotRecognizedMessage>) -> Fallible<()>;
    fn subscribe_session_ended(&self, handler: Callback<SessionEndedMessage>) -> Fallible<()>;
    fn publish_start_session(&self, start_session: StartSessionMessage) -> Fallible<()>;
    fn publish_continue_session(&self, continue_session: ContinueSessionMessage) -> Fallible<()>;
    fn publish_end_session(&self, end_session: EndSessionMessage) -> Fallible<()>;
    fn publish_configure(&self, config: DialogueConfigureMessage) -> Fallible<()>;
}

/// The facade the dialogue manager must use to interact with the lambdas
pub trait DialogueBackendFacade: ComponentBackendFacade + ToggleableBackendFacade {
    fn publish_session_queued(&self, status: SessionQueuedMessage) -> Fallible<()>;
    fn publish_session_started(&self, status: SessionStartedMessage) -> Fallible<()>;
    fn publish_intent(&self, intent: IntentMessage) -> Fallible<()>;
    fn publish_intent_not_recognized(&self, intent_not_recognized: IntentNotRecognizedMessage) -> Fallible<()>;
    fn publish_session_ended(&self, status: SessionEndedMessage) -> Fallible<()>;
    fn subscribe_start_session(&self, handler: Callback<StartSessionMessage>) -> Fallible<()>;
    fn subscribe_continue_session(&self, handler: Callback<ContinueSessionMessage>) -> Fallible<()>;
    fn subscribe_end_session(&self, handler: Callback<EndSessionMessage>) -> Fallible<()>;
    fn subscribe_configure(&self, handler: Callback<DialogueConfigureMessage>) -> Fallible<()>;
}

/// The facade to interact with the injection component
pub trait InjectionFacade: ComponentFacade {
    fn publish_injection_request(&self, request: InjectionRequestMessage) -> Fallible<()>;
    fn publish_injection_status_request(&self) -> Fallible<()>;
    fn publish_injection_reset_request(&self, request: InjectionResetRequestMessage) -> Fallible<()>;
    fn subscribe_injection_status(&self, handler: Callback<InjectionStatusMessage>) -> Fallible<()>;
    fn subscribe_injection_complete(&self, handler: Callback<InjectionCompleteMessage>) -> Fallible<()>;
    fn subscribe_injection_reset_complete(&self, handler: Callback<InjectionResetCompleteMessage>) -> Fallible<()>;
    fn subscribe_injection_failed(&self, handler: Callback<InjectionFailedMessage>) -> Fallible<()>;
    fn subscribe_injection_reset_failed(&self, handler: Callback<InjectionResetFailedMessage>) -> Fallible<()>;
}

/// The facade the injecter must use to receive its orders and advertise when it has finished
pub trait InjectionBackendFacade: ComponentBackendFacade {
    fn subscribe_injection_request(&self, handler: Callback<InjectionRequestMessage>) -> Fallible<()>;
    fn subscribe_injection_status_request(&self, handler: Callback0) -> Fallible<()>;
    fn subscribe_injection_reset_request(&self, handler: Callback<InjectionResetRequestMessage>) -> Fallible<()>;
    fn publish_injection_status(&self, status: InjectionStatusMessage) -> Fallible<()>;
    fn publish_injection_complete(&self, message: InjectionCompleteMessage) -> Fallible<()>;
    fn publish_injection_reset_complete(&self, message: InjectionResetCompleteMessage) -> Fallible<()>;
    fn publish_injection_failed(&self, message: InjectionFailedMessage) -> Fallible<()>;
    fn publish_injection_reset_failed(&self, message: InjectionResetFailedMessage) -> Fallible<()>;
}

pub trait HermesProtocolHandler: Send + Sync + std::fmt::Display {
    fn voice_activity(&self) -> Box<dyn VoiceActivityFacade>;
    fn hotword(&self) -> Box<dyn HotwordFacade>;
    fn sound_feedback(&self) -> Box<dyn SoundFeedbackFacade>;
    fn asr(&self) -> Box<dyn AsrFacade>;
    fn tts(&self) -> Box<dyn TtsFacade>;
    fn nlu(&self) -> Box<dyn NluFacade>;
    fn audio_server(&self) -> Box<dyn AudioServerFacade>;
    fn dialogue(&self) -> Box<dyn DialogueFacade>;
    fn injection(&self) -> Box<dyn InjectionFacade>;
    fn voice_activity_backend(&self) -> Box<dyn VoiceActivityBackendFacade>;
    fn hotword_backend(&self) -> Box<dyn HotwordBackendFacade>;
    fn sound_feedback_backend(&self) -> Box<dyn SoundFeedbackBackendFacade>;
    fn asr_backend(&self) -> Box<dyn AsrBackendFacade>;
    fn tts_backend(&self) -> Box<dyn TtsBackendFacade>;
    fn nlu_backend(&self) -> Box<dyn NluBackendFacade>;
    fn audio_server_backend(&self) -> Box<dyn AudioServerBackendFacade>;
    fn dialogue_backend(&self) -> Box<dyn DialogueBackendFacade>;
    fn injection_backend(&self) -> Box<dyn InjectionBackendFacade>;
}
