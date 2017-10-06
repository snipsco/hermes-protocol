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

    as_play_bytes: HashMap<SiteId, Vec<Callback<PlayBytesMessage>>>,
    as_play_finished: Vec<Callback<PlayFinishedMessage>>,
    as_audio_frame: HashMap<SiteId, Vec<Callback<AudioFrameMessage>>>,

    hotword_detected: Vec<Callback<SiteMessage>>,

    nlu_query: Vec<Callback<NluQueryMessage>>,
    nlu_partial_query: Vec<Callback<NluSlotQueryMessage>>,
    nlu_slot_parsed: Vec<Callback<SlotMessage>>,
    nlu_intent_parsed: Vec<Callback<NluIntentMessage>>,
    nlu_intent_not_recognized: Vec<Callback<NluIntentNotRecognizedMessage>>,

    component_version_request: HashMap<ComponentName, Vec<Callback0>>,
    component_version: HashMap<ComponentName, Vec<Callback<VersionMessage>>>,
    component_error: HashMap<ComponentName, Vec<Callback<ErrorMessage>>>,

    tts_say: Vec<Callback<SayMessage>>,
    tts_say_finished: Vec<Callback<SayFinishedMessage>>,

    toggle_on: HashMap<ComponentName, Vec<Callback<SiteMessage>>>,
    toggle_off: HashMap<ComponentName, Vec<Callback<SiteMessage>>>,

    intent: HashMap<IntentName, Vec<Callback<IntentMessage>>>,
    intents: Vec<Callback<IntentMessage>>,

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
            F: FnOnce(&Handler) -> Option<&Vec<Callback<M>>> + Send + 'static,
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
            h.toggle_on.get(&component_name)
        }, site)
    }
    fn publish_toggle_off(&self, site: SiteMessage) -> Result<()> {
        let component_name = self.name.to_string();
        self.publish_payload("toggle_off", move |h| {
            h.toggle_off.get(&component_name)
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
    p!(publish_play_bytes(bytes : PlayBytesMessage) as_play_bytes[bytes.site_id;]);
    s!(subscribe_play_finished<PlayFinishedMessage> as_play_finished);
    s!(subscribe_audio_frame<AudioFrameMessage>(site_id:SiteId) { as_audio_frame[site_id;] });
}

impl AudioServerBackendFacade for InProcessComponent {
    s!(subscribe_play_bytes<PlayBytesMessage>(site_id:SiteId) { as_play_bytes[site_id;]});
    p!(publish_play_finished<PlayFinishedMessage> as_play_finished);
    p!(publish_audio_frame(frame:AudioFrameMessage) as_audio_frame[frame.site_id;]);
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

    macro_rules! t {
        ($name:ident :
            $s_facade:ident.$s:ident <= $t:ty | $p_facade:ident.$p:ident
            with $object:expr;) => {
                #[test]
                fn $name() {
                    let (handler_source, handler_receiver) = create_handlers();
                    let source = handler_source.$p_facade();
                    let receiver = handler_receiver.$s_facade();
                    let (tx, rx) = ::std::sync::mpsc::channel();
                    let tx = ::std::sync::Mutex::new(tx);
                    receiver.$s(::Callback::new(move |o: &$t| tx.lock().map(|it| it.send(o.clone())).unwrap().unwrap())).unwrap();
                    let message = $object;
                    source.$p(message.clone()).unwrap();
                    let result = rx.recv_timeout(::std::time::Duration::from_secs(1));
                    assert!(result.is_ok(), "didn't receive message after one second");
                    assert_eq!(result.unwrap(), message)
                }
            };
        ($name:ident :
            $s_facade:ident.$s:ident <= $p_facade:ident.$p:ident) => {
                #[test]
                fn $name() {
                    let (handler_source, handler_receiver) = create_handlers();
                    let source = handler_source.$p_facade();
                    let receiver = handler_receiver.$s_facade();
                    let (tx, rx) = ::std::sync::mpsc::channel();
                    let tx = ::std::sync::Mutex::new(tx);
                    receiver.$s(::Callback0::new(move || tx.lock().map(|it| it.send(())).unwrap().unwrap())).unwrap();
                    source.$p().unwrap();
                    let result = rx.recv_timeout(::std::time::Duration::from_secs(1));
                    assert!(result.is_ok(), "didn't receive message after one second");
                }
            };
        ($name:ident :
            $s_facade:ident.$s:ident $a:block <= $t:ty | $p_facade:ident.$p:ident
            with $object:expr;) => {
                #[test]
                fn $name() {
                    let (handler_source, handler_receiver) = create_handlers();
                    let source = handler_source.$p_facade();
                    let receiver = handler_receiver.$s_facade();
                    let (tx, rx) = ::std::sync::mpsc::channel();
                    let tx = ::std::sync::Mutex::new(tx);
                    receiver.$s($a, ::Callback::new(move |o: &$t| tx.lock().map(|it| it.send(o.clone())).unwrap().unwrap())).unwrap();
                    let message = $object;
                    source.$p(message.clone()).unwrap();
                    let result = rx.recv_timeout(::std::time::Duration::from_secs(1));
                    assert!(result.is_ok(), "didn't receive message after one second");
                    assert_eq!(result.unwrap(), message)
                }
            };
    }

    macro_rules! t_toggleable {
        ($name:ident : $f_back:ident | $f:ident) => {
            mod $name {
                use super::*;
                t!(toggle_on_works :
                        $f_back.subscribe_toggle_on <= SiteMessage | $f.publish_toggle_on
                        with SiteMessage { site_id : "some site".into() };);
                t!(toggle_off_works :
                        $f_back.subscribe_toggle_off <= SiteMessage | $f.publish_toggle_off
                        with SiteMessage { site_id : "some site".into() };);
            }

        };
    }

    macro_rules! t_component {
        ($name:ident : $f_back:ident | $f:ident) => {
            mod $name {
                use super::*;
                t!(version_request_works :
                        $f_back.subscribe_version_request <= $f.publish_version_request);
                t!(version_works :
                        $f.subscribe_version <= VersionMessage | $f_back.publish_version
                        with VersionMessage { version : ::semver::Version { major : 1, minor : 0, patch : 0, pre : vec![], build: vec![]} };);
                t!(error_works :
                        $f.subscribe_error <= ErrorMessage | $f_back.publish_error
                        with ErrorMessage { error : "some error".into(), context: None };);
            }

        };
    }


    t_component!(hotword_component : hotword_backend | hotword);
    t_toggleable!(hotword_toggleable : hotword_backend | hotword);
    t!(hotword_detected_works:
            hotword.subscribe_detected <= SiteMessage | hotword_backend.publish_detected
            with SiteMessage { site_id : "some site".into() };);

    t_toggleable!(sound_feedback_toggleable : sound_feedback_backend | sound_feedback );

    t_component!(asr_component : asr_backend | asr);
    t_toggleable!(asr_toggleable : asr_backend | asr);
    t!(asr_text_captured_works :
            asr.subscribe_text_captured <= TextCapturedMessage | asr_backend.publish_text_captured
            with TextCapturedMessage { text : "hello world".into(), likelihood: 0.5, seconds : 4.2, site_id: "Some site".into() };);
    t!(asr_partial_text_captured_works :
            asr.subscribe_partial_text_captured <= TextCapturedMessage | asr_backend.publish_partial_text_captured
            with TextCapturedMessage { text : "hello world".into(), likelihood: 0.5, seconds : 4.2, site_id: "Some site".into() };);

    t_component!(tts_component : tts_backend | tts);
    t!(tts_say_works :
            tts_backend.subscribe_say <= SayMessage | tts.publish_say
            with SayMessage { text: "hello world".into(), lang: None, id: None, site_id: None };
    );
    t!(tts_say_finished_works :
            tts.subscribe_say_finished <= SayFinishedMessage | tts_backend.publish_say_finished
            with SayFinishedMessage { id: Some("my id".into()) };
    );

    t_component!(nlu_component : nlu_backend | nlu);
    t!(nlu_query_works :
            nlu_backend.subscribe_query <= NluQueryMessage | nlu.publish_query
            with NluQueryMessage { text : "hello world".into(), intent_filter : None, id : None };
    );
    t!(nlu_partial_query_works :
            nlu_backend.subscribe_partial_query <= NluSlotQueryMessage | nlu.publish_partial_query
            with NluSlotQueryMessage { text : "hello world".into(), intent_name : "my intent".into(), slot_name : "my slot".into(), id : None };
    );
    t!(nlu_slot_parsed_works :
            nlu.subscribe_slot_parsed <= SlotMessage | nlu_backend.publish_slot_parsed
            with SlotMessage { id : None, slot : Some(Slot { slot_name : "my slot".into(), raw_value : "value".into(), value : ::snips_queries_ontology::SlotValue::Custom("my slot".into()), range : None, entity : "entity".into() }) };
    );
    t!(nlu_intent_parsed_works :
            nlu.subscribe_intent_parsed <= NluIntentMessage | nlu_backend.publish_intent_parsed
            with NluIntentMessage {id : None, input : "hello world".into(), intent : IntentClassifierResult { intent_name : "my intent".into(), probability : 0.73 }, slots: None };);
    t!(nlu_intent_not_recognized_works :
            nlu.subscribe_intent_not_recognized <= NluIntentNotRecognizedMessage | nlu_backend.publish_intent_not_recognized
            with NluIntentNotRecognizedMessage {id : None, input : "hello world".into() };);

    t_component!(audio_server_component : audio_server_backend | audio_server);
    t!(audio_server_play_bytes_works :
            audio_server_backend.subscribe_play_bytes { "some site".into() } <= PlayBytesMessage | audio_server.publish_play_bytes
            with PlayBytesMessage { wav_bytes: vec![42; 1000], id: "my id".into(), site_id: "some site".into() };
    );
    t!(audio_server_play_finished_works :
            audio_server.subscribe_play_finished <= PlayFinishedMessage | audio_server_backend.publish_play_finished
            with PlayFinishedMessage { id: "my id".into() };
    );
    t!(audio_server_audio_frame_works :
            audio_server.subscribe_audio_frame { "some site".into() } <= AudioFrameMessage | audio_server_backend.publish_audio_frame
            with AudioFrameMessage { wav_frame: vec![42; 1000], site_id: "some site".into() };
    );

    t_component!(dialogue_component : dialogue_backend | dialogue);
    t_toggleable!(dialogue_toggleable : dialogue_backend | dialogue);
    t!(dialogue_session_started_works:
            dialogue.subscribe_session_started <= SessionStartedMessage | dialogue_backend.publish_session_started
            with SessionStartedMessage { session_id: "some id".into(), custom_data : None };);
    t!(dialogue_intent_works:
            dialogue.subscribe_intents <= IntentMessage | dialogue_backend.publish_intent
            with IntentMessage { session_id: "some id".into(), custom_data : None,  input : "hello world".into(), intent : IntentClassifierResult { intent_name : "my intent".into(), probability : 0.73 }, slots: None  };);
    t!(dialogue_session_ended_works:
            dialogue.subscribe_session_ended <= SessionEndedMessage | dialogue_backend.publish_session_ended
            with SessionEndedMessage { session_id: "some id".into(), custom_data : None, aborted : None };);
    t!(dialogue_start_session_works:
            dialogue_backend.subscribe_start_session <= StartSessionMessage | dialogue.publish_start_session
            with StartSessionMessage { init: SessionInit::User, custom_data : None, site_id: None };);
    t!(dialogue_continue_session_works:
            dialogue_backend.subscribe_continue_session <= ContinueSessionMessage | dialogue.publish_continue_session
            with ContinueSessionMessage { session_id: "some id".into(), action : SessionAction { text : "some text".into(), expect_response : false, intent_filter : None } };);
    t!(dialogue_end_session_works:
            dialogue_backend.subscribe_end_session <= EndSessionMessage | dialogue.publish_end_session
            with EndSessionMessage { session_id: "some id".into() };);
}
