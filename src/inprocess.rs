use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

use super::*;
use errors::*;

// **facepalm** How did I accept to write this ?

type IntentName = String;

#[derive(Default)]
struct Handler {
    asr_text_captured: Vec<Callback<TextCapturedMessage>>,
    asr_partial_text_captured: Vec<Callback<TextCapturedMessage>>,

    as_play_file: Vec<Callback<PlayFileMessage>>,
    as_play_bytes: Vec<Callback<PlayBytesMessage>>,
    as_play_finished: Vec<Callback<PlayFinishedMessage>>,

    hotword_wait: Vec<Callback0>,
    hotword_detected: Vec<Callback0>,

    nlu_query: Vec<Callback<NluQueryMessage>>,
    nlu_partial_query: Vec<Callback<NluSlotQueryMessage>>,
    nlu_slot_parsed: Vec<Callback<SlotMessage>>,
    nlu_intent_parsed: Vec<Callback<IntentMessage>>,
    nlu_intent_not_recognized: Vec<Callback<IntentNotRecognizedMessage>>,

    component_version_request: Vec<Callback0>,
    component_version: Vec<Callback<VersionMessage>>,
    component_error: Vec<Callback<ErrorMessage>>,

    tts_say: Vec<Callback<SayMessage>>,
    tts_say_finished: Vec<Callback0>,

    toggle_on: Vec<Callback0>,
    toggle_off: Vec<Callback0>,

    intent: HashMap<IntentName, Vec<Callback<IntentMessage>>>,
    empty: Vec<Callback<IntentMessage>>,
}

// -

pub struct InProcessHermesProtocolHandler {
    handler: Arc<Mutex<Handler>>,
}

impl InProcessHermesProtocolHandler {
    pub fn new() -> Result<Self> {
        Ok(Self { handler: Arc::new(Mutex::new(Handler::default())) })
    }

    fn get_handler(&self) -> Box<InProcessComponent> {
        Box::new(InProcessComponent { handler: Arc::clone(&self.handler) })
    }
}

impl HermesProtocolHandler for InProcessHermesProtocolHandler {
    fn asr(&self) -> Box<AsrFacade> { self.get_handler() }
    fn asr_backend(&self) -> Box<AsrBackendFacade> { self.get_handler() }
    fn audio_server(&self) -> Box<AudioServerFacade> { self.get_handler() }
    fn audio_server_backend(&self) -> Box<AudioServerBackendFacade> { self.get_handler() }
    fn hotword(&self) -> Box<HotwordFacade> { self.get_handler() }
    fn hotword_backend(&self) -> Box<HotwordBackendFacade> { self.get_handler() }
    fn intent(&self) -> Box<IntentFacade> { self.get_handler() }
    fn intent_backend(&self) -> Box<IntentBackendFacade> { self.get_handler() }
    fn nlu(&self) -> Box<NluFacade> { self.get_handler() }
    fn nlu_backend(&self) -> Box<NluBackendFacade> { self.get_handler() }
    fn sound_feedback(&self) -> Box<SoundFeedbackFacade> { self.get_handler() }
    fn sound_feedback_backend(&self) -> Box<SoundFeedbackBackendFacade> { self.get_handler() }
    fn tts(&self) -> Box<TtsFacade> { self.get_handler() }
    fn tts_backend(&self) -> Box<TtsBackendFacade> { self.get_handler() }
}

// -

struct InProcessComponent {
    handler: Arc<Mutex<Handler>>,
}

impl InProcessComponent {
    fn publish<F>(&self, retrieve_callbacks: F) -> Result<()>
        where F: FnOnce(&Handler) -> &Vec<Callback0> + Send + 'static
    {
        let _handler = Arc::clone(&self.handler);

        thread::spawn(move || {
            let result = _handler.lock().map(|ref h| {
                for callback in retrieve_callbacks(h) {
                    callback.call();
                }
            });
            if let Err(e) = result {
                error!("Error while publishing an event : {}", e)
            }
        });
        Ok(())
    }

    fn publish_payload<F, M>(&self, retrieve_callbacks: F, message: M) -> Result<()>
        where F: FnOnce(&Handler) -> &Vec<Callback<M>> + Send + 'static,
              M: HermesMessage + Send + 'static
    {
        let _handler = Arc::clone(&self.handler);

        thread::spawn(move || {
            let result = _handler.lock().map(|ref h| {
                debug!("Publishing payload: {:#?}", &message);
                for callback in retrieve_callbacks(h) {
                    callback.call(&message);
                }
            });
            if let Err(e) = result {
                error!("Error while publishing an event with payload: {:#?} : {}", &message, e)
            }
        });
        Ok(())
    }

    fn subscribe<F>(&self, retrieve_callbacks: F, callback: Callback0) -> Result<()>
        where F: FnOnce(&mut Handler) -> &mut Vec<Callback0> + Send + 'static
    {
        Ok(self.handler.lock().map(|mut h| retrieve_callbacks(&mut h).push(callback) )?)
    }

    fn subscribe_payload<F, M>(&self, retrieve_callbacks: F, callback: Callback<M>) -> Result<()>
        where F: FnOnce(&mut Handler) -> &mut Vec<Callback<M>> + Send + 'static,
              M: HermesMessage
    {
        Ok(self.handler.lock().map(|mut h| retrieve_callbacks(&mut h).push(callback) )?)
    }
}

impl ComponentFacade for InProcessComponent {
    fn publish_version_request(&self) -> Result<()> {
        self.publish(|h| &h.component_version_request)
    }
    fn subscribe_version(&self, handler: Callback<VersionMessage>) -> Result<()> {
        self.subscribe_payload(|h| &mut h.component_version, handler)
    }
    fn subscribe_error(&self, handler: Callback<ErrorMessage>) -> Result<()> {
        self.subscribe_payload(|h| &mut h.component_error, handler)
    }
}

impl ComponentBackendFacade for InProcessComponent {
    fn subscribe_version_request(&self, handler: Callback0) -> Result<()> {
        self.subscribe(|h| &mut h.component_version_request, handler)
    }
    fn publish_version(&self, version: VersionMessage) -> Result<()> {
        self.publish_payload(|h| &h.component_version, version)
    }
    fn publish_error(&self, error: ErrorMessage) -> Result<()> {
        self.publish_payload(|h| &h.component_error, error)
    }
}

impl NluFacade for InProcessComponent {
    fn publish_query(&self, query: NluQueryMessage) -> Result<()> {
        self.publish_payload(|h| &h.nlu_query, query)
    }
    fn publish_partial_query(&self, query: NluSlotQueryMessage) -> Result<()> {
        self.publish_payload(|h| &h.nlu_partial_query, query)
    }
    fn subscribe_slot_parsed(&self, handler: Callback<SlotMessage>) -> Result<()> {
        self.subscribe_payload(|h| &mut h.nlu_slot_parsed, handler)
    }
    fn subscribe_intent_parsed(&self, handler: Callback<IntentMessage>) -> Result<()> {
        self.subscribe_payload(|h| &mut h.nlu_intent_parsed, handler)
    }
    fn subscribe_intent_not_recognized(&self, handler: Callback<IntentNotRecognizedMessage>) -> Result<()> {
        self.subscribe_payload(|h| &mut h.nlu_intent_not_recognized, handler)
    }
}

impl NluBackendFacade for InProcessComponent {
    fn subscribe_query(&self, handler: Callback<NluQueryMessage>) -> Result<()> {
        self.subscribe_payload(|h| &mut h.nlu_query, handler)
    }
    fn subscribe_partial_query(&self, handler: Callback<NluSlotQueryMessage>) -> Result<()> {
        self.subscribe_payload(|h| &mut h.nlu_partial_query, handler)
    }
    fn publish_slot_parsed(&self, slot: SlotMessage) -> Result<()> {
        self.publish_payload(|h| &h.nlu_slot_parsed, slot)
    }
    fn publish_intent_parsed(&self, intent: IntentMessage) -> Result<()> {
        self.publish_payload(|h| &h.nlu_intent_parsed, intent)
    }
    fn publish_intent_not_recognized(&self, status: IntentNotRecognizedMessage) -> Result<()> {
        self.publish_payload(|h| &h.nlu_intent_not_recognized, status)
    }
}

impl ToggleableFacade for InProcessComponent {
    fn publish_toggle_on(&self) -> Result<()> {
        self.publish(|h| &h.toggle_on)
    }
    fn publish_toggle_off(&self) -> Result<()> {
        self.publish(|h| &h.toggle_off)
    }
}

impl ToggleableBackendFacade for InProcessComponent {
    fn subscribe_toggle_on(&self, handler: Callback0) -> Result<()> {
        self.subscribe(|h| &mut h.toggle_on, handler)
    }
    fn subscribe_toggle_off(&self, handler: Callback0) -> Result<()> {
        self.subscribe(|h| &mut h.toggle_off, handler)
    }
}

impl HotwordFacade for InProcessComponent {
    fn publish_wait(&self) -> Result<()> {
        self.publish(|h| &h.hotword_wait)
    }
    fn subscribe_detected(&self, handler: Callback0) -> Result<()> {
        self.subscribe(|h| &mut h.hotword_detected, handler)
    }
}

impl HotwordBackendFacade for InProcessComponent {
    fn subscribe_wait(&self, handler: Callback0) -> Result<()> {
        self.subscribe(|h| &mut h.hotword_wait, handler)
    }
    fn publish_detected(&self) -> Result<()> {
        self.publish(|h| &h.hotword_detected)
    }
}

impl SoundFeedbackFacade for InProcessComponent {}

impl SoundFeedbackBackendFacade for InProcessComponent {}

impl AsrFacade for InProcessComponent {
    fn subscribe_text_captured(&self, handler: Callback<TextCapturedMessage>) -> Result<()> {
        self.subscribe_payload(|h| &mut h.asr_text_captured, handler)
    }
    fn subscribe_partial_text_captured(&self, handler: Callback<TextCapturedMessage>) -> Result<()> {
        self.subscribe_payload(|h| &mut h.asr_partial_text_captured, handler)
    }
}

impl AsrBackendFacade for InProcessComponent {
    fn publish_text_captured(&self, text_captured: TextCapturedMessage) -> Result<()> {
        self.publish_payload(|h| &h.asr_text_captured, text_captured)
    }
    fn publish_partial_text_captured(&self, text_captured: TextCapturedMessage) -> Result<()> {
        self.publish_payload(|h| &h.asr_partial_text_captured, text_captured)
    }
}

impl TtsFacade for InProcessComponent {
    fn publish_say(&self, to_say: SayMessage) -> Result<()> {
        self.publish_payload(|h| &h.tts_say, to_say)
    }
    fn subscribe_say_finished(&self, handler: Callback0) -> Result<()> {
        self.subscribe(|h| &mut h.tts_say_finished, handler)
    }
}

impl TtsBackendFacade for InProcessComponent {
    fn subscribe_say(&self, handler: Callback<SayMessage>) -> Result<()> {
        self.subscribe_payload(|h| &mut h.tts_say, handler)
    }
    fn publish_say_finished(&self) -> Result<()> {
        self.publish(|h| &h.tts_say_finished)
    }
}

impl AudioServerFacade for InProcessComponent {
    fn publish_play_file(&self, file: PlayFileMessage) -> Result<()> {
        self.publish_payload(|h| &h.as_play_file, file)
    }
    fn publish_play_bytes(&self, bytes: PlayBytesMessage) -> Result<()> {
        self.publish_payload(|h| &h.as_play_bytes, bytes)
    }
    fn subscribe_play_finished(&self, handler: Callback<PlayFinishedMessage>) -> Result<()> {
        self.subscribe_payload(|h| &mut h.as_play_finished, handler)
    }
}

impl AudioServerBackendFacade for InProcessComponent {
    fn subscribe_play_bytes(&self, handler: Callback<PlayBytesMessage>) -> Result<()> {
        self.subscribe_payload(|h| &mut h.as_play_bytes, handler)
    }
    fn subscribe_play_file(&self, handler: Callback<PlayFileMessage>) -> Result<()> {
        self.subscribe_payload(|h| &mut h.as_play_file, handler)
    }
    fn publish_play_finished(&self, status: PlayFinishedMessage) -> Result<()> {
        self.publish_payload(|h| &h.as_play_finished, status)
    }
}

impl IntentFacade for InProcessComponent {
    fn subscribe_intent(&self, intent_name: String, handler: Callback<IntentMessage>) -> Result<()> {
        self.subscribe_payload(|h| h.intent.entry(intent_name).or_insert_with(|| vec![]), handler)
    }
}

impl IntentBackendFacade for InProcessComponent {
    fn publish_intent(&self, intent: IntentMessage) -> Result<()> {
        let intent_name = intent.intent.intent_name.to_string();
        self.publish_payload(move |h| h.intent.get(&intent_name).unwrap_or(&h.empty), intent)
    }
}
