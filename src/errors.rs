error_chain! {
    foreign_links {
        Serde(::serde_json::Error);
        Rumqtt(::rumqtt::Error) #[cfg(feature = "mqtt")];
        MqttTopicFilter(::rumqtt::TopicFilterError) #[cfg(feature = "mqtt")];
        NulError(::std::ffi::NulError) #[cfg(feature = "ffi")];
        QueriesOntology(::snips_queries_ontology::OntologyError) #[cfg(feature = "ffi")];
    }
}

impl<T> ::std::convert::From<::std::sync::PoisonError<T>> for Error {
    fn from(pe: ::std::sync::PoisonError<T>) -> Error {
        format!("Poisoning error: {:?}", pe).into()
    }
}
