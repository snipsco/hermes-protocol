open TestFramework;
open Ctypes;
open Foreign;
open HermesReason.Structs;
open HermesReason.Enums;

HermesReason.Utils.openDynamicLibrary("../../target/debug/libhermes_ffi_test")

/* Utils */

let ( !* ) = any => ptr @@ any;
let ( ?* ) = any => ptr_opt @@ any;
let ( !** ) = any => ptr @@ ptr @@ any;
let ( ?** ) = any => ptr_opt @@ ptr_opt @@ any;
let (@>>) = (previous, any) => previous @-> returning @@ any;

let hermes_ffi_test_get_last_error =
  foreign("hermes_ffi_test_get_last_error", !*string @>> snips_result);

/* Helpers */

let check_res = result =>
  try(
    switch (result) {
    | Error(i) =>
      let errorStringPtr = allocate(string, "");
      hermes_ffi_test_get_last_error(errorStringPtr) |> ignore;
      Console.error(!@errorStringPtr);
    | _ => ()
    }
  ) {
  | Failure(msg) => raise(Failure(msg))
  };

let roundTrip = (expect, message, typ, call) => {
  let messagePtr = allocate(typ, message);
  let messageDblPtr = allocate(ptr @@ typ, from_voidp(typ, null));
  call(messagePtr, messageDblPtr) |> check_res;
  expect.equal(!@messagePtr, !@ !@messageDblPtr);
};

/* Test suite */

describe("Hermes messages - round trips", ({test, testOnly, testSkip}) => {
  let hermes_ffi_test_round_trip_session_queued =
    foreign(
      "hermes_ffi_test_round_trip_session_queued",
      !*CSessionQueuedMessage.view
      @-> !**CSessionQueuedMessage.view
      @>> snips_result,
    );

  test("CSessionQueuedMessage", ({expect}) => {
    let message: CSessionQueuedMessage.t_view = {
      site_id: "default",
      custom_data: Some("data"),
      session_id: "session_id",
    };
    roundTrip(
      expect,
      message,
      CSessionQueuedMessage.view,
      hermes_ffi_test_round_trip_session_queued,
    );
  });

  let hermes_ffi_test_round_trip_session_started =
    foreign(
      "hermes_ffi_test_round_trip_session_started",
      !*CSessionStartedMessage.view
      @-> !**CSessionStartedMessage.view
      @>> snips_result,
    );

  test("CSessionStartedMessage", ({expect}) => {
    let message: CSessionStartedMessage.t_view = {
      reactivated_from_session_id: Some("session_id_from"),
      site_id: "default",
      custom_data: Some("data"),
      session_id: "session_id",
    };
    roundTrip(
      expect,
      message,
      CSessionStartedMessage.view,
      hermes_ffi_test_round_trip_session_started,
    );
  });

  let hermes_ffi_test_round_trip_session_ended =
    foreign(
      "hermes_ffi_test_round_trip_session_ended",
      !*CSessionEndedMessage.view
      @-> !**CSessionEndedMessage.view
      @>> snips_result,
    );

  test("CSessionEndedMessage", ({expect}) => {
    /* Error */

    let message: CSessionEndedMessage.t_view = {
      site_id: "default",
      termination: {
        termination_type: SNIPS_SESSION_TERMINATION_TYPE_ERROR,
        data: Some("An error message"),
        component: SNIPS_HERMES_COMPONENT_NONE,
      },
      custom_data: None,
      session_id: "session_id",
    };

    roundTrip(
      expect,
      message,
      CSessionEndedMessage.view,
      hermes_ffi_test_round_trip_session_ended,
    );

    /* Timeout */

    let message: CSessionEndedMessage.t_view = {
      site_id: "default",
      termination: {
        termination_type: SNIPS_SESSION_TERMINATION_TYPE_TIMEOUT,
        data: None,
        component: SNIPS_HERMES_COMPONENT_CLIENT_APP,
      },
      custom_data: None,
      session_id: "session_id",
    };

    roundTrip(
      expect,
      message,
      CSessionEndedMessage.view,
      hermes_ffi_test_round_trip_session_ended,
    );
  });

  let hermes_ffi_test_round_trip_nlu_slot_array =
    foreign(
      "hermes_ffi_test_round_trip_nlu_slot_array",
      !*NluSlotList.view @-> !**NluSlotList.view @>> snips_result,
    );

  test("CNluSlotArray", ({expect}) => {
    let nluSlot: NluSlot.t_view = {
      value: {
        value: String("slot_value"),
        value_type: SNIPS_SLOT_VALUE_TYPE_CUSTOM,
      },
      alternatives: [
        {
          value: String("alternative_slot"),
          value_type: SNIPS_SLOT_VALUE_TYPE_CUSTOM,
        },
      ],
      raw_value: "value",
      entity: "entity",
      slot_name: "slot_name",
      range_start: Int32.of_int(0),
      range_end: Int32.of_int(10),
      confidence_score: 0.25,
    };
    let message: NluSlotList.t_view = [nluSlot];

    roundTrip(
      expect,
      message,
      NluSlotList.view,
      hermes_ffi_test_round_trip_nlu_slot_array,
    );
  });

  let hermes_ffi_test_round_trip_intent =
    foreign(
      "hermes_ffi_test_round_trip_intent",
      !*CIntentMessage.view @-> !**CIntentMessage.view @>> snips_result,
    );

  test("CIntentMessage", ({expect}) => {
    let message: CIntentMessage.t_view = {
      session_id: "default",
      custom_data: Some("custom_data"),
      site_id: "site_id",
      input: "some text",
      intent: {
        intent_name: "intent_name",
        confidence_score: 0.5,
      },
      slots: [
        {
          value: {
            value: String("slot_value"),
            value_type: SNIPS_SLOT_VALUE_TYPE_CUSTOM,
          },
          alternatives: [
            {
              value: String("alternative_slot"),
              value_type: SNIPS_SLOT_VALUE_TYPE_CUSTOM,
            },
          ],
          raw_value: "value",
          entity: "entity",
          slot_name: "slot_name",
          range_start: Int32.of_int(0),
          range_end: Int32.of_int(10),
          confidence_score: 0.25,
        },
      ],
      alternatives: [
        {intent_name: None, slots: None, confidence_score: 0.2},
        {
          intent_name: Some("alternative_intent"),
          slots:
            Some([
              {
                value: {
                  value: String("slot_value"),
                  value_type: SNIPS_SLOT_VALUE_TYPE_CUSTOM,
                },
                alternatives: [
                  {
                    value: String("alternative_slot"),
                    value_type: SNIPS_SLOT_VALUE_TYPE_CUSTOM,
                  },
                ],
                raw_value: "value",
                entity: "entity",
                slot_name: "slot_name",
                range_start: Int32.of_int(0),
                range_end: Int32.of_int(10),
                confidence_score: 0.25,
              },
            ]),
          confidence_score: 0.7,
        },
      ],
      asr_tokens: [
        [
          {
            time: {
              start: 0.4,
              end_: 0.8,
            },
            range_start: Int32.of_int(7),
            range_end: Int32.of_int(10),
            confidence: 0.3,
            value: "dog",
          },
        ],
      ],
      asr_confidence: 0.5,
    };

    roundTrip(
      expect,
      message,
      CIntentMessage.view,
      hermes_ffi_test_round_trip_intent,
    );
  });

  let hermes_ffi_test_round_trip_intent_not_recognized =
    foreign(
      "hermes_ffi_test_round_trip_intent_not_recognized",
      !*CIntentNotRecognizedMessage.view
      @-> !**CIntentNotRecognizedMessage.view
      @>> snips_result,
    );

  test("CIntentNotRecognizedMessage", ({expect}) => {
    let message: CIntentNotRecognizedMessage.t_view = {
      confidence_score: 0.99,
      alternatives: [
        {intent_name: None, slots: None, confidence_score: 0.2},
      ],
      custom_data: Some("data"),
      input: Some("text"),
      session_id: "session_id",
      site_id: "default",
    };

    roundTrip(
      expect,
      message,
      CIntentNotRecognizedMessage.view,
      hermes_ffi_test_round_trip_intent_not_recognized,
    );
  });

  let hermes_ffi_test_round_trip_start_session =
    foreign(
      "hermes_ffi_test_round_trip_start_session",
      !*CStartSessionMessage.view
      @-> !**CStartSessionMessage.view
      @>> snips_result,
    );

  test("CStartSessionMessage", ({expect}) => {
    /* Notification */
    let message: CStartSessionMessage.t_view = {
      site_id: "default",
      custom_data: Some("data"),
      init: CSessionInit.Notification("Hello"),
    };

    roundTrip(
      expect,
      message,
      CStartSessionMessage.view,
      hermes_ffi_test_round_trip_start_session,
    );

    /* Action */
    let message: CStartSessionMessage.t_view = {
      site_id: "default",
      custom_data: Some("data"),
      init:
        CSessionInit.Action({
          text: Some("text"),
          intent_filter: Some(["intent1", "intent2"]),
          can_be_enqueued: true,
          send_intent_not_recognized: true,
        }),
    };

    roundTrip(
      expect,
      message,
      CStartSessionMessage.view,
      hermes_ffi_test_round_trip_start_session,
    );
  });

  let hermes_ffi_test_round_trip_continue_session =
    foreign(
      "hermes_ffi_test_round_trip_continue_session",
      !*CContinueSessionMessage.view
      @-> !**CContinueSessionMessage.view
      @>> snips_result,
    );

  test("CContinueSessionMessage", ({expect}) => {
    let message: CContinueSessionMessage.t_view = {
      send_intent_not_recognized: false,
      slot: Some("slotName"),
      custom_data: Some("data"),
      intent_filter: Some(["Test"]),
      text: Some("Hello"),
      session_id: "session_id",
    };

    roundTrip(
      expect,
      message,
      CContinueSessionMessage.view,
      hermes_ffi_test_round_trip_continue_session,
    );
  });

  let hermes_ffi_test_round_trip_end_session =
    foreign(
      "hermes_ffi_test_round_trip_end_session",
      !*CEndSessionMessage.view @-> !**CEndSessionMessage.view @>> snips_result,
    );

  test("CEndSessionMessage", ({expect}) => {
    let message: CEndSessionMessage.t_view = {
      session_id: "session_id",
      text: Some("text"),
    };

    roundTrip(
      expect,
      message,
      CEndSessionMessage.view,
      hermes_ffi_test_round_trip_end_session,
    );
  });

  let hermes_ffi_test_round_trip_injection_request =
    foreign(
      "hermes_ffi_test_round_trip_injection_request",
      !*CInjectionRequestMessage.view
      @-> !**CInjectionRequestMessage.view
      @>> snips_result,
    );

  test("CInjectionRequestMessage", ({expect}) => {
    let message: CInjectionRequestMessage.t_view = {
      id: Some("identifier"),
      cross_language: Some("en"),
      lexicon: [{key: "colors", values: ["red", "blue", "green"]}],
      operations: [
        {
          kind: SNIPS_INJECTION_KIND_ADD,
          values: [{key: "colors", values: ["red", "blue", "green"]}],
        },
      ],
    };

    roundTrip(
      expect,
      message,
      CInjectionRequestMessage.view,
      hermes_ffi_test_round_trip_injection_request,
    );
  });

  let hermes_ffi_test_round_trip_injection_complete =
    foreign(
      "hermes_ffi_test_round_trip_injection_complete",
      !*CInjectionCompleteMessage.view
      @-> !**CInjectionCompleteMessage.view
      @>> snips_result,
    );

  test("CInjectionCompleteMessage", ({expect}) => {
    let message: CInjectionCompleteMessage.t_view = {request_id: "id"};

    roundTrip(
      expect,
      message,
      CInjectionCompleteMessage.view,
      hermes_ffi_test_round_trip_injection_complete,
    );
  });

  let hermes_ffi_test_round_trip_injection_reset_request =
    foreign(
      "hermes_ffi_test_round_trip_injection_reset_request",
      !*CInjectionResetRequestMessage.view
      @-> !**CInjectionResetRequestMessage.view
      @>> snips_result,
    );

  test("CInjectionResetRequestMessage", ({expect}) => {
    let message: CInjectionResetRequestMessage.t_view = {request_id: "id"};

    roundTrip(
      expect,
      message,
      CInjectionResetRequestMessage.view,
      hermes_ffi_test_round_trip_injection_reset_request,
    );
  });

  test("CInjectionResetRequestMessage", ({expect}) => {
    let message: CInjectionResetRequestMessage.t_view = {request_id: "id"};

    roundTrip(
      expect,
      message,
      CInjectionResetRequestMessage.view,
      hermes_ffi_test_round_trip_injection_reset_request,
    );
  });

  let hermes_ffi_test_round_trip_injection_reset_complete =
    foreign(
      "hermes_ffi_test_round_trip_injection_reset_complete",
      !*CInjectionResetCompleteMessage.view
      @-> !**CInjectionResetCompleteMessage.view
      @>> snips_result,
    );

  test("CInjectionResetCompleteMessage", ({expect}) => {
    let message: CInjectionResetCompleteMessage.t_view = {request_id: "id"};

    roundTrip(
      expect,
      message,
      CInjectionResetCompleteMessage.view,
      hermes_ffi_test_round_trip_injection_reset_complete,
    );
  });

  let hermes_ffi_test_round_trip_map_string_to_string_array =
    foreign(
      "hermes_ffi_test_round_trip_map_string_to_string_array",
      !*MapStringToStringList.view
      @-> !**MapStringToStringList.view
      @>> snips_result,
    );

  test("CMapStringToStringArray", ({expect}) => {
    let message: MapStringToStringList.t_view = [
      {key: "one", values: ["1", "one", "One"]},
    ];

    roundTrip(
      expect,
      message,
      MapStringToStringList.view,
      hermes_ffi_test_round_trip_map_string_to_string_array,
    );
  });

  let hermes_ffi_test_round_trip_register_sound =
    foreign(
      "hermes_ffi_test_round_trip_register_sound",
      !*RegisterSoundMessage.view
      @-> !**RegisterSoundMessage.view
      @>> snips_result,
    );

  test("CRegisterSoundMessage", ({expect}) => {
    let wav_bytes_list = [0, 1, 2, 3];

    let message: RegisterSoundMessage.t_view = {
      sound_id: "sound:id",
      wav_sound: wav_bytes_list |> List.map(Unsigned.UInt8.of_int),
      wav_sound_len: wav_bytes_list |> List.length,
    };

    roundTrip(
      expect,
      message,
      RegisterSoundMessage.view,
      hermes_ffi_test_round_trip_register_sound,
    );
  });

  let hermes_ffi_test_round_trip_dialogue_configure_intent =
    foreign(
      "hermes_ffi_test_round_trip_dialogue_configure_intent",
      !*CDialogueConfigureIntent.view
      @-> !**CDialogueConfigureIntent.view
      @>> snips_result,
    );

  test("CDialogueConfigureIntent", ({expect}) => {
    let message: CDialogueConfigureIntent.t_view = {
      enable: true,
      intent_id: "Intent id",
    };

    roundTrip(
      expect,
      message,
      CDialogueConfigureIntent.view,
      hermes_ffi_test_round_trip_dialogue_configure_intent,
    );
  });

  let hermes_ffi_test_round_trip_dialogue_configure_intent_array =
    foreign(
      "hermes_ffi_test_round_trip_dialogue_configure_intent_array",
      !*DialogueConfigureIntentList.view
      @-> !**DialogueConfigureIntentList.view
      @>> snips_result,
    );

  test("CDialogueConfigureIntentArray", ({expect}) => {
    let message: DialogueConfigureIntentList.t_view = [
      {enable: true, intent_id: "Intent id"},
    ];

    roundTrip(
      expect,
      message,
      DialogueConfigureIntentList.view,
      hermes_ffi_test_round_trip_dialogue_configure_intent_array,
    );
  });

  let hermes_ffi_test_round_trip_dialogue_configure =
    foreign(
      "hermes_ffi_test_round_trip_dialogue_configure",
      !*CDialogueConfigureMessage.view
      @-> !**CDialogueConfigureMessage.view
      @>> snips_result,
    );

  test("CDialogueConfigureMessage", ({expect}) => {
    let message: CDialogueConfigureMessage.t_view = {
      site_id: Some("default"),
      intents: Some([{enable: true, intent_id: "Intent_id"}]),
    };

    roundTrip(
      expect,
      message,
      CDialogueConfigureMessage.view,
      hermes_ffi_test_round_trip_dialogue_configure,
    );
  });

  let hermes_ffi_test_round_trip_asr_token =
    foreign(
      "hermes_ffi_test_round_trip_asr_token",
      !*CAsrToken.view @-> !**CAsrToken.view @>> snips_result,
    );

  test("CAsrToken", ({expect}) => {
    let message: CAsrToken.t_view = {
      value: "value",
      confidence: 0.5,
      range_start: Int32.of_int(2),
      range_end: Int32.of_int(7),
      time: {
        start: 1.2,
        end_: 2.4,
      },
    };

    roundTrip(
      expect,
      message,
      CAsrToken.view,
      hermes_ffi_test_round_trip_asr_token,
    );
  });

  let hermes_ffi_test_round_trip_asr_token_array =
    foreign(
      "hermes_ffi_test_round_trip_asr_token_array",
      !*AsrTokenList.view @-> !**AsrTokenList.view @>> snips_result,
    );

  test("CAsrTokenArray", ({expect}) => {
    let message: AsrTokenList.t_view = [
      {
        value: "value",
        confidence: 0.5,
        range_start: Int32.of_int(2),
        range_end: Int32.of_int(7),
        time: {
          start: 1.2,
          end_: 2.4,
        },
      },
    ];

    roundTrip(
      expect,
      message,
      AsrTokenList.view,
      hermes_ffi_test_round_trip_asr_token_array,
    );
  });

  let hermes_ffi_test_round_trip_asr_token_double_array =
    foreign(
      "hermes_ffi_test_round_trip_asr_token_double_array",
      !*AsrTokenDoubleList.view @-> !**AsrTokenDoubleList.view @>> snips_result,
    );

  test("CAsrTokenDoubleArray", ({expect}) => {
    let message: AsrTokenDoubleList.t_view = [
      [
        {
          value: "value",
          confidence: 0.5,
          range_start: Int32.of_int(2),
          range_end: Int32.of_int(7),
          time: {
            start: 1.2,
            end_: 2.4,
          },
        },
      ],
    ];

    roundTrip(
      expect,
      message,
      AsrTokenDoubleList.view,
      hermes_ffi_test_round_trip_asr_token_double_array,
    );
  });

  let hermes_ffi_test_round_trip_text_captured =
    foreign(
      "hermes_ffi_test_round_trip_text_captured",
      !*CTextCapturedMessage.view
      @-> !**CTextCapturedMessage.view
      @>> snips_result,
    );

  test("CTextCapturedMessage", ({expect}) => {
    let message: CTextCapturedMessage.t_view = {
      text: "some text",
      tokens:
        Some([
          {
            value: "value",
            confidence: 0.5,
            range_start: Int32.of_int(2),
            range_end: Int32.of_int(7),
            time: {
              start: 1.2,
              end_: 2.4,
            },
          },
        ]),
      likelihood: 0.6,
      seconds: 2.3,
      site_id: "default",
      session_id: Some("session_id"),
    };

    roundTrip(
      expect,
      message,
      CTextCapturedMessage.view,
      hermes_ffi_test_round_trip_text_captured,
    );
  });

  let hermes_ffi_test_round_trip_nlu_intent_alternative =
    foreign(
      "hermes_ffi_test_round_trip_nlu_intent_alternative",
      !*CNluIntentAlternative.view
      @-> !**CNluIntentAlternative.view
      @>> snips_result,
    );

  test("CNluIntentAlternative", ({expect}) => {
    let message: CNluIntentAlternative.t_view = {
      intent_name: Some("alternative_intent"),
      slots:
        Some([
          {
            value: {
              value: String("slot_value"),
              value_type: SNIPS_SLOT_VALUE_TYPE_CUSTOM,
            },
            alternatives: [
              {
                value: String("alternative_slot"),
                value_type: SNIPS_SLOT_VALUE_TYPE_CUSTOM,
              },
            ],
            raw_value: "value",
            entity: "entity",
            slot_name: "slot_name",
            range_start: Int32.of_int(0),
            range_end: Int32.of_int(10),
            confidence_score: 0.25,
          },
        ]),
      confidence_score: 0.7,
    };

    roundTrip(
      expect,
      message,
      CNluIntentAlternative.view,
      hermes_ffi_test_round_trip_nlu_intent_alternative,
    );
  });

  let hermes_ffi_test_round_trip_nlu_intent_alternative_array =
    foreign(
      "hermes_ffi_test_round_trip_nlu_intent_alternative_array",
      !*NluIntentAlternativeList.view
      @-> !**NluIntentAlternativeList.view
      @>> snips_result,
    );

  test("CNluIntentAlternativeArray", ({expect}) => {
    let message: NluIntentAlternativeList.t_view = [
      {
        intent_name: Some("alternative_intent"),
        slots:
          Some([
            {
              value: {
                value: String("slot_value"),
                value_type: SNIPS_SLOT_VALUE_TYPE_CUSTOM,
              },
              alternatives: [
                {
                  value: String("alternative_slot"),
                  value_type: SNIPS_SLOT_VALUE_TYPE_CUSTOM,
                },
              ],
              raw_value: "value",
              entity: "entity",
              slot_name: "slot_name",
              range_start: Int32.of_int(0),
              range_end: Int32.of_int(10),
              confidence_score: 0.25,
            },
          ]),
        confidence_score: 0.7,
      },
    ];

    roundTrip(
      expect,
      message,
      NluIntentAlternativeList.view,
      hermes_ffi_test_round_trip_nlu_intent_alternative_array,
    );
  });
});