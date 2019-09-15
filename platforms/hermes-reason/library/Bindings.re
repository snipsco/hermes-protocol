open Ctypes;
open Structs;
open Foreign;
open Structs;
open Enums;

/* Utils */

let ( !* ) = any => ptr @@ any;
let ( ?* ) = any => ptr_opt @@ any;
let ( !** ) = any => ptr @@ ptr @@ any;
let ( ?** ) = any => ptr_opt @@ ptr_opt @@ any;
let (@>>) = (previous, any) => previous @-> returning @@ any;

let msgView = (m, drop) =>
  view(
    ~read=
      ptr => {
        let v = !@ptr;
        drop(ptr);
        v;
      },
    ~write=msg => Ctypes.allocate(m, msg),
    ptr(m),
  );

let bind = (name, signature, ~from=?) => foreign(~from?, name, signature);
let handler = (message, drop) =>
  funptr(msgView(message, drop) /* @-> ?*void */ @>> void);

/* Bindings */

let hermes_get_last_error =
  bind("hermes_get_last_error", !*string @>> snips_result);

let hermes_enable_debug_logs =
  bind("hermes_enable_debug_logs", void @>> snips_result);

let hermes_protocol_handler_new_mqtt_with_options =
  bind(
    "hermes_protocol_handler_new_mqtt_with_options",
    !**CProtocolHandler.view
    @-> !*CMqttOptions.view
    @-> !*void
    @>> snips_result,
  );

let hermes_destroy_mqtt_protocol_handler =
  bind(
    "hermes_destroy_mqtt_protocol_handler",
    !*CProtocolHandler.view @>> snips_result,
  );

let hermes_drop_dialogue_facade =
  bind(
    "hermes_drop_dialogue_facade",
    !*CDialogueFacade.view @>> snips_result,
  );

let hermes_drop_error_message =
  bind("hermes_drop_error_message", !*CErrorMessage.view @>> snips_result);

let hermes_drop_injection_complete_message =
  bind(
    "hermes_drop_injection_complete_message",
    !*CInjectionCompleteMessage.view @>> snips_result,
  );

let hermes_drop_injection_facade =
  bind(
    "hermes_drop_injection_facade",
    !*CInjectionFacade.view @>> snips_result,
  );

let hermes_drop_injection_reset_complete_message =
  bind(
    "hermes_drop_injection_reset_complete_message",
    !*CInjectionResetCompleteMessage.view @>> snips_result,
  );

let hermes_drop_injection_status_message =
  bind(
    "hermes_drop_injection_status_message",
    !*CInjectionStatusMessage.view @>> snips_result,
  );

let hermes_drop_intent_message =
  bind("hermes_drop_intent_message", !*CIntentMessage.view @>> snips_result);

let hermes_drop_intent_not_recognized_message =
  bind(
    "hermes_drop_intent_not_recognized_message",
    !*CIntentNotRecognizedMessage.view @>> snips_result,
  );

let hermes_drop_session_ended_message =
  bind(
    "hermes_drop_session_ended_message",
    !*CSessionEndedMessage.view @>> snips_result,
  );

let hermes_drop_session_queued_message =
  bind(
    "hermes_drop_session_queued_message",
    !*CSessionQueuedMessage.view @>> snips_result,
  );

let hermes_drop_session_started_message =
  bind(
    "hermes_drop_session_started_message",
    !*CSessionStartedMessage.view @>> snips_result,
  );

let hermes_drop_sound_feedback_facade =
  bind(
    "hermes_drop_sound_feedback_facade",
    !*CSoundFeedbackFacade.view @>> snips_result,
  );

let hermes_drop_tts_facade =
  bind("hermes_drop_tts_facade", !*CTtsFacade.view @>> snips_result);

let hermes_drop_version_message =
  bind(
    "hermes_drop_version_message",
    !*CVersionMessage.view @>> snips_result,
  );

let hermes_dialogue_publish_configure =
  bind(
    "hermes_dialogue_publish_configure",
    !*CDialogueFacade.view
    @-> !*CDialogueConfigureMessage.view
    @>> snips_result,
  );

let hermes_dialogue_publish_continue_session =
  bind(
    "hermes_dialogue_publish_continue_session",
    !*CDialogueFacade.view @-> !*CContinueSessionMessage.view @>> snips_result,
  );

let hermes_dialogue_publish_end_session =
  bind(
    "hermes_dialogue_publish_end_session",
    !*CDialogueFacade.view @-> !*CStartSessionMessage.view @>> snips_result,
  );

let hermes_dialogue_publish_start_session =
  bind(
    "hermes_dialogue_publish_start_session",
    !*CDialogueFacade.view @-> !*CStartSessionMessage.view @>> snips_result,
  );

let hermes_dialogue_subscribe_intent =
  bind(
    "hermes_dialogue_subscribe_intent",
    !*CDialogueFacade.view
    @-> string
    @-> handler(CIntentMessage.view, hermes_drop_intent_message)
    @>> snips_result,
  );

let hermes_dialogue_subscribe_intent_not_recognized =
  bind(
    "hermes_dialogue_subscribe_intent_not_recognized",
    !*CDialogueFacade.view
    @-> handler(
          CIntentNotRecognizedMessage.view,
          hermes_drop_intent_not_recognized_message,
        )
    @>> snips_result,
  );

let hermes_dialogue_subscribe_intents =
  bind(
    "hermes_dialogue_subscribe_intents",
    !*CDialogueFacade.view
    @-> handler(CIntentMessage.view, hermes_drop_intent_message)
    @>> snips_result,
  );

let hermes_dialogue_subscribe_session_ended =
  bind(
    "hermes_dialogue_subscribe_session_ended",
    !*CDialogueFacade.view
    @-> handler(CSessionEndedMessage.view, hermes_drop_session_ended_message)
    @>> snips_result,
  );

let hermes_dialogue_subscribe_session_queued =
  bind(
    "hermes_dialogue_subscribe_session_queued",
    !*CDialogueFacade.view
    @-> handler(
          CSessionQueuedMessage.view,
          hermes_drop_session_queued_message,
        )
    @>> snips_result,
  );

let hermes_dialogue_subscribe_session_started =
  bind(
    "hermes_dialogue_subscribe_session_started",
    !*CDialogueFacade.view
    @-> handler(
          CSessionStartedMessage.view,
          hermes_drop_session_started_message,
        )
    @>> snips_result,
  );

let hermes_injection_publish_injection_request =
  bind(
    "hermes_injection_publish_injection_request",
    !*CInjectionFacade.view @-> CInjectionRequestMessage.view @>> snips_result,
  );

let hermes_injection_publish_injection_reset_request =
  bind(
    "hermes_injection_publish_injection_reset_request",
    !*CInjectionFacade.view
    @-> CInjectionResetRequestMessage.view
    @>> snips_result,
  );

let hermes_injection_publish_injection_status_request =
  bind(
    "hermes_injection_publish_injection_status_request",
    !*CInjectionFacade.view @>> snips_result,
  );

let hermes_injection_subscribe_injection_complete =
  bind(
    "hermes_injection_subscribe_injection_complete",
    !*CInjectionFacade.view
    @-> handler(
          CInjectionCompleteMessage.view,
          hermes_drop_injection_complete_message,
        )
    @>> snips_result,
  );

let hermes_injection_subscribe_injection_reset_complete =
  bind(
    "hermes_injection_subscribe_injection_reset_complete",
    !*CInjectionFacade.view
    @-> handler(
          CInjectionResetCompleteMessage.view,
          hermes_drop_injection_reset_complete_message,
        )
    @>> snips_result,
  );

let hermes_injection_subscribe_injection_status =
  bind(
    "hermes_injection_subscribe_injection_status",
    !*CInjectionFacade.view
    @-> handler(
          CInjectionStatusMessage.view,
          hermes_drop_injection_status_message,
        )
    @>> snips_result,
  );

let hermes_protocol_handler_dialogue_facade =
  bind(
    "hermes_protocol_handler_dialogue_facade",
    !*CProtocolHandler.view @-> !**CDialogueFacade.view @>> snips_result,
  );

let hermes_protocol_handler_injection_facade =
  bind(
    "hermes_protocol_handler_injection_facade",
    !*CProtocolHandler.view @-> !**CInjectionFacade.view @>> snips_result,
  );

let hermes_protocol_handler_sound_feedback_facade =
  bind(
    "hermes_protocol_handler_sound_feedback_facade",
    !*CProtocolHandler.view @-> !**CSoundFeedbackFacade.view @>> snips_result,
  );

let hermes_protocol_handler_tts_facade =
  bind(
    "hermes_protocol_handler_sound_feedback_facade",
    !*CProtocolHandler.view @-> !**CTtsFacade.view @>> snips_result,
  );

let hermes_sound_feedback_publish_toggle_off =
  bind(
    "hermes_sound_feedback_publish_toggle_off",
    !*CSoundFeedbackFacade.view @-> !*CSiteMessage.view @>> snips_result,
  );

let hermes_sound_feedback_publish_toggle_on =
  bind(
    "hermes_sound_feedback_publish_toggle_on",
    !*CSoundFeedbackFacade.view @-> !*CSiteMessage.view @>> snips_result,
  );

let hermes_tts_publish_register_sound =
  bind(
    "hermes_tts_publish_register_sound",
    !*CTtsFacade.view @-> !*RegisterSoundMessage.view @>> snips_result,
  );