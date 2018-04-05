extern crate failure;
extern crate hermes;
#[cfg(test)]
#[macro_use]
extern crate hermes_test_suite;
#[macro_use]
extern crate log;
extern crate ripb;
#[cfg(test)]
extern crate semver;
#[cfg(test)]
extern crate snips_nlu_ontology;

use std::fmt::Debug;
use std::sync::Mutex;

use hermes::*;

pub struct InProcessHermesProtocolHandler {
    bus: Mutex<ripb::Bus>,
}

impl InProcessHermesProtocolHandler {
    pub fn new() -> Result<Self> {
        Ok(Self {
            bus: Mutex::new(ripb::Bus::new()),
        })
    }

    fn get_handler<T: Send + Sync + Debug>(&self, component: T) -> Box<InProcessComponent<T>> {
        let bus = self.bus.lock().unwrap().clone();
        let subscriber = bus.create_subscriber();
        Box::new(InProcessComponent {
            component,
            bus: Mutex::new(bus),
            subscriber: Mutex::new(subscriber),
        })
    }
}

impl HermesProtocolHandler for InProcessHermesProtocolHandler {
    fn asr(&self) -> Box<AsrFacade> {
        self.get_handler(Asr)
    }
    fn asr_backend(&self) -> Box<AsrBackendFacade> {
        self.get_handler(Asr)
    }
    fn audio_server(&self) -> Box<AudioServerFacade> {
        self.get_handler(AudioServer)
    }
    fn audio_server_backend(&self) -> Box<AudioServerBackendFacade> {
        self.get_handler(AudioServer)
    }
    fn hotword(&self) -> Box<HotwordFacade> {
        self.get_handler(Hotword)
    }
    fn hotword_backend(&self) -> Box<HotwordBackendFacade> {
        self.get_handler(Hotword)
    }
    fn dialogue(&self) -> Box<DialogueFacade> {
        self.get_handler(Dialogue)
    }
    fn dialogue_backend(&self) -> Box<DialogueBackendFacade> {
        self.get_handler(Dialogue)
    }
    fn nlu(&self) -> Box<NluFacade> {
        self.get_handler(Nlu)
    }
    fn nlu_backend(&self) -> Box<NluBackendFacade> {
        self.get_handler(Nlu)
    }
    fn sound_feedback(&self) -> Box<SoundFeedbackFacade> {
        self.get_handler(Sound)
    }
    fn sound_feedback_backend(&self) -> Box<SoundFeedbackBackendFacade> {
        self.get_handler(Sound)
    }
    fn tts(&self) -> Box<TtsFacade> {
        self.get_handler(Tts)
    }
    fn tts_backend(&self) -> Box<TtsBackendFacade> {
        self.get_handler(Tts)
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
    subscriber: Mutex<ripb::Subscriber>,
}

impl<T: Send + Sync + Debug> InProcessComponent<T> {
    fn publish<M: ripb::Message + Debug + 'static>(&self, message: M) -> Result<()> {
        debug!("Publishing {:?}/{:#?}", self.component, message);
        let bus = self.bus.lock().map_err(PoisonLock::from)?;
        bus.publish(message);
        Ok(())
    }

    fn subscribe0<M: ripb::Message + 'static>(&self, callback: Callback0) -> Result<()> {
        let subscriber = self.subscriber.lock().map_err(PoisonLock::from)?;
        subscriber.on_message(move |_: &M| callback.call());
        Ok(())
    }

    fn subscribe<M, P, C>(&self, callback: Callback<P>, converter: C) -> Result<()>
    where
        M: ripb::Message + Debug + 'static,
        P: 'static,
        C: Fn(&M) -> &P + Send + 'static,
    {
        let subscriber = self.subscriber.lock().map_err(PoisonLock::from)?;
        subscriber.on_message(move |m: &M| callback.call(converter(m)));
        Ok(())
    }

    fn subscribe0_filter<M, F>(&self, callback: Callback0, filter: F) -> Result<()>
    where
        M: ripb::Message + 'static,
        F: Fn(&M) -> bool + Send + 'static,
    {
        let subscriber = self.subscriber.lock().map_err(PoisonLock::from)?;
        subscriber.on_message(move |m: &M| {
            if filter(m) {
                callback.call()
            }
        });
        Ok(())
    }

    fn subscribe_filter<M, P, C, F>(
        &self,
        callback: Callback<P>,
        converter: C,
        filter: F,
    ) -> Result<()>
    where
        M: ripb::Message + Debug + 'static,
        P: 'static,
        C: Fn(&M) -> &P + Send + 'static,
        F: Fn(&M) -> bool + Send + 'static,
    {
        let subscriber = self.subscriber.lock().map_err(PoisonLock::from)?;
        subscriber.on_message(move |m: &M| {
            if filter(m) {
                callback.call(converter(m))
            }
        });
        Ok(())
    }
}

macro_rules! subscribe {
    ($sel:ident, $t:ty { $field:ident }, $handler:ident ) => {{
        debug!("Subscribing on {:?}/{}", $sel.component, stringify!($t));
        $sel.subscribe($handler, |it: &$t| &it.$field)
    }};
    ($sel:ident, $t:ty, $handler:ident ) => {{
        debug!("Subscribing on {:?}/{}", $sel.component, stringify!($t));
        $sel.subscribe0::<$t>($handler)
    }};
}

macro_rules! subscribe_filter {
    ($sel:ident, $t:ty { $field:ident }, $handler:ident, $filter:ident) => {{
        debug!("Subscribing on {:?}/{}", $sel.component, stringify!($t));
        $sel.subscribe_filter($handler, |it: &$t| &it.$field, move |it: &$t| it.$field.$filter == $filter)
    }};
    ($sel:ident, $t:ty { $field:ident }, $handler:ident, $filter:ident, | $it:ident | $filter_path:block ) => {{
        debug!("Subscribing on {:?}/{}", $sel.component, stringify!($t));
        $sel.subscribe_filter($handler, |it: &$t| &it.$field, move |$it: &$t| $filter_path == &$filter)
    }};
    ($sel:ident, $t:ty, $handler:ident, $filter:ident) => {{
        debug!("Subscribing on {:?}/{}", $sel.component, stringify!($t));
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
    fn publish_version_request(&self) -> Result<()> {
        self.publish(ComponentVersionRequest {
            component: self.component,
        } as ComponentVersionRequest<T>)
    }
    fn subscribe_version(&self, handler: Callback<VersionMessage>) -> Result<()> {
        subscribe!(self, ComponentVersion<T> { version }, handler)
    }
    fn subscribe_error(&self, handler: Callback<ErrorMessage>) -> Result<()> {
        subscribe!(self, ComponentError<T> { error }, handler)
    }
}

impl<T: Send + Sync + Debug + Copy + 'static> ComponentBackendFacade for InProcessComponent<T> {
    fn subscribe_version_request(&self, handler: Callback0) -> Result<()> {
        subscribe!(self, ComponentVersionRequest<T>, handler)
    }
    fn publish_version(&self, version: VersionMessage) -> Result<()> {
        let component_version: ComponentVersion<T> = ComponentVersion {
            version,
            component: self.component,
        };
        self.publish(component_version)
    }
    fn publish_error(&self, error: ErrorMessage) -> Result<()> {
        let component_error: ComponentError<T> = ComponentError {
            error,
            component: self.component,
        };
        self.publish(component_error)
    }
}

#[derive(Debug)]
struct IdentifiableComponentVersionRequest<T: Debug> {
    site_id: SiteId,
    component: T,
}

#[derive(Debug)]
struct IdentifiableComponentVersion<T: Debug> {
    site_id: SiteId,
    version: VersionMessage,
    component: T,
}

#[derive(Debug)]
struct IdentifiableComponentError<T: Debug> {
    site_id: SiteId,
    error: ErrorMessage,
    component: T,
}

impl<T: Send + Sync + Debug + Copy + 'static> IdentifiableComponentFacade
    for InProcessComponent<T>
{
    fn publish_version_request(&self, site_id: SiteId) -> Result<()> {
        let version_request: IdentifiableComponentVersionRequest<T> =
            IdentifiableComponentVersionRequest {
                site_id,
                component: self.component,
            };
        self.publish(version_request)
    }
    fn subscribe_version(&self, site_id: SiteId, handler: Callback<VersionMessage>) -> Result<()> {
        subscribe_filter!(self, IdentifiableComponentVersion<T> { version }, handler, site_id, |it| {&it.site_id})
    }
    fn subscribe_error(&self, site_id: SiteId, handler: Callback<ErrorMessage>) -> Result<()> {
        subscribe_filter!(self, IdentifiableComponentError<T> { error }, handler, site_id, |it| {&it.site_id})
    }
}

impl<T: Send + Sync + Debug + Copy + 'static> IdentifiableComponentBackendFacade
    for InProcessComponent<T>
{
    fn subscribe_version_request(&self, site_id: SiteId, handler: Callback0) -> Result<()> {
        subscribe_filter!(
            self,
            IdentifiableComponentVersionRequest<T>,
            handler,
            site_id
        )
    }
    fn publish_version(&self, site_id: SiteId, version: VersionMessage) -> Result<()> {
        let component_version: IdentifiableComponentVersion<T> = IdentifiableComponentVersion {
            site_id,
            version,
            component: self.component,
        };
        self.publish(component_version)
    }
    fn publish_error(&self, site_id: SiteId, error: ErrorMessage) -> Result<()> {
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

impl<T: Send + Sync + Debug + Copy + 'static> IdentifiableToggleableFacade
    for InProcessComponent<T>
{
    fn publish_toggle_on(&self, site: SiteMessage) -> Result<()> {
        let toggle_on: IdentifiableToggleableToggleOn<T> = IdentifiableToggleableToggleOn {
            site,
            component: self.component,
        };
        self.publish(toggle_on)
    }
    fn publish_toggle_off(&self, site: SiteMessage) -> Result<()> {
        let toggle_off: IdentifiableToggleableToggleOff<T> = IdentifiableToggleableToggleOff {
            site,
            component: self.component,
        };
        self.publish(toggle_off)
    }
}

impl<T: Send + Sync + Debug + 'static> IdentifiableToggleableBackendFacade
    for InProcessComponent<T>
{
    fn subscribe_toggle_on(&self, handler: Callback<SiteMessage>) -> Result<()> {
        subscribe!(self, IdentifiableToggleableToggleOn<T> { site }, handler)
    }
    fn subscribe_toggle_off(&self, handler: Callback<SiteMessage>) -> Result<()> {
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

impl NluFacade for InProcessComponent<Nlu> {
    fn publish_query(&self, query: NluQueryMessage) -> Result<()> {
        self.publish(NluQuery { query })
    }

    fn publish_partial_query(&self, query: NluSlotQueryMessage) -> Result<()> {
        self.publish(NluPartialQuery { query })
    }

    fn subscribe_slot_parsed(&self, handler: Callback<NluSlotMessage>) -> Result<()> {
        subscribe!(self, NluSlotParsed { slot }, handler)
    }

    fn subscribe_intent_parsed(&self, handler: Callback<NluIntentMessage>) -> Result<()> {
        subscribe!(self, NluIntentParsed { intent }, handler)
    }

    fn subscribe_intent_not_recognized(
        &self,
        handler: Callback<NluIntentNotRecognizedMessage>,
    ) -> Result<()> {
        subscribe!(self, NluIntentNotRecognized { status }, handler)
    }
}

impl NluBackendFacade for InProcessComponent<Nlu> {
    fn subscribe_query(&self, handler: Callback<NluQueryMessage>) -> Result<()> {
        subscribe!(self, NluQuery { query }, handler)
    }

    fn subscribe_partial_query(&self, handler: Callback<NluSlotQueryMessage>) -> Result<()> {
        subscribe!(self, NluPartialQuery { query }, handler)
    }

    fn publish_slot_parsed(&self, slot: NluSlotMessage) -> Result<()> {
        self.publish(NluSlotParsed { slot })
    }

    fn publish_intent_parsed(&self, intent: NluIntentMessage) -> Result<()> {
        self.publish(NluIntentParsed { intent })
    }

    fn publish_intent_not_recognized(&self, status: NluIntentNotRecognizedMessage) -> Result<()> {
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
    fn publish_toggle_on(&self) -> Result<()> {
        let toggle_on: ToggleableToggleOn<T> = ToggleableToggleOn {
            component: self.component,
        };
        self.publish(toggle_on)
    }
    fn publish_toggle_off(&self) -> Result<()> {
        let toggle_off: ToggleableToggleOff<T> = ToggleableToggleOff {
            component: self.component,
        };
        self.publish(toggle_off)
    }
}

impl<T: Send + Sync + Debug + 'static> ToggleableBackendFacade for InProcessComponent<T> {
    fn subscribe_toggle_on(&self, handler: Callback0) -> Result<()> {
        subscribe!(self, ToggleableToggleOn<T>, handler)
    }
    fn subscribe_toggle_off(&self, handler: Callback0) -> Result<()> {
        subscribe!(self, ToggleableToggleOff<T>, handler)
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
    fn subscribe_detected(
        &self,
        id: String,
        handler: Callback<HotwordDetectedMessage>,
    ) -> Result<()> {
        subscribe_filter!(self, HotwordDetected { message }, handler, id, |it| {
            &it.id
        })
    }

    fn subscribe_all_detected(&self, handler: Callback<HotwordDetectedMessage>) -> Result<()> {
        subscribe!(self, HotwordDetected { message }, handler)
    }
}

impl HotwordBackendFacade for InProcessComponent<Hotword> {
    fn publish_detected(&self, id: String, message: HotwordDetectedMessage) -> Result<()> {
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
    site: SiteMessage,
}

#[derive(Debug)]
struct AsrStopListening {
    site: SiteMessage,
}

#[derive(Debug)]
struct AsrReload {
}

#[derive(Debug)]
struct AsrTextCaptured {
    text_captured: TextCapturedMessage,
}

#[derive(Debug)]
struct AsrPartialTextCaptured {
    text_captured: TextCapturedMessage,
}

impl AsrFacade for InProcessComponent<Asr> {
    fn publish_start_listening(&self, site: SiteMessage) -> Result<()> {
        self.publish(AsrStartListening { site })
    }

    fn publish_stop_listening(&self, site: SiteMessage) -> Result<()> {
        self.publish(AsrStopListening { site })
    }

    fn publish_reload(&self) -> Result<()> {
        self.publish(AsrReload {})
    }

    fn subscribe_text_captured(&self, handler: Callback<TextCapturedMessage>) -> Result<()> {
        subscribe!(self, AsrTextCaptured { text_captured }, handler)
    }

    fn subscribe_partial_text_captured(
        &self,
        handler: Callback<TextCapturedMessage>,
    ) -> Result<()> {
        subscribe!(self, AsrPartialTextCaptured { text_captured }, handler)
    }
}

impl AsrBackendFacade for InProcessComponent<Asr> {
    fn subscribe_start_listening(&self, handler: Callback<SiteMessage>) -> Result<()> {
        subscribe!(self, AsrStartListening { site }, handler)
    }

    fn subscribe_stop_listening(&self, handler: Callback<SiteMessage>) -> Result<()> {
        subscribe!(self, AsrStopListening { site }, handler)
    }

    fn subscribe_reload(&self, handler: Callback0) -> Result<()> {
        subscribe!(self, AsrReload, handler)
    }

    fn publish_text_captured(&self, text_captured: TextCapturedMessage) -> Result<()> {
        self.publish(AsrTextCaptured { text_captured })
    }

    fn publish_partial_text_captured(&self, text_captured: TextCapturedMessage) -> Result<()> {
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

impl TtsFacade for InProcessComponent<Tts> {
    fn publish_say(&self, to_say: SayMessage) -> Result<()> {
        self.publish(TtsSay { to_say })
    }

    fn subscribe_say_finished(&self, handler: Callback<SayFinishedMessage>) -> Result<()> {
        subscribe!(self, TtsSayFinished { status }, handler)
    }
}

impl TtsBackendFacade for InProcessComponent<Tts> {
    fn publish_say_finished(&self, status: SayFinishedMessage) -> Result<()> {
        self.publish(TtsSayFinished { status })
    }

    fn subscribe_say(&self, handler: Callback<SayMessage>) -> Result<()> {
        subscribe!(self, TtsSay { to_say }, handler)
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

impl AudioServerFacade for InProcessComponent<AudioServer> {
    fn publish_play_bytes(&self, bytes: PlayBytesMessage) -> Result<()> {
        self.publish(AudioServerPlayBytes { bytes })
    }

    fn subscribe_play_finished(
        &self,
        site_id: SiteId,
        handler: Callback<PlayFinishedMessage>,
    ) -> Result<()> {
        subscribe_filter!(self, AudioServerPlayFinished { status }, handler, site_id)
    }

    fn subscribe_all_play_finished(&self, handler: Callback<PlayFinishedMessage>) -> Result<()> {
        subscribe!(self, AudioServerPlayFinished { status }, handler)
    }

    fn subscribe_audio_frame(
        &self,
        site_id: SiteId,
        handler: Callback<AudioFrameMessage>,
    ) -> Result<()> {
        subscribe_filter!(self, AudioServerAudioFrame { frame }, handler, site_id)
    }
}

impl AudioServerBackendFacade for InProcessComponent<AudioServer> {
    fn subscribe_play_bytes(
        &self,
        site_id: SiteId,
        handler: Callback<PlayBytesMessage>,
    ) -> Result<()> {
        subscribe_filter!(self, AudioServerPlayBytes { bytes }, handler, site_id)
    }

    fn subscribe_all_play_bytes(&self, handler: Callback<PlayBytesMessage>) -> Result<()> {
        subscribe!(self, AudioServerPlayBytes { bytes }, handler)
    }

    fn publish_play_finished(&self, status: PlayFinishedMessage) -> Result<()> {
        self.publish(AudioServerPlayFinished { status })
    }

    fn publish_audio_frame(&self, frame: AudioFrameMessage) -> Result<()> {
        self.publish(AudioServerAudioFrame { frame })
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

impl DialogueFacade for InProcessComponent<Dialogue> {
    fn subscribe_session_queued(&self, handler: Callback<SessionQueuedMessage>) -> Result<()> {
        subscribe!(self, DialogueSessionQueued { status }, handler)
    }

    fn subscribe_session_started(&self, handler: Callback<SessionStartedMessage>) -> Result<()> {
        subscribe!(self, DialogueSessionStarted { status }, handler)
    }

    fn subscribe_intent(
        &self,
        intent_name: String,
        handler: Callback<IntentMessage>,
    ) -> Result<()> {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        subscribe_filter!(self, DialogueIntent { intent }, handler, intent_name, |it| { &it.intent.intent.intent_name })
    }

    fn subscribe_intents(&self, handler: Callback<IntentMessage>) -> Result<()> {
        subscribe!(self, DialogueIntent { intent }, handler)
    }

    fn subscribe_session_ended(&self, handler: Callback<SessionEndedMessage>) -> Result<()> {
        subscribe!(self, DialogueSessionEnded { status }, handler)
    }

    fn publish_start_session(&self, start_session: StartSessionMessage) -> Result<()> {
        self.publish(DialogueStartSession { start_session })
    }

    fn publish_continue_session(&self, continue_session: ContinueSessionMessage) -> Result<()> {
        self.publish(DialogueContinueSession { continue_session })
    }

    fn publish_end_session(&self, end_session: EndSessionMessage) -> Result<()> {
        self.publish(DialogueEndSession { end_session })
    }
}

impl DialogueBackendFacade for InProcessComponent<Dialogue> {
    fn publish_session_queued(&self, status: SessionQueuedMessage) -> Result<()> {
        self.publish(DialogueSessionQueued { status })
    }

    fn publish_session_started(&self, status: SessionStartedMessage) -> Result<()> {
        self.publish(DialogueSessionStarted { status })
    }

    fn publish_intent(&self, intent: IntentMessage) -> Result<()> {
        self.publish(DialogueIntent { intent })
    }

    fn publish_session_ended(&self, status: SessionEndedMessage) -> Result<()> {
        self.publish(DialogueSessionEnded { status })
    }

    fn subscribe_start_session(&self, handler: Callback<StartSessionMessage>) -> Result<()> {
        subscribe!(self, DialogueStartSession { start_session }, handler)
    }

    fn subscribe_continue_session(&self, handler: Callback<ContinueSessionMessage>) -> Result<()> {
        subscribe!(self, DialogueContinueSession { continue_session }, handler)
    }

    fn subscribe_end_session(&self, handler: Callback<EndSessionMessage>) -> Result<()> {
        subscribe!(self, DialogueEndSession { end_session }, handler)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::rc::Rc;

    fn create_handlers() -> (
        Rc<InProcessHermesProtocolHandler>,
        Rc<InProcessHermesProtocolHandler>,
    ) {
        let handler = Rc::new(InProcessHermesProtocolHandler::new().unwrap());
        (Rc::clone(&handler), handler)
    }

    test_suite!();
}
