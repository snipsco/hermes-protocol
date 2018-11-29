#include <stdio.h>
#include "libsnips_hermes_full.h"

void callback(const CSayMessage *ptr, void* user_data) {
	printf("in the c callback\n");
	printf("say messages points to %p\n", ptr);
	printf("%s\n", ptr->text);
	printf("user data is %s\n", user_data);
}

int main() {
	hermes_enable_debug_logs();
	const CProtocolHandler *truc;
	printf("new\n");
	hermes_protocol_handler_new_mqtt(&truc, "localhost:1883", "my user data");
	printf("new done\n");
	const CTtsBackendFacade *tts;
	hermes_protocol_handler_tts_backend_facade(truc, &tts);
	printf("pointer in C : %p\n", callback);
	hermes_tts_backend_subscribe_say(tts, callback);

	while (true)  {}

}
