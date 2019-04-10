use std::fmt::Debug;
use std::sync::Mutex;

use failure::Fallible;
use hermes::*;
use log::*;

pub struct InProcessHermesProtocolHandler {
    bus: Mutex<ripb::Bus>,
}

impl InProcessHermesProtocolHandler {
    pub fn new() -> Self {
        Self {
            bus: Mutex::new(ripb::Bus::new()),
        }
    }

    fn get_handler<T: Send + Sync + Debug>(&self, component: T) -> Box<InProcessComponent<T>> {
        let bus = self.bus.lock().unwrap().clone();
        Box::new(InProcessComponent {
            component,
            bus: Mutex::new(bus),
            subscriber: Mutex::new(None),
        })
    }
}

impl Default for InProcessHermesProtocolHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl HermesProtocolHandler for InProcessHermesProtocolHandler {
    fn voice_activity(&self) -> Box<VoiceActivityFacade> {
        self.get_handler(VoiceActivity)
    }

    fn hotword(&self) -> Box<HotwordFacade> {
        self.get_handler(Hotword)
    }

    fn sound_feedback(&self) -> Box<SoundFeedbackFacade> {
        self.get_handler(Sound)
    }

    fn asr(&self) -> Box<AsrFacade> {
        self.get_handler(Asr)
    }

    fn tts(&self) -> Box<TtsFacade> {
        self.get_handler(Tts)
    }

    fn nlu(&self) -> Box<NluFacade> {
        self.get_handler(Nlu)
    }

    fn audio_server(&self) -> Box<AudioServerFacade> {
        self.get_handler(AudioServer)
    }

    fn dialogue(&self) -> Box<DialogueFacade> {
        self.get_handler(Dialogue)
    }

    fn injection(&self) -> Box<InjectionFacade> {
        self.get_handler(Injection)
    }

    fn voice_activity_backend(&self) -> Box<VoiceActivityBackendFacade> {
        self.get_handler(VoiceActivity)
    }

    fn hotword_backend(&self) -> Box<HotwordBackendFacade> {
        self.get_handler(Hotword)
    }

    fn sound_feedback_backend(&self) -> Box<SoundFeedbackBackendFacade> {
        self.get_handler(Sound)
    }

    fn asr_backend(&self) -> Box<AsrBackendFacade> {
        self.get_handler(Asr)
    }

    fn tts_backend(&self) -> Box<TtsBackendFacade> {
        self.get_handler(Tts)
    }

    fn nlu_backend(&self) -> Box<NluBackendFacade> {
        self.get_handler(Nlu)
    }

    fn audio_server_backend(&self) -> Box<AudioServerBackendFacade> {
        self.get_handler(AudioServer)
    }

    fn dialogue_backend(&self) -> Box<DialogueBackendFacade> {
        self.get_handler(Dialogue)
    }

    fn injection_backend(&self) -> Box<InjectionBackendFacade> {
        self.get_handler(Injection)
    }
}

impl std::fmt::Display for InProcessHermesProtocolHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Snips InProcess Bus")
    }
}

struct InProcessComponent<T: Send + Sync + Debug> {
    component: T,
    bus: Mutex<ripb::Bus>,
    subscriber: Mutex<Option<ripb::Subscriber>>,
}

impl<T: Send + Sync + Debug> Drop for InProcessComponent<T> {
    fn drop(&mut self) {
        // dropping a ripb subscriber removes its subscriptions it. As we don't have unsubscription
        // mechanic in hermes (yet) and there are quite a lot of parts in the code where we just
        // drop the facade after subscribing, let's just forget the subscriber for now.
        if let Ok(mut subscriber) = self.subscriber.lock() {
            if let Some(subscriber) = subscriber.take() {
                std::mem::forget(subscriber)
            }
        }
    }
}

impl<T: Send + Sync + Debug> InProcessComponent<T> {
    fn publish<M: ripb::Message + Debug + 'static>(&self, message: M) -> Fallible<()> {
        debug!("Publishing {:?}/{:#?}", self.component, message);
        self.publish_quiet(message)
    }

    fn publish_quiet<M: ripb::Message + Debug + 'static>(&self, message: M) -> Fallible<()> {
        let bus = self.bus.lock().map_err(PoisonLock::from)?;
        bus.publish(message);
        Ok(())
    }

    fn subscribe0<M: ripb::Message + 'static>(&self, callback: Callback0) -> Fallible<()> {
        let mut subscriber = self.subscriber.lock().map_err(PoisonLock::from)?;
        let subscriber = subscriber.get_or_insert_with(|| self.bus.lock().unwrap().create_subscriber());
        subscriber.on_message(move |_: &M| callback.call());
        Ok(())
    }

    fn subscribe<M, P, C>(&self, callback: Callback<P>, converter: C) -> Fallible<()>
    where
        M: ripb::Message + Debug + 'static,
        P: 'static,
        C: Fn(&M) -> &P + Send + 'static,
    {
        let mut subscriber = self.subscriber.lock().map_err(PoisonLock::from)?;
        let subscriber = subscriber.get_or_insert_with(|| self.bus.lock().unwrap().create_subscriber());
        subscriber.on_message(move |m: &M| callback.call(converter(m)));
        Ok(())
    }

    fn subscribe0_filter<M, F>(&self, callback: Callback0, filter: F) -> Fallible<()>
    where
        M: ripb::Message + 'static,
        F: Fn(&M) -> bool + Send + 'static,
    {
        let mut subscriber = self.subscriber.lock().map_err(PoisonLock::from)?;
        let subscriber = subscriber.get_or_insert_with(|| self.bus.lock().unwrap().create_subscriber());
        subscriber.on_message(move |m: &M| {
            if filter(m) {
                callback.call()
            }
        });
        Ok(())
    }

    fn subscribe_filter<M, P, C, F>(&self, callback: Callback<P>, converter: C, filter: F) -> Fallible<()>
    where
        M: ripb::Message + Debug + 'static,
        P: 'static,
        C: Fn(&M) -> &P + Send + 'static,
        F: Fn(&M) -> bool + Send + 'static,
    {
        let mut subscriber = self.subscriber.lock().map_err(PoisonLock::from)?;
        let subscriber = subscriber.get_or_insert_with(|| self.bus.lock().unwrap().create_subscriber());
        subscriber.on_message(move |m: &M| {
            if filter(m) {
                callback.call(converter(m))
            }
        });
        Ok(())
    }
}

macro_rules! subscribe {
    ($sel:ident, $t:ty { $field:ident }, $handler:ident) => {{
        log::debug!("Subscribing on {:?}/{}", $sel.component, stringify!($t));
        $sel.subscribe($handler, |it: &$t| &it.$field)
    }};
    ($sel:ident, $t:ty, $handler:ident) => {{
        log::debug!("Subscribing on {:?}/{}", $sel.component, stringify!($t));
        $sel.subscribe0::<$t>($handler)
    }};
}

macro_rules! subscribe_filter {
    ($sel:ident, $t:ty { $field:ident }, $handler:ident, $filter:ident) => {{
        log::debug!("Subscribing on {:?}/{}", $sel.component, stringify!($t));
        $sel.subscribe_filter(
            $handler,
            |it: &$t| &it.$field,
            move |it: &$t| it.$field.$filter == $filter,
        )
    }};
    (
        $sel:ident,
        $t:ty { $field:ident },
        $handler:ident,
        $filter:ident, |
        $it:ident |
        $filter_path:expr
    ) => {{
        log::debug!("Subscribing on {:?}/{}", $sel.component, stringify!($t));
        $sel.subscribe_filter($handler, |it: &$t| &it.$field, move |$it: &$t| $filter_path == &$filter)
    }};
    ($sel:ident, $t:ty, $handler:ident, $filter:ident) => {{
        log::debug!("Subscribing on {:?}/{}", $sel.component, stringify!($t));
        $sel.subscribe0_filter($handler, move |it: &$t| it.$filter == $filter)
    }};
}

#[derive(Debug)]
struct ComponentVersionRequest<T: Debug> {
    component: T,
}

#[derive(Debug)]
struct ComponentVersion<T: Debug> {
    version: VersionMessage,
    component: T,
}

#[derive(Debug)]
struct ComponentError<T: Debug> {
    error: ErrorMessage,
    component: T,
}

impl<T: Send + Sync + Debug + Copy + 'static> ComponentFacade for InProcessComponent<T> {
    fn publish_version_request(&self) -> Fallible<()> {
        self.publish(ComponentVersionRequest {
            component: self.component,
        } as ComponentVersionRequest<T>)
    }
    fn subscribe_version(&self, handler: Callback<VersionMessage>) -> Fallible<()> {
        subscribe!(self, ComponentVersion<T> { version }, handler)
    }
    fn subscribe_error(&self, handler: Callback<ErrorMessage>) -> Fallible<()> {
        subscribe!(self, ComponentError<T> { error }, handler)
    }
}

impl<T: Send + Sync + Debug + Copy + 'static> ComponentBackendFacade for InProcessComponent<T> {
    fn subscribe_version_request(&self, handler: Callback0) -> Fallible<()> {
        subscribe!(self, ComponentVersionRequest<T>, handler)
    }

    fn publish_version(&self, version: VersionMessage) -> Fallible<()> {
        let component_version: ComponentVersion<T> = ComponentVersion {
            version,
            component: self.component,
        };
        self.publish(component_version)
    }

    fn publish_error(&self, error: ErrorMessage) -> Fallible<()> {
        let component_error: ComponentError<T> = ComponentError {
            error,
            component: self.component,
        };
        self.publish(component_error)
    }
}

#[derive(Debug)]
struct IdentifiableComponentVersionRequest<T: Debug> {
    site_id: String,
    component: T,
}

#[derive(Debug)]
struct IdentifiableComponentVersion<T: Debug> {
    site_id: String,
    version: VersionMessage,
    component: T,
}

#[derive(Debug)]
struct IdentifiableComponentError<T: Debug> {
    site_id: String,
    error: ErrorMessage,
    component: T,
}

impl<T: Send + Sync + Debug + Copy + 'static> IdentifiableComponentFacade for InProcessComponent<T> {
    fn publish_version_request(&self, site_id: String) -> Fallible<()> {
        let version_request = IdentifiableComponentVersionRequest {
            site_id,
            component: self.component,
        };
        self.publish(version_request)
    }

    fn subscribe_version(&self, site_id: String, handler: Callback<VersionMessage>) -> Fallible<()> {
        subscribe_filter!(self, IdentifiableComponentVersion<T> { version }, handler, site_id, |it| &it.site_id)
    }

    fn subscribe_error(&self, site_id: String, handler: Callback<ErrorMessage>) -> Fallible<()> {
        subscribe_filter!(self, IdentifiableComponentError<T> { error }, handler, site_id, |it| &it.site_id)
    }
}

impl<T: Send + Sync + Debug + Copy + 'static> IdentifiableComponentBackendFacade for InProcessComponent<T> {
    fn subscribe_version_request(&self, site_id: String, handler: Callback0) -> Fallible<()> {
        subscribe_filter!(self, IdentifiableComponentVersionRequest<T>, handler, site_id)
    }

    fn publish_version(&self, site_id: String, version: VersionMessage) -> Fallible<()> {
        let component_version: IdentifiableComponentVersion<T> = IdentifiableComponentVersion {
            site_id,
            version,
            component: self.component,
        };
        self.publish(component_version)
    }

    fn publish_error(&self, site_id: String, error: ErrorMessage) -> Fallible<()> {
        let component_error: IdentifiableComponentError<T> = IdentifiableComponentError {
            site_id,
            error,
            component: self.component,
        };
        self.publish(component_error)
    }
}

#[derive(Debug)]
struct IdentifiableToggleableToggleOn<T> {
    site: SiteMessage,
    component: T,
}

#[derive(Debug)]
struct IdentifiableToggleableToggleOff<T> {
    site: SiteMessage,
    component: T,
}

impl<T: Send + Sync + Debug + Copy + 'static> IdentifiableToggleableFacade for InProcessComponent<T> {
    fn publish_toggle_on(&self, site: SiteMessage) -> Fallible<()> {
        let toggle_on: IdentifiableToggleableToggleOn<T> = IdentifiableToggleableToggleOn {
            site,
            component: self.component,
        };
        self.publish(toggle_on)
    }

    fn publish_toggle_off(&self, site: SiteMessage) -> Fallible<()> {
        let toggle_off: IdentifiableToggleableToggleOff<T> = IdentifiableToggleableToggleOff {
            site,
            component: self.component,
        };
        self.publish(toggle_off)
    }
}

impl<T: Send + Sync + Debug + 'static> IdentifiableToggleableBackendFacade for InProcessComponent<T> {
    fn subscribe_toggle_on(&self, handler: Callback<SiteMessage>) -> Fallible<()> {
        subscribe!(self, IdentifiableToggleableToggleOn<T> { site }, handler)
    }

    fn subscribe_toggle_off(&self, handler: Callback<SiteMessage>) -> Fallible<()> {
        subscribe!(self, IdentifiableToggleableToggleOff<T> { site }, handler)
    }
}

#[derive(Debug, Clone, Copy)]
struct Nlu;

#[derive(Debug)]
struct NluQuery {
    query: NluQueryMessage,
}

#[derive(Debug)]
struct NluPartialQuery {
    query: NluSlotQueryMessage,
}

#[derive(Debug)]
struct NluSlotParsed {
    slot: NluSlotMessage,
}

#[derive(Debug)]
struct NluIntentParsed {
    intent: NluIntentMessage,
}

#[derive(Debug)]
struct NluIntentNotRecognized {
    status: NluIntentNotRecognizedMessage,
}

#[derive(Debug)]
struct NluReload {}

impl NluFacade for InProcessComponent<Nlu> {
    fn publish_query(&self, query: NluQueryMessage) -> Fallible<()> {
        self.publish(NluQuery { query })
    }

    fn publish_partial_query(&self, query: NluSlotQueryMessage) -> Fallible<()> {
        self.publish(NluPartialQuery { query })
    }

    fn publish_reload(&self) -> Fallible<()> {
        self.publish(NluReload {})
    }

    fn subscribe_slot_parsed(&self, handler: Callback<NluSlotMessage>) -> Fallible<()> {
        subscribe!(self, NluSlotParsed { slot }, handler)
    }

    fn subscribe_intent_parsed(&self, handler: Callback<NluIntentMessage>) -> Fallible<()> {
        subscribe!(self, NluIntentParsed { intent }, handler)
    }

    fn subscribe_intent_not_recognized(&self, handler: Callback<NluIntentNotRecognizedMessage>) -> Fallible<()> {
        subscribe!(self, NluIntentNotRecognized { status }, handler)
    }
}

impl NluBackendFacade for InProcessComponent<Nlu> {
    fn subscribe_query(&self, handler: Callback<NluQueryMessage>) -> Fallible<()> {
        subscribe!(self, NluQuery { query }, handler)
    }

    fn subscribe_partial_query(&self, handler: Callback<NluSlotQueryMessage>) -> Fallible<()> {
        subscribe!(self, NluPartialQuery { query }, handler)
    }

    fn subscribe_reload(&self, handler: Callback0) -> Fallible<()> {
        subscribe!(self, NluReload, handler)
    }

    fn publish_slot_parsed(&self, slot: NluSlotMessage) -> Fallible<()> {
        self.publish(NluSlotParsed { slot })
    }

    fn publish_intent_parsed(&self, intent: NluIntentMessage) -> Fallible<()> {
        self.publish(NluIntentParsed { intent })
    }

    fn publish_intent_not_recognized(&self, status: NluIntentNotRecognizedMessage) -> Fallible<()> {
        self.publish(NluIntentNotRecognized { status })
    }
}

#[derive(Debug)]
struct ToggleableToggleOn<T> {
    component: T,
}

#[derive(Debug)]
struct ToggleableToggleOff<T> {
    component: T,
}

impl<T: Send + Sync + Debug + Copy + 'static> ToggleableFacade for InProcessComponent<T> {
    fn publish_toggle_on(&self) -> Fallible<()> {
        let toggle_on: ToggleableToggleOn<T> = ToggleableToggleOn {
            component: self.component,
        };
        self.publish(toggle_on)
    }

    fn publish_toggle_off(&self) -> Fallible<()> {
        let toggle_off: ToggleableToggleOff<T> = ToggleableToggleOff {
            component: self.component,
        };
        self.publish(toggle_off)
    }
}

impl<T: Send + Sync + Debug + 'static> ToggleableBackendFacade for InProcessComponent<T> {
    fn subscribe_toggle_on(&self, handler: Callback0) -> Fallible<()> {
        subscribe!(self, ToggleableToggleOn<T>, handler)
    }

    fn subscribe_toggle_off(&self, handler: Callback0) -> Fallible<()> {
        subscribe!(self, ToggleableToggleOff<T>, handler)
    }
}

#[derive(Debug, Clone, Copy)]
struct VoiceActivity;

#[derive(Debug)]
struct VoiceActivityVadUp {
    vad_up: VadUpMessage,
}

#[derive(Debug)]
struct VoiceActivityVadDown {
    vad_down: VadDownMessage,
}

impl VoiceActivityFacade for InProcessComponent<VoiceActivity> {
    fn subscribe_vad_up(&self, site_id: String, handler: Callback<VadUpMessage>) -> Fallible<()> {
        subscribe_filter!(self, VoiceActivityVadUp { vad_up }, handler, site_id, |it| &it
            .vad_up
            .site_id)
    }

    fn subscribe_vad_down(&self, site_id: String, handler: Callback<VadDownMessage>) -> Fallible<()> {
        subscribe_filter!(self, VoiceActivityVadDown { vad_down }, handler, site_id, |it| &it
            .vad_down
            .site_id)
    }

    fn subscribe_all_vad_up(&self, handler: Callback<VadUpMessage>) -> Fallible<()> {
        subscribe!(self, VoiceActivityVadUp { vad_up }, handler)
    }

    fn subscribe_all_vad_down(&self, handler: Callback<VadDownMessage>) -> Fallible<()> {
        subscribe!(self, VoiceActivityVadDown { vad_down }, handler)
    }
}

impl VoiceActivityBackendFacade for InProcessComponent<VoiceActivity> {
    fn publish_vad_up(&self, vad_up: VadUpMessage) -> Fallible<()> {
        self.publish(VoiceActivityVadUp { vad_up })
    }

    fn publish_vad_down(&self, vad_down: VadDownMessage) -> Fallible<()> {
        self.publish(VoiceActivityVadDown { vad_down })
    }
}

#[derive(Debug, Clone, Copy)]
struct Hotword;

#[derive(Debug)]
struct HotwordDetected {
    id: String,
    message: HotwordDetectedMessage,
}

impl HotwordFacade for InProcessComponent<Hotword> {
    fn subscribe_detected(&self, id: String, handler: Callback<HotwordDetectedMessage>) -> Fallible<()> {
        subscribe_filter!(self, HotwordDetected { message }, handler, id, |it| &it.id)
    }

    fn subscribe_all_detected(&self, handler: Callback<HotwordDetectedMessage>) -> Fallible<()> {
        subscribe!(self, HotwordDetected { message }, handler)
    }
}

impl HotwordBackendFacade for InProcessComponent<Hotword> {
    fn publish_detected(&self, id: String, message: HotwordDetectedMessage) -> Fallible<()> {
        self.publish(HotwordDetected { id, message })
    }
}

#[derive(Debug, Clone, Copy)]
struct Sound;

impl SoundFeedbackFacade for InProcessComponent<Sound> {}

impl SoundFeedbackBackendFacade for InProcessComponent<Sound> {}

#[derive(Debug, Clone, Copy)]
struct Asr;

#[derive(Debug)]
struct AsrStartListening {
    start: AsrStartListeningMessage,
}

#[derive(Debug)]
struct AsrStopListening {
    site: SiteMessage,
}

#[derive(Debug)]
struct AsrReload {}

#[derive(Debug)]
struct AsrTextCaptured {
    text_captured: TextCapturedMessage,
}

#[derive(Debug)]
struct AsrPartialTextCaptured {
    text_captured: TextCapturedMessage,
}

impl AsrFacade for InProcessComponent<Asr> {
    fn publish_start_listening(&self, start: AsrStartListeningMessage) -> Fallible<()> {
        self.publish(AsrStartListening { start })
    }

    fn publish_stop_listening(&self, site: SiteMessage) -> Fallible<()> {
        self.publish(AsrStopListening { site })
    }

    fn publish_reload(&self) -> Fallible<()> {
        self.publish(AsrReload {})
    }

    fn subscribe_text_captured(&self, handler: Callback<TextCapturedMessage>) -> Fallible<()> {
        subscribe!(self, AsrTextCaptured { text_captured }, handler)
    }

    fn subscribe_partial_text_captured(&self, handler: Callback<TextCapturedMessage>) -> Fallible<()> {
        subscribe!(self, AsrPartialTextCaptured { text_captured }, handler)
    }
}

impl AsrBackendFacade for InProcessComponent<Asr> {
    fn subscribe_start_listening(&self, handler: Callback<AsrStartListeningMessage>) -> Fallible<()> {
        subscribe!(self, AsrStartListening { start }, handler)
    }

    fn subscribe_stop_listening(&self, handler: Callback<SiteMessage>) -> Fallible<()> {
        subscribe!(self, AsrStopListening { site }, handler)
    }

    fn subscribe_reload(&self, handler: Callback0) -> Fallible<()> {
        subscribe!(self, AsrReload, handler)
    }

    fn publish_text_captured(&self, text_captured: TextCapturedMessage) -> Fallible<()> {
        self.publish(AsrTextCaptured { text_captured })
    }

    fn publish_partial_text_captured(&self, text_captured: TextCapturedMessage) -> Fallible<()> {
        self.publish(AsrPartialTextCaptured { text_captured })
    }
}

#[derive(Debug, Clone, Copy)]
struct Tts;

#[derive(Debug)]
struct TtsSay {
    to_say: SayMessage,
}

#[derive(Debug)]
struct TtsSayFinished {
    status: SayFinishedMessage,
}

#[derive(Debug)]
struct TtsRegisterSound {
    sound: RegisterSoundMessage,
}

impl TtsFacade for InProcessComponent<Tts> {
    fn publish_say(&self, to_say: SayMessage) -> Fallible<()> {
        self.publish(TtsSay { to_say })
    }

    fn subscribe_say_finished(&self, handler: Callback<SayFinishedMessage>) -> Fallible<()> {
        subscribe!(self, TtsSayFinished { status }, handler)
    }

    fn publish_register_sound(&self, sound: RegisterSoundMessage) -> Fallible<()> {
        self.publish(TtsRegisterSound { sound })
    }
}

impl TtsBackendFacade for InProcessComponent<Tts> {
    fn publish_say_finished(&self, status: SayFinishedMessage) -> Fallible<()> {
        self.publish(TtsSayFinished { status })
    }

    fn subscribe_say(&self, handler: Callback<SayMessage>) -> Fallible<()> {
        subscribe!(self, TtsSay { to_say }, handler)
    }

    fn subscribe_register_sound(&self, handler: Callback<RegisterSoundMessage>) -> Fallible<()> {
        subscribe!(self, TtsRegisterSound { sound }, handler)
    }
}

#[derive(Debug, Clone, Copy)]
struct AudioServer;

#[derive(Debug)]
struct AudioServerPlayBytes {
    bytes: PlayBytesMessage,
}

#[derive(Debug)]
struct AudioServerPlayFinished {
    status: PlayFinishedMessage,
}

#[derive(Debug)]
struct AudioServerAudioFrame {
    frame: AudioFrameMessage,
}

#[derive(Debug)]
struct AudioServerReplayRequest {
    request: ReplayRequestMessage,
}

#[derive(Debug)]
struct AudioServerReplayResponse {
    frame: AudioFrameMessage,
}

impl AudioServerFacade for InProcessComponent<AudioServer> {
    fn publish_play_bytes(&self, bytes: PlayBytesMessage) -> Fallible<()> {
        self.publish(AudioServerPlayBytes { bytes })
    }

    fn subscribe_play_finished(&self, site_id: String, handler: Callback<PlayFinishedMessage>) -> Fallible<()> {
        subscribe_filter!(self, AudioServerPlayFinished { status }, handler, site_id)
    }

    fn subscribe_all_play_finished(&self, handler: Callback<PlayFinishedMessage>) -> Fallible<()> {
        subscribe!(self, AudioServerPlayFinished { status }, handler)
    }

    fn subscribe_audio_frame(&self, site_id: String, handler: Callback<AudioFrameMessage>) -> Fallible<()> {
        subscribe_filter!(self, AudioServerAudioFrame { frame }, handler, site_id)
    }

    fn publish_replay_request(&self, request: ReplayRequestMessage) -> Fallible<()> {
        self.publish(AudioServerReplayRequest { request })
    }

    fn subscribe_replay_response(&self, site_id: String, handler: Callback<AudioFrameMessage>) -> Fallible<()> {
        subscribe_filter!(self, AudioServerReplayResponse { frame }, handler, site_id)
    }
}

impl AudioServerBackendFacade for InProcessComponent<AudioServer> {
    fn subscribe_play_bytes(&self, site_id: String, handler: Callback<PlayBytesMessage>) -> Fallible<()> {
        subscribe_filter!(self, AudioServerPlayBytes { bytes }, handler, site_id)
    }

    fn subscribe_all_play_bytes(&self, handler: Callback<PlayBytesMessage>) -> Fallible<()> {
        subscribe!(self, AudioServerPlayBytes { bytes }, handler)
    }

    fn publish_play_finished(&self, status: PlayFinishedMessage) -> Fallible<()> {
        self.publish(AudioServerPlayFinished { status })
    }

    fn publish_audio_frame(&self, frame: AudioFrameMessage) -> Fallible<()> {
        self.publish_quiet(AudioServerAudioFrame { frame })
    }

    fn subscribe_replay_request(&self, site_id: String, handler: Callback<ReplayRequestMessage>) -> Fallible<()> {
        subscribe_filter!(self, AudioServerReplayRequest { request }, handler, site_id)
    }

    fn publish_replay_response(&self, frame: AudioFrameMessage) -> Fallible<()> {
        self.publish(AudioServerReplayResponse { frame })
    }
}

#[derive(Debug, Clone, Copy)]
struct Dialogue;

#[derive(Debug)]
struct DialogueSessionQueued {
    status: SessionQueuedMessage,
}

#[derive(Debug)]
struct DialogueSessionStarted {
    status: SessionStartedMessage,
}

#[derive(Debug)]
struct DialogueIntent {
    intent: IntentMessage,
}

#[derive(Debug)]
struct DialogueIntentNotRecognized {
    intent_not_recognized: IntentNotRecognizedMessage,
}

#[derive(Debug)]
struct DialogueSessionEnded {
    status: SessionEndedMessage,
}

#[derive(Debug)]
struct DialogueStartSession {
    start_session: StartSessionMessage,
}

#[derive(Debug)]
struct DialogueContinueSession {
    continue_session: ContinueSessionMessage,
}

#[derive(Debug)]
struct DialogueEndSession {
    end_session: EndSessionMessage,
}

#[derive(Debug)]
struct DialogueConfigure {
    config: DialogueConfigureMessage,
}

impl DialogueFacade for InProcessComponent<Dialogue> {
    fn subscribe_session_queued(&self, handler: Callback<SessionQueuedMessage>) -> Fallible<()> {
        subscribe!(self, DialogueSessionQueued { status }, handler)
    }

    fn subscribe_session_started(&self, handler: Callback<SessionStartedMessage>) -> Fallible<()> {
        subscribe!(self, DialogueSessionStarted { status }, handler)
    }

    fn subscribe_intent(&self, intent_name: String, handler: Callback<IntentMessage>) -> Fallible<()> {
        subscribe_filter!(self, DialogueIntent { intent }, handler, intent_name, |it| &it
            .intent
            .intent
            .intent_name)
    }

    fn subscribe_intents(&self, handler: Callback<IntentMessage>) -> Fallible<()> {
        subscribe!(self, DialogueIntent { intent }, handler)
    }

    fn subscribe_intent_not_recognized(&self, handler: Callback<IntentNotRecognizedMessage>) -> Fallible<()> {
        subscribe!(self, DialogueIntentNotRecognized { intent_not_recognized }, handler)
    }

    fn subscribe_session_ended(&self, handler: Callback<SessionEndedMessage>) -> Fallible<()> {
        subscribe!(self, DialogueSessionEnded { status }, handler)
    }

    fn publish_start_session(&self, start_session: StartSessionMessage) -> Fallible<()> {
        self.publish(DialogueStartSession { start_session })
    }

    fn publish_continue_session(&self, continue_session: ContinueSessionMessage) -> Fallible<()> {
        self.publish(DialogueContinueSession { continue_session })
    }

    fn publish_end_session(&self, end_session: EndSessionMessage) -> Fallible<()> {
        self.publish(DialogueEndSession { end_session })
    }

    fn publish_configure(&self, config: DialogueConfigureMessage) -> Fallible<()> {
        self.publish(DialogueConfigure { config })
    }
}

impl DialogueBackendFacade for InProcessComponent<Dialogue> {
    fn publish_session_queued(&self, status: SessionQueuedMessage) -> Fallible<()> {
        self.publish(DialogueSessionQueued { status })
    }

    fn publish_session_started(&self, status: SessionStartedMessage) -> Fallible<()> {
        self.publish(DialogueSessionStarted { status })
    }

    fn publish_intent(&self, intent: IntentMessage) -> Fallible<()> {
        self.publish(DialogueIntent { intent })
    }

    fn publish_intent_not_recognized(&self, intent_not_recognized: IntentNotRecognizedMessage) -> Fallible<()> {
        self.publish(DialogueIntentNotRecognized { intent_not_recognized })
    }

    fn publish_session_ended(&self, status: SessionEndedMessage) -> Fallible<()> {
        self.publish(DialogueSessionEnded { status })
    }

    fn subscribe_start_session(&self, handler: Callback<StartSessionMessage>) -> Fallible<()> {
        subscribe!(self, DialogueStartSession { start_session }, handler)
    }

    fn subscribe_continue_session(&self, handler: Callback<ContinueSessionMessage>) -> Fallible<()> {
        subscribe!(self, DialogueContinueSession { continue_session }, handler)
    }

    fn subscribe_end_session(&self, handler: Callback<EndSessionMessage>) -> Fallible<()> {
        subscribe!(self, DialogueEndSession { end_session }, handler)
    }

    fn subscribe_configure(&self, handler: Callback<DialogueConfigureMessage>) -> Fallible<()> {
        subscribe!(self, DialogueConfigure { config }, handler)
    }
}

#[derive(Debug, Clone, Copy)]
struct Injection;

#[derive(Debug)]
struct InjectionPerform {
    request: InjectionRequestMessage,
}

#[derive(Debug)]
struct InjectionStatus {
    status: InjectionStatusMessage,
}

#[derive(Debug)]
struct InjectionStatusRequest {}

impl InjectionFacade for InProcessComponent<Injection> {
    fn publish_injection_request(&self, request: InjectionRequestMessage) -> Fallible<()> {
        self.publish(InjectionPerform { request })
    }

    fn publish_injection_status_request(&self) -> Fallible<()> {
        self.publish(InjectionStatusRequest {})
    }

    fn subscribe_injection_status(&self, handler: Callback<InjectionStatusMessage>) -> Fallible<()> {
        subscribe!(self, InjectionStatus { status }, handler)
    }
}

impl InjectionBackendFacade for InProcessComponent<Injection> {
    fn subscribe_injection_request(&self, handler: Callback<InjectionRequestMessage>) -> Fallible<()> {
        subscribe!(self, InjectionPerform { request }, handler)
    }

    fn subscribe_injection_status_request(&self, handler: Callback0) -> Fallible<()> {
        subscribe!(self, InjectionStatusRequest, handler)
    }

    fn publish_injection_status(&self, status: InjectionStatusMessage) -> Fallible<()> {
        self.publish(InjectionStatus { status })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::rc::Rc;

    fn create_handlers() -> (Rc<InProcessHermesProtocolHandler>, Rc<InProcessHermesProtocolHandler>) {
        let handler = Rc::new(InProcessHermesProtocolHandler::new());
        (Rc::clone(&handler), handler)
    }

    hermes_test_suite::test_suite!();
}
