#[macro_export]
macro_rules! t {
    (
        $name:ident :
        $s_facade:ident.
        $s:ident <=
        $t:ty |
        $p_facade:ident.
        $p:ident with
        $object:expr;
    ) => {
        #[test]
        fn $name() {
            let (handler_source, handler_receiver) = create_handlers();
            let source = handler_source.$p_facade();
            let receiver = handler_receiver.$s_facade();
            let (tx, rx) = ::std::sync::mpsc::channel();
            let tx = ::std::sync::Mutex::new(tx);
            receiver
                .$s(::Callback::new(move |o: &$t| {
                    tx.lock().map(|it| it.send(o.clone())).unwrap().unwrap()
                }))
                .unwrap();
            let message = $object;
            ::std::thread::sleep(WAIT_DURATION);
            source.$p(message.clone()).unwrap();
            let result = rx.recv_timeout(::std::time::Duration::from_secs(1));
            assert!(result.is_ok(), "didn't receive message after one second");
            assert_eq!(result.unwrap(), message)
        }
    };
    ($name:ident : $s_facade:ident. $s:ident <= $p_facade:ident. $p:ident) => {
        #[test]
        fn $name() {
            let (handler_source, handler_receiver) = create_handlers();
            let source = handler_source.$p_facade();
            let receiver = handler_receiver.$s_facade();
            let (tx, rx) = ::std::sync::mpsc::channel();
            let tx = ::std::sync::Mutex::new(tx);
            receiver
                .$s(::Callback0::new(move || {
                    tx.lock().map(|it| it.send(())).unwrap().unwrap()
                }))
                .unwrap();
            ::std::thread::sleep(WAIT_DURATION);
            source.$p().unwrap();
            let result = rx.recv_timeout(::std::time::Duration::from_secs(1));
            assert!(result.is_ok(), "didn't receive message after one second");
        }
    };
    ($name:ident : $s_facade:ident. $s:ident $a:block <= $p_facade:ident. $p:ident) => {
        #[test]
        fn $name() {
            let (handler_source, handler_receiver) = create_handlers();
            let source = handler_source.$p_facade();
            let receiver = handler_receiver.$s_facade();
            let (tx, rx) = ::std::sync::mpsc::channel();
            let tx = ::std::sync::Mutex::new(tx);
            receiver
                .$s(
                    $a,
                    ::Callback0::new(move || tx.lock().map(|it| it.send(())).unwrap().unwrap()),
                )
                .unwrap();
            ::std::thread::sleep(WAIT_DURATION);
            source.$p($a).unwrap();
            let result = rx.recv_timeout(::std::time::Duration::from_secs(1));
            assert!(result.is_ok(), "didn't receive message after one second");
        }
    };
    (
        $name:ident :
        $s_facade:ident.
        $s:ident
        $a:block <=
        $t:ty |
        $p_facade:ident.
        $p:ident with
        $object:expr;
    ) => {
        #[test]
        fn $name() {
            let (handler_source, handler_receiver) = create_handlers();
            let source = handler_source.$p_facade();
            let receiver = handler_receiver.$s_facade();
            let (tx, rx) = ::std::sync::mpsc::channel();
            let tx = ::std::sync::Mutex::new(tx);
            receiver
                .$s(
                    $a,
                    ::Callback::new(move |o: &$t| {
                        tx.lock().map(|it| it.send(o.clone())).unwrap().unwrap()
                    }),
                )
                .unwrap();
            let message = $object;
            ::std::thread::sleep(WAIT_DURATION);
            source.$p($a, message.clone()).unwrap();
            let result = rx.recv_timeout(::std::time::Duration::from_secs(1));
            assert!(result.is_ok(), "didn't receive message after one second");
            assert_eq!(result.unwrap(), message)
        }
    };
    ($name:ident : OneToMany $s_facade:ident. $s:ident $a:block <= $p_facade:ident. $p:ident) => {
        #[test]
        fn $name() {
            let (handler_source, handler_receiver) = create_handlers();
            let source = handler_source.$p_facade();
            let receiver = handler_receiver.$s_facade();
            let (tx, rx) = ::std::sync::mpsc::channel();
            let tx = ::std::sync::Mutex::new(tx);
            receiver
                .$s(
                    $a,
                    ::Callback0::new(move || tx.lock().map(|it| it.send(())).unwrap().unwrap()),
                )
                .unwrap();
            ::std::thread::sleep(WAIT_DURATION);
            source.$p($a).unwrap();
            let result = rx.recv_timeout(::std::time::Duration::from_secs(1));
            assert!(result.is_ok(), "didn't receive message after one second");
        }
    };
    (
        $name:ident : OneToMany
        $s_facade:ident.
        $s:ident
        $a:block <=
        $t:ty |
        $p_facade:ident.
        $p:ident with
        $object:expr;
    ) => {
        #[test]
        fn $name() {
            let (handler_source, handler_receiver) = create_handlers();
            let source = handler_source.$p_facade();
            let receiver = handler_receiver.$s_facade();
            let (tx, rx) = ::std::sync::mpsc::channel();
            let tx = ::std::sync::Mutex::new(tx);
            receiver
                .$s(
                    $a,
                    ::Callback::new(move |o: &$t| {
                        tx.lock().map(|it| it.send(o.clone())).unwrap().unwrap()
                    }),
                )
                .unwrap();
            let message = $object;
            ::std::thread::sleep(WAIT_DURATION);
            source.$p(message.clone()).unwrap();
            let result = rx.recv_timeout(::std::time::Duration::from_secs(1));
            assert!(result.is_ok(), "didn't receive message after one second");
            assert_eq!(result.unwrap(), message)
        }
    };
    ($name:ident : ManyToOne $s_facade:ident. $s:ident <= $p_facade:ident. $p:ident $a:block) => {
        #[test]
        fn $name() {
            let (handler_source, handler_receiver) = create_handlers();
            let source = handler_source.$p_facade();
            let receiver = handler_receiver.$s_facade();
            let (tx, rx) = ::std::sync::mpsc::channel();
            let tx = ::std::sync::Mutex::new(tx);
            receiver
                .$s(::Callback0::new(move || {
                    tx.lock().map(|it| it.send(())).unwrap().unwrap()
                }))
                .unwrap();
            ::std::thread::sleep(WAIT_DURATION);
            source.$p($a).unwrap();
            let result = rx.recv_timeout(::std::time::Duration::from_secs(1));
            assert!(result.is_ok(), "didn't receive message after one second");
        }
    };
    (
        $name:ident : ManyToOne
        $s_facade:ident.
        $s:ident <=
        $t:ty |
        $p_facade:ident.
        $p:ident
        $a:block with
        $object:expr;
    ) => {
        #[test]
        fn $name() {
            let (handler_source, handler_receiver) = create_handlers();
            let source = handler_source.$p_facade();
            let receiver = handler_receiver.$s_facade();
            let (tx, rx) = ::std::sync::mpsc::channel();
            let tx = ::std::sync::Mutex::new(tx);
            receiver
                .$s(::Callback::new(move |o: &$t| {
                    tx.lock().map(|it| it.send(o.clone())).unwrap().unwrap()
                }))
                .unwrap();
            let message = $object;
            ::std::thread::sleep(WAIT_DURATION);
            source.$p($a, message.clone()).unwrap();
            let result = rx.recv_timeout(::std::time::Duration::from_secs(1));
            assert!(result.is_ok(), "didn't receive message after one second");
            assert_eq!(result.unwrap(), message)
        }
    };
}

#[macro_export]
macro_rules! t_toggleable {
    ($name:ident : $f_back:ident | $f:ident) => {
        mod $name {
            use super::*;
            t!(toggle_on_works:
                                $f_back.subscribe_toggle_on <= $f.publish_toggle_on);
            t!(toggle_off_works:
                                $f_back.subscribe_toggle_off <= $f.publish_toggle_off);
        }
    };
}

#[macro_export]
macro_rules! t_identifiable_toggleable {
        ($name:ident: $f_back:ident | $f:ident) => {
            mod $name {
                use super::*;
                t!(toggle_on_works:
                        $f_back.subscribe_toggle_on <= SiteMessage | $f.publish_toggle_on
                        with SiteMessage { session_id: Some("abc".into()), site_id: "some site".into() };);
                t!(toggle_off_works:
                        $f_back.subscribe_toggle_off <= SiteMessage | $f.publish_toggle_off
                        with SiteMessage { session_id: Some("abc".into()), site_id: "some site".into() };);
            }
        };
    }

#[macro_export]
macro_rules! t_component {
        ($name:ident: $f_back:ident | $f:ident) => {
            mod $name {
                use super::*;
                t!(version_request_works:
                        $f_back.subscribe_version_request <= $f.publish_version_request);
                t!(version_works:
                        $f.subscribe_version <= VersionMessage | $f_back.publish_version
                        with VersionMessage { version: ::semver::Version { major: 1, minor: 0, patch: 0, pre: vec![], build: vec![]} };);
                t!(error_works:
                        $f.subscribe_error <= ErrorMessage | $f_back.publish_error
                        with ErrorMessage { session_id: Some("123abc".into()), error: "some error".into(), context: None };);
            }
        };
    }

#[macro_export]
macro_rules! t_identifiable_component {
        ($name:ident: $f_back:ident | $f:ident) => {
            mod $name {
                use super::*;
                t!(version_request_works:
                        $f_back.subscribe_version_request { "identifier".to_string() } <= $f.publish_version_request);
                t!(version_works:
                        $f.subscribe_version { "identifier".to_string() } <= VersionMessage | $f_back.publish_version
                        with VersionMessage { version: ::semver::Version { major: 1, minor: 0, patch: 0, pre: vec![], build: vec![]} };);
                t!(error_works:
                        $f.subscribe_error { "identifier".to_string() } <= ErrorMessage | $f_back.publish_error
                        with ErrorMessage { session_id: Some("123abc".into()), error: "some error".into(), context: None };);
            }
        };
    }

#[macro_export]
macro_rules! test_suite {
    () => {
        test_suite!(WAIT_DURATION = 0);
    };

    (WAIT_DURATION = $wait_duration:expr) => {
        use snips_nlu_ontology::*;

        const WAIT_DURATION: ::std::time::Duration = ::std::time::Duration::from_millis($wait_duration);

        t_identifiable_component!(hotword_identifiable_component: hotword_backend | hotword);
        t_identifiable_toggleable!(hotword_identifiable_toggleable: hotword_backend | hotword);
        t!(hotword_detected_works:
                    hotword.subscribe_detected { "hotword_identifier".into() } <= HotwordDetectedMessage | hotword_backend.publish_detected
                    with HotwordDetectedMessage { model_id: "some model".into(), site_id: "some site".into(), model_type: Some(::hermes::HotwordModelType::Universal), model_version: Some("1.2.3".into()), current_sensitivity: Some(0.5) };);
        t!(hotword_all_detected_works:
                    ManyToOne
                    hotword.subscribe_all_detected <= HotwordDetectedMessage | hotword_backend.publish_detected { "hotword_identifier".into() }
                    with HotwordDetectedMessage { model_id: "some model".into(), site_id: "some site".into(), model_type: Some(::hermes::HotwordModelType::Universal), model_version: Some("1.2.3".into()), current_sensitivity: Some(0.5) };);

        t_identifiable_toggleable!(sound_feedback_identifiable_toggleable: sound_feedback_backend | sound_feedback );

        t_component!(asr_component: asr_backend | asr);
        t_toggleable!(asr_toggleable: asr_backend | asr);
        t!(asr_text_captured_works:
                    asr.subscribe_text_captured <= TextCapturedMessage | asr_backend.publish_text_captured
                    with TextCapturedMessage { text: "hello world".into(), likelihood: 0.5, seconds: 4.2, site_id: "Some site".into(), session_id: Some("123abc".into()) };);
        t!(asr_partial_text_captured_works:
                    asr.subscribe_partial_text_captured <= TextCapturedMessage | asr_backend.publish_partial_text_captured
                    with TextCapturedMessage { text: "hello world".into(), likelihood: 0.5, seconds: 4.2, site_id: "Some site".into(), session_id: Some("123abc".into()) };);
        t!(asr_start_listening:
                    asr_backend.subscribe_start_listening <= SiteMessage | asr.publish_start_listening
                    with SiteMessage { session_id: Some("abc".into()), site_id: "some site".into() };);
        t!(asr_stop_listening:
                    asr_backend.subscribe_stop_listening <= SiteMessage | asr.publish_stop_listening
                    with SiteMessage { session_id: Some("abc".into()), site_id: "some site".into() };);
        t!(asr_reload:
                asr_backend.subscribe_reload <= asr.publish_reload);
        t!(asr_injection_request:
                    asr_backend.subscribe_injection_request <= InjectionRequest | asr.publish_injection_request
                    with InjectionRequest { operations: vec![], lexicon: ::std::collections::HashMap::new() };);

        t_component!(tts_component: tts_backend | tts);
        t!(tts_say_works:
                    tts_backend.subscribe_say <= SayMessage | tts.publish_say
                    with SayMessage { text: "hello world".into(), lang: None, id: None, site_id: "some site".into(), session_id: Some("abc".into()) };
            );
        t!(tts_say_finished_works:
                    tts.subscribe_say_finished <= SayFinishedMessage | tts_backend.publish_say_finished
                    with SayFinishedMessage { id: Some("my id".into()), session_id: Some("abc".into()) };
            );

        t_component!(nlu_component: nlu_backend | nlu);
        t!(nlu_query_works:
                    nlu_backend.subscribe_query <= NluQueryMessage | nlu.publish_query
                    with NluQueryMessage { input: "hello world".into(), intent_filter: None, id: None, session_id: Some("abc".into()) };
            );
        t!(nlu_partial_query_works:
                    nlu_backend.subscribe_partial_query <= NluSlotQueryMessage | nlu.publish_partial_query
                    with NluSlotQueryMessage { input: "hello world".into(), intent_name: "my intent".into(), slot_name: "my slot".into(), id: None, session_id: Some("abc".into()) };
            );
        t!(nlu_slot_parsed_works:
                    nlu.subscribe_slot_parsed <= NluSlotMessage | nlu_backend.publish_slot_parsed
                    with NluSlotMessage { id: None, input: "some input".into(), intent_name: "some intent".into(), slot: Some(Slot { slot_name: "my slot".into(), raw_value: "value".into(), value: ::snips_nlu_ontology::SlotValue::Custom("my slot".into()), range: None, entity: "entity".into() }), session_id: Some("abc".into()) };
            );
        t!(nlu_intent_parsed_works:
                    nlu.subscribe_intent_parsed <= NluIntentMessage | nlu_backend.publish_intent_parsed
                    with NluIntentMessage {id: None, input: "hello world".into(), intent: IntentClassifierResult { intent_name: "my intent".into(), probability: 0.73 }, slots: None, session_id: Some("abc".into()) };);
        t!(nlu_intent_not_recognized_works:
                    nlu.subscribe_intent_not_recognized <= NluIntentNotRecognizedMessage | nlu_backend.publish_intent_not_recognized
                    with NluIntentNotRecognizedMessage {id: None, input: "hello world".into(), session_id: Some("abc".into()) };);

        t_identifiable_component!(audio_server_component: audio_server_backend | audio_server);
        t_identifiable_toggleable!(audio_server_toggeable: audio_server_backend | audio_server);
        t!(audio_server_play_bytes_works:
                    OneToMany
                    audio_server_backend.subscribe_play_bytes { "some site".into() } <= PlayBytesMessage | audio_server.publish_play_bytes
                    with PlayBytesMessage { wav_bytes: vec![42; 1000], id: "my id".into(), site_id: "some site".into() };
            );
        t!(audio_server_play_all_bytes_works:
                    audio_server_backend.subscribe_all_play_bytes <= PlayBytesMessage | audio_server.publish_play_bytes
                    with PlayBytesMessage { wav_bytes: vec![42; 1000], id: "my id".into(), site_id: "some site".into() };
            );
        t!(audio_server_play_finished_works:
                    OneToMany
                    audio_server.subscribe_play_finished { "some site".into() } <= PlayFinishedMessage | audio_server_backend.publish_play_finished
                    with PlayFinishedMessage { id: "my id".into(), site_id: "some site".into() };
            );
        t!(audio_server_all_play_finished_works:
                    audio_server.subscribe_all_play_finished <= PlayFinishedMessage | audio_server_backend.publish_play_finished
                    with PlayFinishedMessage { id: "my id".into(), site_id: "some site".into() };
            );
        t!(audio_server_audio_frame_works:
                    OneToMany
                    audio_server.subscribe_audio_frame { "some site".into() } <= AudioFrameMessage | audio_server_backend.publish_audio_frame
                    with AudioFrameMessage { wav_frame: vec![42; 1000], site_id: "some site".into() };
            );

        t_component!(dialogue_component: dialogue_backend | dialogue);
        t_toggleable!(dialogue_toggleable: dialogue_backend | dialogue);
        t!(dialogue_session_started_works:
                    dialogue.subscribe_session_started <= SessionStartedMessage | dialogue_backend.publish_session_started
                    with SessionStartedMessage { session_id: "some id".into(), custom_data: None, site_id: "some site".into(), reactivated_from_session_id: None };);
        t!(dialogue_session_queued_works:
                    dialogue.subscribe_session_queued <= SessionQueuedMessage | dialogue_backend.publish_session_queued
                    with SessionQueuedMessage { session_id: "some id".into(), custom_data: None, site_id: "some site".into() };);
        t!(dialogue_intents_works:
                    dialogue.subscribe_intents <= IntentMessage | dialogue_backend.publish_intent
                    with IntentMessage { site_id: "some site".into(), session_id: "some id".into(), custom_data: None, input: "hello world".into(), intent: IntentClassifierResult { intent_name: "my intent".into(), probability: 0.73 }, slots: None };);
        t!(dialogue_intent_works:
                    OneToMany
                    dialogue.subscribe_intent { "my intent".into() } <= IntentMessage | dialogue_backend.publish_intent
                    with IntentMessage { site_id: "some site".into(), session_id: "some id".into(), custom_data: None, input: "hello world".into(), intent: IntentClassifierResult { intent_name: "my intent".into(), probability: 0.73 }, slots: None };);
        t!(dialogue_intent_not_recognized_works:
                    dialogue.subscribe_intent_not_recognized <= IntentNotRecognizedMessage | dialogue_backend.publish_intent_not_recognized
                    with IntentNotRecognizedMessage { site_id: "some site".into(), session_id: "some id".into(), custom_data: None, input: Some("hello world".into()) };);
        t!(dialogue_session_ended_works:
                    dialogue.subscribe_session_ended <= SessionEndedMessage | dialogue_backend.publish_session_ended
                    with SessionEndedMessage { site_id: "some site".into(), session_id: "some id".into(), custom_data: None, termination: SessionTerminationType::Nominal };);
        t!(dialogue_start_session_works:
                    dialogue_backend.subscribe_start_session <= StartSessionMessage | dialogue.publish_start_session
                    with StartSessionMessage { init: SessionInit::Action { text: None, intent_filter: None, can_be_enqueued: false, send_intent_not_recognized: true }, custom_data: None, site_id: None };);
        t!(dialogue_continue_session_works:
                    dialogue_backend.subscribe_continue_session <= ContinueSessionMessage | dialogue.publish_continue_session
                    with ContinueSessionMessage { session_id: "some id".into(), text: "some text".into(), intent_filter: None, send_intent_not_recognized: true, custom_data: Some("custom data".into())  };);
        t!(dialogue_end_session_works:
                    dialogue_backend.subscribe_end_session <= EndSessionMessage | dialogue.publish_end_session
                    with EndSessionMessage { session_id: "some id".into(), text: None };);
    };
}
