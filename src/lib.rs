pub enum HermesProtocol {
    Hotword,
    Intent,
    Speech,
    SpeechToText,
    TextToSpeech,
}

pub trait ToTopic: Sized {
    fn from_topic(topic: &str) -> Option<Self>;
    fn as_topic(&self) -> &str;
}

impl HermesProtocol {
    fn all() -> Vec<Self> {
        vec![
            HermesProtocol::Hotword,
            HermesProtocol::Intent,
            HermesProtocol::Speech,
            HermesProtocol::SpeechToText,
            HermesProtocol::TextToSpeech,
        ]
    }
}

impl ToTopic for HermesProtocol {
    fn from_topic(topic: &str) -> Option<Self> {
        HermesProtocol::all().into_iter().find(|m| m.as_topic() == topic)
    }

    fn as_topic(&self) -> &str {
        match *self {
            HermesProtocol::Hotword => "hermes/hotword",
            HermesProtocol::Intent => "hermes/intent",
            HermesProtocol::Speech => "hermes/speech",
            HermesProtocol::SpeechToText => "hermes/stt",
            HermesProtocol::TextToSpeech => "hermes/tts",
        }
    }
}
