Tutorial
========

The lifecycle of a script using ``hermes-python`` has the following steps :

* Initiating a connection to the MQTT broker
* Registering callback functions to handle incoming intent parsed by the snips platform
* Listening to incoming intents
* Closing the connection

Let's quickly dive into an example :

Let's write an app for a Weather Assistant !
This code implies that you created a weather assistant using the `Snips Console <https://console.snips.ai/>`_, and that it has a ``searchWeatherForecast`` intent. ::

    from hermes_python.hermes import Hermes

    MQTT_ADDR = "localhost:1883"	# Specify host and port for the MQTT broker

    def subscribe_weather_forecast_callback(hermes, intent_message):	# Defining callback functions to handle an intent that asks for the weather.
        print("Parsed intent : {}".format(intent_message.intent.intent_name))

    with Hermes(MQTT_ADDR) as h: # Initialization of a connection to the MQTT broker
        h.subscribe_intent("searchWeatherForecast", subscribe_weather_forecast_callback) \  # Registering callback functions to handle the searchWeatherForecast intent
             .start()
        # We get out of the with block, which closes and releases the connection.

This app is a bit limited as it only prints out which intent was detected by our assistant.
Let's add more features.

Handling the ``IntentMessage`` object
-------------------------------------

In the previous example, we registered a callback that had this signature::

    subscribe_intent_callback(hermes, intent_message)

The ``intent_message`` object contains information that was extracted from the spoken sentence.

For instance, in the previous code snippet, we extracted the name of the recognized intent with ::

    intent_message.intent.intent_name

We could also retrieve the associated confidence score the NLU engine had when classifying this intent with ::

    intent_message.intent.confidence_score


Extracting slots
^^^^^^^^^^^^^^^^
Here are some best practices when dealing with slots.
The ``IntentMessage`` object has a ``slots`` attribute.

This ``slots`` attributes is a **container** that is empty when the intent message doesn't have slots : ::

    assert len(intent_message.slots) == 0

This container is a dictionary where the key is the name of the slot, and the value is a list of all the slot values for
this slot name.

You can access these values in two ways : ::

    assert len(intent_message.slots.slot1) == 0
    assert len(intent_message.slots["slot1"]) == 0

The slot values are of type ``NluSlot`` which is a deeply nested object, we offer convenience methods to rapidly access
the `slot_value` attribute of the `NluSlot`.

To access the first ``slot_value`` of a slot called ``myslot``, you can use :
::

    intent_message.slots.myslot.first()

You can also access all the ``slot_value`` of a slot called ``myslot`` :
::

    intent_message.slots.myslot.all()


Let's add to our Weather assistant example.

We assume that the ``searchWeatherForecast`` has one slot called ``forecast_location``,
that indicates which location the user would like to know the weather at.

Let's print all the ``forecast_location`` slots :

::

    for slot in intent_message.slots.forecast_location:
        name = slot.slot_name
        confidence = slot.confidence_score
        print("For slot : {}, the confidence is : {}".format(name, confidence))


The *dot* notation was used, but we can also use the dictionary notation :

::

    for slot in intent_message.slots.forecast_location:
        name = slot["slot_name"]
        print(name)

Some convenience methods are available to easily retrieve slot values :

*Retrieving the first slot value for a given slot name*

::

    slot_value = intent_message.slots.forecast_location.first()


*Retrieving all slot values for a given slot name*

::

    slot_values = intent_message.slots.forecast_location.all()

Coming back to our example, we can now have the app print the ``forecast_location`` slot value back to the user :

::

    def subscribe_weather_forecast_callback(hermes, intent_message):
        slot_value = intent_message.slots.forecast_location.first().value
        print("The slot was : {}".format(slot_value)


Managing sessions
-----------------

Snips platform includes support for conversations with back and forth communication between the Dialogue Manager and the client code.
For a conversation, the dialogue manager creates **sessions**.

Starting a session
^^^^^^^^^^^^^^^^^^

A session can be started in two manners :

* with a notification
* with an action



Ending a session
^^^^^^^^^^^^^^^^


Slot Filling
^^^^^^^^^^^^






Configuring MQTT options
------------------------

The connection to your MQTT broker can be configured with the ``hermes_python.ffi.utils.MqttOptions`` class.

The ``Hermes`` client uses the options specified in the ``MqttOptions`` class when establishing the connection to the MQTT broker.

Here is a code example :

::

    from hermes_python.hermes import Hermes
    from hermes_python.ffi.utils import MqttOptions

    mqtt_opts = MqttOptions()

    def simple_intent_callback(hermes, intent_message):
        print("I received an intent !")

    with Hermes(mqtt_options=mqtt_opts) as h:
        h.subscribe_intents().loop_forever()



Here are the options you can specify in the MqttOptions class :

* ``broker_address``: The address of the MQTT broker. It should be formatted as ``ip:port``.
* ``username``: Username to use on the broker. Nullable
* ``password``: Password to use on the broker. Nullable
* ``tls_hostname``: Hostname to use for the TLS configuration. Nullable, setting a value enables TLS
* ``tls_ca_file``: CA files to use if TLS is enabled. Nullable
* ``tls_ca_path``: CA path to use if TLS is enabled. Nullable
* ``tls_client_key``: Client key to use if TLS is enabled. Nullable
* ``tls_client_cert``: Client cert to use if TLS is enabled. Nullable
* ``tls_disable_root_store``: Boolean indicating if the root store should be disabled if TLS is enabled.

Let's connect to an external MQTT broker that requires a username and a password :

::

    from hermes_python.hermes import Hermes
    from hermes_python.ffi.utils import MqttOptions

    mqtt_opts = MqttOptions(username="user1", password="password", broker_address="my-mqtt-broker.com:18852")

    def simple_intent_callback(hermes, intent_message):
        print("I received an intent !")

    with Hermes(mqtt_options=mqtt_opts) as h:
        h.subscribe_intents().loop_forever()


Configuring Dialogue
--------------------

``hermes-python`` offers the possibility to configure different aspects of the Dialogue system.

Enabling and disabling intents on the fly
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

It is possible to enable and disable intents of your assistant on the fly.
Once an intent is disabled, it will not be recognized by the NLU.

Note that intents in the intent filters of started or continued session will take precedence over intents that are enabled/disabled in the configuration of the Dialogue.

You can disable/enable intents with the following methods :

::
    from hermes_python.ontology.dialogue.session import DialogueConfiguration

    dialogue_conf = DialogueConfiguration() \
                            .disable_intent("intent1")              \
                            .enable_intent("intent2)                \
                            .enable_intents(["intent1", "intent2"]) \
                            .disable_intents(["intent2, "intent1])

    hermes.configure_dialogue(dialogue_conf)
