use strum::IntoEnumIterator;

use std::collections::HashMap;
use std::path;
use std::string::ToString;
use std::sync::Arc;
use std::sync::Mutex;

use super::*;

use errors::*;

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
            c.publish(topic, rumqtt::QoS::Level0, vec![]
            )
        })??;
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
            ))???;
        Ok(())
    }

    pub fn publish_binary_payload(&self, topic: &HermesTopic, payload: Vec<u8>) -> Result<()> {
        self.mqtt_client.lock().map(|mut c| {
            let topic = &*topic.as_path();
            debug!("Publishing as binary on MQTT topic '{}', with size {}", topic, payload.len());
            c.publish(topic, rumqtt::QoS::Level0, payload)
        }
        )??;
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
        where F: Fn(&HermesTopic, &[u8]) -> () + Send + Sync + 'static{
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
            let topic_filter = rumqtt::TopicFilter::new(topic.to_string())?;
            self.callbacks_wildcard.lock().map(|mut c| {
                c.push((topic_filter, Box::new(callback)))
            })?;
        } else {
            self.callbacks.lock().map(|mut c| {
                c.insert(topic.to_string(), Box::new(callback))
            })?;
        }
        self.mqtt_client.lock().map(|mut c| c.subscribe(vec![(&s_topic_name,
                                                              rumqtt::QoS::Level0)]))??;
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

        let mqtt_client = Mutex::new(rumqtt::MqttClient::start(client_options,
                                                               Some(mqtt_callback))?);

        let mqtt_handler = Arc::new(MqttHandler { callbacks, callbacks_wildcard, mqtt_client });

        Ok(MqttHermesProtocolHandler { mqtt_handler })
    }
}

macro_rules! s {
    ($n:ident<$t:ty> $topic:expr; ) => {
        fn $n(&self, handler : Callback<$t>) -> Result<()> {
            self.mqtt_handler.subscribe_payload($topic, move |p| handler.call(p))
        }
    };

    ($n:ident<$t:ty>($($a:ident: $ta:ty),*) $topic:block) => {
        fn $n(&self, $($a : $ta),*, handler : Callback<$t>) -> Result<()> {
            self.mqtt_handler.subscribe_payload($topic, move |p| handler.call(p))
        }
    };

    ($n:ident $topic:expr; ) => {
        fn $n(&self, handler : Callback0) -> Result<()> {
            self.mqtt_handler.subscribe($topic, move || handler.call())
        }
    };
}

macro_rules! s_bin {
    ($n:ident<$t:ty>($($a:ident: $ta:ty),*) $topic:block |$rt:ident, $p:ident| $decoder:block) => {
        fn $n(&self, $($a : $ta),*, handler : Callback<$t>) -> Result<()> {
            self.mqtt_handler.subscribe_binary_payload($topic, move |$rt, $p| handler.call(&$decoder))
        }
    };
}

macro_rules! p {
    ($n:ident<$t:ty> $topic:expr; ) => {
        fn $n(&self, payload : $t) -> Result<()> {
            self.mqtt_handler.publish_payload($topic, payload)
        }
    };

    ($n:ident($payload:ident : $t:ty) $topic:block ) => {
        fn $n(&self, $payload : $t) -> Result<()> {
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
    ($n:ident($payload:ident : $t:ty) $topic:block $bytes:block ) => {
        fn $n(&self, $payload : $t) -> Result<()> {
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
                self.mqtt_handler.publish(&HermesTopic::Component(self.component, ComponentCommand::VersionRequest))
            }

            fn subscribe_version(&self, handler: Callback<VersionMessage>) -> Result<()> {
                self.mqtt_handler.subscribe_payload(&HermesTopic::Component(self.component, ComponentCommand::Version), move |p| handler.call(p))
            }

            fn subscribe_error(&self, handler: Callback<ErrorMessage>) -> Result<()> {
                self.mqtt_handler.subscribe_payload(&HermesTopic::Component(self.component, ComponentCommand::Error), move |p| handler.call(p))
            }
        }

        impl ComponentBackendFacade for $t {
            fn subscribe_version_request(&self, handler: Callback0) -> Result<()> {
                self.mqtt_handler.subscribe(&HermesTopic::Component(self.component, ComponentCommand::VersionRequest), move || handler.call())
            }

            fn publish_version(&self, version: VersionMessage) -> Result<()> {
                self.mqtt_handler.publish_payload(&HermesTopic::Component(self.component, ComponentCommand::Version), version)
            }

            fn publish_error(&self, error: ErrorMessage) -> Result<()> {
                self.mqtt_handler.publish_payload(&HermesTopic::Component(self.component, ComponentCommand::Error), error)
            }
        }
    };
}


macro_rules! impl_toggleable_facades_for {
    // cannot use s! and p! macros in the impl here because we we would need access to self
    // to get the toggle on/off topics... I'm sad...
    ($t:ty) => {
        impl ToggleableFacade for $t {
            fn publish_toggle_on(&self, site: SiteMessage) -> Result<()> {
                self.mqtt_handler.publish_payload(&self.toggle_on_topic, site)
            }

            fn publish_toggle_off(&self, site: SiteMessage) -> Result<()> {
                self.mqtt_handler.publish_payload(&self.toggle_off_topic, site)
            }
        }

        impl ToggleableBackendFacade for $t {
            fn subscribe_toggle_on(&self, handler: Callback<SiteMessage>) -> Result<()> {
                self.mqtt_handler.subscribe_payload(&self.toggle_on_topic, move |p| handler.call(p))
            }

            fn subscribe_toggle_off(&self, handler: Callback<SiteMessage>) -> Result<()> {
                self.mqtt_handler.subscribe_payload(&self.toggle_off_topic, move |p| handler.call(p))
            }
        }
    };
}

struct MqttComponentFacade {
    component: Component,
    mqtt_handler: Arc<MqttHandler>
}

impl_component_facades_for!(MqttComponentFacade);

struct MqttToggleableFacade {
    toggle_on_topic: HermesTopic,
    toggle_off_topic: HermesTopic,
    mqtt_handler: Arc<MqttHandler>
}
impl_toggleable_facades_for!(MqttToggleableFacade);

struct MqttToggleableComponentFacade {
    component: Component,
    toggle_on_topic: HermesTopic,
    toggle_off_topic: HermesTopic,
    mqtt_handler: Arc<MqttHandler>
}

impl_component_facades_for!(MqttToggleableComponentFacade);
impl_toggleable_facades_for!(MqttToggleableComponentFacade);

impl HotwordFacade for MqttToggleableComponentFacade {
    s!(subscribe_detected<SiteMessage> &HermesTopic::Hotword(HotwordCommand::Detected););
}

impl HotwordBackendFacade for MqttToggleableComponentFacade {
    p!(publish_detected<SiteMessage> &HermesTopic::Hotword(HotwordCommand::Detected););
}

impl SoundFeedbackFacade for MqttToggleableFacade {}

impl SoundFeedbackBackendFacade for MqttToggleableFacade {}

impl AsrFacade for MqttToggleableComponentFacade {
    s!(subscribe_text_captured<TextCapturedMessage> &HermesTopic::Asr(AsrCommand::TextCaptured););
    s!(subscribe_partial_text_captured<TextCapturedMessage> &HermesTopic::Asr(AsrCommand::PartialTextCaptured););
}

impl AsrBackendFacade for MqttToggleableComponentFacade {
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
    s!(subscribe_slot_parsed<SlotMessage> &HermesTopic::Nlu(NluCommand::SlotParsed););
    s!(subscribe_intent_parsed<NluIntentMessage> &HermesTopic::Nlu(NluCommand::IntentParsed););
    s!(subscribe_intent_not_recognized<NluIntentNotRecognizedMessage> &HermesTopic::Nlu(NluCommand::IntentNotRecognized););
}

impl NluBackendFacade for MqttComponentFacade {
    s!(subscribe_query<NluQueryMessage> &HermesTopic::Nlu(NluCommand::Query););
    s!(subscribe_partial_query<NluSlotQueryMessage> &HermesTopic::Nlu(NluCommand::PartialQuery););
    p!(publish_slot_parsed<SlotMessage> &HermesTopic::Nlu(NluCommand::SlotParsed););
    p!(publish_intent_parsed<NluIntentMessage> &HermesTopic::Nlu(NluCommand::IntentParsed););
    p!(publish_intent_not_recognized<NluIntentNotRecognizedMessage> &HermesTopic::Nlu(NluCommand::IntentNotRecognized););
}

impl AudioServerFacade for MqttComponentFacade {
    s_bin!(subscribe_audio_frame<AudioFrameMessage>(site_id: SiteId) { &HermesTopic::AudioServer(AudioServerCommand::AudioFrame(site_id)) }
            |topic, bytes| {
                if let &HermesTopic::AudioServer(AudioServerCommand::AudioFrame(ref site_id)) = topic {
                    AudioFrameMessage { site_id : site_id.to_owned(), wav_frame : bytes.into() }
                } else {
                    unreachable!()
                }
            });
    p_bin!(publish_play_bytes(bytes: PlayBytesMessage)
        { &HermesTopic::AudioServer(AudioServerCommand::PlayBytes(bytes.site_id, bytes.id)) }
        { bytes.wav_bytes });
    s!(subscribe_play_finished<PlayFinishedMessage> &HermesTopic::AudioServer(AudioServerCommand::PlayFinished););
}

impl AudioServerBackendFacade for MqttComponentFacade {
    p_bin!(publish_audio_frame(frame: AudioFrameMessage)
        { &HermesTopic::AudioServer(AudioServerCommand::AudioFrame(frame.site_id)) }
        { frame.wav_frame });
    s_bin!(subscribe_play_bytes<PlayBytesMessage>(site_id: SiteId) { &HermesTopic::AudioServer(AudioServerCommand::PlayBytes(site_id, "#".into())) }
            |topic, bytes| {
                if let &HermesTopic::AudioServer(AudioServerCommand::PlayBytes(ref site_id, ref id)) = topic {
                    PlayBytesMessage { site_id : site_id.to_owned(), id : id.to_owned(), wav_bytes : bytes.into() }
                } else {
                    unreachable!()
                }
            });
    p!(publish_play_finished<PlayFinishedMessage> &HermesTopic::AudioServer(AudioServerCommand::PlayFinished););
}

impl DialogueFacade for MqttToggleableComponentFacade {
    s!(subscribe_session_started<SessionStartedMessage> &HermesTopic::DialogueManager(DialogueManagerCommand::SessionStarted););
    s!(subscribe_intent<IntentMessage>(intent_name: String) { &HermesTopic::Intent(intent_name) });
    s!(subscribe_intents<IntentMessage> &HermesTopic::Intent("#".into()););
    s!(subscribe_session_ended<SessionEndedMessage> &HermesTopic::DialogueManager(DialogueManagerCommand::SessionEnded););
    p!(publish_start_session<StartSessionMessage> &HermesTopic::DialogueManager(DialogueManagerCommand::StartSession););
    p!(publish_continue_session<ContinueSessionMessage> &HermesTopic::DialogueManager(DialogueManagerCommand::ContinueSession););
    p!(publish_end_session<EndSessionMessage> &HermesTopic::DialogueManager(DialogueManagerCommand::EndSession););
}

impl DialogueBackendFacade for MqttToggleableComponentFacade {
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
            toggle_on_topic: HermesTopic::Hotword(HotwordCommand::ToggleOn),
            toggle_off_topic: HermesTopic::Hotword(HotwordCommand::ToggleOff)
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
        self.component(Component::AudioServer)
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
        self.component(Component::AudioServer)
    }

    fn dialogue(&self) -> Box<DialogueFacade> {
        self.dialogue_component()
    }

    fn dialogue_backend(&self) -> Box<DialogueBackendFacade> {
        self.dialogue_component()
    }
}

pub trait ToPath: ToString {
    fn as_path(&self) -> String {
        let raw_path = self.to_string();
        let mut c = raw_path.chars();

        match c.next() {
            None => String::new(),
            Some(f) => f.to_lowercase().chain(c).collect(),
        }
    }
}

pub trait FromPath<T: Sized> {
    fn from_path(&str) -> Option<T>;
}

// - Topics

#[derive(Debug, Clone, PartialEq)]
pub enum HermesTopic {
    Feedback(FeedbackCommand),
    DialogueManager(DialogueManagerCommand),
    Hotword(HotwordCommand),
    Asr(AsrCommand),
    Tts(TtsCommand),
    Nlu(NluCommand),
    Intent(String),
    AudioServer(AudioServerCommand),
    Component(Component, ComponentCommand),
}

impl ToPath for HermesTopic {}

impl FromPath<Self> for HermesTopic {
    fn from_path(path: &str) -> Option<Self> {
        let feedback = SoundCommand::iter().map(|cmd| HermesTopic::Feedback(FeedbackCommand::Sound(cmd)));
        let hotword = HotwordCommand::iter().map(HermesTopic::Hotword);
        let asr = AsrCommand::iter().map(HermesTopic::Asr);
        let tts = TtsCommand::iter().map(HermesTopic::Tts);
        let nlu = NluCommand::iter().map(HermesTopic::Nlu);
        let audio_server = vec![HermesTopic::AudioServer(AudioServerCommand::PlayFinished)];
        let component = ComponentCommand::iter().flat_map(|cmd| {
            Component::iter()
                .map(|component| HermesTopic::Component(component, cmd))
                .collect::<Vec<HermesTopic>>()
        });
        let dialogue_manager = DialogueManagerCommand::iter().map(HermesTopic::DialogueManager);
        let path_buf = path::PathBuf::from(path);
        let path_components = path_buf.components()
            .collect::<Vec<::std::path::Component>>();
        let parametric1 = if path_components.len() >= 1 {
            let p = path_components[path_components.len() - 1].as_os_str().to_string_lossy();
            vec![HermesTopic::Intent(p.to_string()),
                 HermesTopic::AudioServer(AudioServerCommand::AudioFrame(p.into()))]
        } else {
            vec![]
        };
        let parametric2 = if path_components.len() >= 2 {
            let p1 = path_components[path_components.len() - 2].as_os_str().to_string_lossy();
            let p2 = path_components[path_components.len() - 1].as_os_str().to_string_lossy();
            vec![HermesTopic::AudioServer(AudioServerCommand::PlayBytes(p1.into(), p2.into()))]
        } else {
            vec![]
        };
        feedback
            .chain(hotword)
            .chain(asr)
            .chain(tts)
            .chain(nlu)
            .chain(audio_server)
            .chain(dialogue_manager)
            .chain(component)
            .chain(parametric1)
            .chain(parametric2)
            .into_iter()
            .find(|p| p.as_path() == path)
    }
}

impl std::fmt::Display for HermesTopic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let subpath = match *self {
            HermesTopic::Feedback(ref cmd) => format!("feedback/{}", cmd.as_path()),
            HermesTopic::Hotword(ref cmd) => format!("{}/{}", Component::Hotword.as_path(), cmd.as_path()),
            HermesTopic::Asr(ref cmd) => format!("{}/{}", Component::Asr.as_path(), cmd.as_path()),
            HermesTopic::Tts(ref cmd) => format!("{}/{}", Component::Tts.as_path(), cmd.as_path()),
            HermesTopic::Nlu(ref cmd) => format!("{}/{}", Component::Nlu.as_path(), cmd.as_path()),
            HermesTopic::DialogueManager(ref cmd) => format!("{}/{}", Component::DialogueManager.as_path(), cmd.as_path()),
            HermesTopic::Intent(ref intent_name) => format!("intent/{}", intent_name),
            HermesTopic::AudioServer(ref cmd) => format!("{}/{}", Component::AudioServer.as_path(), cmd.as_path()),
            HermesTopic::Component(ref component, ref cmd) => format!("component/{}/{}", component.as_path(), cmd.as_path()),
        };
        write!(f, "hermes/{}", subpath)
    }
}

// - Components

#[derive(Debug, Clone, Copy, PartialEq, ToString, EnumIter)]
pub enum Component {
    Hotword,
    Asr,
    Tts,
    Nlu,
    DialogueManager,
    IntentParserManager,
    SkillManager,
    AudioServer,
}

impl ToPath for Component {}

// - Commands

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FeedbackCommand {
    Sound(SoundCommand),
}

impl ToPath for FeedbackCommand {}

impl std::fmt::Display for FeedbackCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let subpath = match *self {
            FeedbackCommand::Sound(ref cmd) => format!("sound/{}", cmd.as_path()),
        };
        write!(f, "{}", subpath)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, ToString, EnumIter)]
pub enum SoundCommand {
    ToggleOn,
    ToggleOff,
}

impl ToPath for SoundCommand {}

#[derive(Debug, Clone, Copy, PartialEq, ToString, EnumIter)]
pub enum DialogueManagerCommand {
    ToggleOn,
    ToggleOff,
    StartSession,
    ContinueSession,
    EndSession,
    SessionStarted,
    SessionEnded,
}


impl ToPath for DialogueManagerCommand {}

#[derive(Debug, Clone, Copy, PartialEq, ToString, EnumIter)]
pub enum HotwordCommand {
    ToggleOn,
    ToggleOff,
    Wait,
    Detected
}

impl ToPath for HotwordCommand {}

#[derive(Debug, Clone, Copy, PartialEq, ToString, EnumIter)]
pub enum AsrCommand {
    ToggleOn,
    ToggleOff,
    TextCaptured,
    PartialTextCaptured,
}

impl ToPath for AsrCommand {}

#[derive(Debug, Clone, Copy, PartialEq, ToString, EnumIter)]
pub enum TtsCommand {
    Say,
    SayFinished,
}

impl ToPath for TtsCommand {}

#[derive(Debug, Clone, Copy, PartialEq, ToString, EnumIter)]
pub enum NluCommand {
    Query,
    PartialQuery,
    SlotParsed,
    IntentParsed,
    IntentNotRecognized,
}

impl ToPath for NluCommand {}

#[derive(Debug, Clone, PartialEq)]
pub enum AudioServerCommand {
    AudioFrame(SiteId),
    PlayBytes(SiteId, String),
    PlayFinished,
}

impl std::fmt::Display for AudioServerCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let subpath = match *self {
            AudioServerCommand::AudioFrame(ref site_id) => format!("audioFrame/{}", site_id),
            AudioServerCommand::PlayBytes(ref site_id, ref id) => format!("playBytes/{}/{}", site_id, id),
            AudioServerCommand::PlayFinished => "playFinished".into(),
        };
        write!(f, "{}", subpath)
    }
}

impl ToPath for AudioServerCommand {}

#[derive(Debug, Clone, Copy, PartialEq, ToString, EnumIter)]
pub enum ComponentCommand {
    VersionRequest,
    Version,
    Error,
}

impl ToPath for ComponentCommand {}

