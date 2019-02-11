#ifndef LIB_HERMES_JSON_H_
#define LIB_HERMES_JSON_H_

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

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

typedef struct {
  const void *handler;
  void *user_data;
} CProtocolHandler;

typedef struct {
  const void *facade;
  void *user_data;
} CDialogueFacade;

typedef struct {
  const void *facade;
  void *user_data;
} CInjectionFacade;

typedef struct {
  const void *facade;
  void *user_data;
} CSoundFeedbackFacade;

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

SNIPS_RESULT hermes_destroy_mqtt_protocol_handler(CProtocolHandler *handler);

SNIPS_RESULT hermes_dialogue_publish_continue_session_json(const CDialogueFacade *facade,
                                                           const char *message);

SNIPS_RESULT hermes_dialogue_publish_end_session_json(const CDialogueFacade *facade,
                                                      const char *message);

SNIPS_RESULT hermes_dialogue_publish_start_session_json(const CDialogueFacade *facade,
                                                        const char *message);

SNIPS_RESULT hermes_dialogue_subscribe_intent_json(const CDialogueFacade *facade,
                                                   const char *intent_name,
                                                   void (*handler)(const char*, void*));

SNIPS_RESULT hermes_dialogue_subscribe_intent_not_recognized_json(const CDialogueFacade *facade,
                                                                  void (*handler)(const char*, void*));

SNIPS_RESULT hermes_dialogue_subscribe_intents_json(const CDialogueFacade *facade,
                                                    void (*handler)(const char*, void*));

SNIPS_RESULT hermes_dialogue_subscribe_session_ended_json(const CDialogueFacade *facade,
                                                          void (*handler)(const char*, void*));

SNIPS_RESULT hermes_dialogue_subscribe_session_queued_json(const CDialogueFacade *facade,
                                                           void (*handler)(const char*, void*));

SNIPS_RESULT hermes_dialogue_subscribe_session_started_json(const CDialogueFacade *facade,
                                                            void (*handler)(const char*, void*));

SNIPS_RESULT hermes_drop_dialogue_facade(const CDialogueFacade *cstruct);

SNIPS_RESULT hermes_drop_injection_facade(const CInjectionFacade *cstruct);

SNIPS_RESULT hermes_drop_sound_feedback_facade(const CSoundFeedbackFacade *cstruct);

SNIPS_RESULT hermes_enable_debug_logs(void);

/**
 * Used to retrieve the last error that happened in this thread. A function encountered an
 * error if its return type is of type SNIPS_RESULT and it returned SNIPS_RESULT_KO
 */
SNIPS_RESULT hermes_get_last_error(const char **error);

SNIPS_RESULT hermes_injection_publish_injection_request_json(const CInjectionFacade *facade,
                                                             const char *message);

SNIPS_RESULT hermes_injection_publish_injection_status_request_json(const CInjectionFacade *facade);

SNIPS_RESULT hermes_injection_subscribe_injection_status_json(const CInjectionFacade *facade,
                                                              void (*handler)(const char*, void*));

SNIPS_RESULT hermes_protocol_handler_dialogue_facade(const CProtocolHandler *handler,
                                                     const CDialogueFacade **facade);

SNIPS_RESULT hermes_protocol_handler_injection_facade(const CProtocolHandler *handler,
                                                      const CInjectionFacade **facade);

SNIPS_RESULT hermes_protocol_handler_new_mqtt(const CProtocolHandler **handler,
                                              const char *broker_address,
                                              void *user_data);

SNIPS_RESULT hermes_protocol_handler_new_mqtt_with_options(const CProtocolHandler **handler,
                                                           const CMqttOptions *mqtt_options,
                                                           void *user_data);

SNIPS_RESULT hermes_protocol_handler_sound_feedback_facade(const CProtocolHandler *handler,
                                                           const CSoundFeedbackFacade **facade);

SNIPS_RESULT hermes_sound_feedback_publish_toggle_off_json(const CSoundFeedbackFacade *facade,
                                                           const char *message);

SNIPS_RESULT hermes_sound_feedback_publish_toggle_on_json(const CSoundFeedbackFacade *facade,
                                                          const char *message);

#endif /* LIB_HERMES_JSON_H_ */
