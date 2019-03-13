from hermes_python.hermes import Hermes
from hermes_python.ontology.dialogue import IntentMessage
import json

def serialize_missing_slots(state):
	#type:(dict) -> str
	return json.dumps(state)

def deserialize_missing_slots(state):
	# type:(str) -> dict
	return json.loads(state)

def cb(hermes, intent_message):
	#type:(Hermes, IntentMessage) -> Any
	print("intentdectect")
	filled_slots = deserialize_missing_slots(intent_message.custom_data) if intent_message.custom_data else {'size': False, 'topping' : False}

	if not filled_slots['size'] ntent_message.slots.size.first() and not filled_slots['size']:
		filled_slots['size'] = True
	else:
		custom_data = serialize_missing_slots(filled_slots)
		hermes.publish_continue_session(intent_message.session_id, "What size do you want ? ", ["anthoDaIntentGod:order"], custom_data, True, "size")

	if intent_message.slots.topping.first() and not filled_slots['topping']:
		filled_slots['topping'] = True
	else:
		custom_data = serialize_missing_slots(filled_slots)
		hermes.publish_continue_session(intent_message.session_id, "What topping would you like ?", ["anthoDaIntentGod:order"], custom_data, True, "topping")

	all_slots_filled = all([filled for k,filled in filled_slots.items()])
	if all_slots_filled:
		hermes.publish_end_session(intent_message.session_id, "Lovely, you'll be receiving your pizza shortly!")



def cbnr(hermes, intentNotRecognizedMessage):
	hermes.publish_end_session(intentNotRecognizedMessage.session_id, "i didn't understand what you said")


def cbclear(hermes, intent_message):
	hermes.publish_end_session(intent_message.session_id, "Ok")


with Hermes("localhost:1883") as h:
	h                                                   \
		.subscribe_intent("anthoDaIntentGod:order", cb)   \
		.subscribe_intent("anthoDaIntentGod:clear", cbclear)   \
		.subscribe_intent_not_recognized(cbnr)          \
		.start()
