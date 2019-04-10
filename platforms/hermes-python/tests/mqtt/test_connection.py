# coding: utf-8
import pytest
import subprocess
import time
import mock
from paho.mqtt.publish import single

from hermes_python.hermes import Hermes


@pytest.fixture(scope="module")
def mqtt_server():
    print("Starting MQTT Server")
    mqtt_server = subprocess.Popen("mosquitto")
    time.sleep(1)  # Let's wait a bit before it's started
    yield mqtt_server
    print("Tearing down MQTT Server")
    mqtt_server.kill()


class TestPublishingMessages(object):
    def test_publish_continue_session(self, mqtt_server):
        with Hermes("localhost:1883") as h:
            h.publish_continue_session("session_id", "text", [], None)
            h.publish_continue_session("session_id", "text", ["intent_name"], None)
            h.publish_continue_session("session_id", "text", ["intent_name"], "custom_data")
            h.publish_continue_session("session_id", "text", ["intent_name"], "custom_data", True)
            h.publish_continue_session("session_id", "text", ["intent_name"], "custom_data", True, "slot")

    def test_publish_end_session(self, mqtt_server):
        with Hermes("localhost:1883") as h:
            h.publish_end_session("session_id", "goodbye")

    def test_publish_start_session_notification(self, mqtt_server):
        with Hermes("localhost:1883") as h:
            h.publish_start_session_notification("site_id", "initialization", None)
            h.publish_start_session_notification("site_id", "initialization", "custom_data")
            h.publish_start_session_notification("site_id", "initialization", "custom_data", "text")

    def test_publish_start_session_action(self, mqtt_server):
        with Hermes("localhost:1883") as h:
            h.publish_start_session_action(None, None, [], False, False, None)
            h.publish_start_session_action("site_id", None, [], False, False, None)
            h.publish_start_session_action("site_id", "text", [], False, False, None)
            h.publish_start_session_action("site_id", "text", [], False, False, "custom_data")

    def test_configure_dialogue(self, mqtt_server):
        from hermes_python.ontology.dialogue import DialogueConfiguration
        conf = DialogueConfiguration()

        with Hermes("localhost:1883") as h:
            h.configure_dialogue(conf)

    def test_publish_sound_feedback_toggle(self, mqtt_server):
        from hermes_python.ontology.feedback import SiteMessage
        site_message = SiteMessage("kitchen")

        with Hermes("localhost:1883") as h:
            h.publish_enable_sound_feedback(site_message)
            h.publish_disable_sound_feedback(site_message)


def test_subscription_to_intent_message(mqtt_server):
    subscribe_intent_callback = mock.Mock()

    with Hermes("localhost:1883") as h:
        h.subscribe_intent("bundle:searchWeatherForecast", subscribe_intent_callback)
        single("hermes/intent/bundle:searchWeatherForecast",
               payload='{"sessionId":"08f56b9e-b4e4-4688-8a3e-1653c48180ee","customData":null,"siteId":"default","input":"quel temps fait il à paris","asrTokens":[[{"value":"quel","confidence":1.0,"rangeStart":0,"rangeEnd":4,"time":{"start":0.0,"end":0.29999998}},{"value":"temps","confidence":1.0,"rangeStart":5,"rangeEnd":10,"time":{"start":0.29999998,"end":0.51}},{"value":"fait","confidence":1.0,"rangeStart":11,"rangeEnd":15,"time":{"start":0.51,"end":0.57}},{"value":"il","confidence":1.0,"rangeStart":16,"rangeEnd":18,"time":{"start":0.57,"end":0.75}},{"value":"à","confidence":0.94917387,"rangeStart":19,"rangeEnd":20,"time":{"start":0.75,"end":0.8147715}},{"value":"paris","confidence":1.0,"rangeStart":21,"rangeEnd":26,"time":{"start":0.8147715,"end":1.68}}]],"intent":{"intentName":"bundle:searchWeatherForecast","confidenceScore":0.9450033},"slots":[{"rawValue":"paris","value":{"kind":"Custom","value":"Paris"},"range":{"start":21,"end":26},"entity":"locality_fr","slotName":"forecast_locality","confidenceScore":1.0}]}')
        time.sleep(0.5)

    subscribe_intent_callback.assert_called_once()
