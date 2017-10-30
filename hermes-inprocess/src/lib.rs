extern crate hermes;
#[cfg(test)]
#[macro_use]
extern crate hermes_test_suite;
#[macro_use]
extern crate log;
#[cfg(test)]
extern crate semver;
#[cfg(test)]
extern crate snips_queries_ontology;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

use hermes::*;

type IntentName = String;
type ComponentName = String;

#[derive(Default)]
struct Handler {
    asr_start_listening: Vec<Callback<SiteMessage>>,
    asr_stop_listening: Vec<Callback<SiteMessage>>,
    asr_text_captured: Vec<Callback<TextCapturedMessage>>,
    asr_partial_text_captured: Vec<Callback<TextCapturedMessage>>,

    as_play_bytes: HashMap<SiteId, Vec<Callback<PlayBytesMessage>>>,
    as_all_play_bytes: Vec<Callback<PlayBytesMessage>>,
    as_play_finished: HashMap<SiteId, Vec<Callback<PlayFinishedMessage>>>,
    as_all_play_finished: Vec<Callback<PlayFinishedMessage>>,
    as_audio_frame: HashMap<SiteId, Vec<Callback<AudioFrameMessage>>>,

    hotword_detected: HashMap<String, Vec<Callback<SiteMessage>>>,
    hotword_all_detected: Vec<Callback<SiteMessage>>,

    nlu_query: Vec<Callback<NluQueryMessage>>,
    nlu_partial_query: Vec<Callback<NluSlotQueryMessage>>,
    nlu_slot_parsed: Vec<Callback<NluSlotMessage>>,
    nlu_intent_parsed: Vec<Callback<NluIntentMessage>>,
    nlu_intent_not_recognized: Vec<Callback<NluIntentNotRecognizedMessage>>,

    component_version_request: HashMap<ComponentName, Vec<Callback0>>,
    component_version: HashMap<ComponentName, Vec<Callback<VersionMessage>>>,
    component_error: HashMap<ComponentName, Vec<Callback<ErrorMessage>>>,

    tts_say: Vec<Callback<SayMessage>>,
    tts_say_finished: Vec<Callback<SayFinishedMessage>>,

    toggle_on: HashMap<ComponentName, Vec<Callback<SiteMessage>>>,
    toggle_off: HashMap<ComponentName, Vec<Callback<SiteMessage>>>,
    toggle_on_0: Vec<Callback0>,
    toggle_off_0: Vec<Callback0>,

    intent: HashMap<IntentName, Vec<Callback<IntentMessage>>>,
    intents: Vec<Callback<IntentMessage>>,

    dialogue_start_session: Vec<Callback<StartSessionMessage>>,
    dialogue_continue_session: Vec<Callback<ContinueSessionMessage>>,
    dialogue_end_session: Vec<Callback<EndSessionMessage>>,
    dialogue_session_started: Vec<Callback<SessionStartedMessage>>,
    dialogue_session_ended: Vec<Callback<SessionEndedMessage>>,
    dialogue_session_queued: Vec<Callback<SessionQueuedMessage>>,

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
    ($n:ident<$t:ty>($($a:ident : $ta:ty),*)  { $field:ident[$key:expr;] } ) => {
        fn $n(&self, $($a : $ta),*, handler : Callback<$t>) -> Result<()> {
            let key = $key.to_string();
            self.subscribe_payload(&format!("{}[{}]", &stringify!($field), &key), |h| h.$field.entry(key).or_insert_with(|| vec![]), handler)
        }
    };
    ($n:ident<$t:ty> $field:ident) => {
        fn $n(&self, handler : Callback<$t>) -> Result<()> {
            self.subscribe_payload(stringify!($field), |h| &mut h.$field, handler)
        }
    };
}

macro_rules! p {
    ($n:ident<$t:ty> $field:ident) => {
        fn $n(&self, payload : $t) -> Result<()> {
            self.publish_payload(stringify!($field), |h| Some(&h.$field), payload)
        }
    };

    ($n:ident($payload:ident : $t:ty) $field:ident[$key:expr;] ) => {
        fn $n(&self, $payload : $t) -> Result<()> {
            let key = $key.to_string();
            self.publish_payload(&format!("{}[{}]", &stringify!($field), &key), move |h| h.$field.get(&key).map(|it| Some(it)).unwrap_or(None), $payload)
        }
    };

    ($n:ident($param1:ident : $t1:ty : $payload:ident : $t:ty) $field:ident[$key:expr;] ) => {
        fn $n(&self, $param1: $t1, $payload: $t) -> Result<()> {
            let key = $key.to_string();
            self.publish_payload(&format!("{}[{}]", &stringify!($field), &key), move |h| h.$field.get(&key).map(|it| Some(it)).unwrap_or(None), $payload)
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

    fn publish_payload<'de, F, M>(
        &self,
        callback_name: &str,
        retrieve_callbacks: F,
        message: M,
    ) -> Result<()>
        where
            F: FnOnce(&Handler) -> Option<&Vec<Callback<M>>> + Send + 'static,
            M: HermesMessage<'de> + Send + 'static,
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
                .map(|ref h| retrieve_callbacks(h)
                    .map(|callbacks| for callback in callbacks {
                        callback.call(&message);
                    }));
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

    fn subscribe_payload<'de, F, M>(
        &self,
        callback_name: &str,
        retrieve_callbacks: F,
        callback: Callback<M>,
    ) -> Result<()>
        where
            F: FnOnce(&mut Handler) -> &mut Vec<Callback<M>> + Send + 'static,
            M: HermesMessage<'de>,
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
                h.component_version.get(&component_name)
            },
            version,
        )
    }
    fn publish_error(&self, error: ErrorMessage) -> Result<()> {
        let component_name = self.name.to_string();
        self.publish_payload(
            "component_error",
            move |h| {
                h.component_error.get(&component_name)
            },
            error,
        )
    }
}

impl IdentifiableComponentFacade for InProcessComponent {
    fn publish_version_request(&self, site_id: SiteId) -> Result<()> {
        let entry = identifiable_entry(&self.name, &site_id);
        self.publish("component_version_request", move |h| {
            &h.component_version_request
                .get(&entry)
                .unwrap_or(&h.empty_0)
        })
    }
    fn subscribe_version(&self, site_id: SiteId, handler: Callback<VersionMessage>) -> Result<()> {
        let entry = identifiable_entry(&self.name, &site_id);
        self.subscribe_payload(
            "component_version",
            |h| {
                h.component_version
                    .entry(entry)
                    .or_insert_with(|| vec![])
            },
            handler,
        )
    }
    fn subscribe_error(&self, site_id: SiteId, handler: Callback<ErrorMessage>) -> Result<()> {
        let entry = identifiable_entry(&self.name, &site_id);
        self.subscribe_payload(
            "component_error",
            |h| {
                h.component_error
                    .entry(entry)
                    .or_insert_with(|| vec![])
            },
            handler,
        )
    }
}

impl IdentifiableComponentBackendFacade for InProcessComponent {
    fn subscribe_version_request(&self, site_id: SiteId, handler: Callback0) -> Result<()> {
        let entry = identifiable_entry(&self.name, &site_id);
        self.subscribe(
            "component_version_request",
            |h| {
                h.component_version_request
                    .entry(entry)
                    .or_insert_with(|| vec![])
            },
            handler,
        )
    }
    fn publish_version(&self, site_id: SiteId, version: VersionMessage) -> Result<()> {
        let entry = identifiable_entry(&self.name, &site_id);
        self.publish_payload(
            "component_version",
            move |h| {
                h.component_version.get(&entry)
            },
            version,
        )
    }
    fn publish_error(&self, site_id: SiteId, error: ErrorMessage) -> Result<()> {
        let entry = identifiable_entry(&self.name, &site_id);
        self.publish_payload(
            "component_error",
            move |h| { h.component_error.get(&entry) },
            error,
        )
    }
}

impl IdentifiableToggleableFacade for InProcessComponent {
    fn publish_toggle_on(&self, site: SiteMessage) -> Result<()> {
        let component_name = self.name.to_string();
        self.publish_payload("toggle_on", move |h| h.toggle_on.get(&component_name), site)
    }
    fn publish_toggle_off(&self, site: SiteMessage) -> Result<()> {
        let component_name = self.name.to_string();
        self.publish_payload("toggle_off", move |h| h.toggle_off.get(&component_name), site)
    }
}

impl IdentifiableToggleableBackendFacade for InProcessComponent {
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

fn identifiable_entry(component_name: &str, id: &str) -> String {
    format!("{}-{}", component_name, id)
}

impl NluFacade for InProcessComponent {
    p!(publish_query<NluQueryMessage> nlu_query);
    p!(publish_partial_query<NluSlotQueryMessage> nlu_partial_query);
    s!(subscribe_slot_parsed<NluSlotMessage> nlu_slot_parsed);
    s!(subscribe_intent_parsed<NluIntentMessage> nlu_intent_parsed);
    s!(subscribe_intent_not_recognized<NluIntentNotRecognizedMessage> nlu_intent_not_recognized);
}

impl NluBackendFacade for InProcessComponent {
    s!(subscribe_query<NluQueryMessage> nlu_query);
    s!(subscribe_partial_query<NluSlotQueryMessage> nlu_partial_query);
    p!(publish_slot_parsed<NluSlotMessage> nlu_slot_parsed);
    p!(publish_intent_parsed<NluIntentMessage> nlu_intent_parsed);
    p!(publish_intent_not_recognized<NluIntentNotRecognizedMessage> nlu_intent_not_recognized);
}

impl ToggleableFacade for InProcessComponent {
    fn publish_toggle_on(&self) -> Result<()> {
        self.publish("toggle_on", move |h| &h.toggle_on_0)
    }
    fn publish_toggle_off(&self) -> Result<()> {
        self.publish("toggle_off", move |h| &h.toggle_off_0)
    }
}

impl ToggleableBackendFacade for InProcessComponent {
    fn subscribe_toggle_on(&self, handler: Callback0) -> Result<()> {
        self.subscribe(
            "toggle_on",
            |h| &mut h.toggle_on_0,
            handler,
        )
    }
    fn subscribe_toggle_off(&self, handler: Callback0) -> Result<()> {
        self.subscribe(
            "toggle_off",
            |h| &mut h.toggle_off_0,
            handler,
        )
    }
}

impl HotwordFacade for InProcessComponent {
    fn subscribe_detected(&self, id: String, handler: Callback<SiteMessage>) -> Result<()> {
        self.subscribe_payload(
            "hotword_detected",
            |h| h.hotword_detected.entry(id).or_insert_with(|| vec![]),
            handler,
        )
    }

    fn subscribe_all_detected(&self, handler: Callback<SiteMessage>) -> Result<()> {
        self.subscribe_payload(
            "hotword_all_detected",
            |h| &mut h.hotword_all_detected,
            handler,
        )
    }
}

impl HotwordBackendFacade for InProcessComponent {
    fn publish_detected(&self, id: String, message: SiteMessage) -> Result<()> {
        self.publish_payload(
            "hotword_detected",
            move |h| h.hotword_detected.get(&id),
            message.clone(),
        )?;
        self.publish_payload(
            "hotword_all_detected",
            move |h| Some(&h.hotword_all_detected),
            message,
        )
    }
}

impl SoundFeedbackFacade for InProcessComponent {}

impl SoundFeedbackBackendFacade for InProcessComponent {}

impl AsrFacade for InProcessComponent {
    p!(publish_start_listening<SiteMessage> asr_start_listening);
    p!(publish_stop_listening<SiteMessage> asr_stop_listening);
    s!(subscribe_text_captured<TextCapturedMessage> asr_text_captured);
    s!(subscribe_partial_text_captured<TextCapturedMessage> asr_partial_text_captured);
}

impl AsrBackendFacade for InProcessComponent {
    s!(subscribe_start_listening<SiteMessage> asr_start_listening);
    s!(subscribe_stop_listening<SiteMessage> asr_stop_listening);
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
    fn publish_play_bytes(&self, bytes: PlayBytesMessage) -> Result<()> {
        let site_id = bytes.site_id.to_string();
        self.publish_payload("as_play_bytes", move |h| h.as_play_bytes.get(&site_id), bytes.clone())?;
        self.publish_payload("as_all_play_bytes", move |h| Some(&h.as_all_play_bytes), bytes)
    }

    s!(subscribe_play_finished<PlayFinishedMessage>(site_id: SiteId) { as_play_finished[site_id;] });
    s!(subscribe_all_play_finished<PlayFinishedMessage> as_all_play_finished );
    s!(subscribe_audio_frame<AudioFrameMessage>(site_id: SiteId) { as_audio_frame[site_id;] });
}

impl AudioServerBackendFacade for InProcessComponent {
    s!(subscribe_play_bytes<PlayBytesMessage>(site_id: SiteId) { as_play_bytes[site_id;] });
    s!(subscribe_all_play_bytes<PlayBytesMessage> as_all_play_bytes);
    fn publish_play_finished(&self, message: PlayFinishedMessage) -> Result<()> {
        let site_id = message.site_id.to_string();

        let _message = message.clone();

        self.publish_payload(
            &message.site_id,
            move |h| h.as_play_finished.get(&site_id),
            _message,
        )?;
        self.publish_payload("as_all_play_finished", move |h| Some(&h.as_all_play_finished), message)
    }
    p!(publish_audio_frame(frame:AudioFrameMessage) as_audio_frame[frame.site_id;]);
}

impl DialogueFacade for InProcessComponent {
    s!(subscribe_session_queued<SessionQueuedMessage> dialogue_session_queued);
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
    p!(publish_session_queued<SessionQueuedMessage> dialogue_session_queued);
    p!(publish_session_started<SessionStartedMessage> dialogue_session_started);

    fn publish_intent(&self, intent: IntentMessage) -> Result<()> {
        let intent_name = intent.intent.intent_name.to_string();

        let _intent = intent.clone();

        self.publish_payload(
            &format!("intent_{}", &intent_name),
            move |h| h.intent.get(&intent_name),
            _intent,
        )?;

        self.publish_payload("intents", move |h| Some(&h.intents), intent)
    }

    p!(publish_session_ended<SessionEndedMessage> dialogue_session_ended);
    s!(subscribe_start_session<StartSessionMessage> dialogue_start_session);
    s!(subscribe_continue_session<ContinueSessionMessage> dialogue_continue_session);
    s!(subscribe_end_session<EndSessionMessage> dialogue_end_session);
}


#[cfg(test)]
mod tests {
    use super::*;

    use std::rc::Rc;

    fn create_handlers() -> (Rc<InProcessHermesProtocolHandler>, Rc<InProcessHermesProtocolHandler>) {
        let handler = Rc::new(InProcessHermesProtocolHandler::new().unwrap());
        (Rc::clone(&handler), handler)
    }

    test_suite!();
}
