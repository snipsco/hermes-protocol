extern crate hermes;
#[cfg(test)]
#[macro_use]
extern crate hermes_test_suite;
#[macro_use]
extern crate log;
#[cfg(test)]
extern crate rand;
extern crate rumqtt;
#[cfg(test)]
extern crate semver;
extern crate serde;
extern crate serde_json;
#[cfg(test)]
extern crate snips_queries_ontology;
extern crate strum;
#[macro_use]
extern crate strum_macros;

use std::collections::HashMap;
use std::string::ToString;
use std::sync::Arc;
use std::sync::Mutex;

use hermes::*;

mod topics;

use topics::*;

struct MqttHandler {
    pub callbacks: Arc<Mutex<HashMap<String, Box<Fn(&rumqtt::Message) -> () + Send + Sync>>>>,
    pub callbacks_wildcard: Arc<Mutex<Vec<(rumqtt::TopicFilter, Box<Fn(&rumqtt::Message) -> () + Send + Sync>)>>>,
    pub mqtt_client: Mutex<rumqtt::MqttClient>
}

impl MqttHandler {
    pub fn publish(&self, topic: &HermesTopic) -> Result<()> {
        self.mqtt_client.lock().map(|mut c| {
            let topic = &*topic.as_path();
            debug!("Publishing on MQTT topic '{}'", topic);
            c.publish(topic, rumqtt::QoS::Level0, vec![])
        })?.chain_err(|| "Could not publish on MQTT")?;
        Ok(())
    }

    pub fn publish_payload<P: serde::Serialize>(&self, topic: &HermesTopic, payload: P) -> Result<()> {
        self.mqtt_client.lock().map(|mut c|
            serde_json::to_vec(&payload).map(|p| {
                let topic = &*topic.as_path();
                debug!("Publishing on MQTT topic '{}', payload : {}", topic, if p.len() < 2048 {
                    String::from_utf8_lossy(&p).to_string()
                } else {
                    format!("size = {}, start = {}", p.len(), String::from_utf8_lossy(&p[0..128]))
                });
                trace!("Payload : {}", String::from_utf8_lossy(&p));
                c.publish(topic, rumqtt::QoS::Level0, p)
            }
            ))??.chain_err(|| "Could not publish on MQTT")?;
        Ok(())
    }

    pub fn publish_binary_payload(&self, topic: &HermesTopic, payload: Vec<u8>) -> Result<()> {
        self.mqtt_client.lock().map(|mut c| {
            let topic = &*topic.as_path();
            debug!("Publishing as binary on MQTT topic '{}', with size {}", topic, payload.len());
            c.publish(topic, rumqtt::QoS::Level0, payload)
        }
        )?.chain_err(|| "Could not publish on MQTT")?;
        Ok(())
    }

    pub fn subscribe<F>(&self, topic: &HermesTopic, handler: F) -> Result<()> where F: Fn() -> () + Send + Sync + 'static {
        self.inner_subscribe(topic, move |m| {
            debug!("Received a message on MQTT topic '{}'", &**m.topic);
            handler()
        })
    }

    pub fn subscribe_payload<F, P>(&self, topic: &HermesTopic, handler: F) -> Result<()>
        where F: Fn(&P) -> () + Send + Sync + 'static,
              P: serde::de::DeserializeOwned {
        self.inner_subscribe(topic, move |m| {
            debug!("Received a message on MQTT topic '{}', payload : {}", &**m.topic, if m.payload.len() < 2048 {
                String::from_utf8_lossy(&m.payload).to_string()
            } else {
                format!("size = {}, start = {}", m.payload.len(), String::from_utf8_lossy(&m.payload[0..128]))
            });
            trace!("Payload : {}", String::from_utf8_lossy(&m.payload));
            let r = serde_json::from_slice(m.payload.as_slice());
            match r {
                Ok(p) => handler(&p),
                Err(e) => warn!("Error while decoding object on topic {} : {:?}", &**m.topic, e)
            }
        })
    }

    pub fn subscribe_binary_payload<F>(&self, topic: &HermesTopic, handler: F) -> Result<()>
        where F: Fn(&HermesTopic, &[u8]) -> () + Send + Sync + 'static {
        self.inner_subscribe(topic, move |m| {
            debug!("Received a message on MQTT topic '{}', payload : {}", &**m.topic, if m.payload.len() < 2048 {
                String::from_utf8_lossy(&m.payload).to_string()
            } else {
                format!("size = {}, start = {}", m.payload.len(), String::from_utf8_lossy(&m.payload[0..128]))
            });
            trace!("Payload : {}", String::from_utf8_lossy(&m.payload));
            let topic = HermesTopic::from_path(&**m.topic);
            if let Some(topic) = topic {
                handler(&topic, &m.payload)
            } else {
                error!("could not parse topic : {}", &**m.topic)
            }
        })
    }

    fn inner_subscribe<F>(&self, topic: &HermesTopic, callback: F) -> Result<()> where F: Fn(&::rumqtt::Message) -> () + Send + Sync + 'static {
        let topic_name = Arc::new(topic.as_path());
        let s_topic_name = Arc::clone(&topic_name);
        if topic_name.contains("+") || topic_name.contains("#") {
            let topic_filter = rumqtt::TopicFilter::new(topic.to_string())
                .chain_err(|| format!("Not a valid topic : {}", topic))?;
            self.callbacks_wildcard.lock().map(|mut c| {
                c.push((topic_filter, Box::new(callback)))
            })?;
        } else {
            self.callbacks.lock().map(|mut c| {
                c.insert(topic.to_string(), Box::new(callback))
            })?;
        }
        self.mqtt_client.lock()
            .map(|mut c| c.subscribe(vec![(&s_topic_name, rumqtt::QoS::Level0)]))?
            .chain_err(|| "Could not subscribe on MQTT")?;
        debug!("Subscribed on MQTT topic '{}'", topic_name);
        Ok(())
    }
}

pub struct MqttHermesProtocolHandler {
    mqtt_handler: Arc<MqttHandler>
}

impl MqttHermesProtocolHandler {
    pub fn new(broker_address: &str) -> Result<MqttHermesProtocolHandler> {
        let callbacks: Arc<Mutex<HashMap<String, Box<Fn(&rumqtt::Message) -> () + Send + Sync>>>> = Arc::new(Mutex::new(HashMap::new()));
        let callbacks_wildcard: Arc<Mutex<Vec<(rumqtt::TopicFilter, Box<Fn(&rumqtt::Message) -> () + Send + Sync>)>>> = Arc::new(Mutex::new(Vec::new()));

        info!("Connecting to MQTT broker at address {}", broker_address);

        let client_options = rumqtt::MqttOptions::new()
            .set_keep_alive(5)
            .set_reconnect(3)
            .set_broker(&broker_address);

        let client_callbacks = Arc::clone(&callbacks);
        let client_callbacks_wildcard = Arc::clone(&callbacks_wildcard);

        let mqtt_callback = rumqtt::MqttCallback::new().on_message(move |message| {
            client_callbacks.lock().map(|r| {
                if let Some(callback) = r.get(&**message.topic) {
                    (callback)(&message)
                }
            }).unwrap_or_else(|e| {
                error!("Could not get a lock on callbacks, message on topic '{}' dropped: {:?}", &**message.topic, e)
            });
            client_callbacks_wildcard.lock().map(|l| {
                for &(ref topic, ref callback) in l.iter() {
                    if topic.get_matcher().is_match(&message.topic) {
                        (callback)(&message)
                    }
                }
            }).unwrap_or_else(|e| {
                error!("Could not get a lock on callbacks, message on topic '{}' dropped: {:?}", &**message.topic, e)
            })
        });
        let mqtt_client = rumqtt::MqttClient::start(client_options, Some(mqtt_callback))
            .chain_err(|| "Could not start MQTT client")?;

        let mqtt_client = Mutex::new(mqtt_client);

        let mqtt_handler = Arc::new(MqttHandler { callbacks, callbacks_wildcard, mqtt_client });

        Ok(MqttHermesProtocolHandler { mqtt_handler })
    }
}

macro_rules! s {
    ($n:ident<$t:ty> $topic:expr; ) => {
        fn $n(&self, handler: Callback<$t>) -> Result<()> {
            self.mqtt_handler.subscribe_payload($topic, move |p| handler.call(p))
        }
    };

    ($n:ident<$t:ty>($($a:ident: $ta:ty),*) $topic:block) => {
        fn $n(&self, $($a: $ta),*, handler: Callback<$t>) -> Result<()> {
            self.mqtt_handler.subscribe_payload($topic, move |p| handler.call(p))
        }
    };

    ($n:ident $topic:expr; ) => {
        fn $n(&self, handler: Callback0) -> Result<()> {
            self.mqtt_handler.subscribe($topic, move || handler.call())
        }
    };
}

macro_rules! s_bin {
    ($n:ident<$t:ty> $topic:block |$rt:ident, $p:ident| $decoder:block) => {
        fn $n(&self, handler: Callback<$t>) -> Result<()> {
            self.mqtt_handler.subscribe_binary_payload($topic, move |$rt, $p| handler.call(&$decoder))
        }
    };

    ($n:ident<$t:ty>($($a:ident: $ta:ty),*) $topic:block |$rt:ident, $p:ident| $decoder:block) => {
        fn $n(&self, $($a: $ta),*, handler: Callback<$t>) -> Result<()> {
            self.mqtt_handler.subscribe_binary_payload($topic, move |$rt, $p| handler.call(&$decoder))
        }
    };
}

macro_rules! p {
    ($n:ident<$t:ty> $topic:expr; ) => {
        fn $n(&self, payload: $t) -> Result<()> {
            self.mqtt_handler.publish_payload($topic, payload)
        }
    };

    ($n:ident<$t:ty>($param1:ident: $t1:ty) $topic:block ) => {
        fn $n(&self, $param1: $t1, payload: $t) -> Result<()> {
            self.mqtt_handler.publish_payload($topic, payload)
        }
    };

    ($n:ident($payload:ident: $t:ty) $topic:block ) => {
        fn $n(&self, $payload: $t) -> Result<()> {
            self.mqtt_handler.publish_payload($topic, $payload)
        }
    };

    ($n:ident $topic:expr; ) => {
        fn $n(&self) -> Result<()> {
            self.mqtt_handler.publish($topic)
        }
    };
}

macro_rules! p_bin {
    ($n:ident($payload:ident: $t:ty) $topic:block $bytes:block ) => {
        fn $n(&self, $payload: $t) -> Result<()> {
            self.mqtt_handler.publish_binary_payload($topic, $bytes)
        }
    };
}

macro_rules! impl_component_facades_for {
    // cannot use s! and p! macros in the impl here because we we would need access to self
    // to get the component... I'm sad...
    ($t:ty) => {
        impl ComponentFacade for $t {
            fn publish_version_request(&self) -> Result<()> {
                self.mqtt_handler.publish(&HermesTopic::Component(None, self.component, ComponentCommand::VersionRequest))
            }

            fn subscribe_version(&self, handler: Callback<VersionMessage>) -> Result<()> {
                self.mqtt_handler.subscribe_payload(&HermesTopic::Component(None, self.component, ComponentCommand::Version), move |p| handler.call(p))
            }

            fn subscribe_error(&self, handler: Callback<ErrorMessage>) -> Result<()> {
                self.mqtt_handler.subscribe_payload(&HermesTopic::Component(None, self.component, ComponentCommand::Error), move |p| handler.call(p))
            }
        }

        impl ComponentBackendFacade for $t {
            fn subscribe_version_request(&self, handler: Callback0) -> Result<()> {
                self.mqtt_handler.subscribe(&HermesTopic::Component(None, self.component, ComponentCommand::VersionRequest), move || handler.call())
            }

            fn publish_version(&self, version: VersionMessage) -> Result<()> {
                self.mqtt_handler.publish_payload(&HermesTopic::Component(None, self.component, ComponentCommand::Version), version)
            }

            fn publish_error(&self, error: ErrorMessage) -> Result<()> {
                self.mqtt_handler.publish_payload(&HermesTopic::Component(None, self.component, ComponentCommand::Error), error)
            }
        }
    };
}

macro_rules! impl_toggleable_facades_for {
    // cannot use s! and p! macros in the impl here because we we would need access to self
    // to get the toggle on/off topics... I'm sad...
    ($t:ty) => {
        impl ToggleableFacade for $t {
            fn publish_toggle_on(&self) -> Result<()> {
                self.mqtt_handler.publish(&self.toggle_on_topic)
            }

            fn publish_toggle_off(&self) -> Result<()> {
                self.mqtt_handler.publish(&self.toggle_off_topic)
            }
        }

        impl ToggleableBackendFacade for $t {
            fn subscribe_toggle_on(&self, handler: Callback0) -> Result<()> {
                self.mqtt_handler.subscribe(&self.toggle_on_topic, move || handler.call())
            }

            fn subscribe_toggle_off(&self, handler: Callback0) -> Result<()> {
                self.mqtt_handler.subscribe(&self.toggle_off_topic, move || handler.call())
            }
        }
    };
}

macro_rules! impl_identifiable_toggleable_facades_for {
    ($t:ty) => {
        impl IdentifiableToggleableFacade for $t {
            fn publish_toggle_on(&self, site: SiteMessage) -> Result<()> {
                self.mqtt_handler.publish_payload(&self.toggle_on_topic, site)
            }

            fn publish_toggle_off(&self, site: SiteMessage) -> Result<()> {
                self.mqtt_handler.publish_payload(&self.toggle_off_topic, site)
            }
        }

        impl IdentifiableToggleableBackendFacade for $t {
            fn subscribe_toggle_on(&self, handler: Callback<SiteMessage>) -> Result<()> {
                self.mqtt_handler.subscribe_payload(&self.toggle_on_topic, move |p| handler.call(p))
            }

            fn subscribe_toggle_off(&self, handler: Callback<SiteMessage>) -> Result<()> {
                self.mqtt_handler.subscribe_payload(&self.toggle_off_topic, move |p| handler.call(p))
            }
        }
    };
}

macro_rules! impl_identifiable_component_facades_for {
    ($t:ty) => {
        impl IdentifiableComponentFacade for $t {
            fn publish_version_request(&self, site_id: SiteId) -> Result<()> {
                self.mqtt_handler.publish(&HermesTopic::Component(Some(site_id), self.component, ComponentCommand::VersionRequest))
            }

            fn subscribe_version(&self, site_id: SiteId, handler: Callback<VersionMessage>) -> Result<()> {
                self.mqtt_handler.subscribe_payload(&HermesTopic::Component(Some(site_id), self.component, ComponentCommand::Version), move |p| handler.call(p))
            }

            fn subscribe_error(&self, site_id: SiteId, handler: Callback<ErrorMessage>) -> Result<()> {
                self.mqtt_handler.subscribe_payload(&HermesTopic::Component(Some(site_id), self.component, ComponentCommand::Error), move |p| handler.call(p))
            }
        }

        impl IdentifiableComponentBackendFacade for $t {
            fn subscribe_version_request(&self, site_id: SiteId, handler: Callback0) -> Result<()> {
                self.mqtt_handler.subscribe(&HermesTopic::Component(Some(site_id), self.component, ComponentCommand::VersionRequest), move || handler.call())
            }

            fn publish_version(&self, site_id: SiteId, version: VersionMessage) -> Result<()> {
                self.mqtt_handler.publish_payload(&HermesTopic::Component(Some(site_id), self.component, ComponentCommand::Version), version)
            }

            fn publish_error(&self, site_id: SiteId, error: ErrorMessage) -> Result<()> {
                self.mqtt_handler.publish_payload(&HermesTopic::Component(Some(site_id), self.component, ComponentCommand::Error), error)
            }
        }
    }
}

struct MqttComponentFacade {
    component: Component,
    mqtt_handler: Arc<MqttHandler>
}

impl_component_facades_for!(MqttComponentFacade);
impl_identifiable_component_facades_for!(MqttComponentFacade);

struct MqttToggleableFacade {
    toggle_on_topic: HermesTopic,
    toggle_off_topic: HermesTopic,
    mqtt_handler: Arc<MqttHandler>
}

impl_identifiable_toggleable_facades_for!(MqttToggleableFacade);

struct MqttToggleableComponentFacade {
    component: Component,
    toggle_on_topic: HermesTopic,
    toggle_off_topic: HermesTopic,
    mqtt_handler: Arc<MqttHandler>
}

impl_component_facades_for!(MqttToggleableComponentFacade);
impl_toggleable_facades_for!(MqttToggleableComponentFacade);
impl_identifiable_component_facades_for!(MqttToggleableComponentFacade);
impl_identifiable_toggleable_facades_for!(MqttToggleableComponentFacade);

impl HotwordFacade for MqttToggleableComponentFacade {
    s!(subscribe_detected<SiteMessage>(id: String) { &HermesTopic::Hotword(Some(id), HotwordCommand::Detected) });
    s!(subscribe_all_detected<SiteMessage> &HermesTopic::Hotword(Some("+".into()), HotwordCommand::Detected););
}

impl HotwordBackendFacade for MqttToggleableComponentFacade {
    p!(publish_detected<SiteMessage>(id: String) { &HermesTopic::Hotword(Some(id), HotwordCommand::Detected) });
}

impl SoundFeedbackFacade for MqttToggleableFacade {}

impl SoundFeedbackBackendFacade for MqttToggleableFacade {}

impl AsrFacade for MqttToggleableComponentFacade {
    p!(publish_start_listening<SiteMessage> &HermesTopic::Asr(AsrCommand::StartListening););
    p!(publish_stop_listening<SiteMessage> &HermesTopic::Asr(AsrCommand::StopListening););
    s!(subscribe_text_captured<TextCapturedMessage> &HermesTopic::Asr(AsrCommand::TextCaptured););
    s!(subscribe_partial_text_captured<TextCapturedMessage> &HermesTopic::Asr(AsrCommand::PartialTextCaptured););
}

impl AsrBackendFacade for MqttToggleableComponentFacade {
    s!(subscribe_start_listening<SiteMessage> &HermesTopic::Asr(AsrCommand::StartListening););
    s!(subscribe_stop_listening<SiteMessage> &HermesTopic::Asr(AsrCommand::StopListening););
    p!(publish_text_captured<TextCapturedMessage> &HermesTopic::Asr(AsrCommand::TextCaptured););
    p!(publish_partial_text_captured<TextCapturedMessage> &HermesTopic::Asr(AsrCommand::PartialTextCaptured););
}

impl TtsFacade for MqttComponentFacade {
    p!(publish_say<SayMessage> &HermesTopic::Tts(TtsCommand::Say););
    s!(subscribe_say_finished<SayFinishedMessage> &HermesTopic::Tts(TtsCommand::SayFinished););
}

impl TtsBackendFacade for MqttComponentFacade {
    s!(subscribe_say<SayMessage> &HermesTopic::Tts(TtsCommand::Say););
    p!(publish_say_finished<SayFinishedMessage> &HermesTopic::Tts(TtsCommand::SayFinished););
}

impl NluFacade for MqttComponentFacade {
    p!(publish_query<NluQueryMessage> &HermesTopic::Nlu(NluCommand::Query););
    p!(publish_partial_query<NluSlotQueryMessage> &HermesTopic::Nlu(NluCommand::PartialQuery););
    s!(subscribe_slot_parsed<NluSlotMessage> &HermesTopic::Nlu(NluCommand::SlotParsed););
    s!(subscribe_intent_parsed<NluIntentMessage> &HermesTopic::Nlu(NluCommand::IntentParsed););
    s!(subscribe_intent_not_recognized<NluIntentNotRecognizedMessage> &HermesTopic::Nlu(NluCommand::IntentNotRecognized););
}

impl NluBackendFacade for MqttComponentFacade {
    s!(subscribe_query<NluQueryMessage> &HermesTopic::Nlu(NluCommand::Query););
    s!(subscribe_partial_query<NluSlotQueryMessage> &HermesTopic::Nlu(NluCommand::PartialQuery););
    p!(publish_slot_parsed<NluSlotMessage> &HermesTopic::Nlu(NluCommand::SlotParsed););
    p!(publish_intent_parsed<NluIntentMessage> &HermesTopic::Nlu(NluCommand::IntentParsed););
    p!(publish_intent_not_recognized<NluIntentNotRecognizedMessage> &HermesTopic::Nlu(NluCommand::IntentNotRecognized););
}

impl AudioServerFacade for MqttToggleableComponentFacade {
    s_bin!(subscribe_audio_frame<AudioFrameMessage>(site_id: SiteId) { &HermesTopic::AudioServer(Some(site_id), AudioServerCommand::AudioFrame) }
            |topic, bytes| {
                if let &HermesTopic::AudioServer(Some(ref site_id), AudioServerCommand::AudioFrame) = topic {
                    AudioFrameMessage { site_id: site_id.to_owned(), wav_frame: bytes.into() }
                } else {
                    unreachable!()
                }
            });
    p_bin!(publish_play_bytes(bytes: PlayBytesMessage)
        { &HermesTopic::AudioServer(Some(bytes.site_id), AudioServerCommand::PlayBytes(bytes.id)) }
        { bytes.wav_bytes });
    s!(subscribe_play_finished<PlayFinishedMessage>(site_id: SiteId) { &HermesTopic::AudioServer(Some(site_id), AudioServerCommand::PlayFinished) });
    s!(subscribe_all_play_finished<PlayFinishedMessage> &HermesTopic::AudioServer(Some("+".into()), AudioServerCommand::PlayFinished););
}

impl AudioServerBackendFacade for MqttToggleableComponentFacade {
    p_bin!(publish_audio_frame(frame: AudioFrameMessage)
        { &HermesTopic::AudioServer(Some(frame.site_id), AudioServerCommand::AudioFrame) }
        { frame.wav_frame });
    s_bin!(subscribe_all_play_bytes<PlayBytesMessage> { &HermesTopic::AudioServer(Some("+".into()), AudioServerCommand::PlayBytes("#".into())) }
            |topic, bytes| {
                if let &HermesTopic::AudioServer(Some(ref site_id), AudioServerCommand::PlayBytes(ref request_id)) = topic {
                    PlayBytesMessage { session_id: None, site_id: site_id.to_owned(), id: request_id.to_owned(), wav_bytes: bytes.into() }
                } else {
                    unreachable!()
                }
            });
    s_bin!(subscribe_play_bytes<PlayBytesMessage>(site_id: SiteId) { &HermesTopic::AudioServer(Some(site_id), AudioServerCommand::PlayBytes("#".into())) }
            |topic, bytes| {
                if let &HermesTopic::AudioServer(Some(ref site_id), AudioServerCommand::PlayBytes(ref request_id)) = topic {
                    PlayBytesMessage { session_id: None, site_id: site_id.to_owned(), id: request_id.to_owned(), wav_bytes: bytes.into() }
                } else {
                    unreachable!()
                }
            });
    p!(publish_play_finished(message: PlayFinishedMessage) { &HermesTopic::AudioServer(Some(message.site_id.clone()), AudioServerCommand::PlayFinished) });
}

impl DialogueFacade for MqttToggleableComponentFacade {
    s!(subscribe_session_queued<SessionQueuedMessage> &HermesTopic::DialogueManager(DialogueManagerCommand::SessionQueued););
    s!(subscribe_session_started<SessionStartedMessage> &HermesTopic::DialogueManager(DialogueManagerCommand::SessionStarted););
    s!(subscribe_intent<IntentMessage>(intent_name: String) { &HermesTopic::Intent(intent_name) });
    s!(subscribe_intents<IntentMessage> &HermesTopic::Intent("#".into()););
    s!(subscribe_session_ended<SessionEndedMessage> &HermesTopic::DialogueManager(DialogueManagerCommand::SessionEnded););
    p!(publish_start_session<StartSessionMessage> &HermesTopic::DialogueManager(DialogueManagerCommand::StartSession););
    p!(publish_continue_session<ContinueSessionMessage> &HermesTopic::DialogueManager(DialogueManagerCommand::ContinueSession););
    p!(publish_end_session<EndSessionMessage> &HermesTopic::DialogueManager(DialogueManagerCommand::EndSession););
}

impl DialogueBackendFacade for MqttToggleableComponentFacade {
    p!(publish_session_queued<SessionQueuedMessage> &HermesTopic::DialogueManager(DialogueManagerCommand::SessionQueued););
    p!(publish_session_started<SessionStartedMessage> &HermesTopic::DialogueManager(DialogueManagerCommand::SessionStarted););
    p!(publish_intent(intent: IntentMessage) {&HermesTopic::Intent(intent.intent.intent_name.clone())});
    p!(publish_session_ended<SessionEndedMessage> &HermesTopic::DialogueManager(DialogueManagerCommand::SessionEnded););
    s!(subscribe_start_session<StartSessionMessage> &HermesTopic::DialogueManager(DialogueManagerCommand::StartSession););
    s!(subscribe_continue_session<ContinueSessionMessage> &HermesTopic::DialogueManager(DialogueManagerCommand::ContinueSession););
    s!(subscribe_end_session<EndSessionMessage> &HermesTopic::DialogueManager(DialogueManagerCommand::EndSession););
}

impl MqttHermesProtocolHandler {
    fn hotword_component(&self) -> Box<MqttToggleableComponentFacade> {
        Box::new(MqttToggleableComponentFacade {
            mqtt_handler: Arc::clone(&self.mqtt_handler),
            component: Component::Hotword,
            toggle_on_topic: HermesTopic::Hotword(None, HotwordCommand::ToggleOn),
            toggle_off_topic: HermesTopic::Hotword(None, HotwordCommand::ToggleOff)
        })
    }

    fn sound_toggleable(&self) -> Box<MqttToggleableFacade> {
        Box::new(MqttToggleableFacade {
            mqtt_handler: Arc::clone(&self.mqtt_handler),
            toggle_on_topic: HermesTopic::Feedback(FeedbackCommand::Sound(SoundCommand::ToggleOn)),
            toggle_off_topic: HermesTopic::Feedback(FeedbackCommand::Sound(SoundCommand::ToggleOff))
        })
    }

    fn asr_component(&self) -> Box<MqttToggleableComponentFacade> {
        Box::new(MqttToggleableComponentFacade {
            mqtt_handler: Arc::clone(&self.mqtt_handler),
            component: Component::Asr,
            toggle_on_topic: HermesTopic::Asr(AsrCommand::ToggleOn),
            toggle_off_topic: HermesTopic::Asr(AsrCommand::ToggleOff)
        })
    }

    fn dialogue_component(&self) -> Box<MqttToggleableComponentFacade> {
        Box::new(MqttToggleableComponentFacade {
            mqtt_handler: Arc::clone(&self.mqtt_handler),
            component: Component::DialogueManager,
            toggle_on_topic: HermesTopic::DialogueManager(DialogueManagerCommand::ToggleOn),
            toggle_off_topic: HermesTopic::DialogueManager(DialogueManagerCommand::ToggleOff)
        })
    }

    fn audio_server_component(&self) -> Box<MqttToggleableComponentFacade> {
        Box::new(MqttToggleableComponentFacade {
            mqtt_handler: Arc::clone(&self.mqtt_handler),
            component: Component::AudioServer,
            toggle_on_topic: HermesTopic::AudioServer(None, AudioServerCommand::ToggleOn),
            toggle_off_topic: HermesTopic::AudioServer(None, AudioServerCommand::ToggleOff),
        })
    }

    fn component(&self, component: Component) -> Box<MqttComponentFacade> {
        Box::new(MqttComponentFacade {
            mqtt_handler: Arc::clone(&self.mqtt_handler),
            component
        })
    }
}

impl HermesProtocolHandler for MqttHermesProtocolHandler {
    fn hotword(&self) -> Box<HotwordFacade> {
        self.hotword_component()
    }

    fn sound_feedback(&self) -> Box<SoundFeedbackFacade> {
        self.sound_toggleable()
    }

    fn asr(&self) -> Box<AsrFacade> {
        self.asr_component()
    }

    fn tts(&self) -> Box<TtsFacade> {
        self.component(Component::Tts)
    }

    fn nlu(&self) -> Box<NluFacade> {
        self.component(Component::Nlu)
    }

    fn audio_server(&self) -> Box<AudioServerFacade> {
        self.audio_server_component()
    }

    fn hotword_backend(&self) -> Box<HotwordBackendFacade> {
        self.hotword_component()
    }

    fn sound_feedback_backend(&self) -> Box<SoundFeedbackBackendFacade> {
        self.sound_toggleable()
    }

    fn asr_backend(&self) -> Box<AsrBackendFacade> {
        self.asr_component()
    }

    fn tts_backend(&self) -> Box<TtsBackendFacade> {
        self.component(Component::Tts)
    }

    fn nlu_backend(&self) -> Box<NluBackendFacade> {
        self.component(Component::Nlu)
    }

    fn audio_server_backend(&self) -> Box<AudioServerBackendFacade> {
        self.audio_server_component()
    }

    fn dialogue(&self) -> Box<DialogueFacade> {
        self.dialogue_component()
    }

    fn dialogue_backend(&self) -> Box<DialogueBackendFacade> {
        self.dialogue_component()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;
    use std::rc::Rc;

    struct ServerHolder {
        server: ::std::process::Child,
    }

    struct HandlerHolder {
        handler: MqttHermesProtocolHandler,
        // this code is not dead, we need this as there is a drop on server holder that will kill
        // the child process
        #[allow(dead_code)]
        server: Rc<ServerHolder>,
    }

    impl std::ops::Deref for HandlerHolder {
        type Target = MqttHermesProtocolHandler;
        fn deref(&self) -> &MqttHermesProtocolHandler {
            &self.handler
        }
    }

    impl Drop for ServerHolder {
        fn drop(&mut self) {
            self.server.kill().unwrap();
        }
    }

    fn create_handlers() -> (HandlerHolder, HandlerHolder) {
        let port = ::rand::random::<u16>() | 1024;

        let server = Rc::new(ServerHolder {
            server: Command::new("mosquitto")
                .arg("-p")
                .arg(format!("{}", port))
                .arg("-v")
                .spawn()
                .expect("could not start mosquitto"),
        });

        let server_address = format!("localhost:{}", port);

        ::std::thread::sleep(::std::time::Duration::from_millis(200));

        let handler1 = HandlerHolder {
            handler: MqttHermesProtocolHandler::new(&server_address).expect("could not create first client"),
            server: Rc::clone(&server)
        };

        let handler2 = HandlerHolder {
            handler: MqttHermesProtocolHandler::new(&server_address).expect("could not create second client"),
            server: server
        };

        (handler1, handler2)
    }

    //test_suite!();
}
