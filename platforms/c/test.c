#include <stdio.h>
#include "libsnips_hermes.h"

void callback(const CSayMessage *ptr) {
	printf("in the c callback\n");
	printf("say messages points to %p\n", ptr);
	printf("%s\n", ptr->text);
}

int main() {
	hermes_enable_debug_logs();
	const CProtocolHandler *truc;
	printf("new\n");
	hermes_protocol_handler_new_mqtt(&truc, "localhost:1883");
	printf("new done\n");
	const CTtsBackendFacade *tts;
	hermes_protocol_handler_tts_backend_facade(truc, &tts);
	printf("pointer in C : %p\n", callback);
	hermes_tts_backend_subscribe_say(tts, callback);

	while (true)  {}

}
