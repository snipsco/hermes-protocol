use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

use super::*;
use errors::*;

type IntentName = String;
type ComponentName = String;

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

    component_version_request: HashMap<ComponentName, Vec<Callback0>>,
    component_version: HashMap<ComponentName, Vec<Callback<VersionMessage>>>,
    component_error: HashMap<ComponentName, Vec<Callback<ErrorMessage>>>,
    component_version_empty: Vec<Callback<VersionMessage>>,
    component_error_empty: Vec<Callback<ErrorMessage>>,

    tts_say: Vec<Callback<SayMessage>>,
    tts_say_finished: Vec<Callback0>,

    toggle_on: HashMap<ComponentName, Vec<Callback0>>,
    toggle_off: HashMap<ComponentName, Vec<Callback0>>,

    intent: HashMap<IntentName, Vec<Callback<IntentMessage>>>,
    intents: Vec<Callback<IntentMessage>>,
    intent_empty: Vec<Callback<IntentMessage>>, // should always be empty

    empty_0: Vec<Callback0>, // should always be empty
}

// -

pub struct InProcessHermesProtocolHandler {
    handler: Arc<Mutex<Handler>>,
}

impl InProcessHermesProtocolHandler {
    pub fn new() -> Result<Self> {
        Ok(Self {
            handler: Arc::new(Mutex::new(Handler::default())),
        })
    }

    fn get_handler(&self, name: &str) -> Box<InProcessComponent> {
        Box::new(InProcessComponent {
            name: name.to_string(),
            handler: Arc::clone(&self.handler),
        })
    }
}

impl HermesProtocolHandler for InProcessHermesProtocolHandler {
    fn asr(&self) -> Box<AsrFacade> {
        self.get_handler("asr")
    }
    fn asr_backend(&self) -> Box<AsrBackendFacade> {
        self.get_handler("asr")
    }
    fn audio_server(&self) -> Box<AudioServerFacade> {
        self.get_handler("audio_server")
    }
    fn audio_server_backend(&self) -> Box<AudioServerBackendFacade> {
        self.get_handler("audio_server")
    }
    fn hotword(&self) -> Box<HotwordFacade> {
        self.get_handler("hotword")
    }
    fn hotword_backend(&self) -> Box<HotwordBackendFacade> {
        self.get_handler("hotword")
    }
    fn dialogue(&self) -> Box<DialogueFacade> {
        self.get_handler("dialogue")
    }
    fn dialogue_backend(&self) -> Box<DialogueBackendFacade> {
        self.get_handler("dialogue")
    }
    fn nlu(&self) -> Box<NluFacade> {
        self.get_handler("nlu")
    }
    fn nlu_backend(&self) -> Box<NluBackendFacade> {
        self.get_handler("nlu")
    }
    fn sound_feedback(&self) -> Box<SoundFeedbackFacade> {
        self.get_handler("sound_feedback")
    }
    fn sound_feedback_backend(&self) -> Box<SoundFeedbackBackendFacade> {
        self.get_handler("sound_feedback")
    }
    fn tts(&self) -> Box<TtsFacade> {
        self.get_handler("tts")
    }
    fn tts_backend(&self) -> Box<TtsBackendFacade> {
        self.get_handler("tts")
    }
}

// -

struct InProcessComponent {
    name: String,
    handler: Arc<Mutex<Handler>>,
}

impl InProcessComponent {
    fn publish<F>(&self, callback_name: &str, retrieve_callbacks: F) -> Result<()>
    where
        F: FnOnce(&Handler) -> &Vec<Callback0> + Send + 'static,
    {
        debug!("Publishing on '{}/{}'", self.name, callback_name);
        let _handler = Arc::clone(&self.handler);

        thread::spawn(move || {
            let result = _handler
                .lock()
                .map(|ref h| for callback in retrieve_callbacks(h) {
                    callback.call();
                });
            if let Err(e) = result {
                error!("Error while publishing an event : {}", e)
            }
        });
        Ok(())
    }

    fn publish_payload<F, M>(
        &self,
        callback_name: &str,
        retrieve_callbacks: F,
        message: M,
    ) -> Result<()>
    where
        F: FnOnce(&Handler) -> &Vec<Callback<M>> + Send + 'static,
        M: HermesMessage + Send + 'static,
    {
        debug!(
            "Publishing on '{}/{}' :\n{:#?}",
            self.name,
            callback_name,
            &message
        );
        let _handler = Arc::clone(&self.handler);

        thread::spawn(move || {
            let result = _handler
                .lock()
                .map(|ref h| for callback in retrieve_callbacks(h) {
                    callback.call(&message);
                });
            if let Err(e) = result {
                error!(
                    "Error while publishing an event with payload: {:#?} : {}",
                    &message,
                    e
                )
            }
        });
        Ok(())
    }

    fn subscribe<F>(
        &self,
        callback_name: &str,
        retrieve_callbacks: F,
        callback: Callback0,
    ) -> Result<()>
    where
        F: FnOnce(&mut Handler) -> &mut Vec<Callback0> + Send + 'static,
    {
        debug!("Subscribing on '{}/{}'", self.name, callback_name);
        Ok(self.handler
            .lock()
            .map(|mut h| retrieve_callbacks(&mut h).push(callback))?)
    }

    fn subscribe_payload<F, M>(
        &self,
        callback_name: &str,
        retrieve_callbacks: F,
        callback: Callback<M>,
    ) -> Result<()>
    where
        F: FnOnce(&mut Handler) -> &mut Vec<Callback<M>> + Send + 'static,
        M: HermesMessage,
    {
        debug!("Subscribing on '{}/{}'", self.name, callback_name);
        Ok(self.handler
            .lock()
            .map(|mut h| retrieve_callbacks(&mut h).push(callback))?)
    }
}

impl ComponentFacade for InProcessComponent {
    fn publish_version_request(&self) -> Result<()> {
        let component_name = self.name.to_string();
        self.publish("component_version_request", move |h| {
            &h.component_version_request
                .get(&component_name)
                .unwrap_or(&h.empty_0)
        })
    }
    fn subscribe_version(&self, handler: Callback<VersionMessage>) -> Result<()> {
        let component_name = self.name.to_string();
        self.subscribe_payload(
            "component_version",
            |h| {
                h.component_version
                    .entry(component_name)
                    .or_insert_with(|| vec![])
            },
            handler,
        )
    }
    fn subscribe_error(&self, handler: Callback<ErrorMessage>) -> Result<()> {
        let component_name = self.name.to_string();
        self.subscribe_payload(
            "component_error",
            |h| {
                h.component_error
                    .entry(component_name)
                    .or_insert_with(|| vec![])
            },
            handler,
        )
    }
}

impl ComponentBackendFacade for InProcessComponent {
    fn subscribe_version_request(&self, handler: Callback0) -> Result<()> {
        let component_name = self.name.to_string();
        self.subscribe(
            "component_version_request",
            |h| {
                h.component_version_request
                    .entry(component_name)
                    .or_insert_with(|| vec![])
            },
            handler,
        )
    }
    fn publish_version(&self, version: VersionMessage) -> Result<()> {
        let component_name = self.name.to_string();
        self.publish_payload(
            "component_version",
            move |h| {
                h.component_version
                    .get(&component_name)
                    .unwrap_or(&h.component_version_empty)
            },
            version,
        )
    }
    fn publish_error(&self, error: ErrorMessage) -> Result<()> {
        let component_name = self.name.to_string();
        self.publish_payload(
            "component_error",
            move |h| {
                h.component_error
                    .get(&component_name)
                    .unwrap_or(&h.component_error_empty)
            },
            error,
        )
    }
}

impl NluFacade for InProcessComponent {
    fn publish_query(&self, query: NluQueryMessage) -> Result<()> {
        self.publish_payload("nlu_query", |h| &h.nlu_query, query)
    }
    fn publish_partial_query(&self, query: NluSlotQueryMessage) -> Result<()> {
        self.publish_payload("nlu_partial_query", |h| &h.nlu_partial_query, query)
    }
    fn subscribe_slot_parsed(&self, handler: Callback<SlotMessage>) -> Result<()> {
        self.subscribe_payload("nlu_slot_parsed", |h| &mut h.nlu_slot_parsed, handler)
    }
    fn subscribe_intent_parsed(&self, handler: Callback<IntentMessage>) -> Result<()> {
        self.subscribe_payload("nlu_intent_parsed", |h| &mut h.nlu_intent_parsed, handler)
    }
    fn subscribe_intent_not_recognized(
        &self,
        handler: Callback<IntentNotRecognizedMessage>,
    ) -> Result<()> {
        self.subscribe_payload(
            "nlu_intent_not_recognized",
            |h| &mut h.nlu_intent_not_recognized,
            handler,
        )
    }
}

impl NluBackendFacade for InProcessComponent {
    fn subscribe_query(&self, handler: Callback<NluQueryMessage>) -> Result<()> {
        self.subscribe_payload("nlu_query", |h| &mut h.nlu_query, handler)
    }
    fn subscribe_partial_query(&self, handler: Callback<NluSlotQueryMessage>) -> Result<()> {
        self.subscribe_payload("nlu_partial_query", |h| &mut h.nlu_partial_query, handler)
    }
    fn publish_slot_parsed(&self, slot: SlotMessage) -> Result<()> {
        self.publish_payload("nlu_slot_parsed", |h| &h.nlu_slot_parsed, slot)
    }
    fn publish_intent_parsed(&self, intent: IntentMessage) -> Result<()> {
        self.publish_payload("nlu_intent_parsed", |h| &h.nlu_intent_parsed, intent)
    }
    fn publish_intent_not_recognized(&self, status: IntentNotRecognizedMessage) -> Result<()> {
        self.publish_payload(
            "nlu_intent_not_recognized",
            |h| &h.nlu_intent_not_recognized,
            status,
        )
    }
}

impl ToggleableFacade for InProcessComponent {
    fn publish_toggle_on(&self) -> Result<()> {
        let component_name = self.name.to_string();
        self.publish("toggle_on", move |h| {
            &h.toggle_on.get(&component_name).unwrap_or(&h.empty_0)
        })
    }
    fn publish_toggle_off(&self) -> Result<()> {
        let component_name = self.name.to_string();
        self.publish("toggle_off", move |h| {
            &h.toggle_off.get(&component_name).unwrap_or(&h.empty_0)
        })
    }
}

impl ToggleableBackendFacade for InProcessComponent {
    fn subscribe_toggle_on(&self, handler: Callback0) -> Result<()> {
        let component_name = self.name.to_string();
        self.subscribe(
            "toggle_on",
            |h| h.toggle_on.entry(component_name).or_insert_with(|| vec![]),
            handler,
        )
    }
    fn subscribe_toggle_off(&self, handler: Callback0) -> Result<()> {
        let component_name = self.name.to_string();
        self.subscribe(
            "toggle_off",
            |h| h.toggle_off.entry(component_name).or_insert_with(|| vec![]),
            handler,
        )
    }
}

impl HotwordFacade for InProcessComponent {
    fn publish_wait(&self) -> Result<()> {
        self.publish("hotword_wait", |h| &h.hotword_wait)
    }
    fn subscribe_detected(&self, handler: Callback0) -> Result<()> {
        self.subscribe("hotword_detected", |h| &mut h.hotword_detected, handler)
    }
}

impl HotwordBackendFacade for InProcessComponent {
    fn subscribe_wait(&self, handler: Callback0) -> Result<()> {
        self.subscribe("hotword_wait", |h| &mut h.hotword_wait, handler)
    }
    fn publish_detected(&self) -> Result<()> {
        self.publish("hotword_detected", |h| &h.hotword_detected)
    }
}

impl SoundFeedbackFacade for InProcessComponent {}

impl SoundFeedbackBackendFacade for InProcessComponent {}

impl AsrFacade for InProcessComponent {
    fn subscribe_text_captured(&self, handler: Callback<TextCapturedMessage>) -> Result<()> {
        self.subscribe_payload("asr_text_captured", |h| &mut h.asr_text_captured, handler)
    }
    fn subscribe_partial_text_captured(
        &self,
        handler: Callback<TextCapturedMessage>,
    ) -> Result<()> {
        self.subscribe_payload(
            "asr_partial_text_captured",
            |h| &mut h.asr_partial_text_captured,
            handler,
        )
    }
}

impl AsrBackendFacade for InProcessComponent {
    fn publish_text_captured(&self, text_captured: TextCapturedMessage) -> Result<()> {
        self.publish_payload("asr_text_captured", |h| &h.asr_text_captured, text_captured)
    }
    fn publish_partial_text_captured(&self, text_captured: TextCapturedMessage) -> Result<()> {
        self.publish_payload(
            "asr_partial_text_captured",
            |h| &h.asr_partial_text_captured,
            text_captured,
        )
    }
}

impl TtsFacade for InProcessComponent {
    fn publish_say(&self, to_say: SayMessage) -> Result<()> {
        self.publish_payload("tts_say", |h| &h.tts_say, to_say)
    }
    fn subscribe_say_finished(&self, handler: Callback0) -> Result<()> {
        self.subscribe("tts_say_finished", |h| &mut h.tts_say_finished, handler)
    }
}

impl TtsBackendFacade for InProcessComponent {
    fn subscribe_say(&self, handler: Callback<SayMessage>) -> Result<()> {
        self.subscribe_payload("tts_say", |h| &mut h.tts_say, handler)
    }
    fn publish_say_finished(&self) -> Result<()> {
        self.publish("tts_say_finished", |h| &h.tts_say_finished)
    }
}

impl AudioServerFacade for InProcessComponent {
    fn publish_play_file(&self, file: PlayFileMessage) -> Result<()> {
        self.publish_payload("as_play_file", |h| &h.as_play_file, file)
    }
    fn publish_play_bytes(&self, bytes: PlayBytesMessage) -> Result<()> {
        self.publish_payload("as_play_bytes", |h| &h.as_play_bytes, bytes)
    }
    fn subscribe_play_finished(&self, handler: Callback<PlayFinishedMessage>) -> Result<()> {
        self.subscribe_payload("as_play_finished", |h| &mut h.as_play_finished, handler)
    }
}

impl AudioServerBackendFacade for InProcessComponent {
    fn subscribe_play_file(&self, handler: Callback<PlayFileMessage>) -> Result<()> {
        self.subscribe_payload("as_play_file", |h| &mut h.as_play_file, handler)
    }
    fn subscribe_play_bytes(&self, handler: Callback<PlayBytesMessage>) -> Result<()> {
        self.subscribe_payload("as_play_bytes", |h| &mut h.as_play_bytes, handler)
    }
    fn publish_play_finished(&self, status: PlayFinishedMessage) -> Result<()> {
        self.publish_payload("as_play_finished", |h| &h.as_play_finished, status)
    }
}

impl DialogueFacade for InProcessComponent {
    fn subscribe_intent(
        &self,
        intent_name: String,
        handler: Callback<IntentMessage>,
    ) -> Result<()> {
        self.subscribe_payload(
            &format!("intent_{}", intent_name),
            |h| h.intent.entry(intent_name).or_insert_with(|| vec![]),
            handler,
        )
    }

    fn subscribe_intents(
        &self,
        handler: Callback<IntentMessage>,
    ) -> Result<()> {
        self.subscribe_payload(
            "intents",
            |h| &mut h.intents,
            handler,
        )
    }
}

impl DialogueBackendFacade for InProcessComponent {
    fn publish_intent(&self, intent: IntentMessage) -> Result<()> {
        let intent_name = intent.intent.intent_name.to_string();

        let _intent = intent.clone();

        self.publish_payload(
            &format!("intent_{}", &intent_name),
            move |h| h.intent.get(&intent_name).unwrap_or(&h.intent_empty),
            _intent,
        )?;

        self.publish_payload("intents", move |h| &h.intents, intent)
    }
}
