#include "libsnips_hermes_full.h"
#include <stdio.h>

void callback(const CSayMessage *ptr, void *user_data) {
  printf("in the c callback\n");
  printf("say messages points to %p\n", ptr);
  printf("%s\n", ptr->text);
  printf("user data is %s\n", user_data);
}

const char *last_error() {
  const char *nullerror = NULL;
  const char **error = &nullerror;

  hermes_get_last_error(error);

  return *error; // we leak the string here but well crash just after so that's
                 // not that big a problem
}

#define check(hermes_expr)                                                     \
  if (hermes_expr != SNIPS_RESULT_OK) {                                        \
    printf("Assertion failed at %s:%i in function %s:\nFailed to execute "     \
           "%s\nError was %s\n",                                               \
           __FILE__, __LINE__, __func__, #hermes_expr, last_error());          \
    exit(1);                                                                   \
  }

int main() {
  check(hermes_enable_debug_logs());
  const CProtocolHandler *truc;
  printf("new\n");

  const char *value_array[] = {"/path/to/cafile"};
  CStringArray ca_files = {.data = value_array,
                           .size =
                               sizeof(value_array) / sizeof(value_array[0])};

  const CMqttOptions options = {.broker_address = "localhost:1883",
                                .username = NULL,
                                .password = NULL,
                                .tls_hostname = NULL,
                                .tls_ca_file = &ca_files,
                                .tls_ca_path = NULL,
                                .tls_client_key = NULL,
                                .tls_client_cert = NULL,
                                .tls_disable_root_store = 0};

  check(hermes_protocol_handler_new_mqtt_with_options(&truc, &options,
                                                      "my user data"));

  printf("new done\n");
  const CTtsBackendFacade *tts;
  check(hermes_protocol_handler_tts_backend_facade(truc, &tts));
  printf("pointer in C : %p\n", callback);
  check(hermes_tts_backend_subscribe_say(tts, callback));

  while (true) {
  }
}
