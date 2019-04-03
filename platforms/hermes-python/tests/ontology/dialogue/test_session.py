from __future__ import unicode_literals
from hermes_python.ontology.dialogue import DialogueConfiguration


class TestDialogueConfiguration:
    def test_dialogue_configuration_disabling_intent(self):
        conf = DialogueConfiguration()
        conf.disable_intent("intent_1")

        dialogue_configure_messages = conf.build()
        assert len(dialogue_configure_messages) == 1

        dialogue_configure_message = dialogue_configure_messages[0]

        assert dialogue_configure_message.site_id is None
        assert len(dialogue_configure_message.intents) == 1
        assert dialogue_configure_message.intents[0].intent_id == "intent_1"
        assert not dialogue_configure_message.intents[0].enable

    def test_dialogue_configuration_disabling_intents(self):
        conf = DialogueConfiguration()
        conf.disable_intents(["intent_1", "intent_2"])

        dialogue_configure_messages = conf.build()
        assert len(dialogue_configure_messages) == 1

        dialogue_configure_message = dialogue_configure_messages[0]

        assert dialogue_configure_message.site_id is None
        assert len(dialogue_configure_message.intents) == 2


    def test_dialogue_configuration_global_site_id(self):
        conf = DialogueConfiguration()
        conf\
            .for_site_id("kitchen")\
            .disable_intent("intent_1")

        dialogue_configure_message = conf.build()[0]
        assert dialogue_configure_message.site_id == "kitchen"
        assert len(dialogue_configure_message.intents) == 1

    def test_dialogue_configuration_local_site_id(self):
        conf = DialogueConfiguration(site_id="kitchen")
        conf\
            .disable_intent("intent_1", site_id="bedroom")

        dialogue_configure_message = conf.build()[0]
        assert dialogue_configure_message.site_id == "bedroom"
        assert len(dialogue_configure_message.intents) == 1

    def test_dialogue_configuration_multiple_site_ids(self):
        conf = DialogueConfiguration()
        conf\
            .disable_intent("intent_1", site_id="bedroom") \
            .disable_intent("intent_1", site_id="kitchen")

        dialogue_configure_messages = conf.build()
        assert dialogue_configure_messages[0].site_id == "bedroom"
        assert dialogue_configure_messages[1].site_id == "kitchen"

    def test_dialogue_configuration_multiple_site_ids2(self):
        conf = DialogueConfiguration()
        conf \
            .disable_intent("intent_1") \
            .disable_intent("intent_1", site_id="kitchen")

        dialogue_configure_messages = conf.build()
        assert dialogue_configure_messages[0].site_id is None
        assert dialogue_configure_messages[1].site_id == "kitchen"

    def test_dialogue_configuration_multiple_site_ids3(self):
        conf = DialogueConfiguration(site_id="bathroom")
        conf \
            .disable_intent("intent_2") \
            .disable_intent("intent_2", site_id="kitchen")

        dialogue_configure_messages = conf.build()
        assert dialogue_configure_messages[0].site_id == "bathroom"
        assert dialogue_configure_messages[1].site_id == "kitchen"


    def test_dialogue_configuration_toggling_intent(self):
        conf = DialogueConfiguration()
        conf\
            .disable_intent("intent_2") \
            .enable_intent("intent_2")  \
            .disable_intent("intent_2") \
            .enable_intent("intent_2")

        dialogue_configure_messages = conf.build()
        assert dialogue_configure_messages[0].intents[0].enable

