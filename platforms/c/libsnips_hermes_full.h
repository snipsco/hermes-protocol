#ifndef LIB_HERMES_H_
#define LIB_HERMES_H_

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Enum representing the grain of a resolved date related value
 */
typedef enum {
  /**
   * The resolved value has a granularity of a year
   */
  SNIPS_GRAIN_YEAR = 0,
  /**
   * The resolved value has a granularity of a quarter
   */
  SNIPS_GRAIN_QUARTER = 1,
  /**
   * The resolved value has a granularity of a mount
   */
  SNIPS_GRAIN_MONTH = 2,
  /**
   * The resolved value has a granularity of a week
   */
  SNIPS_GRAIN_WEEK = 3,
  /**
   * The resolved value has a granularity of a day
   */
  SNIPS_GRAIN_DAY = 4,
  /**
   * The resolved value has a granularity of an hour
   */
  SNIPS_GRAIN_HOUR = 5,
  /**
   * The resolved value has a granularity of a minute
   */
  SNIPS_GRAIN_MINUTE = 6,
  /**
   * The resolved value has a granularity of a second
   */
  SNIPS_GRAIN_SECOND = 7,
} SNIPS_GRAIN;

typedef enum {
  SNIPS_INJECTION_KIND_ADD = 1,
  SNIPS_INJECTION_KIND_ADD_FROM_VANILLA = 2,
} SNIPS_INJECTION_KIND;

/**
 * Enum describing the precision of a resolved value
 */
typedef enum {
  /**
   * The resolved value is approximate
   */
  SNIPS_PRECISION_APPROXIMATE = 0,
  /**
   * The resolved value is exact
   */
  SNIPS_PRECISION_EXACT = 1,
} SNIPS_PRECISION;

/**
 * Used as a return type of functions that can encounter errors
 */
typedef enum {
  /**
   * The function returned successfully
   */
  SNIPS_RESULT_OK = 0,
  /**
   * The function encountered an error, you can retrieve it using the dedicated function
   */
  SNIPS_RESULT_KO = 1,
} SNIPS_RESULT;

typedef enum {
  SNIPS_SESSION_INIT_TYPE_ACTION = 1,
  SNIPS_SESSION_INIT_TYPE_NOTIFICATION = 2,
} SNIPS_SESSION_INIT_TYPE;

typedef enum {
  SNIPS_SESSION_TERMINATION_TYPE_NOMINAL = 1,
  SNIPS_SESSION_TERMINATION_TYPE_SITE_UNAVAILABLE = 2,
  SNIPS_SESSION_TERMINATION_TYPE_ABORTED_BY_USER = 3,
  SNIPS_SESSION_TERMINATION_TYPE_INTENT_NOT_RECOGNIZED = 4,
  SNIPS_SESSION_TERMINATION_TYPE_TIMEOUT = 5,
  SNIPS_SESSION_TERMINATION_TYPE_ERROR = 6,
} SNIPS_SESSION_TERMINATION_TYPE;

/**
 * Enum type describing how to cast the value of a CSlotValue
 */
typedef enum {
  /**
   * Custom type represented by a char *
   */
  SNIPS_SLOT_VALUE_TYPE_CUSTOM = 1,
  /**
   * Number type represented by a CNumberValue
   */
  SNIPS_SLOT_VALUE_TYPE_NUMBER = 2,
  /**
   * Ordinal type represented by a COrdinalValue
   */
  SNIPS_SLOT_VALUE_TYPE_ORDINAL = 3,
  /**
   * Instant type represented by a CInstantTimeValue
   */
  SNIPS_SLOT_VALUE_TYPE_INSTANTTIME = 4,
  /**
   * Interval type represented by a CTimeIntervalValue
   */
  SNIPS_SLOT_VALUE_TYPE_TIMEINTERVAL = 5,
  /**
   * Amount of money type represented by a CAmountOfMoneyValue
   */
  SNIPS_SLOT_VALUE_TYPE_AMOUNTOFMONEY = 6,
  /**
   * Temperature type represented by a CTemperatureValue
   */
  SNIPS_SLOT_VALUE_TYPE_TEMPERATURE = 7,
  /**
   * Duration type represented by a CDurationValue
   */
  SNIPS_SLOT_VALUE_TYPE_DURATION = 8,
  /**
   * Percentage type represented by a CPercentageValue
   */
  SNIPS_SLOT_VALUE_TYPE_PERCENTAGE = 9,
  /**
   * Music Album type represented by a char *
   */
  SNIPS_SLOT_VALUE_TYPE_MUSICALBUM = 10,
  /**
   * Music Artist type represented by a char *
   */
  SNIPS_SLOT_VALUE_TYPE_MUSICARTIST = 11,
  /**
   * Music Track type represented by a char *
   */
  SNIPS_SLOT_VALUE_TYPE_MUSICTRACK = 12,
} SNIPS_SLOT_VALUE_TYPE;

typedef struct {
  const void *facade;
  void *user_data;
} CAsrBackendFacade;

typedef struct {
  const char *site_id;
  /**
   * Nullable
   */
  const char *session_id;
  int64_t start_signal_ms;
} CAsrStartListeningMessage;

typedef struct {
  const char *site_id;
  /**
   * Nullable
   */
  const char *session_id;
} CSiteMessage;

typedef struct {
  float start;
  float end;
} CAsrDecodingDuration;

typedef struct {
  const char *value;
  float confidence;
  int32_t range_start;
  int32_t range_end;
  CAsrDecodingDuration time;
} CAsrToken;

typedef struct {
  const CAsrToken *const *entries;
  int count;
} CAsrTokenArray;

typedef struct {
  const char *text;
  /**
   * Nullable
   */
  const CAsrTokenArray *tokens;
  float likelihood;
  float seconds;
  const char *site_id;
  /**
   * Nullable
   */
  const char *session_id;
} CTextCapturedMessage;

typedef struct {
  const void *facade;
  void *user_data;
} CAsrFacade;

typedef struct {
  const void *facade;
  void *user_data;
} CAudioServerBackendFacade;

typedef struct {
  const uint8_t *wav_frame;
  int wav_frame_len;
  const char *site_id;
} CAudioFrameMessage;

typedef struct {
  const char *id;
  const char *site_id;
} CPlayFinishedMessage;

typedef struct {
  const char *id;
  const uint8_t *wav_bytes;
  int wav_bytes_len;
  const char *site_id;
} CPlayBytesMessage;

typedef struct {
  const void *facade;
  void *user_data;
} CAudioServerFacade;

typedef struct {
  const void *handler;
  void *user_data;
} CProtocolHandler;

typedef struct {
  const void *facade;
  void *user_data;
} CDialogueBackendFacade;

/**
 * Result of the intent classifier
 */
typedef struct {
  /**
   * Name of the intent detected
   */
  const char *intent_name;
  /**
   * Between 0 and 1
   */
  float confidence_score;
} CNluIntentClassifierResult;

/**
 * A slot value
 */
typedef struct {
  /**
   * Points to either a *const char, a CNumberValue, a COrdinalValue,
   * a CInstantTimeValue, a CTimeIntervalValue, a CAmountOfMoneyValue,
   * a CTemperatureValue or a CDurationValue depending on value_type
   */
  const void *value;
  /**
   * The type of the value
   */
  SNIPS_SLOT_VALUE_TYPE value_type;
} CSlotValue;

/**
 * Struct describing a Slot
 */
typedef struct {
  /**
   * The resolved value of the slot
   */
  CSlotValue value;
  /**
   * The raw value as it appears in the input text
   */
  const char *raw_value;
  /**
   * Name of the entity type of the slot
   */
  const char *entity;
  /**
   * Name of the slot
   */
  const char *slot_name;
  /**
   * Start index of raw value in input text
   */
  int32_t range_start;
  /**
   * End index of raw value in input text
   */
  int32_t range_end;
  /**
   * Confidence score of the slot
   */
  float confidence_score;
} CSlot;

typedef struct {
  const CSlot *nlu_slot;
} CNluSlot;

typedef struct {
  const CNluSlot *const *entries;
  int count;
} CNluSlotArray;

typedef struct {
  const CAsrTokenArray *const *entries;
  int count;
} CAsrTokenDoubleArray;

typedef struct {
  const char *session_id;
  /**
   * Nullable
   */
  const char *custom_data;
  const char *site_id;
  const char *input;
  const CNluIntentClassifierResult *intent;
  /**
   * Nullable
   */
  const CNluSlotArray *slots;
  /**
   * Nullable, the first array level represents the asr invocation, the second one the tokens
   */
  const CAsrTokenDoubleArray *asr_tokens;
  /**
   * Nullable
   */
  const float *asr_confidence;
} CIntentMessage;

typedef struct {
  const char *site_id;
  const char *session_id;
  /**
   * Nullable
   */
  const char *input;
  /**
   * Nullable
   */
  const char *custom_data;
  float confidence_score;
} CIntentNotRecognizedMessage;

typedef struct {
  SNIPS_SESSION_TERMINATION_TYPE termination_type;
  /**
   * Nullable,
   */
  const char *data;
} CSessionTermination;

typedef struct {
  const char *session_id;
  /**
   * Nullable
   */
  const char *custom_data;
  CSessionTermination termination;
  const char *site_id;
} CSessionEndedMessage;

typedef struct {
  const char *session_id;
  /**
   * Nullable
   */
  const char *custom_data;
  const char *site_id;
} CSessionQueuedMessage;

typedef struct {
  const char *session_id;
  /**
   * Nullable
   */
  const char *custom_data;
  const char *site_id;
  /**
   * Nullable
   */
  const char *reactivated_from_session_id;
} CSessionStartedMessage;

typedef struct {
  const char *intent_name;
  /**
   * Optional Boolean 0 => false, 1 => true other values => null
   */
  unsigned char enable;
} CDialogueConfigureIntent;

typedef struct {
  const CDialogueConfigureIntent *const *entries;
  int count;
} CDialogueConfigureIntentArray;

typedef struct {
  /**
   * Nullable
   */
  const char *site_id;
  /**
   * Nullable
   */
  const CDialogueConfigureIntentArray *intents;
} CDialogueConfigureMessage;

/**
 * An array of strings
 */
typedef struct {
  /**
   * Pointer to the first element of the array
   */
  const char *const *data;
  /**
   * Number of elements in the array
   */
  int size;
} CStringArray;

typedef struct {
  const char *session_id;
  const char *text;
  /**
   * Nullable
   */
  const CStringArray *intent_filter;
  /**
   * Nullable
   */
  const char *custom_data;
  /**
   * Nullable
   */
  const char *slot;
  unsigned char send_intent_not_recognized;
} CContinueSessionMessage;

typedef struct {
  const char *session_id;
  /**
   * Nullable
   */
  const char *text;
} CEndSessionMessage;

typedef struct {
  SNIPS_SESSION_INIT_TYPE init_type;
  /**
   * Points to either a *const char, a *const CActionSessionInit
   */
  const void *value;
} CSessionInit;

typedef struct {
  CSessionInit init;
  const char *custom_data;
  const char *site_id;
} CStartSessionMessage;

typedef struct {
  const void *facade;
  void *user_data;
} CDialogueFacade;

typedef struct {
  /**
   * Nullable
   */
  const char *session_id;
  const char *error;
  /**
   * Nullable
   */
  const char *context;
} CErrorMessage;

typedef struct {
  const void *facade;
  void *user_data;
} CHotwordBackendFacade;

typedef struct {
  const char *site_id;
  const char *model_id;
} CHotwordDetectedMessage;

typedef struct {
  const void *facade;
  void *user_data;
} CHotwordFacade;

typedef struct {
  const void *facade;
  void *user_data;
} CInjectionFacade;

typedef struct {
  const char *key;
  const CStringArray *value;
} CMapStringToStringArrayEntry;

typedef struct {
  const CMapStringToStringArrayEntry *const *entries;
  int count;
} CMapStringToStringArray;

typedef struct {
  const CMapStringToStringArray *values;
  SNIPS_INJECTION_KIND kind;
} CInjectionRequestOperation;

typedef struct {
  const CInjectionRequestOperation *const *operations;
  int count;
} CInjectionRequestOperations;

typedef struct {
  const CInjectionRequestOperations *operations;
  const CMapStringToStringArray *lexicon;
  /**
   * Nullable
   */
  const char *cross_language;
  /**
   * Nullable
   */
  const char *id;
} CInjectionRequestMessage;

typedef struct {
  const char *last_injection_date;
} CInjectionStatusMessage;

typedef struct {
  const void *facade;
  void *user_data;
} CNluBackendFacade;

typedef struct {
  const void *facade;
  void *user_data;
} CNluFacade;

typedef struct {
  /**
   * Nullable
   */
  const char *id;
  const char *input;
  const CNluIntentClassifierResult *intent;
  /**
   * Nullable
   */
  const CNluSlotArray *slots;
  /**
   * Nullable
   */
  const char *session_id;
} CNluIntentMessage;

typedef struct {
  const char *input;
  /**
   * Nullable
   */
  const char *id;
  /**
   * Nullable
   */
  const char *session_id;
  float confidence_score;
} CNluIntentNotRecognizedMessage;

typedef struct {
  const char *input;
  /**
   * Nullable
   */
  const CAsrTokenArray *asr_tokens;
  /**
   * Nullable
   */
  const CStringArray *intent_filter;
  /**
   * Nullable
   */
  const char *id;
  /**
   * Nullable
   */
  const char *session_id;
} CNluQueryMessage;

typedef struct {
  /**
   * Nullable
   */
  const char *id;
  const char *input;
  const char *intent_name;
  /**
   * Nullable
   */
  const CNluSlot *slot;
  /**
   * Nullable
   */
  const char *session_id;
} CNluSlotMessage;

typedef struct {
  const char *input;
  const CAsrTokenArray *asr_tokens;
  const char *intent_name;
  const char *slot_name;
  /**
   * Nullable
   */
  const char *id;
  /**
   * Nullable
   */
  const char *session_id;
} CNluSlotQueryMessage;

typedef struct {
  const char *sound_id;
  const uint8_t *wav_sound;
  int wav_sound_len;
} CRegisterSoundMessage;

typedef struct {
  /**
   * Nullable
   */
  const char *id;
  /**
   * Nullable
   */
  const char *session_id;
} CSayFinishedMessage;

typedef struct {
  const char *text;
  /**
   * Nullable
   */
  const char *lang;
  /**
   * Nullable
   */
  const char *id;
  const char *site_id;
  /**
   * Nullable
   */
  const char *session_id;
} CSayMessage;

typedef struct {
  const void *facade;
  void *user_data;
} CSoundFeedbackBackendFacade;

typedef struct {
  const void *facade;
  void *user_data;
} CSoundFeedbackFacade;

typedef struct {
  const void *facade;
  void *user_data;
} CTtsBackendFacade;

typedef struct {
  const void *facade;
  void *user_data;
} CTtsFacade;

typedef struct {
  uint64_t major;
  uint64_t minor;
  uint64_t patch;
} CVersionMessage;

/**
 * A struct representing the configuration of the MQTT client
 */
typedef struct {
  /**
   * Address of the MQTT broker in the form `ip:port`
   */
  char *broker_address;
  /**
   * Username to use on the broker. Nullable
   */
  char *username;
  /**
   * Password to use on the broker. Nullable
   */
  char *password;
  /**
   * Hostname to use for the TLS configuration. Nullable, setting a value enables TLS
   */
  char *tls_hostname;
  /**
   * CA files to use if TLS is enabled. Nullable
   */
  CStringArray *tls_ca_file;
  /**
   * CA path to use if TLS is enabled. Nullable
   */
  CStringArray *tls_ca_path;
  /**
   * Client key to use if TLS is enabled. Nullable
   */
  char *tls_client_key;
  /**
   * Client cert to use if TLS is enabled. Nullable
   */
  char *tls_client_cert;
  /**
   * Boolean indicating if the root store should be disabled if TLS is enabled. The is
   * interpreted as a boolean, 0 meaning false, all other values meaning true
   */
  unsigned char tls_disable_root_store;
} CMqttOptions;

typedef struct {
  /**
   * Nullable
   */
  const char *text;
  /**
   * Nullable
   */
  const CStringArray *intent_filter;
  unsigned char can_be_enqueued;
  unsigned char send_intent_not_recognized;
} CActionSessionInit;

/**
 * Representation of a number value
 */
typedef double CNumberValue;

/**
 * Representation of an ordinal value
 */
typedef int64_t COrdinalValue;

/**
 * Representation of a percentage value
 */
typedef double CPercentageValue;

/**
 * Representation of an instant value
 */
typedef struct {
  /**
   * String representation of the instant
   */
  const char *value;
  /**
   * The grain of the resolved instant
   */
  SNIPS_GRAIN grain;
  /**
   * The precision of the resolved instant
   */
  SNIPS_PRECISION precision;
} CInstantTimeValue;

/**
 * Representation of an interval value
 */
typedef struct {
  /**
   * String representation of the beginning of the interval
   */
  const char *from;
  /**
   * String representation of the end of the interval
   */
  const char *to;
} CTimeIntervalValue;

/**
 * Representation of an amount of money value
 */
typedef struct {
  /**
   * The currency
   */
  const char *unit;
  /**
   * The amount of money
   */
  float value;
  /**
   * The precision of the resolved value
   */
  SNIPS_PRECISION precision;
} CAmountOfMoneyValue;

/**
 * Representation of a temperature value
 */
typedef struct {
  /**
   * The unit used
   */
  const char *unit;
  /**
   * The temperature resolved
   */
  float value;
} CTemperatureValue;

/**
 * Representation of a duration value
 */
typedef struct {
  /**
   * Number of years in the duration
   */
  int64_t years;
  /**
   * Number of quarters in the duration
   */
  int64_t quarters;
  /**
   * Number of months in the duration
   */
  int64_t months;
  /**
   * Number of weeks in the duration
   */
  int64_t weeks;
  /**
   * Number of days in the duration
   */
  int64_t days;
  /**
   * Number of hours in the duration
   */
  int64_t hours;
  /**
   * Number of minutes in the duration
   */
  int64_t minutes;
  /**
   * Number of seconds in the duration
   */
  int64_t seconds;
  /**
   * Precision of the resolved value
   */
  SNIPS_PRECISION precision;
} CDurationValue;

SNIPS_RESULT hermes_asr_backend_publish_start_listening(const CAsrBackendFacade *facade,
                                                        void (*handler)(const CAsrStartListeningMessage*, void*));

SNIPS_RESULT hermes_asr_backend_publish_stop_listening(const CAsrBackendFacade *facade,
                                                       void (*handler)(const CSiteMessage*, void*));

SNIPS_RESULT hermes_asr_backend_subscribe_partial_text_captured(const CAsrBackendFacade *facade,
                                                                const CTextCapturedMessage *message);

SNIPS_RESULT hermes_asr_backend_subscribe_text_captured(const CAsrBackendFacade *facade,
                                                        const CTextCapturedMessage *message);

SNIPS_RESULT hermes_asr_publish_start_listening(const CAsrFacade *facade,
                                                const CAsrStartListeningMessage *message);

SNIPS_RESULT hermes_asr_publish_stop_listening(const CAsrFacade *facade,
                                               const CSiteMessage *message);

SNIPS_RESULT hermes_asr_subscribe_partial_text_captured(const CAsrFacade *facade,
                                                        void (*handler)(const CTextCapturedMessage*, void*));

SNIPS_RESULT hermes_asr_subscribe_text_captured(const CAsrFacade *facade,
                                                void (*handler)(const CTextCapturedMessage*, void*));

SNIPS_RESULT hermes_audio_server_backend_publish_audio_frame(const CAudioServerBackendFacade *facade,
                                                             const CAudioFrameMessage *message);

SNIPS_RESULT hermes_audio_server_backend_publish_play_finished(const CAudioServerBackendFacade *facade,
                                                               const CPlayFinishedMessage *message);

SNIPS_RESULT hermes_audio_server_backend_subscribe_all_play_bytes(const CAudioServerBackendFacade *facade,
                                                                  void (*handler)(const CPlayBytesMessage*, void*));

SNIPS_RESULT hermes_audio_server_backend_subscribe_play_bytes(const CAudioServerBackendFacade *facade,
                                                              const char *site_id,
                                                              void (*handler)(const CPlayBytesMessage*, void*));

SNIPS_RESULT hermes_audio_server_publish_play_bytes(const CAudioServerFacade *facade,
                                                    const CPlayBytesMessage *message);

SNIPS_RESULT hermes_audio_server_subscribe_all_play_finished(const CAudioServerFacade *facade,
                                                             void (*handler)(const CPlayFinishedMessage*, void*));

SNIPS_RESULT hermes_audio_server_subscribe_audio_frame(const CAudioServerFacade *facade,
                                                       const char *site_id,
                                                       void (*handler)(const CAudioFrameMessage*, void*));

SNIPS_RESULT hermes_audio_server_subscribe_play_finished(const CAudioServerFacade *facade,
                                                         const char *site_id,
                                                         void (*handler)(const CPlayFinishedMessage*, void*));

SNIPS_RESULT hermes_destroy_mqtt_protocol_handler(CProtocolHandler *handler);

SNIPS_RESULT hermes_dialogue_backend_publish_intent(const CDialogueBackendFacade *facade,
                                                    const CIntentMessage *message);

SNIPS_RESULT hermes_dialogue_backend_publish_intent_not_recognized(const CDialogueBackendFacade *facade,
                                                                   const CIntentNotRecognizedMessage *message);

SNIPS_RESULT hermes_dialogue_backend_publish_session_ended(const CDialogueBackendFacade *facade,
                                                           const CSessionEndedMessage *message);

SNIPS_RESULT hermes_dialogue_backend_publish_session_queued(const CDialogueBackendFacade *facade,
                                                            const CSessionQueuedMessage *message);

SNIPS_RESULT hermes_dialogue_backend_publish_session_started(const CDialogueBackendFacade *facade,
                                                             const CSessionStartedMessage *message);

SNIPS_RESULT hermes_dialogue_backend_subscribe_configure(const CDialogueBackendFacade *facade,
                                                         void (*handler)(const CDialogueConfigureMessage*, void*));

SNIPS_RESULT hermes_dialogue_backend_subscribe_continue_session(const CDialogueBackendFacade *facade,
                                                                void (*handler)(const CContinueSessionMessage*, void*));

SNIPS_RESULT hermes_dialogue_backend_subscribe_end_session(const CDialogueBackendFacade *facade,
                                                           void (*handler)(const CEndSessionMessage*, void*));

SNIPS_RESULT hermes_dialogue_backend_subscribe_start_session(const CDialogueBackendFacade *facade,
                                                             void (*handler)(const CStartSessionMessage*, void*));

SNIPS_RESULT hermes_dialogue_publish_configure(const CDialogueFacade *facade,
                                               const CDialogueConfigureMessage *message);

SNIPS_RESULT hermes_dialogue_publish_continue_session(const CDialogueFacade *facade,
                                                      const CContinueSessionMessage *message);

SNIPS_RESULT hermes_dialogue_publish_end_session(const CDialogueFacade *facade,
                                                 const CEndSessionMessage *message);

SNIPS_RESULT hermes_dialogue_publish_start_session(const CDialogueFacade *facade,
                                                   const CStartSessionMessage *message);

SNIPS_RESULT hermes_dialogue_subscribe_intent(const CDialogueFacade *facade,
                                              const char *intent_name,
                                              void (*handler)(const CIntentMessage*, void*));

SNIPS_RESULT hermes_dialogue_subscribe_intent_not_recognized(const CDialogueFacade *facade,
                                                             void (*handler)(const CIntentNotRecognizedMessage*, void*));

SNIPS_RESULT hermes_dialogue_subscribe_intents(const CDialogueFacade *facade,
                                               void (*handler)(const CIntentMessage*, void*));

SNIPS_RESULT hermes_dialogue_subscribe_session_ended(const CDialogueFacade *facade,
                                                     void (*handler)(const CSessionEndedMessage*, void*));

SNIPS_RESULT hermes_dialogue_subscribe_session_queued(const CDialogueFacade *facade,
                                                      void (*handler)(const CSessionQueuedMessage*, void*));

SNIPS_RESULT hermes_dialogue_subscribe_session_started(const CDialogueFacade *facade,
                                                       void (*handler)(const CSessionStartedMessage*, void*));

SNIPS_RESULT hermes_drop_asr_backend_facade(const CAsrBackendFacade *cstruct);

SNIPS_RESULT hermes_drop_asr_facade(const CAsrFacade *cstruct);

SNIPS_RESULT hermes_drop_audio_frame_message(const CAudioFrameMessage *cstruct);

SNIPS_RESULT hermes_drop_audio_server_backend_facade(const CAudioServerBackendFacade *cstruct);

SNIPS_RESULT hermes_drop_audio_server_facade(const CAudioServerFacade *cstruct);

SNIPS_RESULT hermes_drop_continue_session_message(const CContinueSessionMessage *cstruct);

SNIPS_RESULT hermes_drop_dialogue_backend_facade(const CDialogueBackendFacade *cstruct);

SNIPS_RESULT hermes_drop_dialogue_configure_message(const CDialogueConfigureMessage *cstruct);

SNIPS_RESULT hermes_drop_dialogue_facade(const CDialogueFacade *cstruct);

SNIPS_RESULT hermes_drop_end_session_message(const CEndSessionMessage *cstruct);

SNIPS_RESULT hermes_drop_error_message(const CErrorMessage *cstruct);

SNIPS_RESULT hermes_drop_hotword_backend_facade(const CHotwordBackendFacade *cstruct);

SNIPS_RESULT hermes_drop_hotword_detected_message(const CHotwordDetectedMessage *cstruct);

SNIPS_RESULT hermes_drop_hotword_facade(const CHotwordFacade *cstruct);

SNIPS_RESULT hermes_drop_injection_facade(const CInjectionFacade *cstruct);

SNIPS_RESULT hermes_drop_injection_request_message(const CInjectionRequestMessage *cstruct);

SNIPS_RESULT hermes_drop_injection_status_message(const CInjectionStatusMessage *cstruct);

SNIPS_RESULT hermes_drop_intent_message(const CIntentMessage *cstruct);

SNIPS_RESULT hermes_drop_intent_not_recognized_message(const CIntentNotRecognizedMessage *cstruct);

SNIPS_RESULT hermes_drop_nlu_backend_facade(const CNluBackendFacade *cstruct);

SNIPS_RESULT hermes_drop_nlu_facade(const CNluFacade *cstruct);

SNIPS_RESULT hermes_drop_nlu_intent_message(const CNluIntentMessage *cstruct);

SNIPS_RESULT hermes_drop_nlu_intent_not_recognized_message(const CNluIntentNotRecognizedMessage *cstruct);

SNIPS_RESULT hermes_drop_nlu_query_message(const CNluQueryMessage *cstruct);

SNIPS_RESULT hermes_drop_nlu_slot_message(const CNluSlotMessage *cstruct);

SNIPS_RESULT hermes_drop_nlu_slot_query_message(const CNluSlotQueryMessage *cstruct);

SNIPS_RESULT hermes_drop_play_bytes_message(const CPlayBytesMessage *cstruct);

SNIPS_RESULT hermes_drop_play_finished_message(const CPlayFinishedMessage *cstruct);

SNIPS_RESULT hermes_drop_register_sound_message(const CRegisterSoundMessage *cstruct);

SNIPS_RESULT hermes_drop_say_finished_message(const CSayFinishedMessage *cstruct);

SNIPS_RESULT hermes_drop_say_message(const CSayMessage *cstruct);

SNIPS_RESULT hermes_drop_session_ended_message(const CSessionEndedMessage *cstruct);

SNIPS_RESULT hermes_drop_session_queued_message(const CSessionQueuedMessage *cstruct);

SNIPS_RESULT hermes_drop_session_started_message(const CSessionStartedMessage *cstruct);

SNIPS_RESULT hermes_drop_site_message(const CSiteMessage *cstruct);

SNIPS_RESULT hermes_drop_sound_feedback_backend_facade(const CSoundFeedbackBackendFacade *cstruct);

SNIPS_RESULT hermes_drop_sound_feedback_facade(const CSoundFeedbackFacade *cstruct);

SNIPS_RESULT hermes_drop_start_session_message(const CStartSessionMessage *cstruct);

SNIPS_RESULT hermes_drop_text_captured_message(const CTextCapturedMessage *cstruct);

SNIPS_RESULT hermes_drop_tts_backend_facade(const CTtsBackendFacade *cstruct);

SNIPS_RESULT hermes_drop_tts_facade(const CTtsFacade *cstruct);

SNIPS_RESULT hermes_drop_version_message(const CVersionMessage *cstruct);

SNIPS_RESULT hermes_enable_debug_logs(void);

/**
 * Used to retrieve the last error that happened in this thread. A function encountered an
 * error if its return type is of type SNIPS_RESULT and it returned SNIPS_RESULT_KO
 */
SNIPS_RESULT hermes_get_last_error(const char **error);

SNIPS_RESULT hermes_hotword_backend_publish_detected(const CHotwordBackendFacade *facade,
                                                     const char *hotword_id,
                                                     const CHotwordDetectedMessage *message);

SNIPS_RESULT hermes_hotword_subscribe_all_detected(const CHotwordFacade *facade,
                                                   void (*handler)(const CHotwordDetectedMessage*, void*));

SNIPS_RESULT hermes_hotword_subscribe_detected(const CHotwordFacade *facade,
                                               const char *hotword_id,
                                               void (*handler)(const CHotwordDetectedMessage*, void*));

SNIPS_RESULT hermes_injection_publish_injection_request(const CInjectionFacade *facade,
                                                        const CInjectionRequestMessage *message);

SNIPS_RESULT hermes_injection_publish_injection_status_request(const CInjectionFacade *facade);

SNIPS_RESULT hermes_injection_subscribe_injection_status(const CInjectionFacade *facade,
                                                         void (*handler)(const CInjectionStatusMessage*, void*));

SNIPS_RESULT hermes_nlu_backend_publish_intent_not_recognized(const CNluBackendFacade *facade,
                                                              const CNluIntentNotRecognizedMessage *message);

SNIPS_RESULT hermes_nlu_backend_publish_intent_parsed(const CNluBackendFacade *facade,
                                                      const CNluIntentMessage *message);

SNIPS_RESULT hermes_nlu_backend_publish_slot_parsed(const CNluBackendFacade *facade,
                                                    const CNluSlotMessage *message);

SNIPS_RESULT hermes_nlu_backend_subscribe_partial_query(const CNluBackendFacade *facade,
                                                        void (*handler)(const CNluSlotQueryMessage*, void*));

SNIPS_RESULT hermes_nlu_backend_subscribe_query(const CNluBackendFacade *facade,
                                                void (*handler)(const CNluQueryMessage*, void*));

SNIPS_RESULT hermes_nlu_publish_partial_query(const CNluFacade *facade,
                                              const CNluSlotQueryMessage *message);

SNIPS_RESULT hermes_nlu_publish_query(const CNluFacade *facade, const CNluQueryMessage *message);

SNIPS_RESULT hermes_nlu_subscribe_intent_not_recognized(const CNluFacade *facade,
                                                        void (*handler)(const CNluIntentNotRecognizedMessage*, void*));

SNIPS_RESULT hermes_nlu_subscribe_intent_parsed(const CNluFacade *facade,
                                                void (*handler)(const CNluIntentMessage*, void*));

SNIPS_RESULT hermes_nlu_subscribe_slot_parsed(const CNluFacade *facade,
                                              void (*handler)(const CNluSlotMessage*, void*));

SNIPS_RESULT hermes_protocol_handler_asr_backend_facade(const CProtocolHandler *handler,
                                                        const CAsrBackendFacade **facade);

SNIPS_RESULT hermes_protocol_handler_asr_facade(const CProtocolHandler *handler,
                                                const CAsrFacade **facade);

SNIPS_RESULT hermes_protocol_handler_audio_server_backend_facade(const CProtocolHandler *handler,
                                                                 const CAudioServerBackendFacade **facade);

SNIPS_RESULT hermes_protocol_handler_audio_server_facade(const CProtocolHandler *handler,
                                                         const CAudioServerFacade **facade);

SNIPS_RESULT hermes_protocol_handler_dialogue_backend_facade(const CProtocolHandler *handler,
                                                             const CDialogueBackendFacade **facade);

SNIPS_RESULT hermes_protocol_handler_dialogue_facade(const CProtocolHandler *handler,
                                                     const CDialogueFacade **facade);

SNIPS_RESULT hermes_protocol_handler_hotword_backend_facade(const CProtocolHandler *handler,
                                                            const CHotwordBackendFacade **facade);

SNIPS_RESULT hermes_protocol_handler_hotword_facade(const CProtocolHandler *handler,
                                                    const CHotwordFacade **facade);

SNIPS_RESULT hermes_protocol_handler_injection_facade(const CProtocolHandler *handler,
                                                      const CInjectionFacade **facade);

SNIPS_RESULT hermes_protocol_handler_new_mqtt(const CProtocolHandler **handler,
                                              const char *broker_address,
                                              void *user_data);

SNIPS_RESULT hermes_protocol_handler_new_mqtt_with_options(const CProtocolHandler **handler,
                                                           const CMqttOptions *mqtt_options,
                                                           void *user_data);

SNIPS_RESULT hermes_protocol_handler_nlu_backend_facade(const CProtocolHandler *handler,
                                                        const CNluBackendFacade **facade);

SNIPS_RESULT hermes_protocol_handler_nlu_facade(const CProtocolHandler *handler,
                                                const CNluFacade **facade);

SNIPS_RESULT hermes_protocol_handler_sound_feedback_backend_facade(const CProtocolHandler *handler,
                                                                   const CSoundFeedbackBackendFacade **facade);

SNIPS_RESULT hermes_protocol_handler_sound_feedback_facade(const CProtocolHandler *handler,
                                                           const CSoundFeedbackFacade **facade);

SNIPS_RESULT hermes_protocol_handler_tts_backend_facade(const CProtocolHandler *handler,
                                                        const CTtsBackendFacade **facade);

SNIPS_RESULT hermes_protocol_handler_tts_facade(const CProtocolHandler *handler,
                                                const CTtsFacade **facade);

SNIPS_RESULT hermes_sound_feedback_publish_toggle_off(const CSoundFeedbackFacade *facade,
                                                      const CSiteMessage *message);

SNIPS_RESULT hermes_sound_feedback_publish_toggle_on(const CSoundFeedbackFacade *facade,
                                                     const CSiteMessage *message);

SNIPS_RESULT hermes_tts_backend_publish_say_finished(const CTtsBackendFacade *facade,
                                                     const CSayFinishedMessage *message);

SNIPS_RESULT hermes_tts_backend_subscribe_register_sound(const CTtsBackendFacade *facade,
                                                         void (*handler)(const CRegisterSoundMessage*, void*));

SNIPS_RESULT hermes_tts_backend_subscribe_say(const CTtsBackendFacade *facade,
                                              void (*handler)(const CSayMessage*, void*));

SNIPS_RESULT hermes_tts_publish_register_sound(const CTtsFacade *facade,
                                               const CRegisterSoundMessage *message);

SNIPS_RESULT hermes_tts_publish_say(const CTtsFacade *facade, const CSayMessage *message);

SNIPS_RESULT hermes_tts_subscribe_say_finished(const CTtsFacade *facade,
                                               void (*handler)(const CSayFinishedMessage*, void*));

#endif /* LIB_HERMES_H_ */
