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

    as_play_bytes: Vec<Callback<PlayBytesMessage>>,
    as_play_finished: Vec<Callback<PlayFinishedMessage>>,

    hotword_detected: Vec<Callback<SiteMessage>>,

    nlu_query: Vec<Callback<NluQueryMessage>>,
    nlu_partial_query: Vec<Callback<NluSlotQueryMessage>>,
    nlu_slot_parsed: Vec<Callback<SlotMessage>>,
    nlu_intent_parsed: Vec<Callback<NluIntentMessage>>,
    nlu_intent_not_recognized: Vec<Callback<NluIntentNotRecognizedMessage>>,

    component_version_request: HashMap<ComponentName, Vec<Callback0>>,
    component_version: HashMap<ComponentName, Vec<Callback<VersionMessage>>>,
    component_error: HashMap<ComponentName, Vec<Callback<ErrorMessage>>>,
    component_version_empty: Vec<Callback<VersionMessage>>,
    component_error_empty: Vec<Callback<ErrorMessage>>,

    tts_say: Vec<Callback<SayMessage>>,
    tts_say_finished: Vec<Callback<SayFinishedMessage>>,

    toggle_on: HashMap<ComponentName, Vec<Callback<SiteMessage>>>,
    toggle_off: HashMap<ComponentName, Vec<Callback<SiteMessage>>>,
    toggle_empty: Vec<Callback<SiteMessage>>,
    // should always be empty

    intent: HashMap<IntentName, Vec<Callback<IntentMessage>>>,
    intents: Vec<Callback<IntentMessage>>,
    intent_empty: Vec<Callback<IntentMessage>>,
    // should always be empty

    dialogue_start_session: Vec<Callback<StartSessionMessage>>,
    dialogue_continue_session: Vec<Callback<ContinueSessionMessage>>,
    dialogue_end_session: Vec<Callback<EndSessionMessage>>,
    dialogue_session_started: Vec<Callback<SessionStartedMessage>>,
    dialogue_session_ended: Vec<Callback<SessionEndedMessage>>,

    empty_0: Vec<Callback0>,
    // should always be empty
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

macro_rules! s {
     ($n:ident<$t:ty> $field:ident) => {
        fn $n(&self, handler : Callback<$t>) -> Result<()> {
            self.subscribe_payload(stringify!($field), |h| &mut h.$field, handler)
        }
    };
}

macro_rules! p {
    ($n:ident<$t:ty>  $field:ident) => {
        fn $n(&self, payload : $t) -> Result<()> {
            self.publish_payload(stringify!($field), |h| &h.$field, payload)
        }
    };
}

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
    p!(publish_query<NluQueryMessage> nlu_query);
    p!(publish_partial_query<NluSlotQueryMessage> nlu_partial_query);
    s!(subscribe_slot_parsed<SlotMessage> nlu_slot_parsed);
    s!(subscribe_intent_parsed<NluIntentMessage> nlu_intent_parsed); 
    s!(subscribe_intent_not_recognized<NluIntentNotRecognizedMessage> nlu_intent_not_recognized); 
}

impl NluBackendFacade for InProcessComponent {
    s!(subscribe_query<NluQueryMessage> nlu_query); 
    s!(subscribe_partial_query<NluSlotQueryMessage> nlu_partial_query); 
    p!(publish_slot_parsed<SlotMessage> nlu_slot_parsed);
    p!(publish_intent_parsed<NluIntentMessage> nlu_intent_parsed);
    p!(publish_intent_not_recognized<NluIntentNotRecognizedMessage> nlu_intent_not_recognized);
}

impl ToggleableFacade for InProcessComponent {
    fn publish_toggle_on(&self, site: SiteMessage) -> Result<()> {
        let component_name = self.name.to_string();
        self.publish_payload("toggle_on", move |h| {
            &h.toggle_on.get(&component_name).unwrap_or(&h.toggle_empty)
        }, site)
    }
    fn publish_toggle_off(&self, site: SiteMessage) -> Result<()> {
        let component_name = self.name.to_string();
        self.publish_payload("toggle_off", move |h| {
            &h.toggle_off.get(&component_name).unwrap_or(&h.toggle_empty)
        }, site)
    }
}

impl ToggleableBackendFacade for InProcessComponent {
    fn subscribe_toggle_on(&self, handler: Callback<SiteMessage>) -> Result<()> {
        let component_name = self.name.to_string();
        self.subscribe_payload(
            "toggle_on",
            |h| h.toggle_on.entry(component_name).or_insert_with(|| vec![]),
            handler,
        )
    }
    fn subscribe_toggle_off(&self, handler: Callback<SiteMessage>) -> Result<()> {
        let component_name = self.name.to_string();
        self.subscribe_payload(
            "toggle_off",
            |h| h.toggle_off.entry(component_name).or_insert_with(|| vec![]),
            handler,
        )
    }
}

impl HotwordFacade for InProcessComponent {
    s!(subscribe_detected<SiteMessage> hotword_detected); 
}

impl HotwordBackendFacade for InProcessComponent {
    p!(publish_detected<SiteMessage> hotword_detected);
}

impl SoundFeedbackFacade for InProcessComponent {}

impl SoundFeedbackBackendFacade for InProcessComponent {}

impl AsrFacade for InProcessComponent {
    s!(subscribe_text_captured<TextCapturedMessage> asr_text_captured); 
    s!(subscribe_partial_text_captured<TextCapturedMessage> asr_partial_text_captured); 
}

impl AsrBackendFacade for InProcessComponent {
    p!(publish_text_captured<TextCapturedMessage> asr_text_captured);
    p!(publish_partial_text_captured<TextCapturedMessage> asr_partial_text_captured);
}

impl TtsFacade for InProcessComponent {
    p!(publish_say<SayMessage> tts_say);
    s!(subscribe_say_finished<SayFinishedMessage> tts_say_finished); 
}

impl TtsBackendFacade for InProcessComponent {
    s!(subscribe_say<SayMessage> tts_say); 
    p!(publish_say_finished<SayFinishedMessage> tts_say_finished);
}

impl AudioServerFacade for InProcessComponent {
    p!(publish_play_bytes<PlayBytesMessage> as_play_bytes);
    s!(subscribe_play_finished<PlayFinishedMessage> as_play_finished);
}

impl AudioServerBackendFacade for InProcessComponent {
    s!(subscribe_play_bytes<PlayBytesMessage> as_play_bytes);
    p!(publish_play_finished<PlayFinishedMessage> as_play_finished);
}

impl DialogueFacade for InProcessComponent {
    s!(subscribe_session_started<SessionStartedMessage> dialogue_session_started);

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

    s!(subscribe_session_ended<SessionEndedMessage> dialogue_session_ended);
    p!(publish_start_session<StartSessionMessage> dialogue_start_session);
    p!(publish_continue_session<ContinueSessionMessage> dialogue_continue_session);
    p!(publish_end_session<EndSessionMessage> dialogue_end_session);
}

impl DialogueBackendFacade for InProcessComponent {
    p!(publish_session_started<SessionStartedMessage> dialogue_session_started);

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

    p!(publish_session_ended<SessionEndedMessage> dialogue_session_ended);
    s!(subscribe_start_session<StartSessionMessage> dialogue_start_session);
    s!(subscribe_continue_session<ContinueSessionMessage> dialogue_continue_session);
    s!(subscribe_end_session<EndSessionMessage> dialogue_end_session);
}
