use chrono::prelude::*;

pub fn now() -> DateTime<Utc> {
    Utc::now()
}

#[macro_export]
macro_rules! t {
    (
        $name:ident :
        $s_facade:ident.
        $s:ident <=
        $t:ty |
        $p_facade:ident.
        $p:ident
    ) => {
        #[test]
        fn $name() {
            let (handler_source, handler_receiver) = create_handlers();
            let source = handler_source.$p_facade();
            let receiver = handler_receiver.$s_facade();
            let (tx, rx) = std::sync::mpsc::channel();
            let tx = std::sync::Mutex::new(tx);
            receiver
                .$s(hermes::Callback::new(move |o: &$t| {
                    tx.lock().map(|it| it.send(o.clone())).unwrap().unwrap()
                }))
                .unwrap();
            use hermes::hermes_utils::Example;
            let message = <$t>::full_example();
            std::thread::sleep(WAIT_DURATION);
            source.$p(message.clone()).unwrap();
            let result = rx.recv_timeout(std::time::Duration::from_secs(1));
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
            let (tx, rx) = std::sync::mpsc::channel();
            let tx = std::sync::Mutex::new(tx);
            receiver
                .$s(hermes::Callback0::new(move || {
                    tx.lock().map(|it| it.send(())).unwrap().unwrap()
                }))
                .unwrap();
            std::thread::sleep(WAIT_DURATION);
            source.$p().unwrap();
            let result = rx.recv_timeout(std::time::Duration::from_secs(1));
            assert!(result.is_ok(), "didn't receive message after one second");
        }
    };
    ($name:ident : $s_facade:ident. $s:ident $a:block <= $p_facade:ident. $p:ident) => {
        #[test]
        fn $name() {
            let (handler_source, handler_receiver) = create_handlers();
            let source = handler_source.$p_facade();
            let receiver = handler_receiver.$s_facade();
            let (tx, rx) = std::sync::mpsc::channel();
            let tx = std::sync::Mutex::new(tx);
            receiver
                .$s(
                    $a,
                    hermes::Callback0::new(move || tx.lock().map(|it| it.send(())).unwrap().unwrap()),
                )
                .unwrap();
            std::thread::sleep(WAIT_DURATION);
            source.$p($a).unwrap();
            let result = rx.recv_timeout(std::time::Duration::from_secs(1));
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
        $p:ident
    ) => {
        #[test]
        fn $name() {
            let (handler_source, handler_receiver) = create_handlers();
            let source = handler_source.$p_facade();
            let receiver = handler_receiver.$s_facade();
            let (tx, rx) = std::sync::mpsc::channel();
            let tx = std::sync::Mutex::new(tx);
            receiver
                .$s(
                    $a,
                    hermes::Callback::new(move |o: &$t| tx.lock().map(|it| it.send(o.clone())).unwrap().unwrap()),
                )
                .unwrap();
            use hermes::hermes_utils::Example;
            let message = <$t>::full_example();
            std::thread::sleep(WAIT_DURATION);
            source.$p($a, message.clone()).unwrap();
            let result = rx.recv_timeout(std::time::Duration::from_secs(1));
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
            let (tx, rx) = std::sync::mpsc::channel();
            let tx = std::sync::Mutex::new(tx);
            receiver
                .$s(
                    $a,
                    hermes::Callback0::new(move || tx.lock().map(|it| it.send(())).unwrap().unwrap()),
                )
                .unwrap();
            std::thread::sleep(WAIT_DURATION);
            source.$p($a).unwrap();
            let result = rx.recv_timeout(std::time::Duration::from_secs(1));
            assert!(result.is_ok(), "didn't receive message after one second");
        }
    };
    (
        $name:ident : OneToMany
        $s_facade:ident.
        $s:ident
        ($($field:ident).+) <=
        $t:ty |
        $p_facade:ident.
        $p:ident
    ) => {
        #[test]
        fn $name() {
            let (handler_source, handler_receiver) = create_handlers();
            let source = handler_source.$p_facade();
            let receiver = handler_receiver.$s_facade();
            let (tx, rx) = std::sync::mpsc::channel();
            let tx = std::sync::Mutex::new(tx);
            let message = <$t>::full_example();
            receiver
                .$s(
                    message.$($field).*.clone(),
                    hermes::Callback::new(move |o: &$t| tx.lock().map(|it| it.send(o.clone())).unwrap().unwrap()),
                )
                .unwrap();
            use hermes::hermes_utils::Example;

            std::thread::sleep(WAIT_DURATION);
            source.$p(message.clone()).unwrap();
            let result = rx.recv_timeout(std::time::Duration::from_secs(1));
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
            let (tx, rx) = std::sync::mpsc::channel();
            let tx = std::sync::Mutex::new(tx);
            receiver
                .$s(hermes::Callback0::new(move || {
                    tx.lock().map(|it| it.send(())).unwrap().unwrap()
                }))
                .unwrap();
            std::thread::sleep(WAIT_DURATION);
            source.$p($a).unwrap();
            let result = rx.recv_timeout(std::time::Duration::from_secs(1));
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
        $a:block
    ) => {
        #[test]
        fn $name() {
            let (handler_source, handler_receiver) = create_handlers();
            let source = handler_source.$p_facade();
            let receiver = handler_receiver.$s_facade();
            let (tx, rx) = std::sync::mpsc::channel();
            let tx = std::sync::Mutex::new(tx);
            receiver
                .$s(hermes::Callback::new(move |o: &$t| {
                    tx.lock().map(|it| it.send(o.clone())).unwrap().unwrap()
                }))
                .unwrap();
            use hermes::hermes_utils::Example;
            let message = <$t>::full_example();
            std::thread::sleep(WAIT_DURATION);
            source.$p($a, message.clone()).unwrap();
            let result = rx.recv_timeout(std::time::Duration::from_secs(1));
            assert!(result.is_ok(), "didn't receive message after one second");
            assert_eq!(result.unwrap(), message)
        }
    };
    (
        $name:ident : ManyToOne
        $s_facade:ident.
        $s:ident <=
        $t:ty |
        $p_facade:ident.
        $p:ident
    ) => {
        #[test]
        fn $name() {
            let (handler_source, handler_receiver) = create_handlers();
            let source = handler_source.$p_facade();
            let receiver = handler_receiver.$s_facade();
            let (tx, rx) = std::sync::mpsc::channel();
            let tx = std::sync::Mutex::new(tx);
            receiver
                .$s(hermes::Callback::new(move |o: &$t| {
                    tx.lock().map(|it| it.send(o.clone())).unwrap().unwrap()
                }))
                .unwrap();
            use hermes::hermes_utils::Example;
            let message = <$t>::full_example();
            std::thread::sleep(WAIT_DURATION);
            source.$p(message.clone()).unwrap();
            let result = rx.recv_timeout(std::time::Duration::from_secs(1));
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
                        $f_back.subscribe_toggle_on <= SiteMessage | $f.publish_toggle_on);
                t!(toggle_off_works:
                        $f_back.subscribe_toggle_off <= SiteMessage | $f.publish_toggle_off);
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
                        $f.subscribe_version <= VersionMessage | $f_back.publish_version);
                t!(error_works:
                        $f.subscribe_error <= ErrorMessage | $f_back.publish_error);
                t!(component_loaded_works:
                        $f.subscribe_component_loaded <= ComponentLoadedMessage | $f_back.publish_component_loaded);
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
                        $f.subscribe_version { "identifier".to_string() } <= VersionMessage | $f_back.publish_version);
                t!(error_works:
                        $f.subscribe_error { "identifier".to_string() } <= SiteErrorMessage | $f_back.publish_error);
                t!(all_error_works:
                        ManyToOne
                        $f.subscribe_all_error <= SiteErrorMessage | $f_back.publish_error { "identifier".into() });
                t!(component_loaded_works:
                        $f.subscribe_component_loaded { "identifier".to_string() } <= ComponentLoadedOnSiteMessage | $f_back.publish_component_loaded );
                t!(components_loaded_works:
                        ManyToOne
                        $f.subscribe_all_component_loaded <= ComponentLoadedOnSiteMessage | $f_back.publish_component_loaded { "site_id".into() });
            }
        };
    }

#[macro_export]
macro_rules! test_suite {
    () => {
        $crate::test_suite!(WAIT_DURATION = 0);
    };

    (WAIT_DURATION = $wait_duration:expr) => {
        use $crate::{t, t_identifiable_component, t_identifiable_toggleable, t_component, t_toggleable};
        use snips_nlu_ontology::Slot;

        const WAIT_DURATION: std::time::Duration = std::time::Duration::from_millis($wait_duration);

        t_identifiable_component!(voice_activity_identifiable_component: voice_activity_backend | voice_activity);
        t!(voice_activity_vad_up_works:
                    OneToMany
                    voice_activity.subscribe_vad_up(site_id) <= VadUpMessage | voice_activity_backend.publish_vad_up);
        t!(voice_activity_vad_down_works:
                    OneToMany
                    voice_activity.subscribe_vad_down(site_id) <= VadDownMessage | voice_activity_backend.publish_vad_down);
        t!(voice_activity_all_vad_up_works:
                    ManyToOne
                    voice_activity.subscribe_all_vad_up <= VadUpMessage | voice_activity_backend.publish_vad_up);
        t!(voice_activity_all_vad_down_works:
                    ManyToOne
                    voice_activity.subscribe_all_vad_down <= VadDownMessage | voice_activity_backend.publish_vad_down);

        t_identifiable_component!(hotword_identifiable_component: hotword_backend | hotword);
        t_identifiable_toggleable!(hotword_identifiable_toggleable: hotword_backend | hotword);
        t!(hotword_detected_works:
                    hotword.subscribe_detected { "hotword_identifier".into() } <= HotwordDetectedMessage | hotword_backend.publish_detected);
        t!(hotword_all_detected_works:
                    ManyToOne
                    hotword.subscribe_all_detected <= HotwordDetectedMessage | hotword_backend.publish_detected { "hotword_identifier".into() });

        t_identifiable_toggleable!(sound_feedback_identifiable_toggleable: sound_feedback_backend | sound_feedback );

        t_component!(asr_component: asr_backend | asr);
        t_toggleable!(asr_toggleable: asr_backend | asr);
        t!(asr_text_captured_works:
                    asr.subscribe_text_captured <= TextCapturedMessage | asr_backend.publish_text_captured);
        t!(asr_partial_text_captured_works:
                    asr.subscribe_partial_text_captured <= TextCapturedMessage | asr_backend.publish_partial_text_captured);
        t!(asr_start_listening:
                    asr_backend.subscribe_start_listening <= AsrStartListeningMessage | asr.publish_start_listening);
        t!(asr_stop_listening:
                    asr_backend.subscribe_stop_listening <= SiteMessage | asr.publish_stop_listening);
        t!(asr_reload:
                    asr_backend.subscribe_component_reload <= RequestComponentReloadMessage | asr.publish_component_reload );

        t_component!(tts_component: tts_backend | tts);
        t!(tts_say_works:
                    tts_backend.subscribe_say <= SayMessage | tts.publish_say);
        t!(tts_say_finished_works:
                    tts.subscribe_say_finished <= SayFinishedMessage | tts_backend.publish_say_finished);
        t!(tts_register_sound_works:
                    tts_backend.subscribe_register_sound <= RegisterSoundMessage | tts.publish_register_sound);

        t_component!(nlu_component: nlu_backend | nlu);
        t!(nlu_query_works:
                    nlu_backend.subscribe_query <= NluQueryMessage | nlu.publish_query);
        t!(nlu_partial_query_works:
                    nlu_backend.subscribe_partial_query <= NluSlotQueryMessage | nlu.publish_partial_query);
        t!(nlu_slot_parsed_works:
                    nlu.subscribe_slot_parsed <= NluSlotMessage | nlu_backend.publish_slot_parsed);
        t!(nlu_intent_parsed_works:
                    nlu.subscribe_intent_parsed <= NluIntentMessage | nlu_backend.publish_intent_parsed);
        t!(nlu_intent_not_recognized_works:
                    nlu.subscribe_intent_not_recognized <= NluIntentNotRecognizedMessage | nlu_backend.publish_intent_not_recognized);
        t!(nlu_reload:
                    nlu_backend.subscribe_component_reload <= RequestComponentReloadMessage | nlu.publish_component_reload );

        t_identifiable_component!(audio_server_component: audio_server_backend | audio_server);
        t_identifiable_toggleable!(audio_server_toggeable: audio_server_backend | audio_server);
        t!(audio_server_play_bytes_works:
                    OneToMany
                    audio_server_backend.subscribe_play_bytes(site_id) <= PlayBytesMessage | audio_server.publish_play_bytes);
        t!(audio_server_play_all_bytes_works:
                    audio_server_backend.subscribe_all_play_bytes <= PlayBytesMessage | audio_server.publish_play_bytes);
        t!(audio_server_play_finished_works:
                    OneToMany
                    audio_server.subscribe_play_finished(site_id) <= PlayFinishedMessage | audio_server_backend.publish_play_finished);
        t!(audio_server_all_play_finished_works:
                    audio_server.subscribe_all_play_finished <= PlayFinishedMessage | audio_server_backend.publish_play_finished);
        t!(audio_server_audio_frame_works:
                    OneToMany
                    audio_server.subscribe_audio_frame(site_id) <= AudioFrameMessage | audio_server_backend.publish_audio_frame);
        t!(audio_server_replay_request:
                    OneToMany
                    audio_server_backend.subscribe_replay_request(site_id) <= ReplayRequestMessage | audio_server.publish_replay_request);
        t!(audio_server_replay_response:
                    OneToMany
                    audio_server.subscribe_replay_response(site_id) <= AudioFrameMessage | audio_server_backend.publish_replay_response);

        t_component!(dialogue_component: dialogue_backend | dialogue);
        t_toggleable!(dialogue_toggleable: dialogue_backend | dialogue);
        t!(dialogue_session_started_works:
                    dialogue.subscribe_session_started <= SessionStartedMessage | dialogue_backend.publish_session_started);
        t!(dialogue_session_queued_works:
                    dialogue.subscribe_session_queued <= SessionQueuedMessage | dialogue_backend.publish_session_queued);
        t!(dialogue_intents_works:
                    dialogue.subscribe_intents <= IntentMessage | dialogue_backend.publish_intent);
        t!(dialogue_intent_works:
                    OneToMany
                    dialogue.subscribe_intent(intent.intent_name) <= IntentMessage | dialogue_backend.publish_intent);
        t!(dialogue_intent_not_recognized_works:
                    dialogue.subscribe_intent_not_recognized <= IntentNotRecognizedMessage | dialogue_backend.publish_intent_not_recognized);
        t!(dialogue_session_ended_works:
                    dialogue.subscribe_session_ended <= SessionEndedMessage | dialogue_backend.publish_session_ended);
        t!(dialogue_start_session_works:
                    dialogue_backend.subscribe_start_session <= StartSessionMessage | dialogue.publish_start_session);
        t!(dialogue_continue_session_works:
                    dialogue_backend.subscribe_continue_session <= ContinueSessionMessage | dialogue.publish_continue_session);
        t!(dialogue_end_session_works:
                    dialogue_backend.subscribe_end_session <= EndSessionMessage | dialogue.publish_end_session);
        t!(dialogue_configure_works:
                    dialogue_backend.subscribe_configure <= DialogueConfigureMessage | dialogue.publish_configure);

        t_component!(injection_component: injection_backend | injection);
        t!(injection_request:
                    injection_backend.subscribe_injection_request <= InjectionRequestMessage | injection.publish_injection_request);
        t!(injection_status_request:
                    injection_backend.subscribe_injection_status_request <= injection.publish_injection_status_request);
        t!(injection_status:
                    injection.subscribe_injection_status <= InjectionStatusMessage | injection_backend.publish_injection_status);
        t!(injection_complete:
                    injection.subscribe_injection_complete <= InjectionCompleteMessage | injection_backend.publish_injection_complete);
        t!(injection_reset_request:
                    injection_backend.subscribe_injection_reset_request <= InjectionResetRequestMessage | injection.publish_injection_reset_request);
        t!(injection_reset_complete:
                    injection.subscribe_injection_reset_complete <= InjectionResetCompleteMessage | injection_backend.publish_injection_reset_complete);
    };
}
