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
   * The resolved value has a granularity of a year
   */
  SNIPS_GRAIN_YEAR = 0,
  /*
   * The resolved value has a granularity of a quarter
   */
  SNIPS_GRAIN_QUARTER = 1,
  /*
   * The resolved value has a granularity of a mount
   */
  SNIPS_GRAIN_MONTH = 2,
  /*
   * The resolved value has a granularity of a week
   */
  SNIPS_GRAIN_WEEK = 3,
  /*
   * The resolved value has a granularity of a day
   */
  SNIPS_GRAIN_DAY = 4,
  /*
   * The resolved value has a granularity of an hour
   */
  SNIPS_GRAIN_HOUR = 5,
  /*
   * The resolved value has a granularity of a minute
   */
  SNIPS_GRAIN_MINUTE = 6,
  /*
   * The resolved value has a granularity of a second
   */
  SNIPS_GRAIN_SECOND = 7,
} SNIPS_GRAIN;

typedef enum {
  SNIPS_INJECTION_KIND_ADD = 1,
  SNIPS_INJECTION_KIND_ADD_FROM_VANILLA = 2,
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
   * Interval type represented by a CTimeIntervalValue
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
   * Duration type represented by a CDurationValue
   */
  SNIPS_SLOT_VALUE_TYPE_DURATION = 8,
  /*
   * Percentage type represented by a CPercentageValue
   */
  SNIPS_SLOT_VALUE_TYPE_PERCENTAGE = 9,
  /*
   * Music Album type represented by a char *
   */
  SNIPS_SLOT_VALUE_TYPE_MUSICALBUM = 10,
  /*
   * Music Artist type represented by a char *
   */
  SNIPS_SLOT_VALUE_TYPE_MUSICARTIST = 11,
  /*
   * Music Track type represented by a char *
   */
  SNIPS_SLOT_VALUE_TYPE_MUSICTRACK = 12,
} SNIPS_SLOT_VALUE_TYPE;

typedef struct {
  const void *handler;
  void *user_data;
} CProtocolHandler;

typedef struct {
  const void *facade;
  void *user_data;
} CDialogueFacade;

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
   * The raw value as it appears in the input text
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

typedef struct {
  float confidence;
  const CSlot *nlu_slot;
} CNluSlot;

typedef struct {
  const CNluSlot *const *entries;
  int count;
} CNluSlotArray;

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
  const CAsrTokenArray *const *entries;
  int count;
} CAsrTokenDoubleArray;

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
  const CNluSlotArray *slots;
  /*
   * Nullable, the first array level represents the asr invocation, the second one the tokens
   */
  const CAsrTokenDoubleArray *asr_tokens;
} CIntentMessage;

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
  void *user_data;
} CInjectionFacade;

typedef struct {
  const char *last_injection_date;
} CInjectionStatusMessage;

typedef struct {
  const void *facade;
  void *user_data;
} CSoundFeedbackFacade;

typedef struct {
  uint64_t major;
  uint64_t minor;
  uint64_t patch;
} CVersionMessage;

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
  /*
   * Nullable
   */
  const char *cross_language;
  /*
   * Nullable
   */
  const char *id;
} CInjectionRequestMessage;

typedef struct {
  const char *site_id;
  /*
   * Nullable
   */
  const char *session_id;
} CSiteMessage;

typedef struct {
  /*
   * Nullable
   */
  const char *text;
  /*
   * Nullable
   */
  const CStringArray *intent_filter;
  unsigned char can_be_enqueued;
  unsigned char send_intent_not_recognized;
} CActionSessionInit;

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
   * Number of months in the duration
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

SNIPS_RESULT hermes_destroy_mqtt_protocol_handler(CProtocolHandler *handler);

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

SNIPS_RESULT hermes_drop_dialogue_facade(const CDialogueFacade *cstruct);

SNIPS_RESULT hermes_drop_error_message(const CErrorMessage *cstruct);

SNIPS_RESULT hermes_drop_injection_facade(const CInjectionFacade *cstruct);

SNIPS_RESULT hermes_drop_injection_status_message(const CInjectionStatusMessage *cstruct);

SNIPS_RESULT hermes_drop_intent_message(const CIntentMessage *cstruct);

SNIPS_RESULT hermes_drop_intent_not_recognized_message(const CIntentNotRecognizedMessage *cstruct);

SNIPS_RESULT hermes_drop_session_ended_message(const CSessionEndedMessage *cstruct);

SNIPS_RESULT hermes_drop_session_queued_message(const CSessionQueuedMessage *cstruct);

SNIPS_RESULT hermes_drop_session_started_message(const CSessionStartedMessage *cstruct);

SNIPS_RESULT hermes_drop_sound_feedback_facade(const CSoundFeedbackFacade *cstruct);

SNIPS_RESULT hermes_drop_version_message(const CVersionMessage *cstruct);

SNIPS_RESULT hermes_enable_debug_logs(void);

/*
 * Used to retrieve the last error that happened in this thread. A function encountered an
 * error if its return type is of type SNIPS_RESULT and it returned SNIPS_RESULT_KO
 */
SNIPS_RESULT hermes_get_last_error(const char **error);

SNIPS_RESULT hermes_injection_publish_injection_request(const CInjectionFacade *facade,
                                                        const CInjectionRequestMessage *message);

SNIPS_RESULT hermes_injection_publish_injection_status_request(const CInjectionFacade *facade);

SNIPS_RESULT hermes_injection_subscribe_injection_status(const CInjectionFacade *facade,
                                                         void (*handler)(const CInjectionStatusMessage*, void*));

SNIPS_RESULT hermes_protocol_handler_dialogue_facade(const CProtocolHandler *handler,
                                                     const CDialogueFacade **facade);

SNIPS_RESULT hermes_protocol_handler_injection_facade(const CProtocolHandler *handler,
                                                      const CInjectionFacade **facade);

SNIPS_RESULT hermes_protocol_handler_new_mqtt(const CProtocolHandler **handler,
                                              const char *broker_address,
                                              void *user_data);

SNIPS_RESULT hermes_protocol_handler_sound_feedback_facade(const CProtocolHandler *handler,
                                                           const CSoundFeedbackFacade **facade);

SNIPS_RESULT hermes_sound_feedback_publish_toggle_off(const CSoundFeedbackFacade *facade,
                                                      const CSiteMessage *message);

SNIPS_RESULT hermes_sound_feedback_publish_toggle_on(const CSoundFeedbackFacade *facade,
                                                     const CSiteMessage *message);

#endif /* LIB_HERMES_H_ */
