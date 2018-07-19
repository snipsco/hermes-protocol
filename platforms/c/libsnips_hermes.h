#ifndef LIB_HERMES_H_
#define LIB_HERMES_H_

#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

/*
 * Enum representing the grain of a resolved date related value
 */
typedef enum {
  /*
   * The resolved value as a granularity of a year
   */
  SNIPS_GRAIN_YEAR = 0,
  /*
   * The resolved value as a granularity of a quarter
   */
  SNIPS_GRAIN_QUARTER = 1,
  /*
   * The resolved value as a granularity of a mount
   */
  SNIPS_GRAIN_MONTH = 2,
  /*
   * The resolved value as a granularity of a week
   */
  SNIPS_GRAIN_WEEK = 3,
  /*
   * The resolved value as a granularity of a day
   */
  SNIPS_GRAIN_DAY = 4,
  /*
   * The resolved value as a granularity of an hour
   */
  SNIPS_GRAIN_HOUR = 5,
  /*
   * The resolved value as a granularity of a minute
   */
  SNIPS_GRAIN_MINUTE = 6,
  /*
   * The resolved value as a granularity of a second
   */
  SNIPS_GRAIN_SECOND = 7,
} SNIPS_GRAIN;

typedef enum {
  SNIPS_INJECTION_KIND_ADD = 1,
} SNIPS_INJECTION_KIND;

/*
 * Enum describing the precision of a resolved value
 */
typedef enum {
  /*
   * The resolved value is approximate
   */
  SNIPS_PRECISION_APPROXIMATE = 0,
  /*
   * The resolved value is exact
   */
  SNIPS_PRECISION_EXACT = 1,
} SNIPS_PRECISION;

/*
 * Used as a return type of functions that can encounter errors
 */
typedef enum {
  /*
   * The function returned successfully
   */
  SNIPS_RESULT_OK = 0,
  /*
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

/*
 * Enum type describing how to cast the value of a CSlotValue
 */
typedef enum {
  /*
   * Custom type represented by a char *
   */
  SNIPS_SLOT_VALUE_TYPE_CUSTOM = 1,
  /*
   * Number type represented by a CNumberValue
   */
  SNIPS_SLOT_VALUE_TYPE_NUMBER = 2,
  /*
   * Ordinal type represented by a COrdinalValue
   */
  SNIPS_SLOT_VALUE_TYPE_ORDINAL = 3,
  /*
   * Instant type represented by a CInstantTimeValue
   */
  SNIPS_SLOT_VALUE_TYPE_INSTANTTIME = 4,
  /*
   * Interval type respresented by a CTimeIntervalValue
   */
  SNIPS_SLOT_VALUE_TYPE_TIMEINTERVAL = 5,
  /*
   * Amount of money type represented by a CAmountOfMoneyValue
   */
  SNIPS_SLOT_VALUE_TYPE_AMOUNTOFMONEY = 6,
  /*
   * Temperature type represented by a CTemperatureValue
   */
  SNIPS_SLOT_VALUE_TYPE_TEMPERATURE = 7,
  /*
   * Duration type reperesented by a CDurationValue
   */
  SNIPS_SLOT_VALUE_TYPE_DURATION = 8,
  /*
   * Percentage type represented by a CPercentageValue
   */
  SNIPS_SLOT_VALUE_TYPE_PERCENTAGE = 9,
  /*
   * Music artist type represented by a char *
   */
  SNIPS_SLOT_VALUE_TYPE_MUSICARTIST = 10,
  /*
   * Music album type represented by a char *
   */
  SNIPS_SLOT_VALUE_TYPE_MUSICALBUM = 11,
  /*
   * Music track type represented by a char *
   */
  SNIPS_SLOT_VALUE_TYPE_MUSICTRACK = 12,
} SNIPS_SLOT_VALUE_TYPE;

typedef struct {
  const void *facade;
} CAsrBackendFacade;

typedef struct {
  const char *site_id;
  const char *session_id;
} CSiteMessage;

typedef struct {
  const char *text;
  float likelihood;
  float seconds;
  const char *site_id;
  const char *session_id;
} CTextCapturedMessage;

typedef struct {
  const void *facade;
} CAsrFacade;

typedef struct {
  const void *facade;
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
} CAudioServerFacade;

typedef struct {
  const void *handler;
} CProtocolHandler;

typedef struct {
  const void *facade;
} CDialogueBackendFacade;

/*
 * Results of the intent classifier
 */
typedef struct {
  /*
   * Name of the intent detected
   */
  const char *intent_name;
  /*
   * Between 0 and 1
   */
  float probability;
} CIntentClassifierResult;

/*
 * A slot value
 */
typedef struct {
  /*
   * Points to either a *const char, a CNumberValue, a COrdinalValue,
   * a CInstantTimeValue, a CTimeIntervalValue, a CAmountOfMoneyValue,
   * a CTemperatureValue or a CDurationValue depending on value_type
   */
  const void *value;
  /*
   * The type of the value
   */
  SNIPS_SLOT_VALUE_TYPE value_type;
} CSlotValue;

/*
 * Struct describing a Slot
 */
typedef struct {
  /*
   * The resolved value of the slot
   */
  CSlotValue value;
  /*
   * The raw value as is appeared in the input text
   */
  const char *raw_value;
  /*
   * Name of the entity type of the slot
   */
  const char *entity;
  /*
   * Name of the slot
   */
  const char *slot_name;
  /*
   * Start index of raw value in input text
   */
  int32_t range_start;
  /*
   * End index of raw value in input text
   */
  int32_t range_end;
} CSlot;

/*
 * Wrapper around a slot list
 */
typedef struct {
  /*
   * Pointer to the first slot of the list
   */
  const CSlot *slots;
  /*
   * Number of slots in the list
   */
  int32_t size;
} CSlotList;

typedef struct {
  const char *session_id;
  /*
   * Nullable
   */
  const char *custom_data;
  const char *site_id;
  const char *input;
  const CIntentClassifierResult *intent;
  /*
   * Nullable
   */
  const CSlotList *slots;
} CIntentMessage;

typedef struct {
  SNIPS_SESSION_TERMINATION_TYPE termination_type;
  /*
   * Nullable,
   */
  const char *data;
} CSessionTermination;

typedef struct {
  const char *session_id;
  /*
   * Nullable
   */
  const char *custom_data;
  CSessionTermination termination;
  const char *site_id;
} CSessionEndedMessage;

typedef struct {
  const char *session_id;
  /*
   * Nullable
   */
  const char *custom_data;
  const char *site_id;
} CSessionQueuedMessage;

typedef struct {
  const char *session_id;
  /*
   * Nullable
   */
  const char *custom_data;
  const char *site_id;
  /*
   * Nullable
   */
  const char *reactivated_from_session_id;
} CSessionStartedMessage;

/*
 * An array of strings
 */
typedef struct {
  /*
   * Pointer to the first element of the array
   */
  const char *const *data;
  /*
   * Number of elements in the array
   */
  int size;
} CStringArray;

typedef struct {
  const char *session_id;
  const char *text;
  /*
   * Nullable
   */
  const CStringArray *intent_filter;
  /*
   * Nullable
   */
  const char *custom_data;
  unsigned char send_intent_not_recognized;
} CContinueSessionMessage;

typedef struct {
  const char *session_id;
  /*
   * Nullable
   */
  const char *text;
} CEndSessionMessage;

typedef struct {
  SNIPS_SESSION_INIT_TYPE init_type;
  /*
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
} CDialogueFacade;

typedef struct {
  /*
   * Nullable
   */
  const char *session_id;
  const char *error;
  /*
   * Nullable
   */
  const char *context;
} CErrorMessage;

typedef struct {
  const void *facade;
} CHotwordBackendFacade;

typedef struct {
  const char *site_id;
  const char *model_id;
} CHotwordDetectedMessage;

typedef struct {
  const void *facade;
} CHotwordFacade;

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
} CInjectionRequestMessage;

typedef struct {
  const char *site_id;
  const char *session_id;
  /*
   * Nullable
   */
  const char *input;
  /*
   * Nullable
   */
  const char *custom_data;
} CIntentNotRecognizedMessage;

typedef struct {
  const void *facade;
} CNluBackendFacade;

typedef struct {
  const void *facade;
} CNluFacade;

typedef struct {
  /*
   * Nullable
   */
  const char *id;
  const char *input;
  const CIntentClassifierResult *intent;
  /*
   * Nullable
   */
  const CSlotList *slots;
  /*
   * Nullable
   */
  const char *session_id;
} CNluIntentMessage;

typedef struct {
  const char *input;
  /*
   * Nullable
   */
  const char *id;
  /*
   * Nullable
   */
  const char *session_id;
} CNluIntentNotRecognizedMessage;

typedef struct {
  const char *input;
  /*
   * Nullable
   */
  const CStringArray *intent_filter;
  /*
   * Nullable
   */
  const char *id;
  /*
   * Nullable
   */
  const char *session_id;
} CNluQueryMessage;

typedef struct {
  /*
   * Nullable
   */
  const char *id;
  const char *input;
  const char *intent_name;
  /*
   * Nullable
   */
  const CSlot *slot;
  /*
   * Nullable
   */
  const char *session_id;
} CNluSlotMessage;

typedef struct {
  const char *input;
  const char *intent_name;
  const char *slot_name;
  /*
   * Nullable
   */
  const char *id;
  /*
   * Nullable
   */
  const char *session_id;
} CNluSlotQueryMessage;

typedef struct {
  /*
   * Nullable
   */
  const char *id;
  /*
   * Nullable
   */
  const char *session_id;
} CSayFinishedMessage;

typedef struct {
  const char *text;
  /*
   * Nullable
   */
  const char *lang;
  /*
   * Nullable
   */
  const char *id;
  const char *site_id;
  /*
   * Nullable
   */
  const char *session_id;
} CSayMessage;

typedef struct {
  const void *facade;
} CSoundFeedbackBackendFacade;

typedef struct {
  const void *facade;
} CSoundFeedbackFacade;

typedef struct {
  const void *facade;
} CTtsBackendFacade;

typedef struct {
  const void *facade;
} CTtsFacade;

typedef struct {
  uint64_t major;
  uint64_t minor;
  uint64_t patch;
} CVersionMessage;

/*
 * Representation of a number value
 */
typedef double CNumberValue;

/*
 * Representation of an ordinal value
 */
typedef int64_t COrdinalValue;

/*
 * Representation of a percentage value
 */
typedef double CPercentageValue;

/*
 * Representation of an instant value
 */
typedef struct {
  /*
   * String representation of the instant
   */
  const char *value;
  /*
   * The grain of the resolved instant
   */
  SNIPS_GRAIN grain;
  /*
   * The precision of the resolved instant
   */
  SNIPS_PRECISION precision;
} CInstantTimeValue;

/*
 * Representation of an interval value
 */
typedef struct {
  /*
   * String representation of the beginning of the interval
   */
  const char *from;
  /*
   * String representation of the end of the interval
   */
  const char *to;
} CTimeIntervalValue;

/*
 * Representation of an amount of money value
 */
typedef struct {
  /*
   * The currency
   */
  const char *unit;
  /*
   * The amount of money
   */
  float value;
  /*
   * The precision of the resolved value
   */
  SNIPS_PRECISION precision;
} CAmountOfMoneyValue;

/*
 * Representation of a temperature value
 */
typedef struct {
  /*
   * The unit used
   */
  const char *unit;
  /*
   * The temperature resolved
   */
  float value;
} CTemperatureValue;

/*
 * Representation of a duration value
 */
typedef struct {
  /*
   * Number of years in the duration
   */
  int64_t years;
  /*
   * Number of quarters in the duration
   */
  int64_t quarters;
  /*
   * Number of mounts in the duration
   */
  int64_t months;
  /*
   * Number of weeks in the duration
   */
  int64_t weeks;
  /*
   * Number of days in the duration
   */
  int64_t days;
  /*
   * Number of hours in the duration
   */
  int64_t hours;
  /*
   * Number of minutes in the duration
   */
  int64_t minutes;
  /*
   * Number of seconds in the duration
   */
  int64_t seconds;
  /*
   * Precision of the resolved value
   */
  SNIPS_PRECISION precision;
} CDurationValue;

SNIPS_RESULT hermes_asr_backend_publish_start_listening(const CAsrBackendFacade *facade,
                                                        void (*handler)(const CSiteMessage*));

SNIPS_RESULT hermes_asr_backend_publish_stop_listening(const CAsrBackendFacade *facade,
                                                       void (*handler)(const CSiteMessage*));

SNIPS_RESULT hermes_asr_backend_subscribe_partial_text_captured(const CAsrBackendFacade *facade,
                                                                const CTextCapturedMessage *message);

SNIPS_RESULT hermes_asr_backend_subscribe_text_captured(const CAsrBackendFacade *facade,
                                                        const CTextCapturedMessage *message);

SNIPS_RESULT hermes_asr_publish_start_listening(const CAsrFacade *facade,
                                                const CSiteMessage *message);

SNIPS_RESULT hermes_asr_publish_stop_listening(const CAsrFacade *facade,
                                               const CSiteMessage *message);

SNIPS_RESULT hermes_asr_subscribe_partial_text_captured(const CAsrFacade *facade,
                                                        void (*handler)(const CTextCapturedMessage*));

SNIPS_RESULT hermes_asr_subscribe_text_captured(const CAsrFacade *facade,
                                                void (*handler)(const CTextCapturedMessage*));

SNIPS_RESULT hermes_audio_server_backend_publish_audio_frame(const CAudioServerBackendFacade *facade,
                                                             const CAudioFrameMessage *message);

SNIPS_RESULT hermes_audio_server_backend_publish_play_finished(const CAudioServerBackendFacade *facade,
                                                               const CPlayFinishedMessage *message);

SNIPS_RESULT hermes_audio_server_backend_subscribe_all_play_bytes(const CAudioServerBackendFacade *facade,
                                                                  void (*handler)(const CPlayBytesMessage*));

SNIPS_RESULT hermes_audio_server_backend_subscribe_play_bytes(const CAudioServerBackendFacade *facade,
                                                              const char *site_id,
                                                              void (*handler)(const CPlayBytesMessage*));

SNIPS_RESULT hermes_audio_server_publish_play_bytes(const CAudioServerFacade *facade,
                                                    const CPlayBytesMessage *message);

SNIPS_RESULT hermes_audio_server_subscribe_all_play_finished(const CAudioServerFacade *facade,
                                                             void (*handler)(const CPlayFinishedMessage*));

SNIPS_RESULT hermes_audio_server_subscribe_audio_frame(const CAudioServerFacade *facade,
                                                       const char *site_id,
                                                       void (*handler)(const CAudioFrameMessage*));

SNIPS_RESULT hermes_audio_server_subscribe_play_finished(const CAudioServerFacade *facade,
                                                         const char *site_id,
                                                         void (*handler)(const CPlayFinishedMessage*));

SNIPS_RESULT hermes_destroy_mqtt_protocol_handler(CProtocolHandler *handler);

SNIPS_RESULT hermes_dialogue_backend_publish_intent(const CDialogueBackendFacade *facade,
                                                    const CIntentMessage *message);

SNIPS_RESULT hermes_dialogue_backend_publish_session_ended(const CDialogueBackendFacade *facade,
                                                           const CSessionEndedMessage *message);

SNIPS_RESULT hermes_dialogue_backend_publish_session_queued(const CDialogueBackendFacade *facade,
                                                            const CSessionQueuedMessage *message);

SNIPS_RESULT hermes_dialogue_backend_publish_session_started(const CDialogueBackendFacade *facade,
                                                             const CSessionStartedMessage *message);

SNIPS_RESULT hermes_dialogue_backend_subscribe_continue_session(const CDialogueBackendFacade *facade,
                                                                void (*handler)(const CContinueSessionMessage*));

SNIPS_RESULT hermes_dialogue_backend_subscribe_end_session(const CDialogueBackendFacade *facade,
                                                           void (*handler)(const CEndSessionMessage*));

SNIPS_RESULT hermes_dialogue_backend_subscribe_start_session(const CDialogueBackendFacade *facade,
                                                             void (*handler)(const CStartSessionMessage*));

SNIPS_RESULT hermes_dialogue_publish_continue_session(const CDialogueFacade *facade,
                                                      const CContinueSessionMessage *message);

SNIPS_RESULT hermes_dialogue_publish_end_session(const CDialogueFacade *facade,
                                                 const CEndSessionMessage *message);

SNIPS_RESULT hermes_dialogue_publish_start_session(const CDialogueFacade *facade,
                                                   const CStartSessionMessage *message);

SNIPS_RESULT hermes_dialogue_subscribe_intent(const CDialogueFacade *facade,
                                              const char *intent_name,
                                              void (*handler)(const CIntentMessage*));

SNIPS_RESULT hermes_dialogue_subscribe_intents(const CDialogueFacade *facade,
                                               void (*handler)(const CIntentMessage*));

SNIPS_RESULT hermes_dialogue_subscribe_session_ended(const CDialogueFacade *facade,
                                                     void (*handler)(const CSessionEndedMessage*));

SNIPS_RESULT hermes_dialogue_subscribe_session_queued(const CDialogueFacade *facade,
                                                      void (*handler)(const CSessionQueuedMessage*));

SNIPS_RESULT hermes_dialogue_subscribe_session_started(const CDialogueFacade *facade,
                                                       void (*handler)(const CSessionStartedMessage*));

SNIPS_RESULT hermes_drop_asr_backend_facade(const CAsrBackendFacade *cstruct);

SNIPS_RESULT hermes_drop_asr_facade(const CAsrFacade *cstruct);

SNIPS_RESULT hermes_drop_audio_frame_message(const CAudioFrameMessage *cstruct);

SNIPS_RESULT hermes_drop_audio_server_backend_facade(const CAudioServerBackendFacade *cstruct);

SNIPS_RESULT hermes_drop_audio_server_facade(const CAudioServerFacade *cstruct);

SNIPS_RESULT hermes_drop_continue_session_message(const CContinueSessionMessage *cstruct);

SNIPS_RESULT hermes_drop_dialogue_backend_facade(const CDialogueBackendFacade *cstruct);

SNIPS_RESULT hermes_drop_dialogue_facade(const CDialogueFacade *cstruct);

SNIPS_RESULT hermes_drop_end_session_message(const CEndSessionMessage *cstruct);

SNIPS_RESULT hermes_drop_error_message(const CErrorMessage *cstruct);

SNIPS_RESULT hermes_drop_hotword_backend_facade(const CHotwordBackendFacade *cstruct);

SNIPS_RESULT hermes_drop_hotword_detected_message(const CHotwordDetectedMessage *cstruct);

SNIPS_RESULT hermes_drop_hotword_facade(const CHotwordFacade *cstruct);

SNIPS_RESULT hermes_drop_injection_request_message(const CInjectionRequestMessage *cstruct);

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

/*
 * Used to retrieve the last error that happened in this thread. A function encountered an
 * error if its return type is of type SNIPS_RESULT and it returned SNIPS_RESULT_KO
 */
SNIPS_RESULT hermes_get_last_error(const char **error);

SNIPS_RESULT hermes_hotword_backend_publish_detected(const CHotwordBackendFacade *facade,
                                                     const char *hotword_id,
                                                     const CHotwordDetectedMessage *message);

SNIPS_RESULT hermes_hotword_subscribe_all_detected(const CHotwordFacade *facade,
                                                   void (*handler)(const CHotwordDetectedMessage*));

SNIPS_RESULT hermes_hotword_subscribe_detected(const CHotwordFacade *facade,
                                               const char *hotword_id,
                                               void (*handler)(const CHotwordDetectedMessage*));

SNIPS_RESULT hermes_nlu_backend_publish_intent_not_recognized(const CNluBackendFacade *facade,
                                                              const CNluIntentNotRecognizedMessage *message);

SNIPS_RESULT hermes_nlu_backend_publish_intent_parsed(const CNluBackendFacade *facade,
                                                      const CNluIntentMessage *message);

SNIPS_RESULT hermes_nlu_backend_publish_slot_parsed(const CNluBackendFacade *facade,
                                                    const CNluSlotMessage *message);

SNIPS_RESULT hermes_nlu_backend_subscribe_partial_query(const CNluBackendFacade *facade,
                                                        void (*handler)(const CNluSlotQueryMessage*));

SNIPS_RESULT hermes_nlu_backend_subscribe_query(const CNluBackendFacade *facade,
                                                void (*handler)(const CNluQueryMessage*));

SNIPS_RESULT hermes_nlu_publish_partial_query(const CNluFacade *facade,
                                              const CNluSlotQueryMessage *message);

SNIPS_RESULT hermes_nlu_publish_query(const CNluFacade *facade, const CNluQueryMessage *message);

SNIPS_RESULT hermes_nlu_subscribe_intent_not_recognized(const CNluFacade *facade,
                                                        void (*handler)(const CNluIntentNotRecognizedMessage*));

SNIPS_RESULT hermes_nlu_subscribe_intent_parsed(const CNluFacade *facade,
                                                void (*handler)(const CNluIntentMessage*));

SNIPS_RESULT hermes_nlu_subscribe_slot_parsed(const CNluFacade *facade,
                                              void (*handler)(const CNluSlotMessage*));

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

SNIPS_RESULT hermes_protocol_handler_new_mqtt(const CProtocolHandler **handler,
                                              const char *broker_address);

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

SNIPS_RESULT hermes_tts_backend_publish_say_finished(const CTtsBackendFacade *facade,
                                                     const CSayFinishedMessage *message);

SNIPS_RESULT hermes_tts_backend_subscribe_say(const CTtsBackendFacade *facade,
                                              void (*handler)(const CSayMessage*));

SNIPS_RESULT hermes_tts_publish_say(const CTtsFacade *facade, const CSayMessage *message);

SNIPS_RESULT hermes_tts_subscribe_say_finished(const CTtsFacade *facade,
                                               void (*handler)(const CSayFinishedMessage*));

#endif /* LIB_HERMES_H_ */
