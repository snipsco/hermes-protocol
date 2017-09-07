use strum::IntoEnumIterator;

use std::collections::HashMap;
use std::path;
use std::string::ToString;
use std::sync::Arc;
use std::sync::Mutex;

use super::*;

use errors::*;

// this is a submodule to prevent rust from complaining of the leakage of private types
mod handler {
    use super::*;

    pub struct MqttHandler {
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

    pub trait HasMqttHandler { fn get_mqtt_handler(&self) -> Arc<MqttHandler>; }

    pub trait HasComponent { fn get_component(&self) -> Component; }

    pub trait HasToggleTopics {
        fn get_toggle_on_topic(&self) -> &HermesTopic;
        fn get_toggle_off_topic(&self) -> &HermesTopic;
    }
}

use self::handler::*;

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

impl<T> ComponentFacade for T where T: HasMqttHandler + HasComponent + Send + Sync {
    fn publish_version_request(&self) -> Result<()> {
        self.get_mqtt_handler().publish(&HermesTopic::Component(self.get_component(), ComponentCommand::VersionRequest))
    }

    fn subscribe_version(&self, handler: Callback<VersionMessage>) -> Result<()> {
        self.get_mqtt_handler().subscribe_payload(&HermesTopic::Component(self.get_component(), ComponentCommand::Version), move |p| handler.call(p))
    }

    fn subscribe_error(&self, handler: Callback<ErrorMessage>) -> Result<()> {
        self.get_mqtt_handler().subscribe_payload(&HermesTopic::Component(self.get_component(), ComponentCommand::Error), move |p| handler.call(p))
    }
}

impl<T> ComponentBackendFacade for T where T: HasMqttHandler + HasComponent + Send + Sync {
    fn subscribe_version_request(&self, handler: Callback0) -> Result<()> {
        self.get_mqtt_handler().subscribe(&HermesTopic::Component(self.get_component(), ComponentCommand::VersionRequest), move || handler.call())
    }

    fn publish_version(&self, version: VersionMessage) -> Result<()> {
        self.get_mqtt_handler().publish_payload(&HermesTopic::Component(self.get_component(), ComponentCommand::Version), version)
    }

    fn publish_error(&self, error: ErrorMessage) -> Result<()> {
        self.get_mqtt_handler().publish_payload(&HermesTopic::Component(self.get_component(), ComponentCommand::Error), error)
    }
}

impl<T> ToggleableFacade for T where T: HasMqttHandler + HasToggleTopics + Send + Sync {
    fn publish_toggle_on(&self) -> Result<()> {
        self.get_mqtt_handler().publish(self.get_toggle_on_topic())
    }

    fn publish_toggle_off(&self) -> Result<()> {
        self.get_mqtt_handler().publish(self.get_toggle_off_topic())
    }
}

impl<T> ToggleableBackendFacade for T where T: HasMqttHandler + HasToggleTopics + Send + Sync {
    fn subscribe_toggle_on(&self, handler: Callback0) -> Result<()> {
        self.get_mqtt_handler().subscribe(self.get_toggle_on_topic(), move || handler.call())
    }

    fn subscribe_toggle_off(&self, handler: Callback0) -> Result<()> {
        self.get_mqtt_handler().subscribe(self.get_toggle_off_topic(), move || handler.call())
    }
}

struct MqttComponentFacade {
    component: Component,
    mqtt_handler: Arc<MqttHandler>
}

impl HasMqttHandler for MqttComponentFacade {
    fn get_mqtt_handler(&self) -> Arc<MqttHandler> {
        Arc::clone(&self.mqtt_handler)
    }
}

impl HasComponent for MqttComponentFacade {
    fn get_component(&self) -> Component {
        self.component
    }
}

struct MqttToggleableFacade {
    toggle_on_topic: HermesTopic,
    toggle_off_topic: HermesTopic,
    mqtt_handler: Arc<MqttHandler>
}

impl HasMqttHandler for MqttToggleableFacade {
    fn get_mqtt_handler(&self) -> Arc<MqttHandler> {
        self.mqtt_handler.clone()
    }
}

impl HasToggleTopics for MqttToggleableFacade {
    fn get_toggle_on_topic(&self) -> &HermesTopic {
        &self.toggle_on_topic
    }

    fn get_toggle_off_topic(&self) -> &HermesTopic {
        &self.toggle_off_topic
    }
}

struct MqttToggleableComponentFacade {
    component: Component,
    toggle_on_topic: HermesTopic,
    toggle_off_topic: HermesTopic,
    mqtt_handler: Arc<MqttHandler>
}

impl HasMqttHandler for MqttToggleableComponentFacade {
    fn get_mqtt_handler(&self) -> Arc<MqttHandler> {
        self.mqtt_handler.clone()
    }
}

impl HasComponent for MqttToggleableComponentFacade {
    fn get_component(&self) -> Component {
        self.component
    }
}

impl HasToggleTopics for MqttToggleableComponentFacade {
    fn get_toggle_on_topic(&self) -> &HermesTopic {
        &self.toggle_on_topic
    }

    fn get_toggle_off_topic(&self) -> &HermesTopic {
        &self.toggle_off_topic
    }
}

impl HotwordFacade for MqttToggleableComponentFacade {
    fn publish_wait(&self) -> Result<()> {
        self.mqtt_handler.publish(&HermesTopic::Hotword(HotwordCommand::Wait))
    }

    fn subscribe_detected(&self, handler: Callback0) -> Result<()> {
        self.mqtt_handler.subscribe(&HermesTopic::Hotword(HotwordCommand::Detected), move || handler.call())
    }
}

impl HotwordBackendFacade for MqttToggleableComponentFacade {
    fn publish_detected(&self) -> Result<()> {
        self.mqtt_handler.publish(&HermesTopic::Hotword(HotwordCommand::Detected))
    }

    fn subscribe_wait(&self, handler: Callback0) -> Result<()> {
        self.mqtt_handler.subscribe(&HermesTopic::Hotword(HotwordCommand::Wait), move || handler.call())
    }
}

impl SoundFeedbackFacade for MqttToggleableFacade {}

impl SoundFeedbackBackendFacade for MqttToggleableFacade {}

impl AsrFacade for MqttToggleableComponentFacade {
    fn subscribe_text_captured(&self, handler: Callback<TextCapturedMessage>) -> Result<()> {
        self.mqtt_handler.subscribe_payload(&HermesTopic::Asr(AsrCommand::TextCaptured), move |p| handler.call(p))
    }

    fn subscribe_partial_text_captured(&self, handler: Callback<TextCapturedMessage>) -> Result<()> {
        self.mqtt_handler.subscribe_payload(&HermesTopic::Asr(AsrCommand::PartialTextCaptured), move |p| handler.call(p))
    }
}

impl AsrBackendFacade for MqttToggleableComponentFacade {
    fn publish_text_captured(&self, text_captured: TextCapturedMessage) -> Result<()> {
        self.mqtt_handler.publish_payload(&HermesTopic::Asr(AsrCommand::TextCaptured), text_captured)
    }

    fn publish_partial_text_captured(&self, text_captured: TextCapturedMessage) -> Result<()> {
        self.mqtt_handler.publish_payload(&HermesTopic::Asr(AsrCommand::PartialTextCaptured), text_captured)
    }
}

impl TtsFacade for MqttComponentFacade {
    fn publish_say(&self, to_say: SayMessage) -> Result<()> {
        self.mqtt_handler.publish_payload(&HermesTopic::Tts(TtsCommand::Say), to_say)
    }

    fn subscribe_say_finished(&self, handler: Callback0) -> Result<()> {
        self.mqtt_handler.subscribe(&HermesTopic::Tts(TtsCommand::SayFinished), move || handler.call())
    }
}

impl TtsBackendFacade for MqttComponentFacade {
    fn publish_say_finished(&self) -> Result<()> {
        self.mqtt_handler.publish(&HermesTopic::Tts(TtsCommand::SayFinished))
    }

    fn subscribe_say(&self, handler: Callback<SayMessage>) -> Result<()> {
        self.mqtt_handler.subscribe_payload(&HermesTopic::Tts(TtsCommand::Say), move |p| handler.call(p))
    }
}

impl NluFacade for MqttComponentFacade {
    fn publish_query(&self, query: NluQueryMessage) -> Result<()> {
        self.mqtt_handler.publish_payload(&HermesTopic::Nlu(NluCommand::Query), query)
    }

    fn publish_partial_query(&self, query: NluSlotQueryMessage) -> Result<()> {
        self.mqtt_handler.publish_payload(&HermesTopic::Nlu(NluCommand::PartialQuery), query)
    }

    fn subscribe_slot_parsed(&self, handler: Callback<SlotMessage>) -> Result<()> {
        self.mqtt_handler.subscribe_payload(&HermesTopic::Nlu(NluCommand::SlotParsed), move |p| handler.call(p))
    }

    fn subscribe_intent_parsed(&self, handler: Callback<IntentMessage>) -> Result<()> {
        self.mqtt_handler.subscribe_payload(&HermesTopic::Nlu(NluCommand::IntentParsed), move |p| handler.call(p))
    }

    fn subscribe_intent_not_recognized(&self, handler: Callback<IntentNotRecognizedMessage>) -> Result<()> {
        self.mqtt_handler.subscribe_payload(&HermesTopic::Nlu(NluCommand::IntentNotRecognized), move |p| handler.call(p))
    }
}

impl NluBackendFacade for MqttComponentFacade {
    fn subscribe_query(&self, handler: Callback<NluQueryMessage>) -> Result<()> {
        self.mqtt_handler.subscribe_payload(&HermesTopic::Nlu(NluCommand::Query), move |p| handler.call(p))
    }

    fn subscribe_partial_query(&self, handler: Callback<NluSlotQueryMessage>) -> Result<()> {
        self.mqtt_handler.subscribe_payload(&HermesTopic::Nlu(NluCommand::PartialQuery), move |p| handler.call(p))
    }

    fn publish_slot_parsed(&self, slot: SlotMessage) -> Result<()> {
        self.mqtt_handler.publish_payload(&HermesTopic::Nlu(NluCommand::SlotParsed), slot)
    }

    fn publish_intent_parsed(&self, intent: IntentMessage) -> Result<()> {
        self.mqtt_handler.publish_payload(&HermesTopic::Nlu(NluCommand::IntentParsed), intent)
    }

    fn publish_intent_not_recognized(&self, status: IntentNotRecognizedMessage) -> Result<()> {
        self.mqtt_handler.publish_payload(&HermesTopic::Nlu(NluCommand::IntentNotRecognized), status)
    }
}

impl AudioServerFacade for MqttComponentFacade {
    fn publish_play_file(&self, file: PlayFileMessage) -> Result<()> {
        self.mqtt_handler.publish_payload(&HermesTopic::AudioServer(AudioServerCommand::PlayFile), file)
    }

    fn publish_play_bytes(&self, bytes: PlayBytesMessage) -> Result<()> {
        self.mqtt_handler.publish_payload(&HermesTopic::AudioServer(AudioServerCommand::PlayBytes), bytes)
    }

    fn subscribe_play_finished(&self, handler: Callback<PlayFinishedMessage>) -> Result<()> {
        self.mqtt_handler.subscribe_payload(&HermesTopic::AudioServer(AudioServerCommand::PlayFinished), move |p| handler.call(p))
    }
}

impl AudioServerBackendFacade for MqttComponentFacade {
    fn subscribe_play_bytes(&self, handler: Callback<PlayBytesMessage>) -> Result<()> {
        self.mqtt_handler.subscribe_payload(&HermesTopic::AudioServer(AudioServerCommand::PlayBytes), move |p| handler.call(p))
    }

    fn subscribe_play_file(&self, handler: Callback<PlayFileMessage>) -> Result<()> {
        self.mqtt_handler.subscribe_payload(&HermesTopic::AudioServer(AudioServerCommand::PlayFile), move |p| handler.call(p))
    }

    fn publish_play_finished(&self, status: PlayFinishedMessage) -> Result<()> {
        self.mqtt_handler.publish_payload(&HermesTopic::AudioServer(AudioServerCommand::PlayFinished), status)
    }
}

impl DialogueFacade for MqttToggleableComponentFacade {
    fn subscribe_intent(&self, intent_name: String, handler: Callback<IntentMessage>) -> Result<()> {
        self.mqtt_handler.subscribe_payload(&HermesTopic::Intent(intent_name), move |p| handler.call(p))
    }

    fn subscribe_intents(&self, handler: Callback<IntentMessage>) -> Result<()> {
        self.mqtt_handler.subscribe_payload(&HermesTopic::Intent("#".into()), move |p| handler.call(p))
    }
}

impl DialogueBackendFacade for MqttToggleableComponentFacade {
    fn publish_intent(&self, intent: IntentMessage) -> Result<()> {
        self.mqtt_handler.publish_payload(&HermesTopic::Intent(intent.intent.intent_name.clone()), intent)
    }
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
        let audio_server = AudioServerCommand::iter().map(HermesTopic::AudioServer);
        let component = ComponentCommand::iter().flat_map(|cmd| {
            Component::iter()
                .map(|component| HermesTopic::Component(component, cmd))
                .collect::<Vec<HermesTopic>>()
        });
        let intent = if let Some(last_component) = path::PathBuf::from(path).components().last() {
            last_component.as_os_str().to_str()
                .map(|intent_name| vec![HermesTopic::Intent(intent_name.to_string())])
                .unwrap_or(vec![])
        } else {
            vec![]
        };

        feedback
            .chain(hotword)
            .chain(asr)
            .chain(tts)
            .chain(nlu)
            .chain(audio_server)
            .chain(component)
            .chain(intent)
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

#[derive(Debug, Clone, Copy, PartialEq, ToString, EnumIter)]
pub enum AudioServerCommand {
    PlayFile,
    PlayBytes,
    PlayFinished,
}

impl ToPath for AudioServerCommand {}

#[derive(Debug, Clone, Copy, PartialEq, ToString, EnumIter)]
pub enum ComponentCommand {
    VersionRequest,
    Version,
    Error,
}

impl ToPath for ComponentCommand {}

