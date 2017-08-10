error_chain! {
    foreign_links {
        Serde(::serde_json::Error);
        Rumqtt(::rumqtt::Error)  #[cfg(feature = "mqtt")];
    }
}

impl<T> ::std::convert::From<::std::sync::PoisonError<T>> for Error {
    fn from(pe: ::std::sync::PoisonError<T>) -> Error {
        format!("Poisoning error: {:?}", pe).into()
    }
}
