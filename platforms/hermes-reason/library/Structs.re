open Ctypes;
open Enums;

module StringCast = {
  type t_view = string;

  let read = ptr => {
    let charPtr = from_voidp(char, ptr);
    let charSize = sizeof(char);
    let rec computeSize = (~size=0, ~index=0, ptr) => {
      let ptrAddress = raw_address_of_ptr(ptr);
      let ptrPos =
        ptr_of_raw_address @@
        Nativeint.(add(ptrAddress, of_int(index * charSize)));
      if (!@from_voidp(char, ptrPos) == Char.chr(0)) {
        size;
      } else {
        computeSize(~size=size + 1, ~index=index + 1, ptr);
      };
    };
    let size = computeSize(ptr);
    Ctypes.string_from_ptr(charPtr, size);
  };
  let write = (str: t_view) => {
    to_voidp(CArray.of_string(str) |> CArray.start);
  };
  let view = view(~read, ~write, ptr(void));
};

%struct
("CProtocolHandler", {handler: "void *", user_data: "void *"});

%struct
("CDialogueFacade", {facade: "void*", user_data: "void*"});

%struct
("CDialogueConfigureIntent", {intent_id: "string", enable: "bool"});

%struct
(
  "CDialogueConfigureIntentArray",
  {entries: "CDialogueConfigureIntent*[count]", count: "int"},
);

module DialogueConfigureIntentList = {
  type t_view = list(CDialogueConfigureIntent.t_view);
  let view =
    view(
      ~read=(array: CDialogueConfigureIntentArray.t_view) => {array.entries},
      ~write=(entries: t_view) => {entries, count: entries |> List.length},
      CDialogueConfigureIntentArray.view,
    );
};

%struct
(
  "CDialogueConfigureMessage",
  {site_id: "string_opt", intents: "DialogueConfigureIntentList*?"},
);

%struct
("CStringArray", {data: "string[size]", size: "int"});

module StringList = {
  type t_view = list(string);
  let view =
    view(
      ~read=(array: CStringArray.t_view) => {array.data},
      ~write=data => {data, size: data |> List.length},
      CStringArray.view,
    );
};

%struct
(
  "CContinueSessionMessage",
  {
    session_id: "string",
    text: "string_opt",
    intent_filter: "StringList*?",
    custom_data: "string_opt",
    slot: "string_opt",
    send_intent_not_recognized: "bool",
  },
);

%struct
("CEndSessionMessage", {session_id: "string", text: "string_opt"});

%struct
(
  "CActionSessionInit",
  {
    text: "string_opt",
    intent_filter: "StringList*?",
    can_be_enqueued: "bool",
    send_intent_not_recognized: "bool",
  },
);

%struct
("CSessionInit_", {init_type: "SNIPS_SESSION_INIT_TYPE", value: "void *"});

module CSessionInit = {
  open CSessionInit_;
  open SNIPS_SESSION_INIT_TYPE;

  type sessionInit =
    | Notification(StringCast.t_view)
    | Action(CActionSessionInit.t_view);

  type t_view = sessionInit;

  let read = record => {
    switch (record.init_type) {
    | SNIPS_SESSION_INIT_TYPE_ACTION =>
      Action(!@from_voidp(CActionSessionInit.view, record.value))
    | SNIPS_SESSION_INIT_TYPE_NOTIFICATION =>
      Notification(StringCast.read(record.value))
    };
  };

  let write = init => {
    switch (init) {
    | Action(sessionInit) => {
        init_type: SNIPS_SESSION_INIT_TYPE_ACTION,
        value: to_voidp(allocate(CActionSessionInit.view, sessionInit)),
      }
    | Notification(str) => {
        init_type: SNIPS_SESSION_INIT_TYPE_NOTIFICATION,
        value: StringCast.write(str),
      }
    };
  };

  let view = Ctypes.view(~read, ~write, CSessionInit_.view);
};

%struct
(
  "CStartSessionMessage",
  {init: "CSessionInit", custom_data: "string_opt", site_id: "string"},
);

%struct
(
  "CNluIntentClassifierResult",
  {intent_name: "string", confidence_score: "float"},
);

%struct
(
  "CInstantTimeValue",
  {value: "string", grain: "SNIPS_GRAIN", precision: "SNIPS_PRECISION"},
);

%struct
("CTimeIntervalValue", {from: "string", to_: "string"});

%struct
(
  "CAmountOfMoneyValue",
  {unit: "string", value: "float", precision: "SNIPS_PRECISION"},
);

%struct
("CTemperatureValue", {unit: "string", value: "float"});

%struct
(
  "CDurationValue",
  {
    years: "int64_t",
    quarters: "int64_t",
    months: "int64_t",
    weeks: "int64_t",
    days: "int64_t",
    hours: "int64_t",
    minutes: "int64_t",
    seconds: "int64_t",
    precision: "SNIPS_PRECISION",
  },
);

%struct
("CSlotValue", {value: "void*", value_type: "SNIPS_SLOT_VALUE_TYPE"});

module SlotValue = {
  type slotValue =
    | String(string)
    | Float(float)
    | Int64(int64)
    | InstantTime(CInstantTimeValue.t_view)
    | TimeInterval(CTimeIntervalValue.t_view)
    | AmountOfMoney(CAmountOfMoneyValue.t_view)
    | Temperature(CTemperatureValue.t_view)
    | Duration(CDurationValue.t_view);

  type t_view = {
    value_type: SNIPS_SLOT_VALUE_TYPE.t,
    value: slotValue,
  };

  let read = ({value_type, value}: CSlotValue.t_view): t_view => {
    let value =
      switch (value_type) {
      | SNIPS_SLOT_VALUE_TYPE_CUSTOM
      | SNIPS_SLOT_VALUE_TYPE_MUSICALBUM
      | SNIPS_SLOT_VALUE_TYPE_MUSICARTIST
      | SNIPS_SLOT_VALUE_TYPE_MUSICTRACK
      | SNIPS_SLOT_VALUE_TYPE_CITY
      | SNIPS_SLOT_VALUE_TYPE_COUNTRY
      | SNIPS_SLOT_VALUE_TYPE_REGION => String(StringCast.read(value))
      | SNIPS_SLOT_VALUE_TYPE_NUMBER
      | SNIPS_SLOT_VALUE_TYPE_PERCENTAGE => Float(!@from_voidp(float, value))
      | SNIPS_SLOT_VALUE_TYPE_ORDINAL => Int64(!@from_voidp(int64_t, value))
      | SNIPS_SLOT_VALUE_TYPE_INSTANTTIME =>
        InstantTime(!@from_voidp(CInstantTimeValue.view, value))
      | SNIPS_SLOT_VALUE_TYPE_TIMEINTERVAL =>
        TimeInterval(!@from_voidp(CTimeIntervalValue.view, value))
      | SNIPS_SLOT_VALUE_TYPE_AMOUNTOFMONEY =>
        AmountOfMoney(!@from_voidp(CAmountOfMoneyValue.view, value))
      | SNIPS_SLOT_VALUE_TYPE_TEMPERATURE =>
        Temperature(!@from_voidp(CTemperatureValue.view, value))
      | SNIPS_SLOT_VALUE_TYPE_DURATION =>
        Duration(!@from_voidp(CDurationValue.view, value))
      };
    {value_type, value};
  };
  let write = ({value_type, value}: t_view): CSlotValue.t_view => {
    let value =
      switch (value) {
      | String(str) => StringCast.write(str)
      | Float(f) => to_voidp(allocate(float, f))
      | Int64(i) => to_voidp(allocate(int64_t, i))
      | InstantTime(v) => to_voidp(allocate(CInstantTimeValue.view, v))
      | TimeInterval(v) => to_voidp(allocate(CTimeIntervalValue.view, v))
      | AmountOfMoney(v) => to_voidp(allocate(CAmountOfMoneyValue.view, v))
      | Temperature(v) => to_voidp(allocate(CTemperatureValue.view, v))
      | Duration(v) => to_voidp(allocate(CDurationValue.view, v))
      };
    {value_type, value};
  };

  let view = Ctypes.view(~read, ~write, CSlotValue.view);
};

%struct
("CSlotValueArray", {slot_values: "SlotValue[]", size: "int32_t"});

module SlotValueList = {
  type t_view = list(SlotValue.t_view);
  let view =
    view(
      ~read=(array: CSlotValueArray.t_view): t_view => {array.slot_values},
      ~write=
        slot_values =>
          {slot_values, size: slot_values |> List.length |> Int32.of_int},
      CSlotValueArray.view,
    );
};

%struct
(
  "CSlot",
  {
    value: "SlotValue*",
    alternatives: "SlotValueList *",
    raw_value: "string",
    entity: "string",
    slot_name: "string",
    range_start: "int32_t",
    range_end: "int32_t",
    confidence_score: "float",
  },
);

%struct
("CNluSlot", {nlu_slot: "CSlot*"});

module NluSlot = {
  type t_view = CSlot.t_view;
  let read = (nluSlot: CNluSlot.t_view) => nluSlot.nlu_slot;
  let write = (slot: t_view): CNluSlot.t_view => {nlu_slot: slot};
  let view = Ctypes.view(~read, ~write, CNluSlot.view);
};

%struct
("CNluSlotArray", {entries: "NluSlot*[count]", count: "int"});

module NluSlotList = {
  type t_view = list(NluSlot.t_view);
  let read = (array: CNluSlotArray.t_view) => {
    array.entries;
  };
  let write = (entries: t_view): CNluSlotArray.t_view => {
    entries,
    count: entries |> List.length,
  };
  let view = view(~read, ~write, CNluSlotArray.view);
};

%struct
(
  "CNluIntentAlternative",
  {
    intent_name: "string_opt",
    slots: "NluSlotList*?",
    confidence_score: "float",
  },
);

%struct
(
  "CNluIntentAlternativeArray",
  {entries: "CNluIntentAlternative*[count]", count: "int"},
);

module NluIntentAlternativeList = {
  type t_view = list(CNluIntentAlternative.t_view);
  let view =
    view(
      ~read=(array: CNluIntentAlternativeArray.t_view) => {array.entries},
      ~write=entries => {entries, count: entries |> List.length},
      CNluIntentAlternativeArray.view,
    );
};

%struct
("CAsrDecodingDuration", {start: "float", end_: "float"});

%struct
(
  "CAsrToken",
  {
    value: "string",
    confidence: "float",
    range_start: "int32_t",
    range_end: "int32_t",
    time: "CAsrDecodingDuration",
  },
);

%struct
("CAsrTokenArray", {entries: "CAsrToken*[count]", count: "int"});

module AsrTokenList = {
  type t_view = list(CAsrToken.t_view);
  let view =
    view(
      ~read=(array: CAsrTokenArray.t_view) => {array.entries},
      ~write=entries => {entries, count: entries |> List.length},
      CAsrTokenArray.view,
    );
};

%struct
("CAsrTokenDoubleArray", {entries: "AsrTokenList*[count]", count: "int"});

module AsrTokenDoubleList = {
  type t_view = list(AsrTokenList.t_view);
  let view =
    view(
      ~read=(array: CAsrTokenDoubleArray.t_view) => {array.entries},
      ~write=entries => {entries, count: entries |> List.length},
      CAsrTokenDoubleArray.view,
    );
};

%struct
(
  "CIntentMessage",
  {
    session_id: "string",
    custom_data: "string_opt",
    site_id: "string",
    input: "string",
    intent: "CNluIntentClassifierResult *",
    slots: "NluSlotList*",
    alternatives: "NluIntentAlternativeList *",
    asr_tokens: "AsrTokenDoubleList *",
    asr_confidence: "float",
  },
);

%struct
(
  "CIntentNotRecognizedMessage",
  {
    site_id: "string",
    session_id: "string",
    input: "string_opt",
    custom_data: "string_opt",
    alternatives: "NluIntentAlternativeList *",
    confidence_score: "float",
  },
);

%struct
(
  "CSessionTermination",
  {
    termination_type: "SNIPS_SESSION_TERMINATION_TYPE",
    data: "string_opt",
    component: "SNIPS_HERMES_COMPONENT",
  },
);

%struct
(
  "CSessionEndedMessage",
  {
    session_id: "string",
    custom_data: "string_opt",
    termination: "CSessionTermination",
    site_id: "string",
  },
);

%struct
(
  "CSessionQueuedMessage",
  {session_id: "string", custom_data: "string_opt", site_id: "string"},
);

%struct
(
  "CSessionStartedMessage",
  {
    session_id: "string",
    custom_data: "string_opt",
    site_id: "string",
    reactivated_from_session_id: "string_opt",
  },
);

%struct
(
  "CErrorMessage",
  {session_id: "string_opt", error: "string", context: "string_opt"},
);

%struct
("CInjectionCompleteMessage", {request_id: "string"});

%struct
("CInjectionFacade", {facade: "void*", user_data: "void*"});

%struct
("CInjectionResetCompleteMessage", {request_id: "string"});

%struct
("CInjectionStatusMessage", {last_injection_date: "string"});

%struct
("CSoundFeedbackFacade", {facade: "void*", user_data: "void*"});

%struct
("CTtsFacade", {facade: "void*", user_data: "void*"});

%struct
(
  "CVersionMessage",
  {major: "uint64_t", minor: "uint64_t", patch: "uint64_t"},
);

%struct
("CMapStringToStringArrayEntry", {key: "string", values: "StringList*"});

%struct
(
  "CMapStringToStringArray",
  {entries: "CMapStringToStringArrayEntry*[count]", count: "int"},
);

module MapStringToStringList = {
  type t_view = list(CMapStringToStringArrayEntry.t_view);

  let read = (array: CMapStringToStringArray.t_view): t_view => array.entries;
  let write = (entries: t_view): CMapStringToStringArray.t_view => {
    entries,
    count: entries |> List.length,
  };
  let view = view(~read, ~write, CMapStringToStringArray.view);
};

%struct
(
  "CInjectionRequestOperation",
  {values: "MapStringToStringList*", kind: "SNIPS_INJECTION_KIND"},
);

%struct
(
  "CInjectionRequestOperations",
  {operations: "CInjectionRequestOperation* [count]", count: "int"},
);

module InjectionRequestOperationList = {
  type t_view = list(CInjectionRequestOperation.t_view);
  let view =
    view(
      ~read=(array: CInjectionRequestOperations.t_view) => {array.operations},
      ~write=operations => {operations, count: operations |> List.length},
      CInjectionRequestOperations.view,
    );
};

%struct
(
  "CInjectionRequestMessage",
  {
    operations: "InjectionRequestOperationList*",
    lexicon: "MapStringToStringList*",
    cross_language: "string_opt",
    id: "string_opt",
  },
);

%struct
("CInjectionResetRequestMessage", {request_id: "string"});

%struct
(
  "CMqttOptions",
  {
    broker_address: "string",
    username: "string_opt",
    password: "string_opt",
    tls_hostname: "string_opt",
    tls_ca_file: "StringList *?",
    tls_ca_path: "StringList *?",
    tls_client_key: "string_opt",
    tls_client_cert: "string_opt",
    tls_disable_root_store: "bool",
  },
);

%struct
("CSiteMessage", {site_id: "string", session_id: "string_opt"});

%struct
(
  "CRegisterSoundMessage",
  {sound_id: "string", wav_sound: "uint8_t*", wav_sound_len: "int"},
);

module RegisterSoundMessage = {
  type t_view = {
    sound_id: string,
    wav_sound: list(Unsigned.UInt8.t),
    wav_sound_len: int,
  };

  let read = (message: CRegisterSoundMessage.t_view): t_view => {
    sound_id: message.sound_id,
    wav_sound:
      CArray.from_ptr(message.wav_sound, message.wav_sound_len)
      |> CArray.to_list,
    wav_sound_len: message.wav_sound_len,
  };
  let write = (message: t_view): CRegisterSoundMessage.t_view => {
    let wav_sound =
      Ctypes.allocate_n(uint8_t, message.wav_sound |> List.length);
    let pointerAddress = raw_address_of_ptr(to_voidp @@ wav_sound);

    message.wav_sound
    |> List.iteri((index, elt) => {
         let ptr =
           Nativeint.(add(pointerAddress, of_int(index * sizeof(uint8_t))));
         from_voidp(uint8_t, ptr_of_raw_address @@ ptr) <-@ elt;
       });

    {
      sound_id: message.sound_id,
      wav_sound,
      wav_sound_len: message.wav_sound |> List.length,
    };
  };

  let view = view(~read, ~write, CRegisterSoundMessage.view);
};

%struct
(
  "CTextCapturedMessage",
  {
    text: "string",
    tokens: "AsrTokenList*?",
    likelihood: "float",
    seconds: "float",
    site_id: "string",
    session_id: "string_opt",
  },
);